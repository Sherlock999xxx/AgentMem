//! Client data models

use agent_mem_core::MemoryType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to add a new memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMemoryRequest {
    /// Agent ID
    pub agent_id: String,

    /// User ID (optional)
    pub user_id: Option<String>,

    /// Memory content
    pub content: String,

    /// Memory type
    pub memory_type: Option<MemoryType>,

    /// Importance score (0.0 to 1.0)
    pub importance: Option<f32>,

    /// Additional metadata
    pub metadata: Option<HashMap<String, String>>,
}

/// Request to update a memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemoryRequest {
    /// New content (optional)
    pub content: Option<String>,

    /// New importance score (optional)
    pub importance: Option<f32>,
}

/// Batch update request item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateItem {
    /// Memory ID to update
    pub memory_id: String,

    /// User ID
    pub user_id: String,

    /// Update request
    pub update_request: UpdateMemoryRequest,
}

/// Batch delete request item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteItem {
    /// Memory ID to delete
    pub memory_id: String,

    /// User ID
    pub user_id: String,
}

/// Response for memory operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryResponse {
    /// Memory ID
    pub id: String,

    /// Response message
    pub message: String,
}

/// Memory data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Memory ID
    pub id: String,

    /// Agent ID
    pub agent_id: String,

    /// User ID (optional)
    pub user_id: Option<String>,

    /// Memory content
    pub content: String,

    /// Memory type
    pub memory_type: Option<MemoryType>,

    /// Importance score
    pub importance: Option<f32>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Additional metadata
    pub metadata: Option<HashMap<String, String>>,
}

/// Request to search memories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMemoriesRequest {
    /// Search query
    pub query: String,

    /// Agent ID (optional)
    pub agent_id: Option<String>,

    /// User ID (optional)
    pub user_id: Option<String>,

    /// Memory type filter (optional)
    pub memory_type: Option<MemoryType>,

    /// Maximum number of results
    pub limit: Option<usize>,

    /// Similarity threshold
    pub threshold: Option<f32>,
}

/// Search result item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Memory data
    pub memory: Memory,

    /// Similarity score
    pub score: f32,
}

/// Response for search operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMemoriesResponse {
    /// Search results
    pub results: Vec<SearchResult>,

    /// Total number of results
    pub total: usize,
}

/// Request for batch operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAddMemoriesRequest {
    /// List of memory requests
    pub memories: Vec<AddMemoryRequest>,
}

/// Response for batch operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResponse {
    /// Number of successful operations
    pub successful: usize,

    /// Number of failed operations
    pub failed: usize,

    /// Results from successful operations
    pub results: Vec<String>,

    /// Error messages from failed operations
    pub errors: Vec<String>,
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Overall health status
    pub status: String,

    /// Timestamp of the health check
    pub timestamp: DateTime<Utc>,

    /// Service version
    pub version: String,

    /// Individual component health checks
    pub checks: HashMap<String, String>,
}

/// Metrics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsResponse {
    /// Timestamp of metrics collection
    pub timestamp: DateTime<Utc>,

    /// Collected metrics
    pub metrics: HashMap<String, f64>,
}

/// Error response from server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error code
    pub code: String,

    /// Error message
    pub message: String,

    /// Additional error details
    pub details: Option<serde_json::Value>,

    /// Timestamp of the error
    pub timestamp: DateTime<Utc>,
}

/// Shared multi-tenant scope for file-centric surfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeDescriptor {
    /// User ID that owns the operation.
    pub user_id: String,

    /// Agent ID within the user scope.
    pub agent_id: Option<String>,
}

/// Lifecycle state for mounted resources.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceStatus {
    Pending,
    Mounted,
    Failed,
    Archived,
}

/// Lifecycle state for categories.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CategoryStatus {
    Active,
    Archived,
    Deleted,
}

/// Cross-language status model for async and long-running operations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

/// Scheduler lifecycle state for proactive orchestration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

/// File-centric error code baseline for server/client/SDK alignment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlatformErrorCode {
    ValidationError,
    CategoryNotFound,
    ResourceUriConflict,
    MigrationConflict,
    TaskTimeout,
    BackgroundTaskUnavailable,
}

