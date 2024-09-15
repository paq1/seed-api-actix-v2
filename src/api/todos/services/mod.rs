use crate::core::todos::services::TodosService;
use async_trait::async_trait;

pub struct TodosServiceImpl {}

#[async_trait]
impl TodosService for TodosServiceImpl {}
