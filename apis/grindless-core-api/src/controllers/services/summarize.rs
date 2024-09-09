use std::sync::Arc;

use crate::{
    services::summary::{
        get_summarize_prompt, SummarizePromptKind, DEFAULT_SUMMARIZE_MAX_INPUT_TOKENS,
        DEFAULT_SUMMARIZE_MAX_OUTPUT_TOKENS, DEFAULT_SUMMARIZE_PRICE_PER_1000_TOKENS_IN_CREDITS,
        SUMMARIZE_SYSTEM_PROMPT,
    },
    services::{CREDIT_PRICE, DEFAULT_MODEL, TOKEN_WEIGHT},
    state::APIStateWrapper,
    util::get_claims_from_header,
};
use actix_web::{web, HttpRequest, Responder, Result};
use grindless_core::{
    entities::account_ops::{AccountMutationCore, AccountQueryCore},
    response::{
        errors::Errors,
        response::{
            build_err, build_ok, ResponseBuilderError, ResponseBuilderOk, ResponseObjectError,
        },
    },
};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizeForm {
    pub kind: Option<String>,
    pub text: Option<String>,
}

pub async fn summarize_controller(
    req: HttpRequest,
    state: APIStateWrapper,
    form: web::Form<SummarizeForm>,
) -> Result<impl Responder> {
    let summary_kind = match &form.kind {
        Some(k) => k,
        None => {
            return Ok(build_err(ResponseBuilderError {
                message: "No kind provided".to_string(),
                errors: vec![],
            }))
        }
    };

    let to_summarize = match &form.text {
        Some(t) => t,
        None => {
            return Ok(build_err(ResponseBuilderError {
                message: "No text provided".to_string(),
                errors: vec![],
            }))
        }
    };

    // 131072 characters
    if to_summarize.len() > DEFAULT_SUMMARIZE_MAX_INPUT_TOKENS * TOKEN_WEIGHT {
        return Ok(build_err(ResponseBuilderError {
            message: "Text too long".to_string(),
            errors: vec![],
        }));
    }

    let kind = match SummarizePromptKind::from_string(&summary_kind) {
        Some(k) => k,
        None => {
            return Ok(build_err(ResponseBuilderError {
                message: "Invalid kind".to_string(),
                errors: vec![],
            }))
        }
    };

    let claims = match get_claims_from_header(
        req.headers().get("Authorization"),
        state.pems.tokens_public.clone(),
    ) {
        Ok(c) => c,
        Err(e) => {
            return Ok(e);
        }
    };

    let account_and_billing = match AccountQueryCore::get_account_by_id_with_billing(
        &state.databases.postgres_conn,
        claims.sub,
    )
    .await
    {
        Ok(a) => a,
        Err(err) => {
            debug!("Error getting account: {:?}", err);
            return Ok(build_err(ResponseBuilderError {
                message: "Error getting account".to_string(),
                errors: vec![],
            }));
        }
    };

    let (account, billing) = match account_and_billing {
        Some(ab) => ab,
        None => {
            return Ok(build_err(ResponseBuilderError {
                message: "Account not found".to_string(),
                errors: vec![],
            }));
        }
    };

    let mut billing = match billing {
        Some(b) => b,
        None => {
            return Ok(build_err(ResponseBuilderError {
                message: "Billing not found".to_string(),
                errors: vec![],
            }));
        }
    };

    // 4 tokens per word
    debug!("Information for summarize request:");

    // Calculate tokens (converted to float for precise calculation)
    let tokens = (to_summarize.len() as f64) / TOKEN_WEIGHT as f64;
    debug!("Tokens in the text: {}", tokens);

    // Calculate total cost using tokens, rounding up to nearest thousand and multiplying by price
    let total_cost =
        (tokens / 1000.0).ceil() * DEFAULT_SUMMARIZE_PRICE_PER_1000_TOKENS_IN_CREDITS as f64;
    debug!("Total cost (credits): {}", total_cost);

    // Convert the total cost to an integer (e.g., as credits or any other unit)
    let total_cost = total_cost.ceil() as i64;
    debug!("Total cost (ceil, in credits): {}", total_cost);

    if billing.credits < total_cost as i64 {
        return Ok(build_err(ResponseBuilderError {
            message: "Insufficient credits".to_string(),
            errors: vec![],
        }));
    }

    let now = chrono::Utc::now().timestamp();
    billing.credits -= total_cost as i64;
    billing.updated_at = now;

    let billing_credits = billing.credits.clone();
    match AccountMutationCore::update_account_billing_by_account_id(
        &state.databases.postgres_conn,
        account.id,
        billing.clone(),
    )
    .await
    {
        Ok(_) => (),
        Err(err) => {
            error!("Error updating billing: {:?}", err);
            return Ok(build_err(ResponseBuilderError {
                message: "Error updating billing".to_string(),
                errors: vec![],
            }));
        }
    }

    let prompt = get_summarize_prompt(to_summarize.to_owned(), kind);

    debug!("Summarize prompt request: {}", prompt);

    let messages = json!([{"role": "user", "content": prompt}]);

    let request = match anthropic_sdk::Client::new()
        .auth(&state.env.llm.anthropic.api_keys[0])
        .model(DEFAULT_MODEL)
        .stream(false)
        .system(SUMMARIZE_SYSTEM_PROMPT)
        .max_tokens(DEFAULT_SUMMARIZE_MAX_OUTPUT_TOKENS as i32)
        .messages(&messages)
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            error!("Error building request: {:?}", e);
            error!("Error after payment");
            error!("Account ID: {}", account.id);
            error!("Credits Deducted: {}", total_cost);
            error!("Remaining Credits: {}", billing_credits);
            error!("Suggested action: Refund the credits");

            billing.credits += total_cost;
            billing.updated_at = now;

            match AccountMutationCore::update_account_billing_by_account_id(
                &state.databases.postgres_conn,
                account.id,
                billing.clone(),
            )
            .await
            {
                Ok(_) => (),
                Err(err) => {
                    error!("Error updating billing: {:?}", err);
                    return Ok(build_err(ResponseBuilderError {
                        message: format!("Error refunding credits, contact support and give them this codee \"id={};gid={};at={};update_at={};total={}\", error message: {}", account.id, account.google_id, now, account.updated_at, total_cost, e.to_string()),
                        errors: vec![],
                    }));
                }
            }

            return Ok(build_err(ResponseBuilderError {
                message: "Error building request".to_string(),
                errors: vec![ResponseObjectError {
                    error_id: Errors::ServiceUnavailable,
                    message: Some(e.to_string()),
                }],
            }));
        }
    };

    let message = Arc::new(Mutex::new(String::new()));
    if let Err(err) = request
        .execute(|text| {
            let message_clone = message.clone();
            async move {
                *message_clone.lock().await = text.to_string();
            }
        })
        .await
    {
        error!("Error executing request: {:?}", err);
        error!("Error after payment");
        error!("Account ID: {}", account.id);
        error!("Credits Deducted: {}", total_cost);
        error!("Remaining Credits: {}", billing_credits);
        error!("Suggested action: Refund the credits");

        billing.credits += total_cost;
        billing.updated_at = now;

        match AccountMutationCore::update_account_billing_by_account_id(
            &state.databases.postgres_conn,
            account.id,
            billing.clone(),
        )
        .await
        {
            Ok(_) => (),
            Err(err) => {
                error!("Error updating billing: {:?}", err);
                return Ok(build_err(ResponseBuilderError {
                        message: format!("Error refunding credits, contact support and give them this codee \"id={};gid={};at={};update_at={};total={}\", error message: External Service Error", account.id, account.google_id, now, account.updated_at, total_cost),
                        errors: vec![],
                    }));
            }
        }

        return Ok(build_err(ResponseBuilderError {
            message: "Error executing request".to_string(),
            errors: vec![ResponseObjectError {
                error_id: Errors::ServiceUnavailable,
                message: Some("So sorry, appear there are a error with the API.".to_owned()),
            }],
        }));
    }

    let message = message.lock().await.to_string();

    Ok(build_ok(ResponseBuilderOk {
        message: Some("OK".to_string()),
        data: Some(json!({
            "message": message,
            "model": DEFAULT_MODEL,
            "tokens_proccesed": tokens,
            "operation_cost_in_credits": total_cost,
            "operation_cost_in_usd": total_cost as f64 * CREDIT_PRICE,
            "remaining_credits": billing_credits,
        })),
    }))
}
