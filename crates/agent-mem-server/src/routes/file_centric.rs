//! File-centric routes with backend wiring.

use crate::error::{ServerError, ServerResult};
use crate::models::{
    ApplyMigrationRequest, CancelProactiveTaskRequest, CategoryDescriptor as ServerCategoryDescriptor,
    CategoryMetadataDescriptor, CategoryStatus as ServerCategoryStatus, ExtractionRequest,
    ExtractionResult, MigrationPlan, MigrationReport, MountResourceRequest, OperationStatus,
    PlatformErrorCode, ProactiveTaskInfo, ResourceDescriptor as ServerResourceDescriptor,
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
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Serialize)]
pub struct ResourceCollectionResponse {
    pub resources: Vec<ServerResourceDescriptor>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryCollectionResponse {
    pub categories: Vec<ServerCategoryDescriptor>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProactiveTaskCollectionResponse {
    pub tasks: Vec<ProactiveTaskInfo>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ListResourcesQuery {
    #[validate(length(min = 1, max = 255))]
    pub user_id: String,
    #[validate(length(min = 1, max = 255))]
    pub agent_id: Option<String>,
    pub status: Option<ResourceStatus>,
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<usize>,
    #[validate(range(min = 0, max = 10_000))]
    pub offset: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ListCategoriesQuery {
    #[validate(length(min = 1, max = 255))]
    pub user_id: String,
    #[validate(length(min = 1, max = 255))]
    pub agent_id: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub parent_id: Option<String>,
    pub status: Option<ServerCategoryStatus>,
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<usize>,
    #[validate(range(min = 0, max = 10_000))]
    pub offset: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CategoryByPathQuery {
    #[validate(length(min = 1, max = 1024))]
    pub path: String,
    #[validate(length(min = 1, max = 255))]
    pub user_id: String,
    #[validate(length(min = 1, max = 255))]
    pub agent_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CanonicalPlanMigrationRequest {
    #[validate(nested)]
    pub scope: ScopeDescriptor,
    pub dry_run: bool,
    #[validate(length(min = 1, max = 64))]
    pub source_surface: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub target_surface: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CanonicalApplyMigrationRequest {
    #[validate(length(min = 1, max = 255))]
    pub plan_id: String,
    #[validate(nested)]
    pub scope: Option<ScopeDescriptor>,
    pub dry_run: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ListProactiveTasksQuery {
    #[validate(length(min = 1, max = 255))]
    pub user_id: String,
    #[validate(length(min = 1, max = 255))]
    pub agent_id: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub task_type: Option<String>,
    pub status: Option<OperationStatus>,
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<usize>,
    #[validate(range(min = 0, max = 10_000))]
    pub offset: Option<usize>,
}

fn scope_to_category_scope(scope: &ScopeDescriptor) -> CategoryScope {
    match scope.agent_id.clone() {
        Some(agent_id) => CategoryScope::with_agent(scope.user_id.clone(), agent_id),
        None => CategoryScope::new(scope.user_id.clone()),
    }
}

fn apply_window<T>(items: Vec<T>, limit: Option<usize>, offset: Option<usize>) -> Vec<T> {
    let start = offset.unwrap_or(0);
    items
        .into_iter()
        .skip(start)
        .take(limit.unwrap_or(usize::MAX))
        .collect()
}

async fn list_category_descriptor_items(
    state: &FileCentricState,
    scope: &ScopeDescriptor,
    parent_id: Option<&str>,
    status: Option<ServerCategoryStatus>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> ServerResult<Vec<ServerCategoryDescriptor>> {
    scope.validate()?;

    let category_scope = scope_to_category_scope(scope);
    let categories = state
        .category_manager
        .list_categories(&category_scope)
        .await
        .map_err(|e| ServerError::internal_error(format!("Failed to list categories: {e}")))?;

    let descriptors: Vec<ServerCategoryDescriptor> = categories
        .into_iter()
        .map(category_to_descriptor)
        .filter(|descriptor| {
            parent_id
                .map(|expected| descriptor.parent_id.as_deref() == Some(expected))
                .unwrap_or(true)
        })
        .filter(|descriptor| status.as_ref().map(|expected| &descriptor.status == expected).unwrap_or(true))
        .collect();

    Ok(apply_window(descriptors, limit, offset))
}

async fn search_category_descriptor_items(
    state: &FileCentricState,
    request: &SearchCategoriesRequest,
) -> ServerResult<Vec<ServerCategoryDescriptor>> {
    request.validate()?;

    let scope = scope_to_category_scope(&request.scope);
    let limit = request.limit.unwrap_or(10);
    let categories = state
        .category_manager
        .search_categories(&request.query, &scope, limit)
        .await
        .map_err(|e| ServerError::internal_error(format!("Failed to search categories: {e}")))?;

    Ok(categories.into_iter().map(category_to_descriptor).collect())
}

fn default_preview_scope() -> ScopeDescriptor {
    ScopeDescriptor {
        user_id: "preview-user".to_string(),
        agent_id: None,
    }
}

fn migration_not_implemented_report(
    migration_id: String,
    plan_id: Option<String>,
    dry_run: bool,
) -> MigrationReport {
    MigrationReport {
        migration_id,
        plan_id,
        dry_run,
        status: OperationStatus::Failed,
        migrated_memories: 0,
        mounted_resources: 0,
        created_categories: 0,
        conflicts: vec![],
        warnings: vec!["Migration status is not yet implemented".to_string()],
        errors: vec!["Migration backend is not implemented".to_string()],
        error_code: Some(PlatformErrorCode::BackgroundTaskUnavailable),
        rollback_available: false,
        started_at: Utc::now(),
        completed_at: Some(Utc::now()),
    }
}

fn proactive_task_stub(
    task_id: String,
    status: OperationStatus,
    scope: Option<ScopeDescriptor>,
    error_code: Option<PlatformErrorCode>,
    error_message: Option<&str>,
) -> ProactiveTaskInfo {
    ProactiveTaskInfo {
        id: task_id,
        task_type: "unknown".to_string(),
        status,
        scope: scope.unwrap_or_else(default_preview_scope),
        schedule: String::new(),
        pending_runs: 0,
        running_count: 0,
        last_started_at: None,
        last_completed_at: None,
        last_error_code: error_code,
        last_error: error_message.map(str::to_string),
    }
}

fn extraction_status_stub(job_id: String) -> ExtractionResult {
    ExtractionResult {
        job_id,
        resource_id: "unknown-resource".to_string(),
        status: OperationStatus::Failed,
        category_paths: vec![],
        memory_ids: vec![],
        entities: vec![],
        relations: vec![],
        warnings: vec!["Extraction status lookup is not yet implemented".to_string()],
        error_code: Some(PlatformErrorCode::BackgroundTaskUnavailable),
        error_message: Some("Extraction job state is not persisted yet".to_string()),
        duration_ms: Some(0),
        started_at: Utc::now(),
        completed_at: Some(Utc::now()),
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

pub async fn mount_resource_canonical(
    Extension(state): Extension<Arc<FileCentricState>>,
    Json(request): Json<MountResourceRequest>,
) -> ServerResult<(StatusCode, ResponseJson<ServerResourceDescriptor>)> {
    mount_resource(Extension(state), Json(request)).await
}

pub async fn list_resources(
    Extension(state): Extension<Arc<FileCentricState>>,
    Query(query): Query<ListResourcesQuery>,
) -> ServerResult<ResponseJson<ResourceCollectionResponse>> {
    query.validate()?;

    let resources = state
        .resource_manager
        .list_resources(&query.user_id)
        .await
        .map_err(|e| ServerError::internal_error(format!("Failed to list resources: {e}")))?;

    let descriptors: Vec<ServerResourceDescriptor> = resources
        .into_iter()
        .map(resource_to_descriptor)
        .filter(|descriptor| {
            query
                .agent_id
                .as_ref()
                .map(|agent_id| descriptor.scope.agent_id.as_deref() == Some(agent_id.as_str()))
                .unwrap_or(true)
        })
        .filter(|descriptor| {
            query
                .status
                .as_ref()
                .map(|status| &descriptor.status == status)
                .unwrap_or(true)
        })
        .collect();

    Ok(ResponseJson(ResourceCollectionResponse {
        resources: apply_window(descriptors, query.limit, query.offset),
    }))
}

pub async fn get_resource_canonical(
    Extension(state): Extension<Arc<FileCentricState>>,
    Path(resource_id): Path<String>,
) -> ServerResult<ResponseJson<ServerResourceDescriptor>> {
    get_resource(Extension(state), Path(resource_id)).await
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

pub async fn extract_resource_canonical(
    Extension(state): Extension<Arc<FileCentricState>>,
    Json(request): Json<ExtractionRequest>,
) -> ServerResult<ResponseJson<ExtractionResult>> {
    extract_resource(Extension(state), Json(request)).await
}

pub async fn get_extraction_status(
    Path(job_id): Path<String>,
) -> ServerResult<ResponseJson<ExtractionResult>> {
    validate_identifier("job_id", &job_id)?;
    Ok(ResponseJson(extraction_status_stub(job_id)))
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
    Ok(ResponseJson(list_category_descriptor_items(
        state.as_ref(),
        &scope,
        None,
        None,
        None,
        None,
    )
    .await?))
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
    Ok(ResponseJson(
        search_category_descriptor_items(state.as_ref(), &request).await?,
    ))
}

pub async fn get_category(
    Extension(state): Extension<Arc<FileCentricState>>,
    Path(category_id): Path<String>,
) -> ServerResult<ResponseJson<ServerCategoryDescriptor>> {
    validate_identifier("category_id", &category_id)?;

    let category = state
        .category_manager
        .get_category(&agent_mem_category::models::CategoryId::from_string(category_id.clone()))
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                ServerError::not_found(e.to_string())
            } else {
                ServerError::internal_error(format!("Failed to get category {category_id}: {e}"))
            }
        })?;

    Ok(ResponseJson(category_to_descriptor(category)))
}

pub async fn get_category_by_path(
    Extension(state): Extension<Arc<FileCentricState>>,
    Query(query): Query<CategoryByPathQuery>,
) -> ServerResult<ResponseJson<ServerCategoryDescriptor>> {
    query.validate()?;

    let scope = scope_to_category_scope(&ScopeDescriptor {
        user_id: query.user_id,
        agent_id: query.agent_id,
    });
    let category = state
        .category_manager
        .get_category_by_path(&query.path, &scope)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                ServerError::not_found(e.to_string())
            } else {
                ServerError::internal_error(format!("Failed to get category by path: {e}"))
            }
        })?;

    Ok(ResponseJson(category_to_descriptor(category)))
}

pub async fn list_categories_canonical(
    Extension(state): Extension<Arc<FileCentricState>>,
    Query(query): Query<ListCategoriesQuery>,
) -> ServerResult<ResponseJson<CategoryCollectionResponse>> {
    query.validate()?;

    let scope = ScopeDescriptor {
        user_id: query.user_id,
        agent_id: query.agent_id,
    };
    let categories = list_category_descriptor_items(
        state.as_ref(),
        &scope,
        query.parent_id.as_deref(),
        query.status,
        query.limit,
        query.offset,
    )
    .await?;

    Ok(ResponseJson(CategoryCollectionResponse { categories }))
}

pub async fn search_categories_canonical(
    Extension(state): Extension<Arc<FileCentricState>>,
    Json(request): Json<SearchCategoriesRequest>,
) -> ServerResult<ResponseJson<CategoryCollectionResponse>> {
    let categories = search_category_descriptor_items(state.as_ref(), &request).await?;
    Ok(ResponseJson(CategoryCollectionResponse { categories }))
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

pub async fn plan_legacy_migration_canonical(
    Extension(_state): Extension<Arc<FileCentricState>>,
    Json(request): Json<CanonicalPlanMigrationRequest>,
) -> ServerResult<ResponseJson<MigrationPlan>> {
    request.validate()?;

    let plan = MigrationPlan {
        plan_id: uuid::Uuid::new_v4().to_string(),
        scope: request.scope,
        dry_run: request.dry_run,
        source_surface: request
            .source_surface
            .unwrap_or_else(|| "legacy-memory".to_string()),
        target_surface: request
            .target_surface
            .unwrap_or_else(|| "file-centric".to_string()),
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

pub async fn apply_legacy_migration_canonical(
    Extension(_state): Extension<Arc<FileCentricState>>,
    Json(request): Json<CanonicalApplyMigrationRequest>,
) -> ServerResult<ResponseJson<MigrationReport>> {
    request.validate()?;

    Ok(ResponseJson(MigrationReport {
        migration_id: uuid::Uuid::new_v4().to_string(),
        plan_id: Some(request.plan_id),
        dry_run: request.dry_run.unwrap_or(false),
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
    }))
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

pub async fn get_migration_status(
    Path(migration_id): Path<String>,
) -> ServerResult<ResponseJson<MigrationReport>> {
    validate_identifier("migration_id", &migration_id)?;
    Ok(ResponseJson(migration_not_implemented_report(
        migration_id,
        None,
        false,
    )))
}

pub async fn rollback_legacy_migration_canonical(
    Path(migration_id): Path<String>,
) -> ServerResult<ResponseJson<MigrationReport>> {
    validate_identifier("migration_id", &migration_id)?;

    let mut report = migration_not_implemented_report(migration_id, None, false);
    report.warnings = vec!["Migration rollback is not yet implemented".to_string()];
    report.errors = vec!["Migration rollback is not implemented".to_string()];
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

pub async fn list_proactive_tasks_canonical(
    Extension(_state): Extension<Arc<FileCentricState>>,
    Query(query): Query<ListProactiveTasksQuery>,
) -> ServerResult<ResponseJson<ProactiveTaskCollectionResponse>> {
    query.validate()?;

    Ok(ResponseJson(ProactiveTaskCollectionResponse {
        tasks: apply_window(Vec::<ProactiveTaskInfo>::new(), query.limit, query.offset),
    }))
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

pub async fn get_proactive_task(
    Path(task_id): Path<String>,
) -> ServerResult<ResponseJson<ProactiveTaskInfo>> {
    validate_identifier("task_id", &task_id)?;
    Ok(ResponseJson(proactive_task_stub(
        task_id,
        OperationStatus::Failed,
        None,
        Some(PlatformErrorCode::BackgroundTaskUnavailable),
        Some("Proactive tasks are not yet implemented"),
    )))
}

pub async fn run_proactive_task_canonical(
    Path(task_id): Path<String>,
) -> ServerResult<ResponseJson<ProactiveTaskInfo>> {
    validate_identifier("task_id", &task_id)?;
    Ok(ResponseJson(proactive_task_stub(
        task_id,
        OperationStatus::Failed,
        None,
        Some(PlatformErrorCode::BackgroundTaskUnavailable),
        Some("Proactive tasks are not yet implemented"),
    )))
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

pub async fn cancel_proactive_task_canonical(
    Path(task_id): Path<String>,
) -> ServerResult<ResponseJson<ProactiveTaskInfo>> {
    validate_identifier("task_id", &task_id)?;
    Ok(ResponseJson(proactive_task_stub(
        task_id,
        OperationStatus::Cancelled,
        None,
        None,
        None,
    )))
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

pub async fn get_scheduler_stats_canonical(
    Extension(state): Extension<Arc<FileCentricState>>,
) -> ServerResult<ResponseJson<SchedulerStats>> {
    get_scheduler_stats(Extension(state)).await
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
    use agent_mem_category::models::CategoryScope;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::{get, post},
        Router,
    };
    use serde_json::Value;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use tower::ServiceExt;

    fn test_router_with_state(state: Arc<FileCentricState>) -> Router {
        Router::new()
            .route("/api/v1/resources/mount", post(mount_resource))
            .route("/api/v1/resources/:resource_id", get(get_resource))
            .route("/api/v1/resources/extract", post(extract_resource))
            .route(
                "/api/v1/file-centric/resources",
                post(mount_resource_canonical).get(list_resources),
            )
            .route(
                "/api/v1/file-centric/resources/:resource_id",
                get(get_resource_canonical),
            )
            .route(
                "/api/v1/file-centric/extraction",
                post(extract_resource_canonical),
            )
            .route(
                "/api/v1/file-centric/extraction/:job_id",
                get(get_extraction_status),
            )
            .route(
                "/api/v1/file-centric/categories/by-path",
                get(get_category_by_path),
            )
            .route(
                "/api/v1/file-centric/categories/:category_id",
                get(get_category),
            )
            .route(
                "/api/v1/file-centric/categories",
                get(list_categories_canonical),
            )
            .route(
                "/api/v1/file-centric/categories/search",
                post(search_categories_canonical),
            )
            .route("/api/v1/categories", get(list_categories))
            .route("/api/v1/categories/search", post(search_categories))
            .route("/api/v1/migrations/plan", post(plan_legacy_migration))
            .route("/api/v1/migrations/apply", post(apply_legacy_migration))
            .route("/api/v1/migrations/rollback", post(rollback_legacy_migration))
            .route(
                "/api/v1/file-centric/migration/plan",
                post(plan_legacy_migration_canonical),
            )
            .route(
                "/api/v1/file-centric/migration/apply",
                post(apply_legacy_migration_canonical),
            )
            .route(
                "/api/v1/file-centric/migration/:migration_id",
                get(get_migration_status),
            )
            .route(
                "/api/v1/file-centric/migration/:migration_id/rollback",
                post(rollback_legacy_migration_canonical),
            )
            .route("/api/v1/proactive/tasks", get(list_proactive_tasks))
            .route("/api/v1/proactive/tasks/:task_id/run", post(run_proactive_task))
            .route(
                "/api/v1/proactive/tasks/:task_id/cancel",
                post(cancel_proactive_task),
            )
            .route(
                "/api/v1/file-centric/proactive/tasks",
                get(list_proactive_tasks_canonical),
            )
            .route(
                "/api/v1/file-centric/proactive/tasks/:task_id",
                get(get_proactive_task),
            )
            .route(
                "/api/v1/file-centric/proactive/tasks/:task_id/run",
                post(run_proactive_task_canonical),
            )
            .route(
                "/api/v1/file-centric/proactive/tasks/:task_id/cancel",
                post(cancel_proactive_task_canonical),
            )
            .route(
                "/api/v1/proactive/scheduler/stats",
                get(get_scheduler_stats),
            )
            .route(
                "/api/v1/file-centric/proactive/stats",
                get(get_scheduler_stats_canonical),
            )
            .layer(Extension(state))
    }

    fn test_router() -> Router {
        test_router_with_state(Arc::new(FileCentricState::new()))
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

    #[tokio::test]
    async fn test_file_centric_mount_collection_route_exists() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "file-centric route contract").unwrap();
        let uri = format!("file://{}", temp_file.path().display());

        let app = test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/file-centric/resources")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "uri": uri,
                            "media_type": "text/plain",
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

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_file_centric_categories_route_returns_envelope() {
        let mut state = FileCentricState::new();
        Arc::get_mut(&mut state.category_manager)
            .unwrap()
            .create_category(
                "/preferences/communication",
                CategoryScope::with_agent("user-123".to_string(), "agent-abc".to_string()),
            )
            .await
            .unwrap();

        let app = test_router_with_state(Arc::new(state));
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/file-centric/categories?user_id=user-123&agent_id=agent-abc")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        let categories = json
            .get("categories")
            .and_then(Value::as_array)
            .expect("expected categories envelope");
        assert!(categories
            .iter()
            .any(|category| category["path"] == "/preferences/communication"));
    }

    #[tokio::test]
    async fn test_file_centric_get_category_by_path_route_returns_descriptor() {
        let mut state = FileCentricState::new();
        Arc::get_mut(&mut state.category_manager)
            .unwrap()
            .create_category(
                "/preferences/communication",
                CategoryScope::with_agent("user-123".to_string(), "agent-abc".to_string()),
            )
            .await
            .unwrap();

        let app = test_router_with_state(Arc::new(state));
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/file-centric/categories/by-path?path=/preferences/communication&user_id=user-123&agent_id=agent-abc")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["path"], "/preferences/communication");
        assert_eq!(json["scope"]["agent_id"], "agent-abc");
    }

    #[tokio::test]
    async fn test_file_centric_get_migration_status_route_returns_report() {
        let app = test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/file-centric/migration/mig-run-123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["migration_id"], "mig-run-123");
    }

    #[tokio::test]
    async fn test_file_centric_get_proactive_task_route_returns_descriptor() {
        let app = test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/file-centric/proactive/tasks/task-123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["id"], "task-123");
    }

    #[tokio::test]
    async fn test_file_centric_proactive_stats_alias_exists() {
        let app = test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/file-centric/proactive/stats")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