/// Open metadata surface for resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetadataDescriptor {
    /// Optional author or producer.
    pub author: Option<String>,

    /// Tag labels used for routing and grouping.
    pub tags: Vec<String>,

    /// Declared size in bytes, when known.
    pub size_bytes: Option<u64>,

    /// Resource-specific last modification time.
    pub modified_at: Option<DateTime<Utc>>,

    /// Extensible metadata attributes.
    pub attributes: HashMap<String, String>,
}

/// Stable resource DTO for the file-centric public contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDescriptor {
    /// Stable resource identifier.
    pub id: String,

    /// File-like URI for the mounted resource.
    pub uri: String,

    /// MIME type string, for example `text/plain`.
    pub media_type: String,

    /// Lifecycle status of the resource.
    pub status: ResourceStatus,

    /// Multi-tenant ownership scope.
    pub scope: ScopeDescriptor,

    /// Structured metadata.
    pub metadata: ResourceMetadataDescriptor,

    /// Creation timestamp.
    pub created_at: DateTime<Utc>,

    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
}

/// Open metadata surface for categories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryMetadataDescriptor {
    /// Tag labels used for browsing and retrieval hints.
    pub tags: Vec<String>,

    /// Extensible metadata attributes.
    pub attributes: HashMap<String, String>,
}

/// Stable category DTO for the file-centric public contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDescriptor {
    /// Stable category identifier.
    pub id: String,

    /// Hierarchical path, for example `/preferences/communication`.
    pub path: String,

    /// Display name for the category.
    pub name: String,

    /// Parent category identifier, if any.
    pub parent_id: Option<String>,

    /// Child category identifiers.
    pub children_ids: Vec<String>,

    /// Generated or curated summary for the category.
    pub summary: Option<String>,

    /// Count of items assigned to the category.
    pub item_count: u64,

    /// Lifecycle status for the category.
    pub status: CategoryStatus,

    /// Multi-tenant ownership scope.
    pub scope: ScopeDescriptor,

    /// Structured metadata.
    pub metadata: CategoryMetadataDescriptor,

    /// Creation timestamp.
    pub created_at: DateTime<Utc>,

    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
}

/// Extracted entity shape exposed in extraction results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedEntity {
    /// Stable entity identifier.
    pub id: String,

    /// Human-readable entity label.
    pub name: String,

    /// Entity type label.
    pub entity_type: String,

    /// Confidence score in the range `0.0..=1.0`.
    pub confidence: f64,

    /// Extensible attributes.
    pub attributes: HashMap<String, String>,

    /// Optional start offset in the source content.
    pub span_start: Option<usize>,

    /// Optional end offset in the source content.
    pub span_end: Option<usize>,
}

/// Extracted relation shape exposed in extraction results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedRelation {
    /// Stable relation identifier.
    pub id: String,

    /// Source entity identifier.
    pub subject_id: String,

    /// Source entity label.
    pub subject: String,

    /// Relation predicate.
    pub predicate: String,

    /// Target entity identifier.
    pub object_id: String,

    /// Target entity label.
    pub object: String,

    /// Relation type label.
    pub relation_type: String,

    /// Confidence score in the range `0.0..=1.0`.
    pub confidence: f64,

    /// Extensible attributes.
    pub attributes: HashMap<String, String>,
}

/// File-centric extraction request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionRequest {
    /// Resource to extract from.
    pub resource_id: String,

    /// Multi-tenant ownership scope.
    pub scope: ScopeDescriptor,

    /// Optional category hints to bias extraction and placement.
    pub category_hint_paths: Vec<String>,

    /// Whether to persist extracted output to storage.
    pub persist_output: bool,

    /// Whether entities should be returned.
    pub include_entities: bool,

    /// Whether relations should be returned.
    pub include_relations: bool,
}

