//! TaskScheduler implementation

use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio::sync::{mpsc, RwLock};
use tokio::time::interval;
use tracing::{debug, error, info, warn};

use crate::error::{ProactiveError, Result};
use crate::models::{
    ProactiveConfig, ProactiveTask, ScheduledTask, SchedulerState, SchedulerStateInner,
    SchedulerStats, TaskConfig, TaskExecutionContext, TaskId, TaskResult, TaskSchedule,
    TaskScheduleConfig, TaskStatus, TriggerType,
};

/// Task executor trait - implemented by different task types
#[async_trait]
pub trait TaskExecutor: Send + Sync {
    /// Get the task type this executor handles
    fn task_type(&self) -> ProactiveTask;

    /// Execute the task
    async fn execute(&self, context: &TaskExecutionContext) -> Result<TaskResult>;
}

/// Main scheduler for proactive tasks
pub struct TaskScheduler {
    /// Internal state
    state: Arc<RwLock<SchedulerStateInner>>,
    /// Configuration
    config: ProactiveConfig,
    /// Task executors
    executors: std::collections::HashMap<String, Box<dyn TaskExecutor>>,
    /// Shutdown signal sender
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl TaskScheduler {
    /// Create a new scheduler
    pub fn new(config: ProactiveConfig) -> Self {
        Self {
            state: Arc::new(RwLock::new(SchedulerStateInner::new())),
            config,
            executors: std::collections::HashMap::new(),
            shutdown_tx: None,
        }
    }

    /// Create scheduler with default config
    pub fn with_default_config() -> Self {
        Self::new(ProactiveConfig::default())
    }

    /// Register a task executor
    pub fn register_executor<E: TaskExecutor + 'static>(&mut self, executor: E) {
        let task_type = executor.task_type().to_string();
        info!("Registering executor for task type: {}", task_type);
        self.executors.insert(task_type, Box::new(executor));
    }

    /// Get scheduler state
    pub async fn state(&self) -> SchedulerState {
        self.state.read().await.state.clone()
    }

    /// Get scheduler stats
    pub async fn stats(&self) -> SchedulerStats {
        self.state.read().await.stats.clone()
    }

    /// Get all tasks
    pub async fn list_tasks(&self) -> Vec<ScheduledTask> {
        self.state
            .read()
            .await
            .list_tasks()
            .into_iter()
            .cloned()
            .collect()
    }

    /// Get task by ID
    pub async fn get_task(&self, task_id: &str) -> Option<ScheduledTask> {
        self.state.read().await.get_task(task_id).cloned()
    }

    /// Schedule a new task
    pub async fn schedule_task(
        &self,
        task_type: ProactiveTask,
        schedule: TaskSchedule,
    ) -> Result<TaskId> {
        let task = ScheduledTask::new(task_type.clone(), schedule.schedule_string());

        // Validate task type has executor
        let task_type_str = task_type.to_string();
        if !self.executors.contains_key(&task_type_str) {
            warn!(
                "No executor registered for task type: {}",
                task_type_str
            );
        }

        let task_id = task.id.clone();
        self.state.write().await.add_task(task);

        info!("Scheduled task {} with ID {}", task_type.display_name(), task_id);
        Ok(task_id)
    }

    /// Unschedule a task
    pub async fn unschedule_task(&self, task_id: &str) -> Result<()> {
        let task = self
            .state
            .write()
            .await
            .remove_task(task_id)
            .ok_or_else(|| ProactiveError::TaskNotFound(task_id.to_string()))?;

        info!("Unscheduled task: {}", task.task_type.display_name());
        Ok(())
    }

    /// Enable a task
    pub async fn enable_task(&self, task_id: &str) -> Result<()> {
        let mut state = self.state.write().await;
        let task = state
            .get_task_mut(task_id)
            .ok_or_else(|| ProactiveError::TaskNotFound(task_id.to_string()))?;

        task.enable();
        info!("Enabled task: {}", task.task_type.display_name());
        Ok(())
    }

    /// Disable a task
    pub async fn disable_task(&self, task_id: &str) -> Result<()> {
        let mut state = self.state.write().await;
        let task = state
            .get_task_mut(task_id)
            .ok_or_else(|| ProactiveError::TaskNotFound(task_id.to_string()))?;

        task.disable();
        info!("Disabled task: {}", task.task_type.display_name());
        Ok(())
    }

