use crate::api::todos::todos_dbo::TodoDboState;
use crate::core::todos::data::states::todo_disable::TodoDisable;
use crate::core::todos::data::states::todo_create::TodoCreate;
use crate::core::todos::data::states::TodosStates;
use framework_cqrs_lib::cqrs::core::data::Entity;
use framework_cqrs_lib::cqrs::infra::daos::dbos::EntityDBO;

impl From<TodoDboState> for TodosStates {
    fn from(value: TodoDboState) -> Self {
        match value {
            TodoDboState::TodoCreatedDbo { kind, data } =>
                TodosStates::TodoCreate(
                    TodoCreate {
                        kind,
                        data: data.into(),
                    }
                ),
            TodoDboState::TodoDisableDbo { kind, data, reason } =>
                TodosStates::TodoDisable(
                    TodoDisable {
                        kind,
                        data: data.into(),
                        reason,
                    }
                )
        }
    }
}

pub fn from_todo_states_to_todo_states_dbo(entity: Entity<TodosStates, String>) -> EntityDBO<TodoDboState, String> {
    EntityDBO {
        id_mongo: None,
        version: entity.version,
        entity_id: entity.entity_id,
        data: entity.data.into(),
    }
}

impl From<TodosStates> for TodoDboState {
    fn from(value: TodosStates) -> Self {
        match value {
            TodosStates::TodoCreate(data) => {
                TodoDboState::TodoCreatedDbo {
                    kind: data.kind,
                    data: data.data.into(),
                }
            }
            TodosStates::TodoDisable(data) => {
                TodoDboState::TodoDisableDbo {
                    kind: data.kind,
                    data: data.data.into(),
                    reason: data.reason,
                }
            }
        }
    }
}

pub fn from_todo_state_dbo_to_entity(dbo: EntityDBO<TodoDboState, String>) -> Entity<TodosStates, String> {
    Entity {
        entity_id: dbo.entity_id,
        data: dbo.data.into(),
        version: dbo.version,
    }
}
