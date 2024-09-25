use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use framework_cqrs_lib::cqrs::core::api_key::services::api_key_service::ApiKeyService;
use framework_cqrs_lib::cqrs::infra::helpers::header_value::CanSanitizeHeader;
use framework_cqrs_lib::cqrs::models::api_key::commands::CreateApiKeyCommand;
use framework_cqrs_lib::cqrs::models::errors::Error;
use std::sync::Arc;

#[utoipa::path(
    tag = "api key",
    responses(
    (status = 201, description = "creer une api key pour une application", body = CreateApiKeyCommand),
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
    tag = "todos exemple api key",
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
    api_key_service: web::Data<Arc<dyn ApiKeyService>>,
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
            match api_key_service.is_authorized(&api_key).await {
                Ok(true) => {
                    HttpResponse::Ok().body(format!("félicitation vous etes authoriser ici"))
                }
                Ok(false) => {
                    HttpResponse::Ok().body(format!("vous n'etes pas authorisé ici"))
                }
                _ => {
                    HttpResponse::InternalServerError().body(format!("erreur lors de l'authentification"))
                }
            }
        }
        None => {
            HttpResponse::Ok().body(format!("authentification sans api key"))
        }
    }
}
