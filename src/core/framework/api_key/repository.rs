use async_trait::async_trait;
use crate::core::framework::api_key::data::ApiKey;
use framework_cqrs_lib::cqrs::core::repositories::entities::RepositoryEntity;

#[async_trait]
pub trait ApiKeyRepository: RepositoryEntity<ApiKey, String> {}