/// File-centric extraction result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    /// Job identifier for the extraction run.
    pub job_id: String,

    /// Resource that was extracted.
    pub resource_id: String,

    /// Long-running operation status.
    pub status: OperationStatus,

    /// Category paths suggested or applied by the pipeline.
    pub category_paths: Vec<String>,

    /// Memory identifiers persisted from the extraction output.
    pub memory_ids: Vec<String>,

    /// Extracted entities.
    pub entities: Vec<ExtractedEntity>,

    /// Extracted relations.
    pub relations: Vec<ExtractedRelation>,

    /// Non-fatal warnings.
    pub warnings: Vec<String>,

    /// Primary error code when the extraction fails.
    pub error_code: Option<PlatformErrorCode>,

    /// Human-readable error message when the extraction fails.
    pub error_message: Option<String>,

    /// Execution time when completed.
    pub duration_ms: Option<u64>,

    /// Start timestamp.
    pub started_at: DateTime<Utc>,

    /// Completion timestamp when available.
    pub completed_at: Option<DateTime<Utc>>,
}

/// Dry-run or planned migration summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPlan {
    /// Stable plan identifier.
    pub plan_id: String,

    /// Multi-tenant ownership scope.
    pub scope: ScopeDescriptor,

    /// Whether the plan is dry-run only.
    pub dry_run: bool,

    /// Source public surface label.
    pub source_surface: String,

    /// Target public surface label.
    pub target_surface: String,

    /// Number of legacy memories covered by the plan.
    pub legacy_memory_count: u64,

    /// Number of resources expected after migration.
    pub projected_resource_count: u64,

    /// Number of categories expected after migration.
    pub projected_category_count: u64,

    /// Non-fatal warnings discovered during planning.
    pub warnings: Vec<String>,

    /// Plan creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Applied migration result or rollback-capable report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationReport {
    /// Stable migration run identifier.
    pub migration_id: String,

    /// Optional source plan identifier.
    pub plan_id: Option<String>,

    /// Whether the migration ran as dry-run only.
    pub dry_run: bool,

    /// Long-running operation status.
    pub status: OperationStatus,

    /// Number of migrated memory items.
    pub migrated_memories: u64,

    /// Number of mounted or linked resources.
    pub mounted_resources: u64,

    /// Number of created categories.
    pub created_categories: u64,

    /// Structured conflict summaries.
    pub conflicts: Vec<String>,

    /// Non-fatal warnings.
    pub warnings: Vec<String>,

    /// Fatal or per-item errors.
    pub errors: Vec<String>,

    /// Primary error code when the migration fails.
    pub error_code: Option<PlatformErrorCode>,

    /// Whether rollback remains available.
    pub rollback_available: bool,

    /// Start timestamp.
    pub started_at: DateTime<Utc>,

    /// Completion timestamp when available.
    pub completed_at: Option<DateTime<Utc>>,
}

/// Public proactive task surface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProactiveTaskInfo {
    /// Stable task identifier.
    pub id: String,

    /// Built-in or custom proactive task type.
    pub task_type: String,

    /// Long-running operation status.
    pub status: OperationStatus,

    /// Multi-tenant ownership scope.
    pub scope: ScopeDescriptor,

    /// Stable display form of the configured schedule.
    pub schedule: String,

    /// Queued task executions.
    pub pending_runs: u32,

    /// Currently executing runs.
    pub running_count: u32,

    /// Last start time, if any.
    pub last_started_at: Option<DateTime<Utc>>,

    /// Last completion time, if any.
    pub last_completed_at: Option<DateTime<Utc>>,

    /// Last error code, if any.
    pub last_error_code: Option<PlatformErrorCode>,

    /// Last error message, if any.
    pub last_error: Option<String>,
}

/// Public scheduler statistics surface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerStats {
    /// Current scheduler lifecycle state.
    pub state: SchedulerState,

    /// Number of registered tasks.
    pub total_tasks: u64,

    /// Number of tasks currently executing.
    pub running_tasks: u64,

    /// Number of tasks that completed successfully.
    pub completed_tasks: u64,

    /// Number of tasks that failed.
    pub failed_tasks: u64,

    /// Number of tasks that were cancelled.
    pub cancelled_tasks: u64,

    /// Aggregated execution time across all tasks.
    pub total_execution_time_ms: u64,

    /// Last scheduler-level error message.
    pub last_error: Option<String>,

    /// Timestamp of the last stats update.
    pub updated_at: DateTime<Utc>,
}

