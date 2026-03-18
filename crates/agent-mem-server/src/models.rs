//! API request and response models

use agent_mem_core::MemoryType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use validator::Validate;

/// Request to add a new memory
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct MemoryRequest {
    /// Agent ID (optional, defaults to "default-agent" or "default-agent-{user_id}")
    #[validate(length(min = 1, max = 255))]
    pub agent_id: Option<String>,

    /// User ID (optional)
    #[validate(length(max = 255))]
    pub user_id: Option<String>,

    /// Memory content
    #[validate(length(min = 1, max = 10000))]
    pub content: String,

    /// Memory type
    pub memory_type: Option<MemoryType>,

    /// Importance score (0.0 to 1.0)
    #[validate(range(min = 0.0, max = 1.0))]
    pub importance: Option<f32>,

    /// Additional metadata
    pub metadata: Option<HashMap<String, String>>,
}

/// Request to update a memory
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateMemoryRequest {
    /// New content (optional)
    #[validate(length(max = 10000))]
    pub content: Option<String>,

    /// New importance score (optional)
    #[validate(range(min = 0.0, max = 1.0))]
    pub importance: Option<f32>,
}

/// Response for memory operations
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MemoryResponse {
    /// Memory ID
    pub id: String,

    /// Response message
    pub message: String,
}

/// Request to search memories
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct SearchRequest {
    /// Search query
    #[validate(length(min = 1, max = 1000))]
    pub query: String,

    /// Whether to prefetch memories before search (optional, default: false)
    pub prefetch: Option<bool>,

    /// Agent ID (optional)
    #[validate(length(max = 255))]
    pub agent_id: Option<String>,

    /// User ID (optional)
    #[validate(length(max = 255))]
    pub user_id: Option<String>,

    /// Memory type filter (optional)
    pub memory_type: Option<MemoryType>,

    /// Maximum number of results
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<usize>,

    /// Similarity threshold
    #[validate(range(min = 0.0, max = 1.0))]
    pub threshold: Option<f32>,

    /// 🆕 Phase 2.12: 智能过滤参数
    /// Minimum importance threshold (0.0-1.0, optional)
    #[validate(range(min = 0.0, max = 1.0))]
    pub min_importance: Option<f32>,

    /// Maximum age in days (optional, filters out memories older than this)
    pub max_age_days: Option<u64>,

    /// Minimum access count (optional, filters out memories with fewer accesses)
    pub min_access_count: Option<i64>,

    /// 🆕 Phase 2.13: 分页参数
    /// Offset for pagination (optional, default: 0)
    #[validate(range(min = 0))]
    pub offset: Option<usize>,
}

/// Response for search operations
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SearchResponse {
    /// Search results
    pub results: Vec<serde_json::Value>,

    /// Total number of results
    pub total: usize,

    /// 🆕 Phase 2.13: 分页信息
    /// Current offset
    pub offset: usize,

    /// Current limit
    pub limit: usize,

    /// Whether there are more results
    pub has_more: bool,
}

/// Request for batch search operations
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct BatchSearchRequest {
    /// List of search queries
    #[validate(length(min = 1, max = 50))]
    pub queries: Vec<SearchRequest>,

    /// Common agent ID (optional, can be overridden by individual queries)
    #[validate(length(max = 255))]
    pub agent_id: Option<String>,

    /// Common user ID (optional, can be overridden by individual queries)
    #[validate(length(max = 255))]
    pub user_id: Option<String>,
}

/// Response for batch search operations
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BatchSearchResponse {
    /// Number of successful searches
    pub successful: usize,

    /// Number of failed searches
    pub failed: usize,

    /// Search results for each query (in order)
    pub results: Vec<Vec<serde_json::Value>>,

    /// Error messages for failed searches (in order, None if successful)
    pub errors: Vec<Option<String>>,
}

/// Request for batch operations
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct BatchRequest {
    /// List of memory requests
    #[validate(length(min = 1, max = 100))]
    pub memories: Vec<MemoryRequest>,
}

/// Response for batch operations
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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

