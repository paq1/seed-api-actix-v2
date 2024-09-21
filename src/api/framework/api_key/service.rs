use std::sync::Arc;
use crate::core::framework::api_key::repository::ApiKeyRepository;
use crate::core::framework::api_key::service::ApiKeyService;

pub struct ApiKeyServiceImpl {
    pub repo: Arc<dyn ApiKeyRepository>,
}

// impl ApiKeyRepository for Mon

impl ApiKeyService for ApiKeyServiceImpl {
    fn get_repo(&self) -> Arc<dyn ApiKeyRepository> {
        self.repo.clone()
    }
}