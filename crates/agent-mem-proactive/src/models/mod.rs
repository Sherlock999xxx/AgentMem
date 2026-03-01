//! Models for ProactiveAgent

pub mod task;
pub mod scheduler;
pub mod config;

// Re-export from task module
pub use task::{
    ProactiveTask, TaskId, TaskStatus, TaskResult, TaskExecutionContext, TaskConfig,
    ScheduledTask,
};

// Re-export from scheduler module
pub use scheduler::{SchedulerState, SchedulerStateInner, SchedulerStats};

// Re-export from config module
pub use config::{ProactiveConfig, TaskSchedule, TaskScheduleConfig, TriggerType, RetryConfig};
