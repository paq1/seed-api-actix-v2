mod build_mock_api_key_repo;


use crate::unit_tests::api_key_service_spec::build_mock_api_key_repo::BuilderMockApiKeyRepo;
use framework_cqrs_lib::cqrs::core::data::Entity;
use framework_cqrs_lib::cqrs::models::errors::Error;
use seed_api_actix_v2::core::framework::api_key::data::ApiKey;
use seed_api_actix_v2::core::framework::api_key::services::api_key_service::ApiKeyService;
use seed_api_actix_v2::core::framework::api_key::services::impl_api_key_service::ImplApiKeyService;
use std::sync::Arc;

#[tokio::test]
pub async fn api_key_service_should_err_when_try_create_api_and_name_already_exist() {

    // given
    let mock_repo = BuilderMockApiKeyRepo::new()
        .with_fetch_one(
            Ok(Some(Entity {
                entity_id: "whatever".to_string(),
                data: ApiKey {
                    name: "whatever".to_string(),
                    key: "pouet".to_string(),
                },
                version: Some(1),
            }))
        )
        .build();
    let service: Arc<dyn ApiKeyService> = Arc::new(ImplApiKeyService {
        repo: Arc::new(mock_repo)
    });

    // when
    let res = service.create_api_key(&"whatever".to_string()).await;

    // then
    match res {
        Err(_) => assert!(true),
        _ => assert!(false)
    }
}

#[tokio::test]
pub async fn api_key_service_should_create_api_key_when_name_not_found() {

    // given
    let mock_repo = BuilderMockApiKeyRepo::new()
        .with_fetch_one(
            Ok(None)
        )
        .with_insert_response(Ok("whatever".to_string()))
        .build();

    let service: Arc<dyn ApiKeyService> = Arc::new(ImplApiKeyService {
        repo: Arc::new(mock_repo)
    });

    // when
    let res = service.create_api_key(&"whatever".to_string()).await;

    // then
    match res {
        Err(_) => assert!(false),
        _ => assert!(true)
    }
}

#[tokio::test]
pub async fn api_key_service_should_authorized_when_name_match() {

    // given
    let mock_repo = BuilderMockApiKeyRepo::new()
        .with_fetch_one_by_key_response(
            Ok(vec![ApiKey { name: "xx".to_string(), key: "xx".to_string() }])
        )
        .build();

    let service: Arc<dyn ApiKeyService> = Arc::new(ImplApiKeyService {
        repo: Arc::new(mock_repo)
    });

    // when
    let res = service.is_authorized(&"whatever".to_string()).await;

    // then
    match res {
        Ok(res) => assert!(res),
        _ => assert!(true)
    }
}

#[tokio::test]
pub async fn api_key_service_should_unauthorized_when_name_match() {

    // given
    let mock_repo = BuilderMockApiKeyRepo::new()
        .with_fetch_one_by_key_response(
            Ok(vec![])
        )
        .build();
    let service: Arc<dyn ApiKeyService> = Arc::new(ImplApiKeyService {
        repo: Arc::new(mock_repo)
    });

    // when
    let res = service.is_authorized(&"whatever".to_string()).await;

    // then
    match res {
        Ok(res) => assert!(!res),
        _ => assert!(true)
    }
}

#[tokio::test]
pub async fn api_key_service_should_error_when_repo_error() {

    // given
    let mock_repo = BuilderMockApiKeyRepo::new()
        .with_fetch_one_by_key_response(
            Err(Error::Simple("whatever".to_string()))
        )
        .build();
    let service: Arc<dyn ApiKeyService> = Arc::new(ImplApiKeyService {
        repo: Arc::new(mock_repo)
    });

    // when
    let res = service.is_authorized(&"whatever".to_string()).await;

    // then
    match res {
        Ok(_) => assert!(false),
        Err(_) => assert!(true)
    }
}