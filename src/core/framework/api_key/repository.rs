use crate::core::framework::api_key::data::ApiKey;
use framework_cqrs_lib::cqrs::core::repositories::entities::RepositoryEntity;

pub trait ApiKeyRepository: RepositoryEntity<ApiKey, String> {}
