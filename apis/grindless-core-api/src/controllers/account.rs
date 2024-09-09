use crate::{state::APIStateWrapper, util::get_claims_from_header};
use actix_web::{HttpRequest, Responder, Result};
use grindless_core::{
    entities::account_ops::AccountQueryCore,
    response::response::{build_err, build_ok, ResponseBuilderError, ResponseBuilderOk},
};
use serde_json::json;

pub async fn get_me_controller(req: HttpRequest, state: APIStateWrapper) -> Result<impl Responder> {
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
        Err(_) => {
            return Ok(build_err(ResponseBuilderError {
                message: "Error getting account".to_string(),
                errors: vec![],
            }))
        }
    };

    let (account, billing)  = match account_and_billing {
        Some(a) => a,
        None => {
            return Ok(build_err(ResponseBuilderError {
                message: "Account not found".to_string(),
                errors: vec![],
            }))
        }
    };

    let billing = match billing {
        Some(b) => b,
        None => {
            return Ok(build_err(ResponseBuilderError {
                message: "Billing not found".to_string(),
                errors: vec![],
            }))
        }
    };

    Ok(build_ok(ResponseBuilderOk {
        message: Some("Account found".to_string()),
        data: Some(json!({
            "account": account,
            "billing": billing
        })),
    }))
}
