// Declare submodules
#[path = "notification.models.rs"]
pub mod notification_models;
#[path = "notification.dto.rs"]
pub mod notification_dto;
#[path = "notification.repository.rs"]
pub mod notification_repository;
#[path = "notification.handlers.rs"]
pub mod notification_handlers;
#[path = "notification.service.rs"]
pub mod notification_service;

// Re-export public items
pub use notification_models::Notification;
pub use notification_dto::UpdateNotificationPreferencesRequest;
pub use notification_repository::NotificationRepository;
pub use notification_handlers::{get_notifications, notification_stream, mark_notification_read, delete_notification, update_notification_preferences};
pub use notification_service::start_notification_service;
