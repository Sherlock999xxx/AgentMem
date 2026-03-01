//! Auto-categorize executor
//!
//! Automatically categorizes new memory items into appropriate categories.

use async_trait::async_trait;
use chrono::Utc;
use tracing::info;

use crate::error::{ProactiveError, Result};
use crate::models::{
    ProactiveTask, TaskExecutionContext, TaskResult, TaskStatus,
};
use crate::scheduler::TaskExecutor;

/// Auto-categorize executor
///
/// Automatically categorizes uncategorized memory items:
/// - Scans for items without category assignments
/// - Uses LLM to determine appropriate categories
/// - Updates item category assignments
pub struct AutoCategorizeExecutor {
    /// Maximum items to process per run
    batch_size: u32,
    /// Similarity threshold for category matching
    similarity_threshold: f32,
    /// Whether to create new categories if needed
    create_new_categories: bool,
}

impl AutoCategorizeExecutor {
    /// Create a new auto-categorize executor
    pub fn new() -> Self {
        Self {
            batch_size: 100,
            similarity_threshold: 0.8,
            create_new_categories: true,
        }
    }

    /// Create with custom configuration
    pub fn with_config(batch_size: u32, similarity_threshold: f32) -> Self {
        Self {
            batch_size,
            similarity_threshold,
            create_new_categories: true,
        }
    }

    /// Execute auto-categorization
    async fn perform_categorization(&self, context: &TaskExecutionContext) -> Result<TaskResult> {
        let started_at = Utc::now();
        let task_id = format!("auto-categorize-{}", started_at.timestamp());

        info!(
            "Starting auto-categorization (batch_size: {}, threshold: {})...",
            self.batch_size, self.similarity_threshold
        );

        // TODO: Integrate with agent-mem-category
        // TODO: Integrate with agent-mem-extraction for LLM classification
        //
        // Implementation plan:
        // 1. Query for uncategorized memory items
        // 2. For each item, use LLM to determine category
        // 3. Match against existing categories using embeddings
        // 4. Create new category if threshold not met and create_new_categories=true
        // 5. Update item category assignments

        // Placeholder implementation - returns mock results
        let items_processed = 0u64;
        let items_categorized = 0u64;

        let mut result = TaskResult::new(task_id, ProactiveTask::AutoCategorize, started_at);
        result.completed(items_processed, items_categorized);

        info!(
            "Auto-categorization completed: {} items categorized",
            items_categorized
        );

        Ok(result)
    }
}

impl Default for AutoCategorizeExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TaskExecutor for AutoCategorizeExecutor {
    fn task_type(&self) -> ProactiveTask {
        ProactiveTask::AutoCategorize
    }

    async fn execute(&self, context: &TaskExecutionContext) -> Result<TaskResult> {
        // In dry-run mode, just return success without actual processing
        if context.dry_run {
            let started_at = Utc::now();
            let task_id = format!("auto-categorize-dry-{}", started_at.timestamp());
            let mut result =
                TaskResult::new(task_id, ProactiveTask::AutoCategorize, started_at);
            result.completed(0, 0);
            return Ok(result);
        }

        self.perform_categorization(context).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_auto_categorize_executor() {
        let executor = AutoCategorizeExecutor::new();
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
    async fn test_auto_categorize_dry_run() {
        let executor = AutoCategorizeExecutor::new();
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
    fn test_auto_categorize_task_type() {
        let executor = AutoCategorizeExecutor::new();
        assert_eq!(executor.task_type(), ProactiveTask::AutoCategorize);
    }
}
