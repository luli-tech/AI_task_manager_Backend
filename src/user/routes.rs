// Declare submodules
#[path = "user.models.rs"]
pub mod user_models;
#[path = "user.dto.rs"]
pub mod user_dto;
#[path = "user.repository.rs"]
pub mod user_repository;
#[path = "user.handlers.rs"]
pub mod user_handlers;
#[path = "user.service.rs"]
pub mod user_service;

// Re-export public items
pub use user_models::{User, UserResponse};
pub use user_dto::{UpdateProfileRequest, UserStatsResponse};
pub use user_repository::UserRepository;
pub use user_handlers::{get_current_user, update_current_user, get_user_stats};
pub use user_service::UserService;
