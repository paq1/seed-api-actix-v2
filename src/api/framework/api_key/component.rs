use std::sync::Arc;
use framework_cqrs_lib::cqrs::infra::repositories::mongo_entity_repository::MongoEntityRepository;
use futures::lock::Mutex;
use crate::api::framework::api_key::dao::MongoApiKeyDAO;
use crate::api::framework::api_key::dbo::ApiKeyDbo;
use crate::api::framework::api_key::mongo_repository::MongoApiKeyRepository;
use crate::core::framework::api_key::services::api_key_service::ApiKeyService;
use crate::core::framework::api_key::services::impl_api_key_service::ImplApiKeyService;

pub struct ApiKeyComponent {
    pub service: Arc<dyn ApiKeyService>
}

impl ApiKeyComponent {
    pub async fn new(dbname: &str, aggregat_name: &str) -> Self {

        let api_key_dao = Arc::new(
            Mutex::new(
                MongoApiKeyDAO::new(dbname, &format!("{aggregat_name}_api_key")).await
            )
        );
        let api_key_repository: Arc<MongoEntityRepository<ApiKeyDbo>> = Arc::new(
            MongoApiKeyRepository {
                dao: api_key_dao.clone()
            }
        );
        let api_key_service: Arc<dyn ApiKeyService> = Arc::new(ImplApiKeyService { repo: api_key_repository.clone() });

        Self {
            service: api_key_service
        }
    }
}