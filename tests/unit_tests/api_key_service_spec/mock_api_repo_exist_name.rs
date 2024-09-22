use async_trait::async_trait;
use framework_cqrs_lib::cqrs::core::data::Entity;
use framework_cqrs_lib::cqrs::core::repositories::can_fetch_all::CanFetchAll;
use framework_cqrs_lib::cqrs::core::repositories::entities::{ReadOnlyEntityRepo, RepositoryEntity, WriteOnlyEntityRepo};
use framework_cqrs_lib::cqrs::core::repositories::query::Query;
use framework_cqrs_lib::cqrs::core::repositories::CanFetchMany;
use framework_cqrs_lib::cqrs::models::errors::ResultErr;
use seed_api_actix_v2::core::framework::api_key::data::ApiKey;
use seed_api_actix_v2::core::framework::api_key::repository::ApiKeyRepository;


pub struct MockApiKeyRepoWithExistingKey {}

#[async_trait]
impl RepositoryEntity<ApiKey, String> for MockApiKeyRepoWithExistingKey {}

#[async_trait]
impl ReadOnlyEntityRepo<ApiKey, String> for MockApiKeyRepoWithExistingKey {
    async fn fetch_one(&self, id: &String) -> ResultErr<Option<Entity<ApiKey, String>>> {
        Ok(Some(Entity {
            entity_id: "whatever".to_string(),
            data: ApiKey {
                name: "whatever".to_string(),
                key: "pouet".to_string(),
            },
            version: Some(1),
        }))
    }
}

#[async_trait]
impl CanFetchAll<Entity<ApiKey, String>> for MockApiKeyRepoWithExistingKey {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<Entity<ApiKey, String>>> {
        todo!()
    }
}

#[async_trait]
impl CanFetchMany<Entity<ApiKey, String>> for MockApiKeyRepoWithExistingKey {}

#[async_trait]
impl WriteOnlyEntityRepo<ApiKey, String> for MockApiKeyRepoWithExistingKey {
    async fn insert(&self, entity: &Entity<ApiKey, String>) -> ResultErr<String> {
        todo!()
    }

    async fn update(&self, id: &String, entity: &Entity<ApiKey, String>) -> ResultErr<String> {
        todo!()
    }
}

#[async_trait]
impl ApiKeyRepository for MockApiKeyRepoWithExistingKey {}