pub mod mock_api_repo_exist_name;
mod mock_api_repo_without_data;

use crate::unit_tests::api_key_service_spec::mock_api_repo_exist_name::MockApiKeyRepoWithExistingKey;
use crate::unit_tests::api_key_service_spec::mock_api_repo_without_data::MockApiKeyRepoWithoutData;
use framework_cqrs_lib::cqrs::core::repositories::can_fetch_all::CanFetchAll;
use framework_cqrs_lib::cqrs::core::repositories::entities::{ReadOnlyEntityRepo, RepositoryEntity, WriteOnlyEntityRepo};
use framework_cqrs_lib::cqrs::core::repositories::CanFetchMany;
use seed_api_actix_v2::core::framework::api_key::repository::ApiKeyRepository;
use seed_api_actix_v2::core::framework::api_key::services::api_key_service::ApiKeyService;
use seed_api_actix_v2::core::framework::api_key::services::impl_api_key_service::ImplApiKeyService;
use std::sync::Arc;

#[tokio::test]
pub async fn api_key_service_should_err_when_try_create_api_and_name_already_exist() {

    // given

    let service: Arc<dyn ApiKeyService> = Arc::new(ImplApiKeyService {
        repo: Arc::new(MockApiKeyRepoWithExistingKey {})
    });

    // when
    let res = service.create_api_key(&"whatever".to_string()).await;

    // then
    match res {
        Err(e) => assert!(true),
        _ => assert!(false)
    }
}

#[tokio::test]
pub async fn api_key_service_should_create_api_key_when_name_not_found() {

    // given

    let service: Arc<dyn ApiKeyService> = Arc::new(ImplApiKeyService {
        repo: Arc::new(MockApiKeyRepoWithoutData {})
    });

    // when
    let res = service.create_api_key(&"whatever".to_string()).await;

    // then
    match res {
        Err(e) => assert!(false),
        _ => assert!(true)
    }
}