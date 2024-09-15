use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::Modify;

use crate::api::todos::routes::read_routes::__path_fetch_events_events;
use crate::api::todos::routes::read_routes::__path_fetch_many_events;
use crate::api::todos::routes::read_routes::__path_fetch_one_event;
use crate::api::todos::routes::write_routes::__path_disable_one_event;
use crate::api::todos::routes::write_routes::__path_insert_one_event;
use crate::api::todos::routes::write_routes::__path_update_one_event;
use crate::models::todos::commands::*;
use crate::models::todos::views::*;
use framework_cqrs_lib::cqrs::core::repositories::query::{InfoPaged, Page};
use framework_cqrs_lib::cqrs::models::jsonapi::ManyView;
use framework_cqrs_lib::cqrs::models::views::command_handler_view::ApiView;
use framework_cqrs_lib::cqrs::models::views::DataWrapperView;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        fetch_many_events,
        fetch_one_event,
        insert_one_event,
        update_one_event,
        disable_one_event,
        fetch_events_events,
    ),
    components(
        schemas(
            CreateTodoCommand,
            TodoCreatedView,
            ManyView < TodoViewState >,
            CreateTodoCommand,
            UpdateTodoCommand,
            DisableTodoCommand,
            DataWrapperView < ApiView < TodosViewEvent > >,
            InfoPaged,
            Page
        )
    ),
    modifiers(& SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // let c = CreateEventCommand {}

        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build()
            ),
        )
    }
}