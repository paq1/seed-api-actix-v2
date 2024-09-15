use crate::core::todos::data::states::TodosStates;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::core::repositories::entities::RepositoryEntity;


#[async_trait]
pub trait CustomTodosRepository: RepositoryEntity<TodosStates, String> {
    // todo ajouter les queries specifiques a l'ontology todos
}
