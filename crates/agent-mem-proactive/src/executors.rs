//! Task executors for proactive tasks
//!
//! This module provides implementations of TaskExecutor for each proactive task type.

pub mod auto_categorize;
pub mod dedupe_merge;
pub mod generate_summaries;
pub mod health_check;
pub mod index_optimization;
pub mod resource_archival;

// Re-export all executors
pub use auto_categorize::AutoCategorizeExecutor;
pub use dedupe_merge::DedupeMergeExecutor;
pub use generate_summaries::GenerateSummariesExecutor;
pub use health_check::HealthCheckExecutor;
pub use index_optimization::IndexOptimizationExecutor;
pub use resource_archival::ResourceArchivalExecutor;

use crate::error::Result;
use crate::models::{ProactiveTask, TaskExecutionContext, TaskResult};

/// Helper to create a successful task result
macro_rules! ok_result {
    ($task_id:expr, $task_type:expr, $started_at:expr, $processed:expr, $affected:expr) => {
        {
            let mut result = TaskResult::new($task_id, $task_type, $started_at);
            result.completed($processed, $affected);
            Ok(result)
        }
    };
}

pub(crate) use ok_result;
