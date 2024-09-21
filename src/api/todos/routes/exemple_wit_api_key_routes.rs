use actix_web::{post, HttpRequest, HttpResponse, Responder};
use framework_cqrs_lib::cqrs::infra::helpers::header_value::CanSanitizeHeader;

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
