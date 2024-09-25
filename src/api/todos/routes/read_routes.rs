use std::collections::HashMap;
use std::sync::Arc;

use actix_web::web::Query;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::api::todos::query::{from_todo_query_to_query_repo, TodoQuery};
use crate::core::todos::data::events::TodosEvents;
use crate::core::todos::repositories::CustomTodosRepository;
use crate::models::todos::views::{TodoViewState, TodosViewEvent};
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::repositories::events::RepositoryEvents;
use framework_cqrs_lib::cqrs::core::repositories::filter::{Expr, ExprGeneric, Filter, Operation};
use framework_cqrs_lib::cqrs::core::repositories::query::{Paged, Query as QueryCore};
use framework_cqrs_lib::cqrs::infra::helpers::context::CanDecoreFromHttpRequest;
use framework_cqrs_lib::cqrs::infra::mappers::event_api_view::from_entity_event_to_view;
use framework_cqrs_lib::cqrs::infra::mappers::state_view::{from_states_to_entity_view, from_states_to_view, CanBeManyView};
use framework_cqrs_lib::cqrs::models::errors::StandardHttpError;
use framework_cqrs_lib::cqrs::models::jsonapi::CanBeView;
use framework_cqrs_lib::cqrs::models::views::entities::EntityView;

#[utoipa::path(
    tag = "todos",
    responses(
        (status = 200, description = "fait ca", body = Paged<EntityView<TodoViewState>>)
    ),
    params(
        TodoQuery
    )
)]
#[get("/todos")]
pub async fn fetch_many_events(
    store: web::Data<Arc<dyn CustomTodosRepository>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<TodoQuery>,
    req: HttpRequest,
) -> impl Responder {
    let ctx: Context = Context::empty()
        .decore_with_http_header(&req)
        .clone_with_filter(
            HashMap::from([
                ("page[number]".to_string(), query.number.map(|x| x.to_string()).unwrap_or("0".to_string())),
                ("page[size]".to_string(), query.size.map(|x| x.to_string()).unwrap_or("10".to_string())),
            ])
        );

    match store.fetch_many(
        from_todo_query_to_query_repo(query)
    ).await {
        Ok(items) => {
            let paged_view: Paged<EntityView<TodoViewState>> = items.map(|entity| {
                from_states_to_entity_view(entity, "todos".to_string(), &ctx)
            });

            HttpResponse::Ok().json(paged_view.to_many_view(&ctx, "todos".to_string(), HashMap::from([("todos".to_string(), "todos".to_string())])))
        }
        Err(_) => HttpResponse::InternalServerError().json(&http_error.internal_server_error)
    }
}

#[utoipa::path(
    tag = "todos",
    responses(
        (
        status = 200,
        description = "Get the current state.",
        body = TodosStates
        )
    )
)]
#[get("/todos/{entity_id}")]
pub async fn fetch_one_event(
    path: web::Path<String>,
    store: web::Data<Arc<dyn CustomTodosRepository>>,
    http_error: web::Data<StandardHttpError>,
    req: HttpRequest,
) -> impl Responder {
    let id = path.into_inner();

    let ctx = Context::empty().decore_with_http_header(&req);

    match store.fetch_one(&id).await {
        Ok(Some(entity)) => {
            let view = from_states_to_view(entity, "todos".to_string(), &ctx);

            HttpResponse::Ok().json(view)
        }
        Ok(_) => HttpResponse::NotFound().json(&http_error.not_found),
        Err(_) => HttpResponse::InternalServerError().json(&http_error.internal_server_error)
    }
}

#[utoipa::path(
    tag = "todos",
    responses(
        (
        status = 200,
        description = "Get the all events ",
        body = TodoView
        )
    ),
    params(
        TodoQuery
    )
)]
#[get("/todos/{entity_id}/events")]
pub async fn fetch_events_events(
    path: web::Path<String>,
    journal: web::Data<Arc<dyn RepositoryEvents<TodosEvents, String>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<TodoQuery>,
    req: HttpRequest,
) -> impl Responder {
    let id = path.into_inner();
    let query_core: QueryCore = from_todo_query_to_query_repo(query.clone());

    let ctx: Context = Context::empty()
        .decore_with_http_header(&req)
        .clone_with_filter(
            HashMap::from([
                ("page[number]".to_string(), query.number.map(|x| x.to_string()).unwrap_or("0".to_string())),
                ("page[size]".to_string(), query.size.map(|x| x.to_string()).unwrap_or("10".to_string())),
            ])
        );

    let query_core_with_filter = QueryCore {
        filter: Filter::Expr(
            Expr::ExprStr(
                ExprGeneric::<String> {
                    field: "entity_id".to_string(),
                    operation: Operation::EqualsTo,
                    head: id.to_string(),
                }
            )
        ),
        ..query_core
    };

    match journal.fetch_many(query_core_with_filter).await {
        Ok(items) => {
            let paged_view = items.map(|entity_event| {
                EntityView {
                    r#type: "urn:api:todos:todos".to_string(),
                    id: entity_event.entity_id,
                    attributes: entity_event.data.to_view(),
                    links: None,
                }
            });

            HttpResponse::Ok().json(paged_view.to_many_view(&ctx, "todos".to_string(), HashMap::new()))
        }
        Err(_) => HttpResponse::InternalServerError().json(&http_error.internal_server_error)
    }
}


#[utoipa::path(
    tag = "todos",
    responses(
        (
        status = 200,
        description = "Get event.",
        body = DataWrapperView < EventView < TodoViewEvent >>
        )
    )
)]
#[get("/todos/{entity_id}/events/{event_id}")]
pub async fn fetch_one_event_event(
    path: web::Path<(String, String)>,
    journal: web::Data<Arc<dyn RepositoryEvents<TodosEvents, String>>>,
    http_error: web::Data<StandardHttpError>,
    req: HttpRequest,
) -> impl Responder {
    let (_, event_id) = path.into_inner();

    let ctx = Context::empty()
        .decore_with_http_header(&req);

    match journal.fetch_one(&event_id).await {
        Ok(maybe_event) => {
            match maybe_event {
                Some(event) => {
                    let view = from_entity_event_to_view::<TodosEvents, TodosViewEvent>(
                        event,
                        "todos".to_string(),
                        "urn:api:todos:todos".to_string(),
                        &ctx,
                    );
                    HttpResponse::Ok().json(view)
                }
                None => {
                    HttpResponse::InternalServerError().json(&http_error.not_found)
                }
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(&http_error.internal_server_error)
    }
}
