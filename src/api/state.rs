use core_flow::flow::flow_manager::FlowManager;
use implementations::conversation_repository::MongoConversationRepository;

pub struct AppState {
    pub flow_manager: FlowManager,
    pub mongo_conversation_repository: MongoConversationRepository,
}
