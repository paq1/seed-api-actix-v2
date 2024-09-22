use crate::core::framework::api_key::data::ApiKey;
use crate::core::framework::api_key::repository::ApiKeyRepository;
use crate::core::framework::api_key::services::api_key_service::ApiKeyService;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::core::data::Entity;
use framework_cqrs_lib::cqrs::models::errors::{Error, ErrorHttpCustom, ResultErr};
use std::sync::Arc;
use uuid::Uuid;

pub struct ImplApiKeyService {
    pub repo: Arc<dyn ApiKeyRepository>,
}

#[async_trait]
impl ApiKeyService for ImplApiKeyService {
    async fn create_api_key(&self, name: &String) -> ResultErr<ApiKey> {
        match self.repo.clone().fetch_one(name).await {
            Ok(Some(_)) => {
                Err(
                    Error::Http(
                        ErrorHttpCustom::new(
                            "l'api key existe déjà",
                            "00APIKAE",
                            vec![],
                            Some(400)
                        )
                    )
                )
            }
            Ok(_) => {
                self.unsafe_create_api_key(name).await
            }
            Err(e) => Err(e)
        }
    }

    async fn is_authorized(&self, key: &String) -> ResultErr<bool> {
        self.repo
            .fetch_by_key(key).await
            .map(|api_keys| api_keys.len() > 0)
    }
}

impl ImplApiKeyService {
    async fn unsafe_create_api_key(&self, name: &String) -> ResultErr<ApiKey> {
        let next_api_key = self.generate_api_key();

        self
            .repo.clone()
            .insert(&Entity {
                entity_id: name.clone(),
                data: ApiKey::new(name, &next_api_key),
                version: Some(1u32)
            })
            .await
            .map(|_| {
                ApiKey::new(name, &next_api_key)
            })
    }

    fn generate_api_key(&self) -> String {
        // fixme generer une api key de meilleure qualitée
        Uuid::new_v4().to_string()
    }
}
