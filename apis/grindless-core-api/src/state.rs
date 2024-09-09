use actix_web::web;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::env::Enviroment;

#[derive(Clone)]
pub struct APIStateDatabases {
    pub postgres_conn: Arc<DatabaseConnection>,
}

pub struct Pems {
    pub tokens_public: Vec<u8>,
    pub tokens_private: Vec<u8>,
}

pub struct Anthropic {
    pub client: anthropic_sdk::Client,
}

pub struct OpenAI {
    pub base_url: String,
}
pub struct LLM {
    pub anthropic: Anthropic,
    pub openai: OpenAI,
}

pub struct APIState {
    pub env: Enviroment,
    pub databases: APIStateDatabases,
    pub pems: Arc<Pems>,
    pub limiter: Arc<Mutex<nervio_limiter::limiter::Limiter>>,
    pub llm: LLM,
}

pub type APIStateWrapper = web::Data<APIState>;