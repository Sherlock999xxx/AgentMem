//! File-centric platform DTOs and preview request types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    }
}
