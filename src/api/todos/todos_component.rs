use crate::api::todos::todos_dbo::{TodoDboEvent, TodoDboState};
use crate::api::todos::todos_event_mongo_repository::TodosEventMongoRepository;
use crate::api::todos::todos_mongo_dao::{EventsEventMongoDAO, EventsMongoDAO};
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::api::todos::services::TodosServiceImpl;
use crate::core::todos::command_handler::create_handler::TodoCreateHandler;
use crate::core::todos::command_handler::disable_handler::TodoDisableHandler;
use crate::core::todos::command_handler::update_handler::TodoUpdateHandler;
use crate::core::todos::data::events::TodosEvents;
use crate::core::todos::data::states::TodosStates;
use crate::core::todos::reducer::TodosReducer;
use crate::core::todos::repositories::CustomTodosRepository;
use crate::core::todos::services::TodosService;
use crate::models::todos::commands::TodosCommands;
use framework_cqrs_lib::cqrs::core::daos::DAO;
use framework_cqrs_lib::cqrs::core::event_sourcing::engine::Engine;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandler;
use framework_cqrs_lib::cqrs::core::repositories::events::RepositoryEvents;
use framework_cqrs_lib::cqrs::infra::authentication::AuthenticationComponent;
use framework_cqrs_lib::cqrs::infra::daos::dbos::{EntityDBO, EventDBO};
use futures::lock::Mutex;
use std::sync::Arc;

pub struct TodosComponent {
    pub store: Arc<dyn CustomTodosRepository>,
    pub journal: Arc<dyn RepositoryEvents<TodosEvents, String>>,
    pub service: Arc<dyn TodosService>,
    pub engine: Arc<Engine<TodosStates, TodosCommands, TodosEvents>>,
}

impl TodosComponent {
    pub async fn new(_authentication_component: &Arc<AuthenticationComponent>) -> Self {
        let dbname = "seedv2todos";

        let dao_store: Arc<Mutex<dyn DAO<EntityDBO<TodoDboState, String>, String>>> =
            Arc::new(Mutex::new(EventsMongoDAO::new(dbname, "todos_store_actix").await));
        let dao_journal: Arc<Mutex<dyn DAO<EventDBO<TodoDboEvent, String>, String>>> =
            Arc::new(Mutex::new(EventsEventMongoDAO::new(dbname, "todos_journal_actix").await));

        // repo
        let store = Arc::new(
            TodosMongoRepository {
                dao: Arc::clone(&dao_store)
            }
        );
        let journal: Arc<dyn RepositoryEvents<TodosEvents, String>> = Arc::new(
            TodosEventMongoRepository {
                dao: Arc::clone(&dao_journal)
            }
        );

        // services
        let service: Arc<dyn TodosService> = Arc::new(
            TodosServiceImpl {}
        );

        let engine: Arc<Engine<TodosStates, TodosCommands, TodosEvents>> = Arc::new(Engine {
            handlers: vec![
                CommandHandler::Create(
                    Box::new(
                        TodoCreateHandler {}
                    )
                ),
                CommandHandler::Update(Box::new(TodoUpdateHandler {})),
                CommandHandler::Update(Box::new(TodoDisableHandler {})),
            ],
            reducer: TodosReducer::new().underlying,
            store: store.clone(),
            journal: journal.clone(),
        });

        Self {
            store,
            journal,
            service,
            engine,
        }
    }
}