use crate::core::framework::api_key::service::ApiKeyService;
use crate::models::framework::api_key::commands::CreateApiKeyCommand;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use framework_cqrs_lib::cqrs::infra::helpers::header_value::CanSanitizeHeader;
use framework_cqrs_lib::cqrs::models::errors::Error;
use std::sync::Arc;

#[utoipa::path(
    responses(
    (status = 201, description = "creer une api key pour une application", body = String),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[post("/api-key/create")]
pub async fn create_api_key(
    body: web::Json<CreateApiKeyCommand>,
    _req: HttpRequest,
    api_key_service: web::Data<Arc<dyn ApiKeyService>>,
) -> impl Responder {
    match api_key_service.clone()
        .create_api_key(&body.name)
        .await {
        Ok(a) => HttpResponse::Ok().body(format!("your api key is : {}", a.key)),
        Err(e) => {
            match e {
                Error::Http(e) => {
                    HttpResponse::InternalServerError().body(format!("[{}]error {}", e.code, e.title))
                }
                _ => HttpResponse::InternalServerError().body(format!("unknown error"))
            }
        }
    }
}

#[utoipa::path(
    responses(
    (status = 201, description = "mettre la description ici", body = String),
    ),
    security(
    ("api_key_auth" = [])
    )
)]
#[post("/todos/exemple-api-key")]
pub async fn exemple_api_key(
    req: HttpRequest,
) -> impl Responder {
    let maybe_api_key: Option<String> = req
        .headers()
        .get("X-API-KEY")
        .map(|header_value| {
            header_value.clone()
                .sanitize_header("X-API-KEY".to_string())
                .map(|x| x.1)
                .unwrap_or("invalid_key".to_string())
        });

    match maybe_api_key {
        Some(api_key) => {
            HttpResponse::Ok().body(format!("api key : {api_key}"))
        }
        None => {
            HttpResponse::Ok().body(format!("authentification sans api key"))
        }
    }
}
