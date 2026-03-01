//! AgentMem ProactiveAgent
//!
//! This crate provides a proactive agent for background memory organization,
//! enabling 24/7 automatic maintenance of the memory system.
//!
//! # Features
//!
//! - Timer-based task scheduling (cron expressions and intervals)
//! - Event-driven task triggering
//! - Batch processing during off-peak hours
//! - Automatic memory categorization
//! - Duplicate detection and merging
//! - Category summary generation
//! - Search index optimization
//! - Resource archival
//! - Health monitoring
//! - Resource usage limits (<5% CPU overhead)
//!
//! # Task Types
//!
//! - **AutoCategorize**: Automatically categorize new memory items
//! - **DedupeMerge**: Detect and merge duplicate memories
//! - **GenerateSummaries**: Generate LLM-powered category summaries
//! - **IndexOptimization**: Optimize search indices
//! - **ResourceArchival**: Archive old resources
//! - **HealthCheck**: Monitor system health
//!
//! # Example
//!
//! ```no_run
//! use agent_mem_proactive::{ProactiveConfig, TaskScheduler, ProactiveTask, TaskSchedule};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create scheduler with default config
//! let config = ProactiveConfig::default();
//! let scheduler = TaskScheduler::new(config);
//!
//! // Schedule some tasks
//! scheduler.schedule_task(
//!     ProactiveTask::HealthCheck,
//!     TaskSchedule::interval(5), // Every 5 minutes
//! ).await?;
//!
//! // List scheduled tasks
//! let tasks = scheduler.list_tasks().await;
//! println!("Scheduled {} tasks", tasks.len());
//!
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod models;
pub mod scheduler;

// Re-exports
pub use error::{ProactiveError, Result};
pub use models::{
    ProactiveConfig, ProactiveTask, RetryConfig, ScheduledTask, SchedulerState, SchedulerStats,
    TaskConfig, TaskExecutionContext, TaskResult, TaskSchedule, TaskScheduleConfig,
    TaskStatus, TriggerType, TaskId,
};
pub use scheduler::{TaskExecutor, TaskScheduler};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const LIB_NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(LIB_NAME, "agent-mem-proactive");
    }

    #[test]
    fn test_proactive_config_default() {
        let config = ProactiveConfig::default();
        assert!(config.enabled);
        assert_eq!(config.default_cpu_limit, 5); // <5% CPU overhead
    }

    #[test]
    fn test_proactive_task_display() {
        assert_eq!(
            ProactiveTask::AutoCategorize.to_string(),
            "auto_categorize"
        );
        assert_eq!(ProactiveTask::DedupeMerge.to_string(), "dedupe_merge");
    }
}
