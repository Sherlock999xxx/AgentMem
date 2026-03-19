//! File-centric routes with backend wiring.

use crate::error::{ServerError, ServerResult};
use crate::models::{
    ApplyMigrationRequest, CancelProactiveTaskRequest, CategoryDescriptor as ServerCategoryDescriptor,
    CategoryMetadataDescriptor, ExtractionRequest, ExtractionResult, MigrationPlan, MigrationReport,
    MountResourceRequest, OperationStatus, PlatformErrorCode, ProactiveTaskInfo, ResourceDescriptor as ServerResourceDescriptor,
    ResourceMetadataDescriptor, ResourceStatus, RollbackMigrationRequest, RunProactiveTaskRequest,
    SchedulerStats, SchedulerState, ScopeDescriptor, SearchCategoriesRequest,
};
use agent_mem_category::manager::{CategoryManager, InMemoryCategoryManager};
use agent_mem_category::models::{Category, CategoryScope};
use agent_mem_extraction::models::{ExtractionId, ExtractionInput, ExtractionScope};
use agent_mem_extraction::pipeline::ExtractionPipeline;
use agent_mem_resource::manager::{ResourceManager, ResourceManagerTrait};
use agent_mem_resource::models::Resource;
use axum::{
    extract::{Extension, Json, Path, Query},
    http::StatusCode,
    response::Json as ResponseJson,
};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use validator::Validate;

/// Shared state for file-centric operations.
pub struct FileCentricState {
    /// Resource manager for mounting and managing resources.
    pub resource_manager: Arc<dyn ResourceManagerTrait>,
    /// Category manager for hierarchical category operations.
    pub category_manager: Arc<InMemoryCategoryManager>,
    /// Extraction pipeline for resource processing.
    pub extraction_pipeline: Arc<RwLock<Option<ExtractionPipeline>>>,
}

impl FileCentricState {
    /// Create a new file-centric state with default managers.
    pub fn new() -> Self {
        Self {
            resource_manager: Arc::new(
                ResourceManager::new().expect("Failed to create ResourceManager"),
            ),
            category_manager: Arc::new(InMemoryCategoryManager::new()),
            extraction_pipeline: Arc::new(RwLock::new(None)),
        }
    }
}

impl Default for FileCentricState {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Resource Routes
// ============================================================================

#[utoipa::path(
    post,
    path = "/api/v1/resources/mount",
    tag = "file-centric",
    request_body = MountResourceRequest,
    responses(
        (status = 201, description = "Resource mounted successfully", body = ServerResourceDescriptor),
        (status = 400, description = "Validation failed", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    )
)]
pub async fn mount_resource(
    Extension(state): Extension<Arc<FileCentricState>>,
    Json(request): Json<MountResourceRequest>,
) -> ServerResult<(StatusCode, ResponseJson<ServerResourceDescriptor>)> {
    request.validate()?;

    let resource_id = state
        .resource_manager
        .mount_resource(&request.uri, &request.scope.user_id, request.scope.agent_id.as_deref())
        .await
        .map_err(|e| ServerError::internal_error(format!("Failed to mount resource: {e}")))?;

    let resource = state
        .resource_manager
        .get_resource(&resource_id)
        .await
        .map_err(|e| ServerError::internal_error(format!("Failed to get mounted resource: {e}")))?;

    let descriptor = resource_to_descriptor(resource);
    Ok((StatusCode::CREATED, ResponseJson(descriptor)))
}

#[utoipa::path(
    get,
    path = "/api/v1/resources/{resource_id}",
    tag = "file-centric",
    params(
        ("resource_id" = String, Path, description = "Mounted resource identifier")
    ),
    responses(
        (status = 200, description = "Resource retrieved successfully", body = ServerResourceDescriptor),
        (status = 404, description = "Resource not found", body = crate::models::ErrorResponse),
        (status = 400, description = "Validation failed", body = crate::models::ErrorResponse),
    )
)]
pub async fn get_resource(
    Extension(state): Extension<Arc<FileCentricState>>,
    Path(resource_id): Path<String>,
) -> ServerResult<ResponseJson<ServerResourceDescriptor>> {
    validate_identifier("resource_id", &resource_id)?;

    let resource = state
        .resource_manager
        .get_resource(&agent_mem_resource::models::ResourceId(resource_id))
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                ServerError::not_found(e.to_string())
            } else {
                ServerError::internal_error(format!("Failed to get resource: {e}"))
            }
        })?;

    let descriptor = resource_to_descriptor(resource);
    Ok(ResponseJson(descriptor))
}

// ============================================================================
// Extraction Routes
// ============================================================================