    /// Run a task immediately
    pub async fn run_task_now(&self, task_id: &str) -> Result<TaskResult> {
        let task = self
            .state
            .read()
            .await
            .get_task(task_id)
            .cloned()
            .ok_or_else(|| ProactiveError::TaskNotFound(task_id.to_string()))?;

        self.execute_task(&task, None).await
    }

    /// Execute a task
    async fn execute_task(
        &self,
        task: &ScheduledTask,
        override_config: Option<TaskConfig>,
    ) -> Result<TaskResult> {
        let task_type_str = task.task_type.to_string();
        let executor = self
            .executors
            .get(&task_type_str)
            .ok_or_else(|| ProactiveError::TaskExecution(format!(
                "No executor for task type: {}",
                task_type_str
            )))?;

        // Update task status to running
        {
            let mut state = self.state.write().await;
            state.update_task_status(&task.id, TaskStatus::Running);
        }

        // Create execution context
        let context = TaskExecutionContext {
            user_id: "system".to_string(), // TODO: Make configurable
            agent_id: None,
            config: override_config.unwrap_or_default(),
            max_cpu_percent: self.config.default_cpu_limit,
            max_memory_mb: self.config.default_memory_limit_mb,
            dry_run: false,
        };

        // Execute task
        let result = executor.execute(&context).await;

        // Update task status and result
        let task_id = task.id.clone();
        let task_type = task.task_type.clone();
        let task_updated_at = task.updated_at;

        // Extract result details before borrowing state
        let (status, last_result, duration_ms, error_msg) = match &result {
            Ok(task_result) => (
                TaskStatus::Completed,
                Some(task_result.clone()),
                task_result.duration_ms,
                None,
            ),
            Err(e) => {
                let mut task_result = TaskResult::new(
                    task_id.clone(),
                    task_type.clone(),
                    task_updated_at,
                );
                task_result.failed(e.to_string());
                (TaskStatus::Failed, Some(task_result.clone()), 0, Some(e.to_string()))
            }
        };

        // Update stats
        let mut stats_update = None;
        if let Some(err) = error_msg {
            stats_update = Some((false, err));
        } else {
            stats_update = Some((true, String::new()));
        }

        {
            let mut state = self.state.write().await;
            if let Some(task) = state.get_task_mut(&task_id) {
                task.status = status;
                task.last_result = last_result;
                task.updated_at = Utc::now();
            }
            // Update stats after releasing task borrow
            if let Some((success, err_msg)) = stats_update {
                if success {
                    state.stats.record_completion(duration_ms);
                } else {
                    state.stats.record_failure(err_msg);
                }
            }
        }

        result
    }

    /// Start the scheduler
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting task scheduler...");

        // Set state to starting
        {
            let mut state = self.state.write().await;
            state.set_state(SchedulerState::Starting);
        }

        // Create shutdown channel
        let (tx, mut rx) = mpsc::channel::<()>(1);
        self.shutdown_tx = Some(tx);

        // Set state to running
        {
            let mut state = self.state.write().await;
            state.set_state(SchedulerState::Running);
        }

        info!("Task scheduler started successfully");

        // Wait for shutdown signal
        tokio::select! {
            _ = rx.recv() => {
                info!("Shutdown signal received");
            }
        }

        Ok(())
    }

    /// Stop the scheduler
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping task scheduler...");

        // Set state to stopping
        {
            let mut state = self.state.write().await;
            state.set_state(SchedulerState::Stopping);
        }

        // Send shutdown signal
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(()).await;
        }

        // Set state to stopped
        {
            let mut state = self.state.write().await;
            state.set_state(SchedulerState::Stopped);
        }

        info!("Task scheduler stopped");
        Ok(())
    }

    /// Add default task schedules based on config
    pub async fn add_default_tasks(&self) -> Result<()> {
        // Auto-categorize - event-driven
        self.schedule_task(
            ProactiveTask::AutoCategorize,
            TaskSchedule::event(),
        ).await?;

        // Dedupe merge - every 5 minutes
        self.schedule_task(
            ProactiveTask::DedupeMerge,
            TaskSchedule::interval(5),
        ).await?;

        // Generate summaries - every hour
        self.schedule_task(
            ProactiveTask::GenerateSummaries,
            TaskSchedule::interval(60),
        ).await?;

        // Health check - every minute
        self.schedule_task(
            ProactiveTask::HealthCheck,
            TaskSchedule::interval(1),
        ).await?;

        Ok(())
    }
}

