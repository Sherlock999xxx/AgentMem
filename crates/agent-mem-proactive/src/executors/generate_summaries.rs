//! Generate summaries executor
//!
//! Generates LLM-powered summaries for categories.

use async_trait::async_trait;
use chrono::Utc;
use tracing::info;

use crate::error::{ProactiveError, Result};
use crate::models::{
    ProactiveTask, TaskExecutionContext, TaskResult, TaskStatus,
};
use crate::scheduler::TaskExecutor;

/// Generate summaries executor
///
/// Generates LLM-powered summaries for categories:
/// - Identifies stale categories (not updated recently)
/// - Uses LLM to generate category summaries from contained items
/// - Updates category summary field
pub struct GenerateSummariesExecutor {
    /// Maximum categories to process per run
    batch_size: u32,
    /// Whether to only update stale categories
    stale_only: bool,
    /// Stale threshold in days
    stale_threshold_days: u32,
    /// Maximum items to include in summary context
    max_context_items: u32,
}

impl GenerateSummariesExecutor {
    /// Create a new generate summaries executor
    pub fn new() -> Self {
        Self {
            batch_size: 10,
            stale_only: true,
            stale_threshold_days: 7,
            max_context_items: 50,
        }
    }

    /// Create with custom configuration
    pub fn with_config(batch_size: u32, stale_only: bool, stale_threshold_days: u32) -> Self {
        Self {
            batch_size,
            stale_only,
            stale_threshold_days,
            max_context_items: 50,
        }
    }

    /// Execute summary generation
    async fn perform_summary_generation(
        &self,
        context: &TaskExecutionContext,
    ) -> Result<TaskResult> {
        let started_at = Utc::now();
        let task_id = format!("generate-summaries-{}", started_at.timestamp());

        info!(
            "Starting summary generation (batch_size: {}, stale_only: {})...",
            self.batch_size, self.stale_only
        );

        // TODO: Integration with agent-mem-category
        //
        // Implementation plan:
        // 1. Query categories (all or stale only)
        // 2. For each category:
        //    a. Fetch contained memory items
        //    b. Prepare context (item content, metadata)
        //    c. Call LLM to generate summary
        //    d. Update category with new summary
        // 3. Track statistics

        // Placeholder implementation - returns mock results
        let categories_processed = 0u64;
        let summaries_generated = 0u64;

        let mut result =
            TaskResult::new(task_id, ProactiveTask::GenerateSummaries, started_at);
        result.completed(categories_processed, summaries_generated);

        info!(
            "Summary generation completed: {} categories processed, {} summaries generated",
            categories_processed, summaries_generated
        );

        Ok(result)
    }
}

impl Default for GenerateSummariesExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TaskExecutor for GenerateSummariesExecutor {
    fn task_type(&self) -> ProactiveTask {
        ProactiveTask::GenerateSummaries
    }

    async fn execute(&self, context: &TaskExecutionContext) -> Result<TaskResult> {
        // Check config for stale_only setting
        let stale_only = context
            .config
            .stale_categories_only
            .unwrap_or(self.stale_only);

        // In dry-run mode, just return success without actual processing
        if context.dry_run {
            let started_at = Utc::now();
            let task_id = format!("generate-summaries-dry-{}", started_at.timestamp());
            let mut result =
                TaskResult::new(task_id, ProactiveTask::GenerateSummaries, started_at);
            result.completed(0, 0);
            return Ok(result);
        }

        // Use config stale_only if provided
        let executor = if stale_only != self.stale_only {
            Self::with_config(self.batch_size, stale_only, self.stale_threshold_days)
        } else {
            Self {
                batch_size: self.batch_size,
                stale_only,
                stale_threshold_days: self.stale_threshold_days,
                max_context_items: self.max_context_items,
            }
        };

        executor.perform_summary_generation(context).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_summaries_executor() {
        let executor = GenerateSummariesExecutor::new();
        let context = TaskExecutionContext {
            user_id: "system".to_string(),
            agent_id: None,
            config: Default::default(),
            max_cpu_percent: 5,
            max_memory_mb: 512,
            dry_run: false,
        };

        let result = executor.execute(&context).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_summaries_dry_run() {
        let executor = GenerateSummariesExecutor::new();
        let context = TaskExecutionContext {
            user_id: "system".to_string(),
            agent_id: None,
            config: Default::default(),
            max_cpu_percent: 5,
            max_memory_mb: 512,
            dry_run: true,
        };

        let result = executor.execute(&context).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.status, TaskStatus::Completed);
        assert_eq!(result.items_processed, 0);
    }

    #[test]
    fn test_generate_summaries_task_type() {
        let executor = GenerateSummariesExecutor::new();
        assert_eq!(
            executor.task_type(),
            ProactiveTask::GenerateSummaries
        );
    }
}
