//! HTTP client implementation

use crate::{
    config::ClientConfig,
    error::{ClientError, ClientResult},
    models::*,
    retry::{RetryExecutor, RetryPolicy},
};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

use tracing::{debug, error};
use url::Url;

/// Asynchronous AgentMem client
pub struct AsyncAgentMemClient {
    client: Client,
    config: ClientConfig,
    retry_executor: RetryExecutor,
}

impl AsyncAgentMemClient {
    /// Create a new async client
    pub fn new(config: ClientConfig) -> ClientResult<Self> {
        // Validate configuration
        config.validate().map_err(ClientError::ConfigError)?;

        // Create HTTP client
        let client = Client::builder()
            .timeout(config.timeout)
            .connect_timeout(config.connect_timeout)
            .user_agent(&config.user_agent)
            .pool_max_idle_per_host(config.pool_max_idle_per_host)
            .pool_idle_timeout(config.pool_idle_timeout)
            .build()
            .map_err(ClientError::HttpError)?;

        // Create retry policy
        let retry_policy = RetryPolicy::new(config.max_retries)
            .with_base_delay(config.retry_base_delay)
            .with_max_delay(config.retry_max_delay);

        let retry_executor = RetryExecutor::new(retry_policy);

        Ok(Self {
            client,
            config,
            retry_executor,
        })
    }

