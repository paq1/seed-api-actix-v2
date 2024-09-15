use async_trait::async_trait;

#[async_trait]
pub trait TodosService: Send + Sync {}