package agentmem

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"sync"
	"time"

	"github.com/go-resty/resty/v2"
)

// cacheEntry represents a cached response
type cacheEntry struct {
	data      interface{}
	timestamp time.Time
}

// Client represents the AgentMem API client
type Client struct {
	config     *Config
	httpClient *resty.Client
	cache      map[string]*cacheEntry
	cacheMutex sync.RWMutex
}

// NewClient creates a new AgentMem client with the provided configuration
func NewClient(config *Config) (*Client, error) {
	if err := config.Validate(); err != nil {
		return nil, fmt.Errorf("invalid configuration: %w", err)
	}

	client := &Client{
		config: config,
		cache:  make(map[string]*cacheEntry),
	}

	client.setupHTTPClient()
	return client, nil
}

// NewClientFromEnv creates a new client using environment variables
func NewClientFromEnv() (*Client, error) {
	config, err := NewConfigFromEnv()
	if err != nil {
		return nil, err
	}
	return NewClient(config)
}

// setupHTTPClient configures the HTTP client
func (c *Client) setupHTTPClient() {
	c.httpClient = resty.New()
	c.httpClient.SetBaseURL(c.config.GetAPIBaseURL())
	c.httpClient.SetTimeout(c.config.Timeout)
	c.httpClient.SetHeaders(c.config.GetDefaultHeaders())

	// Enable compression if configured
	if c.config.EnableCompression {
		c.httpClient.SetHeader("Accept-Encoding", "gzip, deflate")
	}

	// Setup retry logic
	c.httpClient.SetRetryCount(c.config.MaxRetries)
	c.httpClient.SetRetryWaitTime(c.config.RetryDelay)
	c.httpClient.SetRetryMaxWaitTime(c.config.RetryDelay * 10)

	// Retry on server errors and network errors
	c.httpClient.AddRetryCondition(func(r *resty.Response, err error) bool {
		return err != nil || r.StatusCode() >= 500
	})

	// Setup logging if enabled
	if c.config.EnableLogging {
		c.httpClient.SetLogger(log.Default())
		c.httpClient.EnableTrace()
	}

	// Response middleware for error handling
	c.httpClient.OnAfterResponse(func(client *resty.Client, resp *resty.Response) error {
		if resp.IsError() {
			var errorMsg string
			if resp.Body() != nil {
				var apiResp APIResponse
				if err := json.Unmarshal(resp.Body(), &apiResp); err == nil && apiResp.Error != nil {
					errorMsg = *apiResp.Error
				} else if apiResp.Message != nil {
					errorMsg = *apiResp.Message
				}
			}
			if errorMsg == "" {
				errorMsg = fmt.Sprintf("HTTP %d: %s", resp.StatusCode(), resp.Status())
			}
			return handleHTTPError(resp.StatusCode(), errorMsg)
		}
		return nil
	})
}

// getCacheKey generates a cache key for the request
func (c *Client) getCacheKey(method, endpoint string, params interface{}) string {
	key := fmt.Sprintf("%s:%s", method, endpoint)
	if params != nil {
		if paramBytes, err := json.Marshal(params); err == nil {
			key += ":" + string(paramBytes)
		}
	}
	return key
}

// getFromCache retrieves data from cache if valid
func (c *Client) getFromCache(key string) (interface{}, bool) {
	if !c.config.EnableCaching {
		return nil, false
	}

	c.cacheMutex.RLock()
	defer c.cacheMutex.RUnlock()

	entry, exists := c.cache[key]
	if !exists {
		return nil, false
	}

	// Check if cache entry is still valid
	if time.Since(entry.timestamp) > c.config.CacheTTL {
		// Cache expired, remove it
		delete(c.cache, key)
		return nil, false
	}

	return entry.data, true
}

// setCache stores data in cache
func (c *Client) setCache(key string, data interface{}) {
	if !c.config.EnableCaching {
		return
	}

	c.cacheMutex.Lock()
	defer c.cacheMutex.Unlock()

	c.cache[key] = &cacheEntry{
		data:      data,
		timestamp: time.Now(),
	}
}

