use crate::api::todos::todos_dbo::{TodoDataDbo, TodoDboState};
use crate::core::todos::data::states::{todo_create, todo_disable};
use crate::core::todos::data::states::TodosStates;
use crate::core::todos::data::todo_data::TodoData;
use framework_cqrs_lib::cqrs::infra::repositories::mongo_entity_repository::{CanTransform, MongoEntityRepository};
use crate::core::todos::repositories::CustomTodosRepository;

pub type MongoTodosRepository = MongoEntityRepository<TodoDboState>;

impl CustomTodosRepository for MongoTodosRepository {

}

impl CanTransform<TodosStates> for TodoDboState {
    fn transform_into_other(&self) -> TodosStates {
        match self {
            TodoDboState::TodoCreatedDbo {kind, data} => TodosStates::TodoCreate(todo_create::TodoCreate {
                kind: kind.clone(),
                data: data.transform_into_other(),
            }),
            TodoDboState::TodoDisableDbo {kind, data, reason } => TodosStates::TodoDisable(todo_disable::TodoDisable {
                kind: kind.clone(),
                reason: reason.clone(),
                data: data.transform_into_other()
            })
        }
    }
}

impl CanTransform<TodoDboState> for TodosStates {
    fn transform_into_other(&self) -> TodoDboState {
        match self {
            TodosStates::TodoCreate(todo_create::TodoCreate {data, kind}) => TodoDboState::TodoCreatedDbo {
                kind: kind.clone(),
                data: data.transform_into_other()
            },
            TodosStates::TodoDisable(todo_disable::TodoDisable {data, reason, kind}) => TodoDboState::TodoDisableDbo {
                kind: kind.clone(),
                reason: reason.clone(),
                data: data.transform_into_other()
            }
        }
    }
}



impl CanTransform<TodoDataDbo> for TodoData {
    fn transform_into_other(&self) -> TodoDataDbo {
        TodoDataDbo {
            name: self.name.clone(),
            description: self.description.clone(),
            date: self.date.clone(),
            url_image: self.url_image.clone(),
            flags: self.flags.clone(),
        }
    }
}

impl CanTransform<TodoData> for TodoDataDbo {
    fn transform_into_other(&self) -> TodoData {
        TodoData {
            name: self.name.clone(),
            description: self.description.clone(),
            date: self.date.clone(),
            url_image: self.url_image.clone(),
            flags: self.flags.clone(),
        }
    }
}