/// Search statistics response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SearchStatsResponse {
    /// Total number of searches
    pub total_searches: u64,

    /// Number of cache hits
    pub cache_hits: u64,

    /// Number of cache misses
    pub cache_misses: u64,

    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,

    /// Number of exact queries (LibSQL)
    pub exact_queries: u64,

    /// Number of vector searches
    pub vector_searches: u64,

    /// Average search latency in milliseconds
    pub avg_latency_ms: f64,

    /// Current cache size
    pub cache_size: usize,

    /// Timestamp of last update
    pub last_updated: DateTime<Utc>,
}

/// Component health status
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ComponentStatus {
    /// Component status (healthy, degraded, unhealthy)
    pub status: String,

    /// Optional status message
    pub message: Option<String>,

    /// Last check timestamp
    pub last_check: DateTime<Utc>,
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    /// Overall health status
    pub status: String,

    /// Timestamp of the health check
    pub timestamp: DateTime<Utc>,

    /// Service version
    pub version: String,

    /// Individual component health checks
    pub checks: HashMap<String, ComponentStatus>,
}

/// Metrics response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MetricsResponse {
    /// Timestamp of metrics collection
    pub timestamp: DateTime<Utc>,

    /// Collected metrics
    pub metrics: HashMap<String, f64>,
}

/// Error response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct ScopeDescriptor {
    /// User ID that owns the operation.
    #[validate(length(min = 1, max = 255))]
    pub user_id: String,

    /// Agent ID within the user scope.
    #[validate(length(min = 1, max = 255))]
    pub agent_id: Option<String>,
}

/// Lifecycle state for mounted resources.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceStatus {
    Pending,
    Mounted,
    Failed,
    Archived,
}

/// Lifecycle state for categories.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CategoryStatus {
    Active,
    Archived,
    Deleted,
}

/// Cross-language status model for async and long-running operations.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

/// Scheduler lifecycle state for proactive orchestration.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

/// File-centric error code baseline for server/client/SDK alignment.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryMetadataDescriptor {
    /// Tag labels used for browsing and retrieval hints.
    pub tags: Vec<String>,

    /// Extensible metadata attributes.
    pub attributes: HashMap<String, String>,
}

/// Stable category DTO for the file-centric public contract.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct ExtractionRequest {
    /// Resource to extract from.
    #[validate(length(min = 1, max = 255))]
    pub resource_id: String,

    /// Multi-tenant ownership scope.
    #[validate(nested)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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

/// Generic API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    /// Response data
    pub data: T,

    /// Success status
    #[serde(default = "default_true")]
    pub success: bool,

    /// Optional message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

fn default_true() -> bool {
    true
}

impl<T> ApiResponse<T> {
    /// Create a successful response
    pub fn success(data: T) -> Self {
        Self {
            data,
            success: true,
            message: None,
        }
    }

    /// Create a successful response with a message
    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            data,
            success: true,
            message: Some(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_memory_request_validation() {
        let request = MemoryRequest {
            agent_id: Some("test_agent".to_string()),
            user_id: Some("test_user".to_string()),
            content: "Test memory content".to_string(),
            memory_type: Some(MemoryType::Episodic),
            importance: Some(0.8),
            metadata: None,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_memory_request_validation_fails() {
        let request = MemoryRequest {
            agent_id: Some("".to_string()), // Empty agent_id should fail
            user_id: Some("test_user".to_string()),
            content: "Test memory content".to_string(),
            memory_type: Some(MemoryType::Episodic),
            importance: Some(0.8),
            metadata: None,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_search_request_validation() {
        let request = SearchRequest {
            query: "test query".to_string(),
            prefetch: None,
            agent_id: Some("test_agent".to_string()),
            user_id: Some("test_user".to_string()),
            memory_type: Some(MemoryType::Semantic),
            limit: Some(10),
            threshold: Some(0.3), // 🔧 降低阈值以支持商品ID等精确查询
            min_importance: None,
            max_age_days: None,
            min_access_count: None,
            offset: None,
        };

        assert!(request.validate().is_ok());
    }

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
    fn test_extraction_request_validation_fails_when_resource_id_empty() {
        let request = ExtractionRequest {
            resource_id: String::new(),
            scope: ScopeDescriptor {
                user_id: "user-123".to_string(),
                agent_id: Some("agent-abc".to_string()),
            },
            category_hint_paths: vec!["/preferences/communication".to_string()],
            persist_output: true,
            include_entities: true,
            include_relations: true,
        };

        assert!(request.validate().is_err());
    }
}
