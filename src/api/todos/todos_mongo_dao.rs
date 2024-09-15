use crate::api::todos::todos_dbo::{TodoDboEvent, TodoDboState};
use framework_cqrs_lib::cqrs::infra::daos::mongo_entity_dao::{EntityMongoDAO, EventMongoDAO};

pub type EventsMongoDAO = EntityMongoDAO<TodoDboState>;
pub type EventsEventMongoDAO = EventMongoDAO<TodoDboEvent>;