// makeRequest performs an HTTP request with caching support
func (c *Client) makeRequest(ctx context.Context, method, endpoint string, body interface{}, result interface{}, useCache bool) error {
	// Check cache for GET requests
	if method == "GET" && useCache {
		cacheKey := c.getCacheKey(method, endpoint, body)
		if cachedData, found := c.getFromCache(cacheKey); found {
			if c.config.EnableLogging {
				log.Printf("[AgentMem] Cache hit for %s %s", method, endpoint)
			}
			// Copy cached data to result
			if resultBytes, err := json.Marshal(cachedData); err == nil {
				return json.Unmarshal(resultBytes, result)
			}
		}
	}

	// Prepare request
	req := c.httpClient.R().SetContext(ctx)

	if body != nil {
		if method == "GET" {
			// For GET requests, body contains query parameters
			if params, ok := body.(map[string]interface{}); ok {
				for key, value := range params {
					req.SetQueryParam(key, fmt.Sprintf("%v", value))
				}
			}
		} else {
			req.SetBody(body)
		}
	}

	if result != nil {
		req.SetResult(result)
	}

	// Make request
	var resp *resty.Response
	var err error

	switch method {
	case "GET":
		resp, err = req.Get(endpoint)
	case "POST":
		resp, err = req.Post(endpoint)
	case "PUT":
		resp, err = req.Put(endpoint)
	case "DELETE":
		resp, err = req.Delete(endpoint)
	default:
		return fmt.Errorf("unsupported HTTP method: %s", method)
	}

	if err != nil {
		return NewNetworkError(fmt.Sprintf("Request failed: %v", err))
	}

	// Cache successful GET responses
	if method == "GET" && useCache && resp.IsSuccess() && result != nil {
		cacheKey := c.getCacheKey(method, endpoint, body)
		c.setCache(cacheKey, result)
	}

	return nil
}

// AddMemory adds a new memory
func (c *Client) AddMemory(ctx context.Context, params CreateMemoryParams) (string, error) {
	var response CreateMemoryResponse
	err := c.makeRequest(ctx, "POST", "/memories", params, &response, false)
	if err != nil {
		return "", err
	}
	return response.ID, nil
}

// GetMemory retrieves a memory by ID
func (c *Client) GetMemory(ctx context.Context, memoryID string) (*Memory, error) {
	var memory Memory
	err := c.makeRequest(ctx, "GET", fmt.Sprintf("/memories/%s", memoryID), nil, &memory, true)
	if err != nil {
		return nil, err
	}
	return &memory, nil
}

// UpdateMemory updates an existing memory
func (c *Client) UpdateMemory(ctx context.Context, memoryID string, params UpdateMemoryParams) (*Memory, error) {
	var memory Memory
	err := c.makeRequest(ctx, "PUT", fmt.Sprintf("/memories/%s", memoryID), params, &memory, false)
	if err != nil {
		return nil, err
	}
	return &memory, nil
}

// DeleteMemory deletes a memory
func (c *Client) DeleteMemory(ctx context.Context, memoryID string) error {
	return c.makeRequest(ctx, "DELETE", fmt.Sprintf("/memories/%s", memoryID), nil, nil, false)
}

// SearchMemories searches for memories
func (c *Client) SearchMemories(ctx context.Context, query SearchQuery) ([]SearchResult, error) {
	var response SearchResponse
	err := c.makeRequest(ctx, "POST", "/memories/search", query, &response, false)
	if err != nil {
		return nil, err
	}
	return response.Results, nil
}

// BatchAddMemories adds multiple memories in batch
func (c *Client) BatchAddMemories(ctx context.Context, params BatchCreateMemoryParams) ([]string, error) {
	var response BatchCreateResponse
	err := c.makeRequest(ctx, "POST", "/memories/batch", params, &response, false)
	if err != nil {
		return nil, err
	}
	return response.IDs, nil
}

// GetMemoryStats retrieves memory statistics for an agent
func (c *Client) GetMemoryStats(ctx context.Context, agentID string) (*MemoryStats, error) {
	var stats MemoryStats
	queryParams := map[string]interface{}{
		"agent_id": agentID,
	}
	err := c.makeRequest(ctx, "GET", "/memories/stats", queryParams, &stats, true)
	if err != nil {
		return nil, err
	}
	return &stats, nil
}

// HealthCheck checks API health status
func (c *Client) HealthCheck(ctx context.Context) (*HealthStatus, error) {
	var health HealthStatus
	err := c.makeRequest(ctx, "GET", "/health", nil, &health, true)
	if err != nil {
		return nil, err
	}
	return &health, nil
}

