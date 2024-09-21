use crate::api::framework::api_key::dbo::ApiKeyDbo;
use framework_cqrs_lib::cqrs::infra::daos::mongo_entity_dao::EntityMongoDAO;

pub type MongoApiKeyDAO = EntityMongoDAO<ApiKeyDbo>;