// Declare submodules
#[path = "task.models.rs"]
pub mod task_models;
#[path = "task.dto.rs"]
pub mod task_dto;
#[path = "task.repository.rs"]
pub mod task_repository;
#[path = "task.handlers.rs"]
pub mod task_handlers;
#[path = "task.service.rs"]
pub mod task_service;

// Re-export public items
pub use task_models::{Task, TaskStatus, TaskPriority};
pub use task_dto::{CreateTaskRequest, UpdateTaskRequest, UpdateTaskStatusRequest, PaginatedResponse};
pub use task_repository::{TaskRepository, TaskFilters};
pub use task_handlers::{get_tasks, get_task, create_task, update_task, delete_task, update_task_status, task_stream};
pub use task_service::TaskService;