// GetMetrics retrieves system metrics
func (c *Client) GetMetrics(ctx context.Context) (*SystemMetrics, error) {
	var metrics SystemMetrics
	err := c.makeRequest(ctx, "GET", "/metrics", nil, &metrics, true)
	if err != nil {
		return nil, err
	}
	return &metrics, nil
}

// ClearCache clears the client's cache
func (c *Client) ClearCache() {
	c.cacheMutex.Lock()
	defer c.cacheMutex.Unlock()
	c.cache = make(map[string]*cacheEntry)
}

// GetConfig returns the client's configuration (with masked API key)
func (c *Client) GetConfig() *Config {
	config := c.config.Clone()
	config.APIKey = "***" // Mask API key for security
	return config
}

// ============================================================================
// File-Centric API Methods (Phase D2 - Go SDK Stabilization)
// ============================================================================

// --- Resource Operations ---

// MountResource mounts a resource from a URI
func (c *Client) MountResource(ctx context.Context, uri string, mediaType string, scope ScopeDescriptor, metadata *ResourceMetadataDescriptor) (*ResourceDescriptor, error) {
	data := map[string]interface{}{
		"uri":         uri,
		"media_type":  mediaType,
		"scope":       scope,
	}
	if metadata != nil {
		data["metadata"] = metadata
	}

	var response ResourceDescriptor
	err := c.makeRequest(ctx, "POST", "/api/v1/file-centric/resources", data, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}

// GetResource retrieves a resource by ID
func (c *Client) GetResource(ctx context.Context, resourceID string) (*ResourceDescriptor, error) {
	var response ResourceDescriptor
	err := c.makeRequest(ctx, "GET", fmt.Sprintf("/api/v1/file-centric/resources/%s", resourceID), nil, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}

// ListResources lists resources for a user/agent
func (c *Client) ListResources(ctx context.Context, userID, agentID string, status *ResourceStatus) ([]ResourceDescriptor, error) {
	params := map[string]string{
		"user_id":  userID,
		"agent_id": agentID,
	}
	if status != nil {
		params["status"] = string(*status)
	}

	var response struct {
		Resources []ResourceDescriptor `json:"resources"`
	}
	err := c.makeRequest(ctx, "GET", "/api/v1/file-centric/resources", params, &response, false)
	if err != nil {
		return nil, err
	}
	return response.Resources, nil
}

// --- Category Operations ---

// GetCategory retrieves a category by ID
func (c *Client) GetCategory(ctx context.Context, categoryID string) (*CategoryDescriptor, error) {
	var response CategoryDescriptor
	err := c.makeRequest(ctx, "GET", fmt.Sprintf("/api/v1/file-centric/categories/%s", categoryID), nil, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}

// GetCategoryByPath retrieves a category by path
func (c *Client) GetCategoryByPath(ctx context.Context, path, userID, agentID string) (*CategoryDescriptor, error) {
	params := map[string]string{
		"path":     path,
		"user_id":  userID,
		"agent_id": agentID,
	}

	var response CategoryDescriptor
	err := c.makeRequest(ctx, "GET", "/api/v1/file-centric/categories/by-path", params, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}

// ListCategories lists categories for a user/agent
func (c *Client) ListCategories(ctx context.Context, userID, agentID string, parentID *string) ([]CategoryDescriptor, error) {
	params := map[string]string{
		"user_id":  userID,
		"agent_id": agentID,
	}
	if parentID != nil {
		params["parent_id"] = *parentID
	}

	var response struct {
		Categories []CategoryDescriptor `json:"categories"`
	}
	err := c.makeRequest(ctx, "GET", "/api/v1/file-centric/categories", params, &response, false)
	if err != nil {
		return nil, err
	}
	return response.Categories, nil
}

// SearchCategories searches categories by query
func (c *Client) SearchCategories(ctx context.Context, userID, agentID, query string, limit *int) ([]CategoryDescriptor, error) {
	data := map[string]interface{}{
		"user_id":  userID,
		"agent_id": agentID,
		"query":    query,
	}
	if limit != nil {
		data["limit"] = *limit
	}

	var response struct {
		Categories []CategoryDescriptor `json:"categories"`
	}
	err := c.makeRequest(ctx, "POST", "/api/v1/file-centric/categories/search", data, &response, false)
	if err != nil {
		return nil, err
	}
	return response.Categories, nil
}

// --- Extraction Operations ---

// ExtractResource extracts structured data from a mounted resource
func (c *Client) ExtractResource(ctx context.Context, request ExtractionRequest) (*ExtractionResult, error) {
	var response ExtractionResult
	err := c.makeRequest(ctx, "POST", "/api/v1/file-centric/extraction/extract", request, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}

// GetExtractionStatus gets the status of an extraction job
func (c *Client) GetExtractionStatus(ctx context.Context, jobID string) (*ExtractionResult, error) {
	var response ExtractionResult
	err := c.makeRequest(ctx, "GET", fmt.Sprintf("/api/v1/file-centric/extraction/jobs/%s", jobID), nil, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}

// --- Migration Operations ---

// PlanLegacyMigration creates a migration plan for legacy memories
func (c *Client) PlanLegacyMigration(ctx context.Context, scope ScopeDescriptor, dryRun bool) (*MigrationPlan, error) {
	data := map[string]interface{}{
		"scope":    scope,
		"dry_run":  dryRun,
	}

	var response MigrationPlan
	err := c.makeRequest(ctx, "POST", "/api/v1/file-centric/migration/plan", data, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}

// ApplyLegacyMigration applies a migration plan
func (c *Client) ApplyLegacyMigration(ctx context.Context, planID string) (*MigrationReport, error) {
	data := map[string]interface{}{
		"plan_id": planID,
	}

	var response MigrationReport
	err := c.makeRequest(ctx, "POST", "/api/v1/file-centric/migration/apply", data, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}

// GetMigrationStatus gets the status of a migration
func (c *Client) GetMigrationStatus(ctx context.Context, migrationID string) (*MigrationReport, error) {
	var response MigrationReport
	err := c.makeRequest(ctx, "GET", fmt.Sprintf("/api/v1/file-centric/migration/migrations/%s", migrationID), nil, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}

// RollbackMigration rolls back a migration
func (c *Client) RollbackMigration(ctx context.Context, migrationID string) (bool, error) {
	var response struct {
		Success bool `json:"success"`
	}
	err := c.makeRequest(ctx, "POST", fmt.Sprintf("/api/v1/file-centric/migration/migrations/%s/rollback", migrationID), nil, &response, false)
	if err != nil {
		return false, err
	}
	return response.Success, nil
}

// --- Proactive Task Operations ---

// ListProactiveTasks lists proactive tasks for a user/agent
func (c *Client) ListProactiveTasks(ctx context.Context, userID, agentID string, taskType *string) ([]ProactiveTaskInfo, error) {
	params := map[string]string{
		"user_id":  userID,
		"agent_id": agentID,
	}
	if taskType != nil {
		params["task_type"] = *taskType
	}

	var response struct {
		Tasks []ProactiveTaskInfo `json:"tasks"`
	}
	err := c.makeRequest(ctx, "GET", "/api/v1/file-centric/proactive/tasks", params, &response, false)
	if err != nil {
		return nil, err
	}
	return response.Tasks, nil
}

// GetProactiveTask gets a proactive task by ID
func (c *Client) GetProactiveTask(ctx context.Context, taskID string) (*ProactiveTaskInfo, error) {
	var response ProactiveTaskInfo
	err := c.makeRequest(ctx, "GET", fmt.Sprintf("/api/v1/file-centric/proactive/tasks/%s", taskID), nil, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}

// RunProactiveTask triggers a proactive task to run immediately
func (c *Client) RunProactiveTask(ctx context.Context, taskID string) (*ProactiveTaskInfo, error) {
	var response ProactiveTaskInfo
	err := c.makeRequest(ctx, "POST", fmt.Sprintf("/api/v1/file-centric/proactive/tasks/%s/run", taskID), nil, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}

// CancelProactiveTask cancels a running proactive task
func (c *Client) CancelProactiveTask(ctx context.Context, taskID string) (*ProactiveTaskInfo, error) {
	var response ProactiveTaskInfo
	err := c.makeRequest(ctx, "POST", fmt.Sprintf("/api/v1/file-centric/proactive/tasks/%s/cancel", taskID), nil, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}

// GetSchedulerStats gets scheduler statistics
func (c *Client) GetSchedulerStats(ctx context.Context) (*SchedulerStats, error) {
	var response SchedulerStats
	err := c.makeRequest(ctx, "GET", "/api/v1/file-centric/proactive/scheduler/stats", nil, &response, false)
	if err != nil {
		return nil, err
	}
	return &response, nil
}
