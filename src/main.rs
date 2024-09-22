use futures::lock::Mutex;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};


use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::todos::routes::read_routes::{fetch_many_events, fetch_one_event};
use api::todos::routes::write_routes::{insert_one_event, update_one_event};

use crate::api::framework::api_key::dao::MongoApiKeyDAO;
use crate::api::framework::api_key::dbo::ApiKeyDbo;
use crate::api::framework::api_key::mongo_repository::MongoApiKeyRepository;

use crate::api::swagger::ApiDoc;
use crate::api::todos::routes::exemple_wit_api_key_routes::{create_api_key, exemple_api_key};
use crate::api::todos::routes::read_routes::{fetch_events_events, fetch_one_event_event};
use crate::api::todos::routes::write_routes::disable_one_event;
use crate::api::todos::todos_component::TodosComponent;
use crate::core::framework::api_key::services::api_key_service::ApiKeyService;
use crate::core::framework::api_key::services::impl_api_key_service::ImplApiKeyService;
use framework_cqrs_lib::cqrs::infra::authentication::AuthenticationComponent;


use framework_cqrs_lib::cqrs::infra::repositories::mongo_entity_repository::MongoEntityRepository;
use framework_cqrs_lib::cqrs::models::errors::StandardHttpError;
use log::info;
use crate::api::framework::api_key::component::ApiKeyComponent;

mod core;
mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("lancement du server");

    let authentication_component = Arc::new(AuthenticationComponent::new().unwrap());
    let api_key_component = Arc::new(ApiKeyComponent::new(
        "seedv2todos", "todo"
    ).await);

    // todos aggregat
    let todo_component = TodosComponent::new(&authentication_component.clone()).await;

    let openapi = ApiDoc::openapi();
    let api_address = std::env::var("API_ADDRESS").unwrap();
    let api_port = std::env::var("API_PORT").unwrap().parse::<u16>().unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .supports_credentials();

        let standard_http_error = StandardHttpError::new();

        App::new()
            .wrap(cors)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone(),
            ))
            .app_data(web::Data::new(standard_http_error))
            .app_data(web::Data::new(authentication_component.jwt_token_decoder_service.clone()))
            // todos services
            .app_data(web::Data::new(Arc::clone(&todo_component.engine)))
            .app_data(
                web::Data::new(Arc::clone(&todo_component.store))
            )
            .app_data(
                web::Data::new(Arc::clone(&todo_component.journal))
            )
            .app_data(
                web::Data::new(Arc::clone(&todo_component.service))
            )
            .app_data(
                web::Data::new(api_key_component.service.clone())
            )
            .service(fetch_one_event)
            .service(fetch_one_event_event)
            .service(fetch_many_events)
            .service(fetch_events_events)
            .service(insert_one_event)
            .service(update_one_event)
            .service(disable_one_event)
            .service(exemple_api_key)
            .service(create_api_key)
    })
        .workers(2)
        .bind((api_address.clone(), api_port.clone()))?
        .run()
        .await
}