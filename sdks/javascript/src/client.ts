/**
 * AgentMem JavaScript SDK - Main Client
 * 
 * Official JavaScript/TypeScript client for AgentMem API.
 */

import axios, { AxiosInstance, AxiosResponse, AxiosError } from 'axios';
import axiosRetry from 'axios-retry';
import {
  Config,
  Memory,
  MemoryType,
  SearchQuery,
  SearchResult,
  MemoryStats,
  CreateMemoryParams,
  UpdateMemoryParams,
  BatchCreateMemoryParams,
  HealthStatus,
  SystemMetrics,
  RequestOptions,
  AgentMemError,
  AuthenticationError,
  ValidationError,
  NetworkError,
  NotFoundError,
  RateLimitError,
  ServerError,
  // File-centric types (Phase D1)
  ResourceDescriptor,
  CategoryDescriptor,
  ScopeDescriptor,
  ExtractionRequest,
  ExtractionResult,
  MigrationPlan,
  MigrationReport,
  ProactiveTaskInfo,
  SchedulerStats,
  OperationStatus,
} from './types';
import { createConfig, getApiBaseUrl, getDefaultHeaders } from './config';

/**
 * Cache entry interface
 */
interface CacheEntry {
  data: any;
  timestamp: number;
}

/**
 * AgentMem JavaScript client for interacting with AgentMem API
 * 
 * @example
 * ```typescript
 * import { AgentMemClient, MemoryType } from '@agentmem/client';
 * 
 * const client = new AgentMemClient({
 *   apiKey: 'your-api-key',
 *   baseUrl: 'https://api.agentmem.dev'
 * });
 * 
 * // Add a memory
 * const memoryId = await client.addMemory({
 *   content: 'Important project information',
 *   agent_id: 'agent_1',
 *   memory_type: MemoryType.SEMANTIC,
 *   importance: 0.8
 * });
 * 
 * // Search memories
 * const results = await client.searchMemories({
 *   agent_id: 'agent_1',
 *   text_query: 'project information',
 *   limit: 5
 * });
 * ```
 */
export class AgentMemClient {
  private config: Config;
  private httpClient: AxiosInstance;
  private cache: Map<string, CacheEntry> = new Map();

  /**
   * Initialize AgentMem client
   */
  constructor(config: Partial<Config>) {
    this.config = createConfig(config);
    this.httpClient = this.createHttpClient();
    this.setupRetryLogic();
  }

  /**
   * Create HTTP client instance
   */
  private createHttpClient(): AxiosInstance {
    const client = axios.create({
      baseURL: getApiBaseUrl(this.config),
      timeout: this.config.timeout,
      headers: getDefaultHeaders(this.config),
    });

    // Request interceptor for logging
    if (this.config.enableLogging) {
      client.interceptors.request.use(
        (config) => {
          console.log(`[AgentMem] ${config.method?.toUpperCase()} ${config.url}`);
          return config;
        },
        (error) => {
          console.error('[AgentMem] Request error:', error);
          return Promise.reject(error);
        }
      );
    }

    // Response interceptor for error handling
    client.interceptors.response.use(
      (response) => {
        if (this.config.enableLogging) {
          console.log(`[AgentMem] Response ${response.status} ${response.config.url}`);
        }
        return response;
      },
      (error) => {
        return Promise.reject(this.handleError(error));
      }
    );

    return client;
  }

  /**
   * Setup retry logic
   */
  private setupRetryLogic(): void {
    axiosRetry(this.httpClient, {
      retries: this.config.maxRetries,
      retryDelay: (retryCount) => {
        return this.config.retryDelay * Math.pow(2, retryCount - 1); // Exponential backoff
      },
      retryCondition: (error) => {
        // Retry on network errors and 5xx status codes
        return axiosRetry.isNetworkOrIdempotentRequestError(error) ||
               (error.response?.status >= 500 && error.response?.status < 600);
      },
    });
  }

  /**
   * Handle HTTP errors and convert to AgentMem errors
   */
  private handleError(error: AxiosError): AgentMemError {
    if (error.response) {
      const status = error.response.status;
      const message = (error.response.data as any)?.message || error.message;

      switch (status) {
        case 401:
          return new AuthenticationError(message);
        case 400:
          return new ValidationError(message);
        case 404:
          return new NotFoundError(message);
        case 429:
          return new RateLimitError(message);
        case 500:
        case 502:
        case 503:
        case 504:
          return new ServerError(message);
        default:
          return new AgentMemError(message, status);
      }
    } else if (error.request) {
      return new NetworkError('Network error: ' + error.message);
    } else {
      return new AgentMemError('Request error: ' + error.message);
    }
  }

  /**
   * Generate cache key
   */
  private getCacheKey(method: string, url: string, params?: any): string {
    const keyParts = [method, url];
    if (params) {
      keyParts.push(JSON.stringify(params));
    }
    return keyParts.join('|');
  }

