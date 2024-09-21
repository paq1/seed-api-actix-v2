use crate::api::framework::api_key::dbo::ApiKeyDbo;
use framework_cqrs_lib::cqrs::infra::repositories::mongo_entity_repository::{CanTransform, MongoEntityRepository};
use crate::core::framework::api_key::data::ApiKey;
use crate::core::framework::api_key::repository::ApiKeyRepository;

pub type MongoApiKeyRepository = MongoEntityRepository<ApiKeyDbo>;

impl ApiKeyRepository for MongoApiKeyRepository {}

impl CanTransform<ApiKeyDbo> for ApiKey {
    fn transform_into_other(&self) -> ApiKeyDbo {
        ApiKeyDbo {
            name: self.name.clone(),
            key: self.key.clone()
        }
    }
}

impl CanTransform<ApiKey> for ApiKeyDbo {
    fn transform_into_other(&self) -> ApiKey {
        ApiKey {
            name: self.name.clone(),
            key: self.key.clone()
        }
    }
}
