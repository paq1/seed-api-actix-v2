use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use framework_cqrs_lib::cqrs::infra::helpers::header_value::CanSanitizeHeader;
use crate::models::framework::api_key::commands::CreateApiKeyCommand;

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
    _body: web::Json<CreateApiKeyCommand>,
    _req: HttpRequest,
) -> impl Responder {
    // todo creer l'api key
    HttpResponse::Ok().body(format!("not implemented"))
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