  /**
   * Check if cache entry is valid
   */
  private isCacheValid(entry: CacheEntry): boolean {
    const age = Date.now() - entry.timestamp;
    return age < this.config.cacheTtl;
  }

  /**
   * Get from cache
   */
  private getFromCache(key: string): any | null {
    if (!this.config.enableCaching) {
      return null;
    }

    const entry = this.cache.get(key);
    if (entry && this.isCacheValid(entry)) {
      return entry.data;
    }

    // Clean up expired entry
    if (entry) {
      this.cache.delete(key);
    }

    return null;
  }

  /**
   * Set cache entry
   */
  private setCache(key: string, data: any): void {
    if (this.config.enableCaching) {
      this.cache.set(key, {
        data,
        timestamp: Date.now(),
      });
    }
  }

  /**
   * Make HTTP request with caching support
   */
  private async makeRequest<T>(
    method: string,
    endpoint: string,
    data?: any,
    options: RequestOptions = {}
  ): Promise<T> {
    const url = endpoint;
    
    // Check cache for GET requests
    if (method === 'GET' && options.useCache !== false) {
      const cacheKey = this.getCacheKey(method, url, data);
      const cachedResult = this.getFromCache(cacheKey);
      if (cachedResult !== null) {
        return cachedResult;
      }
    }

    // Prepare request config
    const requestConfig: any = {
      method,
      url,
      timeout: options.timeout || this.config.timeout,
      headers: { ...getDefaultHeaders(this.config), ...options.headers },
    };

    if (data) {
      if (method === 'GET') {
        requestConfig.params = data;
      } else {
        requestConfig.data = data;
      }
    }

    // Make request
    const response: AxiosResponse<T> = await this.httpClient.request(requestConfig);
    
    // Cache successful GET requests
    if (method === 'GET' && options.useCache !== false) {
      const cacheKey = this.getCacheKey(method, url, data);
      this.setCache(cacheKey, response.data);
    }

    return response.data;
  }

  /**
   * Add a new memory
   */
  async addMemory(params: CreateMemoryParams): Promise<string> {
    const response = await this.makeRequest<{ id: string }>('POST', '/memories', params);
    return response.id;
  }

  /**
   * Get a memory by ID
   */
  async getMemory(memoryId: string): Promise<Memory> {
    return this.makeRequest<Memory>('GET', `/memories/${memoryId}`, undefined, { useCache: true });
  }

  /**
   * Update an existing memory
   */
  async updateMemory(memoryId: string, params: UpdateMemoryParams): Promise<Memory> {
    return this.makeRequest<Memory>('PUT', `/memories/${memoryId}`, params);
  }

  /**
   * Delete a memory
   */
  async deleteMemory(memoryId: string): Promise<void> {
    await this.makeRequest<void>('DELETE', `/memories/${memoryId}`);
  }

  /**
   * Search memories
   */
  async searchMemories(query: SearchQuery): Promise<SearchResult[]> {
    const response = await this.makeRequest<{ results: SearchResult[] }>('POST', '/memories/search', query);
    return response.results;
  }

  /**
   * Add multiple memories in batch
   */
  async batchAddMemories(params: BatchCreateMemoryParams): Promise<string[]> {
    const response = await this.makeRequest<{ ids: string[] }>('POST', '/memories/batch', params);
    return response.ids;
  }

  /**
   * Get memory statistics for an agent
   */
  async getMemoryStats(agentId: string): Promise<MemoryStats> {
    return this.makeRequest<MemoryStats>('GET', '/memories/stats', { agent_id: agentId }, { useCache: true });
  }

  /**
   * Check API health status
   */
  async healthCheck(): Promise<HealthStatus> {
    return this.makeRequest<HealthStatus>('GET', '/health', undefined, { useCache: true });
  }

  /**
   * Get system metrics
   */
  async getMetrics(): Promise<SystemMetrics> {
    return this.makeRequest<SystemMetrics>('GET', '/metrics', undefined, { useCache: true });
  }

  /**
   * Clear cache
   */
  clearCache(): void {
    this.cache.clear();
  }

  /**
   * Get current configuration (with masked API key)
   */
  getConfig(): Partial<Config> {
    return {
      ...this.config,
      apiKey: '***', // Mask API key for security
    };
  }

  // ============================================================================
  // File-Centric API Methods (Phase D1)
  // ============================================================================

  // Resource Operations

  /**
   * Mount a file-like resource
   */
  async mountResource(params: {
    uri: string;
    media_type: string;
    scope: ScopeDescriptor;
    metadata?: any;
  }): Promise<ResourceDescriptor> {
    return this.makeRequest<ResourceDescriptor>('POST', '/file-centric/resources', params);
  }

  /**
   * Get a resource by ID
   */
  async getResource(resourceId: string): Promise<ResourceDescriptor> {
    return this.makeRequest<ResourceDescriptor>('GET', `/file-centric/resources/${resourceId}`, undefined, { useCache: true });
  }

