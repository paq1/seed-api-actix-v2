use chrono::{DateTime, Utc};

use crate::core::todos::data::todo_data::TodoData;
use crate::models::todos::views::{TodoCreatedView, TodoDisabledView, TodoUpdatedView, TodosViewEvent};
use framework_cqrs_lib::cqrs::models::jsonapi::CanBeView;

impl CanBeView<TodosViewEvent> for TodosEvents {
    fn to_view(&self) -> TodosViewEvent {
        match self.clone() {
            TodosEvents::Created(c) => TodosViewEvent::Created(TodoCreatedView { data: c.data.into() }), // MKDMKD fixme mettre le into dans un mapper dans la partie api?
            TodosEvents::Updated(u) => TodosViewEvent::Updated(TodoUpdatedView { data: u.data.into() }), // MKDMKD fixme idem
            TodosEvents::Disabled(disabled) => TodosViewEvent::Disabled(TodoDisabledView { reason: disabled.reason }),
        }
    }
}


#[derive(Clone)]
pub enum TodosEvents {
    Created(TodoCreated),
    Updated(TodoUpdated),
    Disabled(TodoDisabled),
}

#[derive(Clone)]
pub struct TodoCreated {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: TodoData,
}

#[derive(Clone)]
pub struct TodoUpdated {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: TodoData,
}

#[derive(Clone)]
pub struct TodoDisabled {
    pub by: String,
    pub at: DateTime<Utc>,
    pub reason: String,
}