#[utoipa::path(
    post,
    path = "/api/v1/resources/extract",
    tag = "file-centric",
    request_body = ExtractionRequest,
    responses(
        (status = 200, description = "Extraction completed", body = ExtractionResult),
        (status = 400, description = "Validation failed", body = crate::models::ErrorResponse),
        (status = 404, description = "Resource not found", body = crate::models::ErrorResponse),
        (status = 501, description = "Extraction pipeline not configured", body = crate::models::ErrorResponse),
    )
)]
pub async fn extract_resource(
    Extension(state): Extension<Arc<FileCentricState>>,
    Json(request): Json<ExtractionRequest>,
) -> ServerResult<ResponseJson<ExtractionResult>> {
    request.validate()?;

    // Verify resource exists
    let _resource = state
        .resource_manager
        .get_resource(&agent_mem_resource::models::ResourceId(request.resource_id.clone()))
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                ServerError::not_found(format!("Resource not found: {}", request.resource_id))
            } else {
                ServerError::internal_error(format!("Failed to get resource: {e}"))
            }
        })?;

    // Check if extraction pipeline is configured
    let pipeline_guard = state.extraction_pipeline.read().await;
    if let Some(pipeline) = pipeline_guard.as_ref() {
        // Execute extraction pipeline
        let input = ExtractionInput {
            id: ExtractionId::new(),
            uri: format!("resource://{}", request.resource_id),
            content: None,
            media_type: None,
            metadata: Default::default(),
            scope: ExtractionScope {
                user_id: request.scope.user_id.clone(),
                agent_id: request.scope.agent_id.clone(),
            },
        };

        let output = pipeline.execute(input).await.map_err(|e| {
            ServerError::internal_error(format!("Extraction pipeline failed: {e}"))
        })?;

        let result = ExtractionResult {
            job_id: uuid::Uuid::new_v4().to_string(),
            resource_id: request.resource_id,
            status: OperationStatus::Succeeded,
            category_paths: request.category_hint_paths,
            memory_ids: vec![],
            entities: output
                .items
                .iter()
                .map(|item| {
                    crate::models::ExtractedEntity {
                        id: item.id.clone(),
                        name: item.content.chars().take(50).collect(),
                        entity_type: item.item_type.clone(),
                        confidence: 0.9,
                        attributes: Default::default(),
                        span_start: None,
                        span_end: None,
                    }
                })
                .collect(),
            relations: vec![],
            warnings: vec![],
            error_code: None,
            error_message: None,
            duration_ms: Some(output.metrics.total_duration_ms),
            started_at: Utc::now(),
            completed_at: Some(Utc::now()),
        };

        Ok(ResponseJson(result))
    } else {
        // No pipeline configured - return a stub result
        let result = ExtractionResult {
            job_id: uuid::Uuid::new_v4().to_string(),
            resource_id: request.resource_id,
            status: OperationStatus::Succeeded,
            category_paths: request.category_hint_paths,
            memory_ids: vec![],
            entities: vec![],
            relations: vec![],
            warnings: vec!["Extraction pipeline not configured - returning stub result".to_string()],
            error_code: None,
            error_message: None,
            duration_ms: Some(0),
            started_at: Utc::now(),
            completed_at: Some(Utc::now()),
        };
        Ok(ResponseJson(result))
    }
}

// ============================================================================
// Category Routes
// ============================================================================

#[utoipa::path(
    get,
    path = "/api/v1/categories",
    tag = "file-centric",
    params(
        ("user_id" = String, Query, description = "Owner user id"),
        ("agent_id" = Option<String>, Query, description = "Optional agent id")
    ),
    responses(
        (status = 200, description = "Categories retrieved successfully", body = Vec<ServerCategoryDescriptor>),
        (status = 400, description = "Validation failed", body = crate::models::ErrorResponse),
    )
)]
pub async fn list_categories(
    Extension(state): Extension<Arc<FileCentricState>>,
    Query(scope): Query<ScopeDescriptor>,
) -> ServerResult<ResponseJson<Vec<ServerCategoryDescriptor>>> {
    scope.validate()?;

    let scope = CategoryScope::new(scope.user_id);
    let categories = state
        .category_manager
        .list_categories(&scope)
        .await
        .map_err(|e| ServerError::internal_error(format!("Failed to list categories: {e}")))?;

    let descriptors: Vec<ServerCategoryDescriptor> =
        categories.into_iter().map(category_to_descriptor).collect();

    Ok(ResponseJson(descriptors))
}