  /**
   * List resources with optional filters
   */
  async listResources(params?: {
    scope?: ScopeDescriptor;
    status?: string;
    limit?: number;
    offset?: number;
  }): Promise<ResourceDescriptor[]> {
    const response = await this.makeRequest<{ resources: ResourceDescriptor[] }>('GET', '/file-centric/resources', params, { useCache: true });
    return response.resources;
  }

  // Category Operations

  /**
   * Get a category by ID
   */
  async getCategory(categoryId: string): Promise<CategoryDescriptor> {
    return this.makeRequest<CategoryDescriptor>('GET', `/file-centric/categories/${categoryId}`, undefined, { useCache: true });
  }

  /**
   * Get a category by path
   */
  async getCategoryByPath(path: string, scope: ScopeDescriptor): Promise<CategoryDescriptor> {
    return this.makeRequest<CategoryDescriptor>('GET', '/file-centric/categories/by-path', { path, ...scope }, { useCache: true });
  }

  /**
   * List categories with optional filters
   */
  async listCategories(params?: {
    scope?: ScopeDescriptor;
    parent_id?: string;
    status?: string;
    limit?: number;
    offset?: number;
  }): Promise<CategoryDescriptor[]> {
    const response = await this.makeRequest<{ categories: CategoryDescriptor[] }>('GET', '/file-centric/categories', params, { useCache: true });
    return response.categories;
  }

  /**
   * Search categories by query
   */
  async searchCategories(params: {
    query: string;
    scope: ScopeDescriptor;
    limit?: number;
  }): Promise<CategoryDescriptor[]> {
    const response = await this.makeRequest<{ categories: CategoryDescriptor[] }>('POST', '/file-centric/categories/search', params);
    return response.categories;
  }

  // Extraction Operations

  /**
   * Extract structured data from a resource
   */
  async extractResource(params: ExtractionRequest): Promise<ExtractionResult> {
    return this.makeRequest<ExtractionResult>('POST', `/file-centric/resources/${params.resource_id}/extract`, params);
  }

  /**
   * Get extraction job status
   */
  async getExtractionStatus(jobId: string): Promise<ExtractionResult> {
    return this.makeRequest<ExtractionResult>('GET', `/file-centric/extractions/${jobId}`, undefined, { useCache: true });
  }

  // Migration Operations

  /**
   * Plan migration from legacy memories
   */
  async planLegacyMigration(params: {
    scope: ScopeDescriptor;
    dry_run: boolean;
  }): Promise<MigrationPlan> {
    return this.makeRequest<MigrationPlan>('POST', '/file-centric/migrations/plan', params);
  }

  /**
   * Apply a migration plan
   */
  async applyLegacyMigration(params: {
    plan_id: string;
    scope: ScopeDescriptor;
  }): Promise<MigrationReport> {
    return this.makeRequest<MigrationReport>('POST', '/file-centric/migrations/apply', params);
  }

  /**
   * Get migration status
   */
  async getMigrationStatus(migrationId: string): Promise<MigrationReport> {
    return this.makeRequest<MigrationReport>('GET', `/file-centric/migrations/${migrationId}`, undefined, { useCache: true });
  }

  /**
   * Rollback a migration
   */
  async rollbackMigration(migrationId: string): Promise<MigrationReport> {
    return this.makeRequest<MigrationReport>('POST', `/file-centric/migrations/${migrationId}/rollback`);
  }

  // Proactive Operations

  /**
   * List proactive background tasks
   */
  async listProactiveTasks(params?: {
    scope?: ScopeDescriptor;
    task_type?: string;
    status?: OperationStatus;
    limit?: number;
    offset?: number;
  }): Promise<ProactiveTaskInfo[]> {
    const response = await this.makeRequest<{ tasks: ProactiveTaskInfo[] }>('GET', '/file-centric/proactive/tasks', params, { useCache: true });
    return response.tasks;
  }

  /**
   * Get a proactive task by ID
   */
  async getProactiveTask(taskId: string): Promise<ProactiveTaskInfo> {
    return this.makeRequest<ProactiveTaskInfo>('GET', `/file-centric/proactive/tasks/${taskId}`, undefined, { useCache: true });
  }

  /**
   * Trigger a proactive task
   */
  async runProactiveTask(taskId: string): Promise<{ run_id: string; status: OperationStatus }> {
    return this.makeRequest<{ run_id: string; status: OperationStatus }>('POST', `/file-centric/proactive/tasks/${taskId}/run`);
  }

  /**
   * Cancel a running proactive task
   */
  async cancelProactiveTask(taskId: string, runId: string): Promise<{ status: OperationStatus }> {
    return this.makeRequest<{ status: OperationStatus }>('POST', `/file-centric/proactive/tasks/${taskId}/runs/${runId}/cancel`);
  }

  /**
   * Get scheduler statistics
   */
  async getSchedulerStats(): Promise<SchedulerStats> {
    return this.makeRequest<SchedulerStats>('GET', '/file-centric/proactive/scheduler/stats', undefined, { useCache: true });
  }
}
