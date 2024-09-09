use crate::{
    env::Enviroment, routers::build_api_router, services::DEFAULT_MODEL, state::{APIState, APIStateDatabases, Anthropic, OpenAI, Pems, LLM}
};
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::{error, info};
use nervio_limiter::{limiter::{BucketConfig, LimitEntityType, Limiter}, middleware::actix_web::ActixWebLimiterMiddleware, storage::StorageType};
use sea_orm::DatabaseConnection;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

pub async fn init_server(
    postgres_conn: DatabaseConnection,
    enviroment: Enviroment,
) -> std::io::Result<()> {
    let cert = include_bytes!("../cert.pem");
    let key = include_bytes!("../key.pem");

    let postgres_conn = Arc::new(postgres_conn);

    let limiter = Arc::new(Mutex::new(Limiter::new(
        StorageType::InMemory,
        None,
        None,
        Some(200),
        Duration::from_secs(10),
    )));

    let limiter_middleware = ActixWebLimiterMiddleware {
        limiter: limiter.clone(),
        middleware_bucket_config: BucketConfig {
            name: "global_ip".to_string(),
            limit_by: LimitEntityType::IP,
            max_requests_per_cycle: 200,
            cycle_duration: Duration::from_secs(60),
        },
    };

    let anthropic_client = anthropic_sdk::Client::new()
        .auth(&enviroment.llm.anthropic.api_keys[0].clone())
        .model(DEFAULT_MODEL)
        .stream(false);

    openai::set_base_url(enviroment.llm.openai.base_url.clone());
    openai::set_key(enviroment.llm.openai.api_keys[0].clone());

    let state = web::Data::new(APIState {
        env: enviroment.clone(),
        databases: APIStateDatabases {
            postgres_conn: postgres_conn.clone(),
        },
        pems: Arc::new(Pems {
            tokens_public: cert.to_vec(),
            tokens_private: key.to_vec(),
        }),
        limiter: limiter.clone(),
        llm: LLM {
            anthropic: Anthropic {
                client: anthropic_client,
            },
            openai: OpenAI {
                base_url: enviroment.llm.openai.base_url.clone(),
            }
        },
    });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(limiter_middleware.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(build_api_router())
    })
    .workers(2);

    let port = match enviroment.production {
        true => enviroment.port,
        false => enviroment.dev_port,
    };

        match server.bind((enviroment.host.clone(), port)) {
            Ok(srv) => {
                info!(
                    "Starting server at http://{}:{}",
                    enviroment.host, port
                );
                srv.run().await
            }
            Err(e) => {
                error!("Error starting server: {:?}", e);
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error starting server",
                ))
            }
        }
    
}