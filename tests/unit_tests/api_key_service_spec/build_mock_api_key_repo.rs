use async_trait::async_trait;
use framework_cqrs_lib::cqrs::core::data::Entity;
use framework_cqrs_lib::cqrs::core::repositories::can_fetch_all::CanFetchAll;
use framework_cqrs_lib::cqrs::core::repositories::entities::{ReadOnlyEntityRepo, RepositoryEntity, WriteOnlyEntityRepo};
use framework_cqrs_lib::cqrs::core::repositories::query::Query;
use framework_cqrs_lib::cqrs::core::repositories::CanFetchMany;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};
use seed_api_actix_v2::core::framework::api_key::data::ApiKey;
use seed_api_actix_v2::core::framework::api_key::repository::ApiKeyRepository;

pub struct BuilderMockApiKeyRepo {
    api_key_repo: MockApiKeyRepo,
}

impl BuilderMockApiKeyRepo {
    pub fn new() -> Self {
        Self {
            api_key_repo: MockApiKeyRepo::default()
        }
    }

    pub fn build(&self) -> MockApiKeyRepo {
        self.api_key_repo.clone()
    }

    pub fn with_fetch_one(&self, r: ResultErr<Option<Entity<ApiKey, String>>>) -> Self {
        Self {
            api_key_repo: MockApiKeyRepo {
                fetch_one_response: r,
                ..self.api_key_repo.clone()
            }
        }
    }

    pub fn with_fetch_all_response(&self, r: ResultErr<Vec<Entity<ApiKey, String>>>) -> Self {
        Self {
            api_key_repo: MockApiKeyRepo {
                fetch_all_response: r,
                ..self.api_key_repo.clone()
            }
        }
    }

    pub fn with_insert_response(&self, r: ResultErr<String>) -> Self {
        Self {
            api_key_repo: MockApiKeyRepo {
                insert_response: r,
                ..self.api_key_repo.clone()
            }
        }
    }

    pub fn with_update_response(&self, r: ResultErr<String>) -> Self {
        Self {
            api_key_repo: MockApiKeyRepo {
                update_response: r,
                ..self.api_key_repo.clone()
            }
        }
    }

    pub fn with_fetch_one_by_key_response(&self, r: ResultErr<Vec<ApiKey>>) -> Self {
        Self {
            api_key_repo: MockApiKeyRepo {
                fetch_one_by_key_response: r,
                ..self.api_key_repo.clone()
            }
        }
    }
}


#[derive(Clone)]
pub struct MockApiKeyRepo {
    pub fetch_one_response: ResultErr<Option<Entity<ApiKey, String>>>,
    pub fetch_all_response: ResultErr<Vec<Entity<ApiKey, String>>>,
    pub insert_response: ResultErr<String>,
    pub update_response: ResultErr<String>,
    pub fetch_one_by_key_response: ResultErr<Vec<ApiKey>>,
}

impl Default for MockApiKeyRepo {
    fn default() -> Self {
        fn any_err<T>() -> ResultErr<T> {
            Err(Error::Simple("whatever".to_string()))
        }

        Self {
            fetch_one_response: any_err(),
            fetch_all_response: any_err(),
            insert_response: any_err(),
            update_response: any_err(),
            fetch_one_by_key_response: any_err(),
        }
    }
}

#[async_trait]
impl RepositoryEntity<ApiKey, String> for MockApiKeyRepo {}

#[async_trait]
impl ReadOnlyEntityRepo<ApiKey, String> for MockApiKeyRepo {
    async fn fetch_one(&self, _id: &String) -> ResultErr<Option<Entity<ApiKey, String>>> {
        self.fetch_one_response.clone()
    }
}

#[async_trait]
impl CanFetchAll<Entity<ApiKey, String>> for MockApiKeyRepo {
    async fn fetch_all(&self, _query: Query) -> ResultErr<Vec<Entity<ApiKey, String>>> {
        self.fetch_all_response.clone()
    }
}

#[async_trait]
impl CanFetchMany<Entity<ApiKey, String>> for MockApiKeyRepo {}

#[async_trait]
impl WriteOnlyEntityRepo<ApiKey, String> for MockApiKeyRepo {
    async fn insert(&self, _entity: &Entity<ApiKey, String>) -> ResultErr<String> {
        self.insert_response.clone()
    }

    async fn update(&self, _id: &String, _entity: &Entity<ApiKey, String>) -> ResultErr<String> {
        self.update_response.clone()
    }
}

#[async_trait]
impl ApiKeyRepository for MockApiKeyRepo {
    async fn fetch_by_key(&self, _key: &String) -> ResultErr<Vec<ApiKey>> {
        self.fetch_one_by_key_response.clone()
    }
}