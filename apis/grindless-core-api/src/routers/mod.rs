use actix_web::{web, Scope};

use crate::controllers::{account::get_me_controller, oauth::access_google_controller, services::summarize::summarize_controller};

pub fn build_api_router() -> Scope {
    web::scope("/api")
        .service(build_oauth_router())
        .service(build_account_router())
        .service(build_services_router())
}

fn build_oauth_router() -> Scope {
    web::scope("/oauth")
        .route("/access/google", web::post().to(access_google_controller))
}

fn build_account_router() -> Scope {
    web::scope("accounts")
        .route("/@me", web::get().to(get_me_controller))
}

fn build_services_router() -> Scope {
    web::scope("/services")
        // 1 Token = 4 Characters
        // 32768 Tokens = 131072 Characters
        // 65536 Tokens = 262144 Characters
        .app_data(web::FormConfig::default().limit(262_144))
        .route("/summarize", web::post().to(summarize_controller))
}