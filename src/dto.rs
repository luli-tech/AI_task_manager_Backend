use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// Auth DTOs
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 255))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user: crate::models::UserResponse,
}

// Task DTOs
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTaskRequest {
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub reminder_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTaskRequest {
    #[validate(length(min = 1, max = 500))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub reminder_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTaskStatusRequest {
    pub status: String,
}

// Notification DTOs
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateNotificationPreferencesRequest {
    pub notification_enabled: bool,
}
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}