/// Preview request for mounting a resource onto the file-centric surface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountResourceRequest {
    /// File-like URI to mount.
    pub uri: String,

    /// Optional MIME type hint supplied by the caller.
    pub media_type: Option<String>,

    /// Multi-tenant ownership scope.
    pub scope: ScopeDescriptor,

    /// Optional metadata supplied at mount time.
    pub metadata: Option<ResourceMetadataDescriptor>,
}

/// Request for category-aware search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCategoriesRequest {
    /// Multi-tenant ownership scope.
    pub scope: ScopeDescriptor,

    /// Search query to match against category name and summary.
    pub query: String,

    /// Maximum number of categories to return.
    pub limit: Option<usize>,
}

/// Preview request for planning legacy migration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanMigrationRequest {
    /// Multi-tenant ownership scope.
    pub scope: ScopeDescriptor,

    /// Whether to keep the operation as dry-run only.
    pub dry_run: bool,

    /// Source public surface label.
    pub source_surface: String,

    /// Target public surface label.
    pub target_surface: String,
}

/// Preview request for applying a legacy migration plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyMigrationRequest {
    /// Existing migration plan identifier.
    pub plan_id: String,

    /// Multi-tenant ownership scope.
    pub scope: ScopeDescriptor,
}

/// Preview request for rolling back a migration run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackMigrationRequest {
    /// Existing migration run identifier.
    pub migration_id: String,

    /// Multi-tenant ownership scope.
    pub scope: ScopeDescriptor,
}

/// Preview request for running a proactive task immediately.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunProactiveTaskRequest {
    /// Multi-tenant ownership scope.
    pub scope: ScopeDescriptor,
}

/// Preview request for cancelling a proactive task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelProactiveTaskRequest {
    /// Multi-tenant ownership scope.
    pub scope: ScopeDescriptor,
}

impl AddMemoryRequest {
    /// Create a new memory request
    pub fn new(agent_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
            user_id: None,
            content: content.into(),
            memory_type: None,
            importance: None,
            metadata: None,
        }
    }

    /// Set user ID
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Set memory type
    pub fn with_memory_type(mut self, memory_type: MemoryType) -> Self {
        self.memory_type = Some(memory_type);
        self
    }

    /// Set importance score
    pub fn with_importance(mut self, importance: f32) -> Self {
        self.importance = Some(importance);
        self
    }

    /// Set metadata
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl SearchMemoriesRequest {
    /// Create a new search request
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            agent_id: None,
            user_id: None,
            memory_type: None,
            limit: None,
            threshold: None,
        }
    }

    /// Set agent ID filter
    pub fn with_agent_id(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = Some(agent_id.into());
        self
    }

    /// Set user ID filter
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Set memory type filter
    pub fn with_memory_type(mut self, memory_type: MemoryType) -> Self {
        self.memory_type = Some(memory_type);
        self
    }

    /// Set result limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set similarity threshold
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = Some(threshold);
        self
    }
}

impl MountResourceRequest {
    /// Create a preview mount-resource request.
    pub fn new(uri: impl Into<String>, scope: ScopeDescriptor) -> Self {
        Self {
            uri: uri.into(),
            media_type: None,
            scope,
            metadata: None,
        }
    }

    /// Set an explicit media type hint.
    pub fn with_media_type(mut self, media_type: impl Into<String>) -> Self {
        self.media_type = Some(media_type.into());
        self
    }

