use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;

use crate::api::todos::todos_dbo::TodoDboState;
use crate::api::todos::mappers::states::{from_todo_state_dbo_to_entity, from_todo_states_to_todo_states_dbo};
use crate::core::todos::data::states::TodosStates;
use crate::core::todos::repositories::CustomTodosRepository;
use framework_cqrs_lib::cqrs::core::daos::DAO;
use framework_cqrs_lib::cqrs::core::data::Entity;
use framework_cqrs_lib::cqrs::core::repositories::can_fetch_all::CanFetchAll;
use framework_cqrs_lib::cqrs::core::repositories::entities::{ReadOnlyEntityRepo, RepositoryEntity, WriteOnlyEntityRepo};
use framework_cqrs_lib::cqrs::core::repositories::query::Query;
use framework_cqrs_lib::cqrs::core::repositories::CanFetchMany;
use framework_cqrs_lib::cqrs::infra::daos::dbos::EntityDBO;
use framework_cqrs_lib::cqrs::models::errors::ResultErr;

pub struct TodosMongoRepository {
    pub dao: Arc<Mutex<dyn DAO<EntityDBO<TodoDboState, String>, String>>>,
}

#[async_trait]
impl RepositoryEntity<TodosStates, String> for TodosMongoRepository {}

#[async_trait]
impl CustomTodosRepository for TodosMongoRepository {
    // todo impl les queries custom ici
}

#[async_trait]
impl CanFetchAll<Entity<TodosStates, String>> for TodosMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<Entity<TodosStates, String>>> {
        self.dao
            .lock().await
            .fetch_all(query)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(|dbo| from_todo_state_dbo_to_entity(dbo))
                    .collect()
            })
    }
}

#[async_trait]
impl CanFetchMany<Entity<TodosStates, String>> for TodosMongoRepository {}

#[async_trait]
impl ReadOnlyEntityRepo<TodosStates, String> for TodosMongoRepository {
    async fn fetch_one(&self, id: &String) -> ResultErr<Option<Entity<TodosStates, String>>> {
        self.dao
            .lock().await
            .fetch_one(id).await
            .map(|maybedata| maybedata.map(|dbo| from_todo_state_dbo_to_entity(dbo)))
    }
}

#[async_trait]
impl WriteOnlyEntityRepo<TodosStates, String> for TodosMongoRepository {
    async fn insert(&self, entity: &Entity<TodosStates, String>) -> ResultErr<String> {
        let entity_dbo: EntityDBO<TodoDboState, String> = from_todo_states_to_todo_states_dbo(entity.clone());

        let sanitize_version: EntityDBO<TodoDboState, String> = EntityDBO {
            version: Some(0),
            ..entity_dbo.clone()
        };

        self.dao
            .lock().await
            .insert(&sanitize_version, &entity_dbo.entity_id).await
    }

    async fn update(&self, id: &String, entity: &Entity<TodosStates, String>) -> ResultErr<String> {
        let entity_dbo: EntityDBO<TodoDboState, String> = from_todo_states_to_todo_states_dbo(entity.clone());
        let sanitize_version: EntityDBO<TodoDboState, String> = EntityDBO {
            version: entity_dbo.version.map(|current| current + 1),
            ..entity_dbo
        };

        self.dao
            .lock().await
            .update(id, &sanitize_version).await
    }
}
