//! Dedupe-merge executor
//!
//! Detects and merges duplicate or similar memory items.

use async_trait::async_trait;
use chrono::Utc;
use tracing::info;

use crate::error::{ProactiveError, Result};
use crate::models::{
    ProactiveTask, TaskExecutionContext, TaskResult, TaskStatus,
};
use crate::scheduler::TaskExecutor;

/// Dedupe-merge executor
///
/// Detects and merges duplicate memory items:
/// - Uses embedding similarity to find potential duplicates
/// - Applies configurable similarity threshold
/// - Merges duplicates using configurable strategy (keep newest, keep oldest, manual)
pub struct DedupeMergeExecutor {
    /// Similarity threshold (0.0-1.0)
    similarity_threshold: f32,
    /// Maximum items to process per run
    batch_size: u32,
    /// Merge strategy
    strategy: MergeStrategy,
}

#[derive(Debug, Clone)]
enum MergeStrategy {
    /// Keep the newest item
    KeepNewest,
    /// Keep the oldest item
    KeepOldest,
    /// Merge content into existing item
    MergeContent,
}

impl DedupeMergeExecutor {
    /// Create a new dedupe-merge executor with default settings
    pub fn new() -> Self {
        Self {
            similarity_threshold: 0.9,
            batch_size: 1000,
            strategy: MergeStrategy::KeepNewest,
        }
    }

    /// Create with custom similarity threshold
    pub fn with_threshold(threshold: f32) -> Self {
        Self {
            similarity_threshold: threshold,
            batch_size: 1000,
            strategy: MergeStrategy::KeepNewest,
        }
    }

    /// Set merge strategy
    pub fn with_strategy(mut self, strategy: &str) -> Self {
        self.strategy = match strategy {
            "keep_newest" => MergeStrategy::KeepNewest,
            "keep_oldest" => MergeStrategy::KeepOldest,
            "merge_content" => MergeStrategy::MergeContent,
            _ => MergeStrategy::KeepNewest,
        };
        self
    }

    /// Execute deduplication and merging
    async fn perform_dedupe(&self, context: &TaskExecutionContext) -> Result<TaskResult> {
        let started_at = Utc::now();
        let task_id = format!("dedupe-merge-{}", started_at.timestamp());

        info!(
            "Starting dedupe-merge (threshold: {}, batch_size: {})...",
            self.similarity_threshold, self.batch_size
        );

        // TODO: Integration with agent-mem-extraction
        //
        // Implementation plan:
        // 1. Query all memory items (or recent items for efficiency)
        // 2. Generate embeddings for all items
        // 3. Use vector similarity search to find duplicates
        // 4. Group similar items together
        // 5. Apply merge strategy to each group
        // 6. Update database with merged results

        // Placeholder implementation - returns mock results
        let items_scanned = 0u64;
        let duplicates_found = 0u64;
        let items_merged = 0u64;

        let mut result = TaskResult::new(task_id, ProactiveTask::DedupeMerge, started_at);
        result.completed(items_scanned, items_merged);

        info!(
            "Dedupe-merge completed: {} duplicates found, {} items merged",
            duplicates_found, items_merged
        );

        Ok(result)
    }
}

impl Default for DedupeMergeExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TaskExecutor for DedupeMergeExecutor {
    fn task_type(&self) -> ProactiveTask {
        ProactiveTask::DedupeMerge
    }

    async fn execute(&self, context: &TaskExecutionContext) -> Result<TaskResult> {
        // Check for custom similarity threshold in config
        let threshold = context
            .config
            .similarity_threshold
            .unwrap_or(self.similarity_threshold);

        // In dry-run mode, just return success without actual processing
        if context.dry_run {
            let started_at = Utc::now();
            let task_id = format!("dedupe-merge-dry-{}", started_at.timestamp());
            let mut result =
                TaskResult::new(task_id, ProactiveTask::DedupeMerge, started_at);
            result.completed(0, 0);
            return Ok(result);
        }

        // Use config threshold if provided, otherwise use default
        let executor = if (threshold - self.similarity_threshold).abs() > f32::EPSILON {
            Self::with_threshold(threshold)
        } else {
            Self {
                similarity_threshold: self.similarity_threshold,
                batch_size: self.batch_size,
                strategy: MergeStrategy::KeepNewest,
            }
        };

        executor.perform_dedupe(context).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dedupe_merge_executor() {
        let executor = DedupeMergeExecutor::new();
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
    async fn test_dedupe_merge_with_custom_threshold() {
        let executor = DedupeMergeExecutor::with_threshold(0.85);
        let context = TaskExecutionContext {
            user_id: "system".to_string(),
            agent_id: None,
            config: crate::models::TaskConfig::dedupe_merge(0.85),
            max_cpu_percent: 5,
            max_memory_mb: 512,
            dry_run: false,
        };

        let result = executor.execute(&context).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_dedupe_merge_task_type() {
        let executor = DedupeMergeExecutor::new();
        assert_eq!(executor.task_type(), ProactiveTask::DedupeMerge);
    }
}
