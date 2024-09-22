use crate::core::framework::api_key::data::ApiKey;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::models::errors::ResultErr;


#[async_trait]
pub trait ApiKeyService: Sync + Send {
    async fn create_api_key(&self, name: &String) -> ResultErr<ApiKey>;
    async fn is_authorized(&self, key: &String) -> ResultErr<bool>;
}
