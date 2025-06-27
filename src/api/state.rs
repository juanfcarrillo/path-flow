use core_flow::flow::flow_manager::FlowManager;
use crate::api::repository::MemoryConversationRepository;

pub struct AppState {
    pub flow_manager: FlowManager,
    pub memory_conversation_repository: MemoryConversationRepository,
}
