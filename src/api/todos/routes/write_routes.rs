use std::sync::Arc;

use crate::core::todos::data::events::TodosEvents;
use crate::core::todos::data::states::TodosStates;
use crate::models::todos::commands::{CreateTodoCommand, DisableTodoCommand, TodosCommands, UpdateTodoCommand};
use crate::models::todos::views::TodosViewEvent;
use actix_web::{post, put, web, HttpRequest, HttpResponse, Responder};
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::engine::Engine;
use framework_cqrs_lib::cqrs::infra::helpers::http_response::{CanToHttpResponse, HttpKindResponse};
use framework_cqrs_lib::cqrs::infra::mappers::event_api_view::from_entity_event_to_view;
use framework_cqrs_lib::cqrs::infra::token::authenticated::authenticated;
use framework_cqrs_lib::cqrs::infra::token::services::jwt_hmac::JwtHMACTokenService;
use framework_cqrs_lib::cqrs::models::errors::StandardHttpError;
use uuid::Uuid;

#[utoipa::path(
    tag = "todos",
    path = "/todos/commands/create",
    request_body = CreateTodoCommand,
    responses(
    (status = 201, description = "mettre la description ici", body = String),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[post("/commands/create")]
pub async fn insert_one_event(
    body: web::Json<CreateTodoCommand>,
    engine: web::Data<Arc<Engine<TodosStates, TodosCommands, TodosEvents>>>,
) -> impl Responder {
    let ctx = Context::empty();

    let command = TodosCommands::Create(body.into_inner());

    let entity_id = Uuid::new_v4().to_string();

    let event = engine
        .compute(command, entity_id, "create-todo".to_string(), &ctx).await;

    event.map(|(event, _)| {
        from_entity_event_to_view::<TodosEvents, TodosViewEvent>(
            event,
            "todos".to_string(),
            "urn:api:todos:todos".to_string(),
            &ctx,
        )
    })
        .to_http_response_with_error_mapping(HttpKindResponse::Created)
}

#[utoipa::path(
    tag = "todos",
    path = "/todos/{entity_id}/commands/update",
    request_body = UpdateTodoCommand,
    responses(
    (status = 200, description = "fait ca", body = String),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[put("/{entity_id}/commands/update")]
pub async fn update_one_event(
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Json<UpdateTodoCommand>,
    jwt_token_service: web::Data<Arc<JwtHMACTokenService>>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Engine<TodosStates, TodosCommands, TodosEvents>>>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let id = path.into_inner();
            let command = TodosCommands::Update(body.into_inner());

            let event = engine
                .compute(command, id, "update-todo".to_string(), &ctx).await;

            event.map(|(event, _)| {
                from_entity_event_to_view::<TodosEvents, TodosViewEvent>(
                    event,
                    "todos".to_string(),
                    "urn:api:todos:todos".to_string(),
                    &ctx,
                )
            })
                .to_http_response_with_error_mapping(HttpKindResponse::Ok)
        }
        Err(_err) => HttpResponse::Unauthorized().json(&http_error.unauthorized)
    }
}

#[utoipa::path(
    tag = "todos",
    path = "/todos/{entity_id}/commands/disable",
    request_body = DisableTodoCommand,
    responses(
    (status = 200, description = "???", body = String),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[put("/{entity_id}/commands/disable")]
pub async fn disable_one_event(
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Json<DisableTodoCommand>,
    jwt_token_service: web::Data<Arc<JwtHMACTokenService>>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Engine<TodosStates, TodosCommands, TodosEvents>>>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let id = path.into_inner();
            let command = TodosCommands::Disable(body.into_inner());

            let event = engine
                .compute(command, id, "disable-todo".to_string(), &ctx).await;


            event.map(|(event, _)| {
                from_entity_event_to_view::<TodosEvents, TodosViewEvent>(
                    event,
                    "todos".to_string(),
                    "urn:api:todos:todos".to_string(),
                    &ctx,
                )
            })
                .to_http_response_with_error_mapping(HttpKindResponse::Ok)
        }
        Err(_err) => HttpResponse::Unauthorized().json(&http_error.unauthorized)
    }
}