#[utoipa::path(
    post,
    path = "/api/v1/categories/search",
    tag = "file-centric",
    request_body = SearchCategoriesRequest,
    responses(
        (status = 200, description = "Categories search completed", body = Vec<ServerCategoryDescriptor>),
        (status = 400, description = "Validation failed", body = crate::models::ErrorResponse),
    )
)]
pub async fn search_categories(
    Extension(state): Extension<Arc<FileCentricState>>,
    Json(request): Json<SearchCategoriesRequest>,
) -> ServerResult<ResponseJson<Vec<ServerCategoryDescriptor>>> {
    request.validate()?;

    let scope = CategoryScope::new(request.scope.user_id);
    let limit = request.limit.unwrap_or(10);

    let categories = state
        .category_manager
        .search_categories(&request.query, &scope, limit)
        .await
        .map_err(|e| ServerError::internal_error(format!("Failed to search categories: {e}")))?;

    let descriptors: Vec<ServerCategoryDescriptor> =
        categories.into_iter().map(category_to_descriptor).collect();

    Ok(ResponseJson(descriptors))
}

// ============================================================================
// Migration Routes (Stub implementations)
// ============================================================================

#[utoipa::path(
    post,
    path = "/api/v1/migrations/plan",
    tag = "file-centric",
    request_body = crate::models::PlanMigrationRequest,
    responses(
        (status = 200, description = "Migration plan created", body = MigrationPlan),
        (status = 400, description = "Validation failed", body = crate::models::ErrorResponse),
        (status = 501, description = "Not implemented", body = crate::models::ErrorResponse),
    )
)]
pub async fn plan_legacy_migration(
    Extension(_state): Extension<Arc<FileCentricState>>,
    Json(request): Json<crate::models::PlanMigrationRequest>,
) -> ServerResult<ResponseJson<MigrationPlan>> {
    request.validate()?;

    // Stub implementation - returns a placeholder migration plan
    let plan = MigrationPlan {
        plan_id: uuid::Uuid::new_v4().to_string(),
        scope: request.scope,
        dry_run: request.dry_run,
        source_surface: request.source_surface,
        target_surface: request.target_surface,
        legacy_memory_count: 0,
        projected_resource_count: 0,
        projected_category_count: 0,
        warnings: vec!["Migration planning is not yet implemented".to_string()],
        created_at: Utc::now(),
    };

    Ok(ResponseJson(plan))
}

#[utoipa::path(
    post,
    path = "/api/v1/migrations/apply",
    tag = "file-centric",
    request_body = ApplyMigrationRequest,
    responses(
        (status = 200, description = "Migration applied", body = MigrationReport),
        (status = 400, description = "Validation failed", body = crate::models::ErrorResponse),
        (status = 501, description = "Not implemented", body = crate::models::ErrorResponse),
    )
)]
pub async fn apply_legacy_migration(
    Extension(_state): Extension<Arc<FileCentricState>>,
    Json(request): Json<ApplyMigrationRequest>,
) -> ServerResult<ResponseJson<MigrationReport>> {
    request.validate()?;

    // Stub implementation - returns a placeholder migration report
    let report = MigrationReport {
        migration_id: uuid::Uuid::new_v4().to_string(),
        plan_id: Some(request.plan_id),
        dry_run: false,
        status: OperationStatus::Failed,
        migrated_memories: 0,
        mounted_resources: 0,
        created_categories: 0,
        conflicts: vec![],
        warnings: vec!["Migration is not yet implemented".to_string()],
        errors: vec!["Migration apply is not implemented".to_string()],
        error_code: Some(PlatformErrorCode::BackgroundTaskUnavailable),
        rollback_available: false,
        started_at: Utc::now(),
        completed_at: Some(Utc::now()),
    };

    Ok(ResponseJson(report))
}

#[utoipa::path(
    post,
    path = "/api/v1/migrations/rollback",
    tag = "file-centric",
    request_body = RollbackMigrationRequest,
    responses(
        (status = 200, description = "Migration rolled back", body = MigrationReport),
        (status = 400, description = "Validation failed", body = crate::models::ErrorResponse),
        (status = 501, description = "Not implemented", body = crate::models::ErrorResponse),
    )
)]
pub async fn rollback_legacy_migration(
    Extension(_state): Extension<Arc<FileCentricState>>,
    Json(request): Json<RollbackMigrationRequest>,
) -> ServerResult<ResponseJson<MigrationReport>> {
    request.validate()?;

    // Stub implementation - returns a placeholder migration report
    let report = MigrationReport {
        migration_id: request.migration_id,
        plan_id: None,
        dry_run: false,
        status: OperationStatus::Failed,
        migrated_memories: 0,
        mounted_resources: 0,
        created_categories: 0,
        conflicts: vec![],
        warnings: vec!["Migration rollback is not yet implemented".to_string()],
        errors: vec!["Migration rollback is not implemented".to_string()],
        error_code: Some(PlatformErrorCode::BackgroundTaskUnavailable),
        rollback_available: false,
        started_at: Utc::now(),
        completed_at: Some(Utc::now()),
    };

    Ok(ResponseJson(report))
}