    /// Add a new memory
    pub async fn add_memory(&self, request: AddMemoryRequest) -> ClientResult<MemoryResponse> {
        let url = self.build_url("/api/v1/memories")?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.post(&url).json(&request).send().await?;

                self.handle_response(response).await
            })
            .await
    }

    /// Get a memory by ID
    pub async fn get_memory(&self, memory_id: &str) -> ClientResult<Memory> {
        let url = self.build_url(&format!("/api/v1/memories/{memory_id}"))?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.get(&url).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Search memories
    pub async fn search_memories(
        &self,
        request: SearchMemoriesRequest,
    ) -> ClientResult<SearchMemoriesResponse> {
        let url = self.build_url("/api/v1/memories/search")?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.post(&url).json(&request).send().await?;

                self.handle_response(response).await
            })
            .await
    }

    /// Mount a resource onto the preview file-centric surface.
    pub async fn mount_resource(
        &self,
        request: MountResourceRequest,
    ) -> ClientResult<ResourceDescriptor> {
        let url = self.build_url("/api/v1/resources/mount")?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.post(&url).json(&request).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Get a mounted resource descriptor by ID.
    pub async fn get_resource(&self, resource_id: &str) -> ClientResult<ResourceDescriptor> {
        let url = self.build_url(&format!("/api/v1/resources/{resource_id}"))?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.get(&url).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Start preview extraction for a mounted resource.
    pub async fn extract_resource(
        &self,
        request: ExtractionRequest,
    ) -> ClientResult<ExtractionResult> {
        let url = self.build_url("/api/v1/resources/extract")?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.post(&url).json(&request).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// List categories for a scope through the preview file-centric surface.
    pub async fn list_categories(
        &self,
        scope: &ScopeDescriptor,
    ) -> ClientResult<Vec<CategoryDescriptor>> {
        let url = self.build_url_with_query("/api/v1/categories", scope)?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.get(&url).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Search categories through the preview file-centric surface.
    pub async fn search_categories(
        &self,
        request: SearchCategoriesRequest,
    ) -> ClientResult<Vec<CategoryDescriptor>> {
        let url = self.build_url("/api/v1/categories/search")?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.post(&url).json(&request).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Plan a legacy migration through the preview file-centric surface.
    pub async fn plan_legacy_migration(
        &self,
        request: PlanMigrationRequest,
    ) -> ClientResult<MigrationPlan> {
        let url = self.build_url("/api/v1/migrations/plan")?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.post(&url).json(&request).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Apply a legacy migration through the preview file-centric surface.
    pub async fn apply_legacy_migration(
        &self,
        request: ApplyMigrationRequest,
    ) -> ClientResult<MigrationReport> {
        let url = self.build_url("/api/v1/migrations/apply")?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.post(&url).json(&request).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Roll back a legacy migration through the preview file-centric surface.
    pub async fn rollback_legacy_migration(
        &self,
        request: RollbackMigrationRequest,
    ) -> ClientResult<MigrationReport> {
        let url = self.build_url("/api/v1/migrations/rollback")?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.post(&url).json(&request).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// List proactive tasks for a scope.
    pub async fn list_proactive_tasks(
        &self,
        scope: &ScopeDescriptor,
    ) -> ClientResult<Vec<ProactiveTaskInfo>> {
        let url = self.build_url_with_query("/api/v1/proactive/tasks", scope)?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.get(&url).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Run a proactive task immediately.
    pub async fn run_proactive_task(
        &self,
        task_id: &str,
        request: RunProactiveTaskRequest,
    ) -> ClientResult<ProactiveTaskInfo> {
        let url = self.build_url(&format!("/api/v1/proactive/tasks/{task_id}/run"))?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.post(&url).json(&request).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Cancel a proactive task.
    pub async fn cancel_proactive_task(
        &self,
        task_id: &str,
        request: CancelProactiveTaskRequest,
    ) -> ClientResult<ProactiveTaskInfo> {
        let url = self.build_url(&format!("/api/v1/proactive/tasks/{task_id}/cancel"))?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.post(&url).json(&request).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Fetch scheduler statistics for the proactive plane.
    pub async fn get_scheduler_stats(&self) -> ClientResult<SchedulerStats> {
        let url = self.build_url("/api/v1/proactive/scheduler/stats")?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.get(&url).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Get health status
    pub async fn health_check(&self) -> ClientResult<HealthResponse> {
        let url = self.build_url("/health")?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.get(&url).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Get metrics
    pub async fn get_metrics(&self) -> ClientResult<MetricsResponse> {
        let url = self.build_url("/metrics")?;

        self.retry_executor
            .execute(|| async {
                let response = self.client.get(&url).send().await?;
                self.handle_response(response).await
            })
            .await
    }

    /// Build full URL from path
    fn build_url(&self, path: &str) -> ClientResult<String> {
        let base_url = Url::parse(&self.config.base_url)?;
        let full_url = base_url.join(path)?;
        Ok(full_url.to_string())
    }

    /// Build full URL with scope-based query parameters.
    fn build_url_with_query(&self, path: &str, scope: &ScopeDescriptor) -> ClientResult<String> {
        let mut url = Url::parse(&self.config.base_url)?.join(path)?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("user_id", &scope.user_id);
            if let Some(agent_id) = &scope.agent_id {
                pairs.append_pair("agent_id", agent_id);
            }
        }
        Ok(url.to_string())
    }

    /// Handle HTTP response and deserialize JSON
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> ClientResult<T> {
        let status = response.status();

        if self.config.enable_logging {
            debug!("HTTP response: {} {}", status, response.url());
        }

        if status.is_success() {
            let body = response.text().await?;

            if self.config.enable_logging {
                debug!("Response body: {}", body);
            }

            serde_json::from_str(&body).map_err(|e| {
                error!("Failed to deserialize response: {}", e);
                ClientError::InvalidResponse(format!("JSON deserialization failed: {e}"))
            })
        } else {
            let body = response.text().await.unwrap_or_default();

            // Try to parse error response
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&body) {
                error!(
                    "Server error: {} - {}",
                    error_response.code, error_response.message
                );
                Err(ClientError::ServerError {
                    status: status.as_u16(),
                    message: error_response.message,
                })
            } else {
                error!("HTTP error {}: {}", status, body);
                Err(ClientError::ServerError {
                    status: status.as_u16(),
                    message: body,
                })
            }
        }
    }
}

/// Synchronous AgentMem client (wrapper around async client)
pub struct AgentMemClient {
    async_client: AsyncAgentMemClient,
    runtime: tokio::runtime::Runtime,
}

impl AgentMemClient {
    /// Create a new sync client
    pub fn new(config: ClientConfig) -> ClientResult<Self> {
        let async_client = AsyncAgentMemClient::new(config)?;
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| ClientError::InternalError(format!("Failed to create runtime: {e}")))?;

        Ok(Self {
            async_client,
            runtime,
        })
    }

    /// Add a new memory (sync)
    pub fn add_memory(&self, request: AddMemoryRequest) -> ClientResult<MemoryResponse> {
        self.runtime.block_on(self.async_client.add_memory(request))
    }

    /// Get a memory by ID (sync)
    pub fn get_memory(&self, memory_id: &str) -> ClientResult<Memory> {
        self.runtime
            .block_on(self.async_client.get_memory(memory_id))
    }

    /// Search memories (sync)
    pub fn search_memories(
        &self,
        request: SearchMemoriesRequest,
    ) -> ClientResult<SearchMemoriesResponse> {
        self.runtime
            .block_on(self.async_client.search_memories(request))
    }

    /// Mount a resource (sync).
    pub fn mount_resource(
        &self,
        request: MountResourceRequest,
    ) -> ClientResult<ResourceDescriptor> {
        self.runtime
            .block_on(self.async_client.mount_resource(request))
    }

    /// Get a resource descriptor by ID (sync).
    pub fn get_resource(&self, resource_id: &str) -> ClientResult<ResourceDescriptor> {
        self.runtime
            .block_on(self.async_client.get_resource(resource_id))
    }

    /// Extract a resource (sync).
    pub fn extract_resource(&self, request: ExtractionRequest) -> ClientResult<ExtractionResult> {
        self.runtime
            .block_on(self.async_client.extract_resource(request))
    }

    /// List categories (sync).
    pub fn list_categories(
        &self,
        scope: &ScopeDescriptor,
    ) -> ClientResult<Vec<CategoryDescriptor>> {
        self.runtime
            .block_on(self.async_client.list_categories(scope))
    }

    /// Search categories (sync).
    pub fn search_categories(
        &self,
        request: SearchCategoriesRequest,
    ) -> ClientResult<Vec<CategoryDescriptor>> {
        self.runtime
            .block_on(self.async_client.search_categories(request))
    }

    /// Plan a legacy migration (sync).
    pub fn plan_legacy_migration(
        &self,
        request: PlanMigrationRequest,
    ) -> ClientResult<MigrationPlan> {
        self.runtime
            .block_on(self.async_client.plan_legacy_migration(request))
    }

    /// Apply a legacy migration (sync).
    pub fn apply_legacy_migration(
        &self,
        request: ApplyMigrationRequest,
    ) -> ClientResult<MigrationReport> {
        self.runtime
            .block_on(self.async_client.apply_legacy_migration(request))
    }

    /// Roll back a legacy migration (sync).
    pub fn rollback_legacy_migration(
        &self,
        request: RollbackMigrationRequest,
    ) -> ClientResult<MigrationReport> {
        self.runtime
            .block_on(self.async_client.rollback_legacy_migration(request))
    }

    /// List proactive tasks (sync).
    pub fn list_proactive_tasks(
        &self,
        scope: &ScopeDescriptor,
    ) -> ClientResult<Vec<ProactiveTaskInfo>> {
        self.runtime
            .block_on(self.async_client.list_proactive_tasks(scope))
    }

    /// Run a proactive task (sync).
    pub fn run_proactive_task(
        &self,
        task_id: &str,
        request: RunProactiveTaskRequest,
    ) -> ClientResult<ProactiveTaskInfo> {
        self.runtime
            .block_on(self.async_client.run_proactive_task(task_id, request))
    }

    /// Cancel a proactive task (sync).
    pub fn cancel_proactive_task(
        &self,
        task_id: &str,
        request: CancelProactiveTaskRequest,
    ) -> ClientResult<ProactiveTaskInfo> {
        self.runtime
            .block_on(self.async_client.cancel_proactive_task(task_id, request))
    }

    /// Fetch scheduler statistics (sync).
    pub fn get_scheduler_stats(&self) -> ClientResult<SchedulerStats> {
        self.runtime
            .block_on(self.async_client.get_scheduler_stats())
    }

    /// Get health status (sync)
    pub fn health_check(&self) -> ClientResult<HealthResponse> {
        self.runtime.block_on(self.async_client.health_check())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        matchers::{method, path, query_param},
        Mock, MockServer, ResponseTemplate,
    };

    const RESOURCE_DESCRIPTOR_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/file-centric-fixtures/resource_descriptor.json"
    ));
    const CATEGORY_DESCRIPTOR_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/file-centric-fixtures/category_descriptor.json"
    ));
    const ERROR_RESPONSE_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/file-centric-fixtures/error_response.json"
    ));

    fn expected_resource_descriptor() -> ResourceDescriptor {
        serde_json::from_str(RESOURCE_DESCRIPTOR_FIXTURE).unwrap()
    }

    fn expected_category_descriptor() -> CategoryDescriptor {
        serde_json::from_str(CATEGORY_DESCRIPTOR_FIXTURE).unwrap()
    }

    fn expected_error_response() -> ErrorResponse {
        serde_json::from_str(ERROR_RESPONSE_FIXTURE).unwrap()
    }

    #[tokio::test]
    async fn test_async_client_creation() {
        let config = ClientConfig::new("http://localhost:8080");
        let client = AsyncAgentMemClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_sync_client_creation() {
        let config = ClientConfig::new("http://localhost:8080");
        let client = AgentMemClient::new(config);
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_url_building() {
        let config = ClientConfig::new("http://localhost:8080");
        let client = AsyncAgentMemClient::new(config).unwrap();

        let url = client.build_url("/api/v1/memories").unwrap();
        assert_eq!(url, "http://localhost:8080/api/v1/memories");

        let url = client.build_url("/health").unwrap();
        assert_eq!(url, "http://localhost:8080/health");
    }

    #[tokio::test]
    async fn test_mount_resource_preview_route() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/resources/mount"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/json")
                    .set_body_string(RESOURCE_DESCRIPTOR_FIXTURE),
            )
            .mount(&server)
            .await;

        let client = AsyncAgentMemClient::new(ClientConfig::new(server.uri())).unwrap();
        let resource = client
            .mount_resource(MountResourceRequest::new(
                "file:///tmp/note.md",
                ScopeDescriptor {
                    user_id: "user-123".to_string(),
                    agent_id: Some("agent-abc".to_string()),
                },
            ))
            .await
            .unwrap();

        let expected = expected_resource_descriptor();
        assert_eq!(resource.id, expected.id);
        assert_eq!(resource.uri, expected.uri);
    }

    #[tokio::test]
    async fn test_list_categories_includes_scope_query_params() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/categories"))
            .and(query_param("user_id", "user-123"))
            .and(query_param("agent_id", "agent-abc"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/json")
                    .set_body_raw(
                        format!("[{CATEGORY_DESCRIPTOR_FIXTURE}]"),
                        "application/json",
                    ),
            )
            .mount(&server)
            .await;

        let client = AsyncAgentMemClient::new(ClientConfig::new(server.uri())).unwrap();
        let categories = client
            .list_categories(&ScopeDescriptor {
                user_id: "user-123".to_string(),
                agent_id: Some("agent-abc".to_string()),
            })
            .await
            .unwrap();

        let expected = expected_category_descriptor();
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].id, expected.id);
        assert_eq!(categories[0].path, expected.path);
    }

    #[tokio::test]
    async fn test_search_categories_surfaces_preview_errors() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/categories/search"))
            .respond_with(
                ResponseTemplate::new(501)
                    .insert_header("content-type", "application/json")
                    .set_body_string(ERROR_RESPONSE_FIXTURE),
            )
            .mount(&server)
            .await;

        let client = AsyncAgentMemClient::new(ClientConfig::new(server.uri())).unwrap();
        let error = client
            .search_categories(SearchCategoriesRequest::new(
                ScopeDescriptor {
                    user_id: "user-123".to_string(),
                    agent_id: None,
                },
                "communication",
            ))
            .await
            .unwrap_err();

        let expected = expected_error_response();
        match error {
            ClientError::ServerError { status, message } => {
                assert_eq!(status, 501);
                assert_eq!(message, expected.message);
            }
            other => panic!("Expected preview server error, got {other:?}"),
        }
    }
}
