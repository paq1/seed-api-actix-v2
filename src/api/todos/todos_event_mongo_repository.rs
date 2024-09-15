use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;

use crate::api::todos::todos_dbo::TodoDboEvent;
use crate::api::todos::mappers::events::{from_todo_event_dbo_to_event, from_todo_event_to_dbo};
use crate::core::todos::data::events::TodosEvents;
use framework_cqrs_lib::cqrs::core::daos::DAO;
use framework_cqrs_lib::cqrs::core::data::EntityEvent;
use framework_cqrs_lib::cqrs::core::repositories::can_fetch_all::CanFetchAll;
use framework_cqrs_lib::cqrs::core::repositories::events::{ReadOnlyEventRepo, RepositoryEvents, WriteOnlyEventRepo};
use framework_cqrs_lib::cqrs::core::repositories::query::Query;
use framework_cqrs_lib::cqrs::core::repositories::CanFetchMany;
use framework_cqrs_lib::cqrs::infra::daos::dbos::EventDBO;
use framework_cqrs_lib::cqrs::models::errors::ResultErr;

pub struct TodosEventMongoRepository {
    pub dao: Arc<Mutex<dyn DAO<EventDBO<TodoDboEvent, String>, String>>>,
}

#[async_trait]
impl RepositoryEvents<TodosEvents, String> for TodosEventMongoRepository {}

#[async_trait]
impl CanFetchAll<EntityEvent<TodosEvents, String>> for TodosEventMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<EntityEvent<TodosEvents, String>>> {
        self.dao
            .lock().await
            .fetch_all(query)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(|dbo| from_todo_event_dbo_to_event(dbo))
                    .collect()
            })
    }
}

#[async_trait]
impl CanFetchMany<EntityEvent<TodosEvents, String>> for TodosEventMongoRepository {}

#[async_trait]
impl ReadOnlyEventRepo<TodosEvents, String> for TodosEventMongoRepository {
    async fn fetch_one(&self, event_id: &String) -> ResultErr<Option<EntityEvent<TodosEvents, String>>> {
        self.dao.lock().await.fetch_one(event_id).await.map(|maybevent| {
            maybevent.map(|event_dbo| {
                from_todo_event_dbo_to_event(event_dbo)
            })
        })
    }
}

#[async_trait]
impl WriteOnlyEventRepo<TodosEvents, String> for TodosEventMongoRepository {
    async fn insert(&self, entity_event: &EntityEvent<TodosEvents, String>) -> ResultErr<String> {
        let dao: EventDBO<TodoDboEvent, String> = from_todo_event_to_dbo(entity_event.clone());

        let dao_sanitize_version: EventDBO<TodoDboEvent, String> = EventDBO {
            version: Some(0),
            ..dao.clone()
        };

        self.dao.lock().await.insert(&dao_sanitize_version, &dao.entity_id).await
    }
}