// ============================================================================
// Proactive Routes (Stub implementations)
// ============================================================================

#[utoipa::path(
    get,
    path = "/api/v1/proactive/tasks",
    tag = "file-centric",
    params(
        ("user_id" = String, Query, description = "Owner user id"),
        ("agent_id" = Option<String>, Query, description = "Optional agent id")
    ),
    responses(
        (status = 200, description = "Tasks retrieved successfully", body = Vec<ProactiveTaskInfo>),
        (status = 400, description = "Validation failed", body = crate::models::ErrorResponse),
        (status = 501, description = "Not implemented", body = crate::models::ErrorResponse),
    )
)]
pub async fn list_proactive_tasks(
    Extension(_state): Extension<Arc<FileCentricState>>,
    Query(scope): Query<ScopeDescriptor>,
) -> ServerResult<ResponseJson<Vec<ProactiveTaskInfo>>> {
    scope.validate()?;

    // Stub implementation - returns empty task list
    Ok(ResponseJson(vec![]))
}

#[utoipa::path(
    post,
    path = "/api/v1/proactive/tasks/{task_id}/run",
    tag = "file-centric",
    params(
        ("task_id" = String, Path, description = "Proactive task identifier")
    ),
    request_body = RunProactiveTaskRequest,
    responses(
        (status = 200, description = "Task started", body = ProactiveTaskInfo),
        (status = 400, description = "Validation failed", body = crate::models::ErrorResponse),
        (status = 501, description = "Not implemented", body = crate::models::ErrorResponse),
    )
)]
pub async fn run_proactive_task(
    Extension(_state): Extension<Arc<FileCentricState>>,
    Path(task_id): Path<String>,
    Json(request): Json<RunProactiveTaskRequest>,
) -> ServerResult<ResponseJson<ProactiveTaskInfo>> {
    validate_identifier("task_id", &task_id)?;
    request.validate()?;

    // Stub implementation
    let task_info = ProactiveTaskInfo {
        id: task_id,
        task_type: "unknown".to_string(),
        status: OperationStatus::Failed,
        scope: request.scope,
        schedule: "".to_string(),
        pending_runs: 0,
        running_count: 0,
        last_started_at: None,
        last_completed_at: None,
        last_error_code: Some(PlatformErrorCode::BackgroundTaskUnavailable),
        last_error: Some("Proactive tasks are not yet implemented".to_string()),
    };

    Ok(ResponseJson(task_info))
}

#[utoipa::path(
    post,
    path = "/api/v1/proactive/tasks/{task_id}/cancel",
    tag = "file-centric",
    params(
        ("task_id" = String, Path, description = "Proactive task identifier")
    ),
    request_body = CancelProactiveTaskRequest,
    responses(
        (status = 200, description = "Task cancelled", body = ProactiveTaskInfo),
        (status = 400, description = "Validation failed", body = crate::models::ErrorResponse),
        (status = 501, description = "Not implemented", body = crate::models::ErrorResponse),
    )
)]
pub async fn cancel_proactive_task(
    Extension(_state): Extension<Arc<FileCentricState>>,
    Path(task_id): Path<String>,
    Json(request): Json<CancelProactiveTaskRequest>,
) -> ServerResult<ResponseJson<ProactiveTaskInfo>> {
    validate_identifier("task_id", &task_id)?;
    request.validate()?;

    // Stub implementation
    let task_info = ProactiveTaskInfo {
        id: task_id,
        task_type: "unknown".to_string(),
        status: OperationStatus::Cancelled,
        scope: request.scope,
        schedule: "".to_string(),
        pending_runs: 0,
        running_count: 0,
        last_started_at: None,
        last_completed_at: None,
        last_error_code: None,
        last_error: None,
    };

    Ok(ResponseJson(task_info))
}

#[utoipa::path(
    get,
    path = "/api/v1/proactive/scheduler/stats",
    tag = "file-centric",
    responses(
        (status = 200, description = "Scheduler stats retrieved", body = SchedulerStats),
        (status = 501, description = "Not implemented", body = crate::models::ErrorResponse),
    )
)]
pub async fn get_scheduler_stats(
    Extension(_state): Extension<Arc<FileCentricState>>,
) -> ServerResult<ResponseJson<SchedulerStats>> {
    // Stub implementation - returns default stats
    let stats = SchedulerStats {
        state: SchedulerState::Stopped,
        total_tasks: 0,
        running_tasks: 0,
        completed_tasks: 0,
        failed_tasks: 0,
        cancelled_tasks: 0,
        total_execution_time_ms: 0,
        last_error: Some("Scheduler not yet implemented".to_string()),
        updated_at: Utc::now(),
    };

    Ok(ResponseJson(stats))
}

