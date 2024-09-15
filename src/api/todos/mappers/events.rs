use crate::api::todos::todos_dbo::{TodoCreatedDbo, TodoDboEvent, TodoDisabledDbo, TodoUpdatedDbo};
use crate::core::todos::data::events::{TodoCreated, TodoDisabled, TodosEvents, TodoUpdated};
use framework_cqrs_lib::cqrs::core::data::EntityEvent;
use framework_cqrs_lib::cqrs::infra::daos::dbos::EventDBO;

impl From<TodoDboEvent> for TodosEvents {
    fn from(value: TodoDboEvent) -> Self {
        match value {
            TodoDboEvent::Created(event_dbo) =>
                TodosEvents::Created(TodoCreated {
                    by: event_dbo.by,
                    at: event_dbo.at,
                    data: event_dbo.data.into(),
                }),
            TodoDboEvent::Updated(event_dbo) =>
                TodosEvents::Updated(
                    TodoUpdated {
                        by: event_dbo.by,
                        at: event_dbo.at,
                        data: event_dbo.data.into()
                    }
                ),
            TodoDboEvent::Disable(event_dbo) =>
                TodosEvents::Disabled(
                    TodoDisabled {
                        by: event_dbo.by,
                        at: event_dbo.at,
                        reason: event_dbo.reason
                    }
                )
        }
    }
}

pub fn from_todo_event_dbo_to_event(dbo: EventDBO<TodoDboEvent, String>) -> EntityEvent<TodosEvents, String> {
    EntityEvent {
        entity_id: dbo.entity_id,
        data: dbo.data.into(),
        event_id: dbo.event_id,
    }
}

pub fn from_todo_event_to_dbo(dbo: EntityEvent<TodosEvents, String>) -> EventDBO<TodoDboEvent, String> {
    EventDBO {
        id_mongo: None,
        version: None,
        entity_id: dbo.entity_id,
        data: dbo.data.into(),
        event_id: dbo.event_id,
    }
}

impl From<TodosEvents> for TodoDboEvent {
    fn from(value: TodosEvents) -> Self {
        match value {
            TodosEvents::Created(
                TodoCreated {
                    by,
                    at,
                    data
                }
            ) => TodoDboEvent::Created(TodoCreatedDbo { by, at, data: data.into() }),
            TodosEvents::Updated(updated) =>
                TodoDboEvent::Updated(
                    TodoUpdatedDbo {
                        by: updated.by,
                        at: updated.at,
                        data: updated.data.into()
                    }
                ),
            TodosEvents::Disabled(disabled) =>
                TodoDboEvent::Disable(
                    TodoDisabledDbo {
                        by: disabled.by,
                        at: disabled.at,
                        reason: disabled.reason
                    }
                ),
        }
    }
}

