//! TaskScheduler implementation

use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use chrono::Utc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::interval;
use tracing::{error, info, warn};

use crate::error::{ProactiveError, Result};
use crate::models::{
    ProactiveConfig, ProactiveTask, ScheduledTask, SchedulerState, SchedulerStateInner,
    SchedulerStats, TaskConfig, TaskExecutionContext, TaskId, TaskResult, TaskSchedule,
    TaskStatus, TriggerType,
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
    /// Task executors (protected by RwLock for thread-safety)
    executors: Arc<RwLock<std::collections::HashMap<String, Box<dyn TaskExecutor>>>>,
    /// Shutdown signal sender
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl TaskScheduler {
    /// Create a new scheduler
    pub fn new(config: ProactiveConfig) -> Self {
        Self {
            state: Arc::new(RwLock::new(SchedulerStateInner::new())),
            config,
            executors: Arc::new(RwLock::new(std::collections::HashMap::new())),
            shutdown_tx: None,
        }
    }

    /// Create scheduler with default config
    pub fn with_default_config() -> Self {
        Self::new(ProactiveConfig::default())
    }

    /// Register a task executor
    pub async fn register_executor<E: TaskExecutor + 'static>(&self, executor: E) {
        let task_type = executor.task_type().to_string();
        info!("Registering executor for task type: {}", task_type);
        self.executors.write().await.insert(task_type, Box::new(executor));
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
        if !self.executors.read().await.contains_key(&task_type_str) {
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

        // Take the executor from the registry (removes it temporarily)
        let executor: Box<dyn TaskExecutor> = {
            let mut executors = self.executors.write().await;
            match executors.remove(&task_type_str) {
                Some(exec) => exec,
                None => return Err(ProactiveError::TaskExecution(format!(
                    "No executor for task type: {}",
                    task_type_str
                ))),
            }
        };

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

        // Execute task (using mutable reference since we own the Box)
        let result = executor.execute(&context).await;

        // Put the executor back in the registry
        {
            let mut executors = self.executors.write().await;
            executors.insert(task_type_str, executor);
        }

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

        // Update task and stats in state
        {
            let mut state = self.state.write().await;
            if let Some(task) = state.get_task_mut(&task_id) {
                task.status = status;
                task.last_result = last_result;
                task.updated_at = Utc::now();
            }
            // Update stats
            if let Some(err_msg) = error_msg {
                state.stats.record_failure(err_msg);
            } else {
                state.stats.record_completion(duration_ms);
            }
        }

        result
    }

    /// Start the scheduler - runs tasks on their schedules
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

        // Main scheduler loop - tick every 30 seconds
        let mut tick_interval = interval(Duration::from_secs(30));

        loop {
            tokio::select! {
                _ = rx.recv() => {
                    info!("Shutdown signal received, stopping scheduler");
                    break;
                }
                _ = tick_interval.tick() => {
                    // Check and execute due tasks
                    self.check_and_execute_tasks().await;
                }
            }
        }

        // Set state to stopped
        {
            let mut state = self.state.write().await;
            state.set_state(SchedulerState::Stopped);
        }

        info!("Task scheduler stopped");
        Ok(())
    }

    /// Check for tasks that are due to run and execute them
    async fn check_and_execute_tasks(&self) {
        let tasks = self.list_tasks().await;

        for task in tasks {
            // Skip disabled tasks
            if !task.enabled {
                continue;
            }

            // Check if task should run
            if self.should_run_task(&task).await {
                info!("Executing task: {} (ID: {})", task.task_type.display_name(), task.id);

                // Execute in background without blocking the scheduler
                let scheduler = Arc::new(self.clone_inner());
                let task_id = task.id.clone();
                let task_type = task.task_type.clone();

                tokio::spawn(async move {
                    if let Err(e) = scheduler.run_task_now(&task_id).await {
                        error!("Task {} failed: {}", task_type.display_name(), e);
                    }
                });
            }
        }
    }

    /// Check if a task should run based on its schedule
    async fn should_run_task(&self, task: &ScheduledTask) -> bool {
        let now = Utc::now();

        // Check last result to determine if task should run
        if let Some(ref last_result) = task.last_result {
            match task.task_type {
                ProactiveTask::AutoCategorize => {
                    // Event-driven, skip interval check
                    return false;
                }
                _ => {
                    // For interval-based tasks, check if enough time has passed
                    let default_interval = task.task_type.default_interval_minutes()
                        .unwrap_or(5);

                    // If task is currently running, don't start another
                    if last_result.status == TaskStatus::Running {
                        return false;
                    }

                    // Check if enough time has passed since last completion
                    let time_since_completion = now - last_result.completed_at;
                    let interval_duration = chrono::TimeDelta::minutes(default_interval as i64);

                    return time_since_completion > interval_duration;
                }
            }
        }

        // No previous result - task has never run
        true
    }

    /// Clone the scheduler for use in spawned tasks
    fn clone_inner(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            config: self.config.clone(),
            executors: self.executors.clone(),
            shutdown_tx: None,
        }
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