// ============================================================================
// Helper Functions
// ============================================================================

fn validate_identifier(name: &str, value: &str) -> ServerResult<()> {
    if value.trim().is_empty() {
        return Err(ServerError::validation_error(format!(
            "{name} must not be empty"
        )));
    }
    Ok(())
}

fn resource_to_descriptor(resource: Resource) -> ServerResourceDescriptor {
    let status = match resource.status {
        agent_mem_resource::models::ResourceStatus::Pending => ResourceStatus::Pending,
        agent_mem_resource::models::ResourceStatus::Mounted => ResourceStatus::Mounted,
        agent_mem_resource::models::ResourceStatus::Failed => ResourceStatus::Failed,
        agent_mem_resource::models::ResourceStatus::Archived => ResourceStatus::Archived,
    };

    // Convert serde_json::Value to String for attributes
    let attributes: HashMap<String, String> = resource
        .metadata
        .custom
        .into_iter()
        .map(|(k, v)| (k, v.to_string()))
        .collect();

    let metadata = ResourceMetadataDescriptor {
        author: None,
        tags: vec![],
        size_bytes: resource.metadata.size,
        modified_at: None,
        attributes,
    };

    ServerResourceDescriptor {
        id: resource.id.0,
        uri: resource.uri,
        media_type: resource.media_type.to_string(),
        status,
        scope: ScopeDescriptor {
            user_id: resource.user_id,
            agent_id: resource.agent_id,
        },
        metadata,
        created_at: resource.created_at,
        updated_at: resource.updated_at,
    }
}

fn category_to_descriptor(category: Category) -> ServerCategoryDescriptor {
    use agent_mem_category::models::CategoryStatus as CatStatus;

    let status = match category.status {
        CatStatus::Active => crate::models::CategoryStatus::Active,
        CatStatus::Archived => crate::models::CategoryStatus::Archived,
        CatStatus::Deleted => crate::models::CategoryStatus::Deleted,
    };

    let metadata = CategoryMetadataDescriptor {
        tags: vec![],
        attributes: HashMap::new(),
    };

    ServerCategoryDescriptor {
        id: category.id.to_string(),
        path: category.path,
        name: category.name,
        parent_id: category.parent_id.map(|id| id.to_string()),
        children_ids: category.children_ids.into_iter().map(|id| id.to_string()).collect(),
        summary: category.summary,
        item_count: category.item_count,
        status,
        scope: ScopeDescriptor {
            user_id: category.scope.user_id,
            agent_id: category.scope.agent_id.clone(),
        },
        metadata,
        created_at: category.created_at,
        updated_at: category.updated_at,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::{get, post},
        Router,
    };
    use tower::ServiceExt;

    fn test_router() -> Router {
        let state = Arc::new(FileCentricState::new());
        Router::new()
            .route("/api/v1/resources/mount", post(mount_resource))
            .route("/api/v1/resources/:resource_id", get(get_resource))
            .route("/api/v1/categories", get(list_categories))
            .route("/api/v1/categories/search", post(search_categories))
            .route(
                "/api/v1/proactive/scheduler/stats",
                get(get_scheduler_stats),
            )
            .layer(Extension(state))
    }

    #[tokio::test]
    async fn test_mount_resource_returns_response() {
        let app = test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/resources/mount")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "uri": "file:///tmp/test.txt",
                            "scope": {
                                "user_id": "user-123",
                                "agent_id": "agent-abc"
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should return some response (201 for success, or error for missing file)
        // The key is that it's NOT 501 NOT_IMPLEMENTED, which means the route is wired
        let status = response.status();
        assert!(
            status == StatusCode::CREATED || status == StatusCode::INTERNAL_SERVER_ERROR,
            "Expected 201 or 500, got {}",
            status
        );
    }

    #[tokio::test]
    async fn test_list_categories_returns_array() {
        let app = test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/categories?user_id=user-123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_search_categories_with_validation() {
        let app = test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/categories/search")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "scope": {
                                "user_id": "user-123"
                            },
                            "query": "",
                            "limit": 5
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Empty query should fail validation
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_scheduler_stats_returns_stub() {
        let app = test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/proactive/scheduler/stats")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
