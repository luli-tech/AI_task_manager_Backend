// Declare submodules
#[path = "message.models.rs"]
pub mod message_models;
#[path = "message.dto.rs"]
pub mod message_dto;
#[path = "message.repository.rs"]
pub mod message_repository;
#[path = "message.handlers.rs"]
pub mod message_handlers;
#[path = "message.service.rs"]
pub mod message_service;

// Re-export public items
pub use message_models::{Message, MessageResponse};
pub use message_dto::{SendMessageRequest, ConversationUser};
pub use message_repository::MessageRepository;
pub use message_handlers::{send_message, get_conversation, get_conversations, mark_message_read, message_stream};
pub use message_service::MessageService;