    /// Attach structured metadata.
    pub fn with_metadata(mut self, metadata: ResourceMetadataDescriptor) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl SearchCategoriesRequest {
    /// Create a new category-search request.
    pub fn new(scope: ScopeDescriptor, query: impl Into<String>) -> Self {
        Self {
            scope,
            query: query.into(),
            limit: None,
        }
    }

    /// Set the maximum result count.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    const RESOURCE_DESCRIPTOR_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/file-centric-fixtures/resource_descriptor.json"
    ));
    const CATEGORY_DESCRIPTOR_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/file-centric-fixtures/category_descriptor.json"
    ));
    const EXTRACTION_REQUEST_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/file-centric-fixtures/extraction_request.json"
    ));
    const EXTRACTION_RESULT_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/file-centric-fixtures/extraction_result.json"
    ));
    const MIGRATION_PLAN_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/file-centric-fixtures/migration_plan.json"
    ));
    const MIGRATION_REPORT_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/file-centric-fixtures/migration_report.json"
    ));
    const PROACTIVE_TASK_INFO_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/file-centric-fixtures/proactive_task_info.json"
    ));
    const SCHEDULER_STATS_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/file-centric-fixtures/scheduler_stats.json"
    ));
    const ERROR_RESPONSE_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/file-centric-fixtures/error_response.json"
    ));

    fn assert_fixture_roundtrip<T>(fixture: &str)
    where
        T: for<'de> serde::Deserialize<'de> + serde::Serialize,
    {
        let expected: Value = serde_json::from_str(fixture).unwrap();
        let parsed: T = serde_json::from_str(fixture).unwrap();
        let actual = serde_json::to_value(parsed).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_add_memory_request_builder() {
        let request = AddMemoryRequest::new("agent1", "test content")
            .with_user_id("user1")
            .with_memory_type(MemoryType::Episodic)
            .with_importance(0.8);

        assert_eq!(request.agent_id, "agent1");
        assert_eq!(request.content, "test content");
        assert_eq!(request.user_id, Some("user1".to_string()));
        assert_eq!(request.memory_type, Some(MemoryType::Episodic));
        assert_eq!(request.importance, Some(0.8));
    }

    #[test]
    fn test_search_request_builder() {
        let request = SearchMemoriesRequest::new("test query")
            .with_agent_id("agent1")
            .with_limit(10)
            .with_threshold(0.7);

        assert_eq!(request.query, "test query");
        assert_eq!(request.agent_id, Some("agent1".to_string()));
        assert_eq!(request.limit, Some(10));
        assert_eq!(request.threshold, Some(0.7));
    }

    #[test]
    fn test_file_centric_contract_fixtures_roundtrip() {
        assert_fixture_roundtrip::<ResourceDescriptor>(RESOURCE_DESCRIPTOR_FIXTURE);
        assert_fixture_roundtrip::<CategoryDescriptor>(CATEGORY_DESCRIPTOR_FIXTURE);
        assert_fixture_roundtrip::<ExtractionRequest>(EXTRACTION_REQUEST_FIXTURE);
        assert_fixture_roundtrip::<ExtractionResult>(EXTRACTION_RESULT_FIXTURE);
        assert_fixture_roundtrip::<MigrationPlan>(MIGRATION_PLAN_FIXTURE);
        assert_fixture_roundtrip::<MigrationReport>(MIGRATION_REPORT_FIXTURE);
        assert_fixture_roundtrip::<ProactiveTaskInfo>(PROACTIVE_TASK_INFO_FIXTURE);
        assert_fixture_roundtrip::<SchedulerStats>(SCHEDULER_STATS_FIXTURE);
        assert_fixture_roundtrip::<ErrorResponse>(ERROR_RESPONSE_FIXTURE);
    }

    #[test]
    fn test_file_centric_status_and_error_code_serialization() {
        assert_eq!(
            serde_json::to_string(&OperationStatus::Succeeded).unwrap(),
            "\"succeeded\""
        );
        assert_eq!(
            serde_json::to_string(&PlatformErrorCode::CategoryNotFound).unwrap(),
            "\"category_not_found\""
        );
        assert_eq!(
            serde_json::to_string(&ResourceStatus::Mounted).unwrap(),
            "\"mounted\""
        );
    }

    #[test]
    fn test_mount_resource_request_builder() {
        let request = MountResourceRequest::new(
            "file:///tmp/note.md",
            ScopeDescriptor {
                user_id: "user-123".to_string(),
                agent_id: Some("agent-abc".to_string()),
            },
        )
        .with_media_type("text/markdown");

        assert_eq!(request.uri, "file:///tmp/note.md");
        assert_eq!(request.media_type.as_deref(), Some("text/markdown"));
    }

    #[test]
    fn test_search_categories_request_builder() {
        let request = SearchCategoriesRequest::new(
            ScopeDescriptor {
                user_id: "user-123".to_string(),
                agent_id: None,
            },
            "communication",
        )
        .with_limit(5);

        assert_eq!(request.query, "communication");
        assert_eq!(request.limit, Some(5));
    }
}