/// Extension trait for TaskSchedule
trait TaskScheduleExt {
    fn schedule_string(&self) -> String;
    fn cron(&self) -> Option<&str>;
    fn interval_minutes(&self) -> Option<u64>;
    fn trigger_type(&self) -> &TriggerType;
}

impl TaskScheduleExt for TaskSchedule {
    fn schedule_string(&self) -> String {
        match self.trigger_type() {
            TriggerType::Cron => self.cron().unwrap_or("* * * * *").to_string(),
            TriggerType::Interval => format!("interval:{}min", self.interval_minutes().unwrap_or(60)),
            TriggerType::Event => "event".to_string(),
            TriggerType::Manual => "manual".to_string(),
        }
    }

    fn cron(&self) -> Option<&str> {
        self.cron.as_deref()
    }

    fn interval_minutes(&self) -> Option<u64> {
        self.interval_minutes
    }

    fn trigger_type(&self) -> &TriggerType {
        &self.trigger_type
    }
}

/// Extension for ProactiveTask to convert to string
impl std::fmt::Display for ProactiveTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProactiveTask::AutoCategorize => write!(f, "auto_categorize"),
            ProactiveTask::DedupeMerge => write!(f, "dedupe_merge"),
            ProactiveTask::GenerateSummaries => write!(f, "generate_summaries"),
            ProactiveTask::IndexOptimization => write!(f, "index_optimization"),
            ProactiveTask::ResourceArchival => write!(f, "resource_archival"),
            ProactiveTask::HealthCheck => write!(f, "health_check"),
            ProactiveTask::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

impl std::str::FromStr for ProactiveTask {
    type Err = ProactiveError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "auto_categorize" => Ok(ProactiveTask::AutoCategorize),
            "dedupe_merge" => Ok(ProactiveTask::DedupeMerge),
            "generate_summaries" => Ok(ProactiveTask::GenerateSummaries),
            "index_optimization" => Ok(ProactiveTask::IndexOptimization),
            "resource_archival" => Ok(ProactiveTask::ResourceArchival),
            "health_check" => Ok(ProactiveTask::HealthCheck),
            s if s.starts_with("custom:") => Ok(ProactiveTask::Custom(s[7..].to_string())),
            _ => Err(ProactiveError::InvalidConfig(format!(
                "Unknown task type: {}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockExecutor {
        task_type: ProactiveTask,
    }

    #[async_trait]
    impl TaskExecutor for MockExecutor {
        fn task_type(&self) -> ProactiveTask {
            self.task_type.clone()
        }

        async fn execute(&self, _context: &TaskExecutionContext) -> Result<TaskResult> {
            Ok(TaskResult::new(
                "test-1".to_string(),
                self.task_type.clone(),
                Utc::now(),
            ))
        }
    }

    #[tokio::test]
    async fn test_scheduler_creation() {
        let scheduler = TaskScheduler::with_default_config();
        assert_eq!(scheduler.state().await, SchedulerState::Stopped);
    }

    #[tokio::test]
    async fn test_schedule_task() {
        let scheduler = TaskScheduler::with_default_config();
        let task_id = scheduler
            .schedule_task(ProactiveTask::HealthCheck, TaskSchedule::interval(5))
            .await
            .unwrap();

        let task = scheduler.get_task(&task_id).await;
        assert!(task.is_some());
    }

    #[tokio::test]
    async fn test_enable_disable_task() {
        let scheduler = TaskScheduler::with_default_config();
        let task_id = scheduler
            .schedule_task(ProactiveTask::HealthCheck, TaskSchedule::interval(5))
            .await
            .unwrap();

        scheduler.disable_task(&task_id).await.unwrap();
        let task = scheduler.get_task(&task_id).await.unwrap();
        assert!(!task.enabled);

        scheduler.enable_task(&task_id).await.unwrap();
        let task = scheduler.get_task(&task_id).await.unwrap();
        assert!(task.enabled);
    }

    #[tokio::test]
    async fn test_unschedule_task() {
        let scheduler = TaskScheduler::with_default_config();
        let task_id = scheduler
            .schedule_task(ProactiveTask::HealthCheck, TaskSchedule::interval(5))
            .await
            .unwrap();

        scheduler.unschedule_task(&task_id).await.unwrap();
        let task = scheduler.get_task(&task_id).await;
        assert!(task.is_none());
    }
}
