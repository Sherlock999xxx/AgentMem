/**
 * AgentMem JavaScript SDK - Type Definitions
 * 
 * Core data types and interfaces for the AgentMem JavaScript SDK.
 */

/**
 * Memory type enumeration
 */
export enum MemoryType {
  EPISODIC = 'episodic',
  SEMANTIC = 'semantic',
  PROCEDURAL = 'procedural',
  UNTYPED = 'untyped',
}

/**
 * Memory importance level
 */
export enum ImportanceLevel {
  LOW = 1,
  MEDIUM = 2,
  HIGH = 3,
  CRITICAL = 4,
}

/**
 * Search match type
 */
export enum MatchType {
  EXACT_TEXT = 'exact_text',
  PARTIAL_TEXT = 'partial_text',
  SEMANTIC = 'semantic',
  METADATA = 'metadata',
}

/**
 * Memory data structure
 */
export interface Memory {
  id: string;
  content: string;
  memory_type: MemoryType;
  agent_id: string;
  user_id?: string;
  session_id?: string;
  importance: number;
  metadata?: Record<string, any>;
  created_at?: string;
  updated_at?: string;
  access_count: number;
  last_accessed?: string;
  embedding?: number[];
}

/**
 * Search query parameters
 */
export interface SearchQuery {
  agent_id: string;
  text_query?: string;
  vector_query?: number[];
  memory_type?: MemoryType;
  user_id?: string;
  min_importance?: number;
  max_age_seconds?: number;
  limit?: number;
  metadata_filters?: Record<string, any>;
}

/**
 * Search result with score and match type
 */
export interface SearchResult {
  memory: Memory;
  score: number;
  match_type: MatchType;
}

/**
 * Memory statistics
 */
export interface MemoryStats {
  total_memories: number;
  memories_by_type: Record<string, number>;
  memories_by_agent: Record<string, number>;
  average_importance: number;
  oldest_memory_age_days: number;
  most_accessed_memory_id?: string;
  total_access_count: number;
}

/**
 * Client configuration options
 */
export interface Config {
  /** API key for authentication */
  apiKey: string;
  
  /** Base URL for the AgentMem API */
  baseUrl?: string;
  
  /** API version */
  apiVersion?: string;
  
  /** Request timeout in milliseconds */
  timeout?: number;
  
  /** Maximum number of retry attempts */
  maxRetries?: number;
  
  /** Delay between retries in milliseconds */
  retryDelay?: number;
  
  /** Enable request/response compression */
  enableCompression?: boolean;
  
  /** Enable response caching */
  enableCaching?: boolean;
  
  /** Cache TTL in milliseconds */
  cacheTtl?: number;
  
  /** Enable debug logging */
  enableLogging?: boolean;
  
  /** Custom headers to include in requests */
  customHeaders?: Record<string, string>;
}

/**
 * Memory creation parameters
 */
export interface CreateMemoryParams {
  content: string;
  agent_id: string;
  memory_type?: MemoryType;
  user_id?: string;
  session_id?: string;
  importance?: number;
  metadata?: Record<string, any>;
}

/**
 * Memory update parameters
 */
export interface UpdateMemoryParams {
  content?: string;
  importance?: number;
  metadata?: Record<string, any>;
}

/**
 * Batch memory creation parameters
 */
export interface BatchCreateMemoryParams {
  memories: CreateMemoryParams[];
}

/**
 * API response wrapper
 */
export interface ApiResponse<T = any> {
  data?: T;
  error?: string;
  message?: string;
  status: number;
}

/**
 * Health check response
 */
export interface HealthStatus {
  status: 'healthy' | 'unhealthy';
  version: string;
  uptime: number;
  timestamp: string;
  services: Record<string, 'up' | 'down'>;
}

/**
 * System metrics
 */
export interface SystemMetrics {
  requests_per_second: number;
  average_response_time: number;
  active_connections: number;
  memory_usage: number;
  cpu_usage: number;
  cache_hit_rate: number;
}

/**
 * Error types
 */
export class AgentMemError extends Error {
  constructor(message: string, public statusCode?: number, public code?: string) {
    super(message);
    this.name = 'AgentMemError';
  }
}

export class AuthenticationError extends AgentMemError {
  constructor(message: string = 'Authentication failed') {
    super(message, 401, 'AUTHENTICATION_ERROR');
    this.name = 'AuthenticationError';
  }
}

export class ValidationError extends AgentMemError {
  constructor(message: string = 'Request validation failed') {
    super(message, 400, 'VALIDATION_ERROR');
    this.name = 'ValidationError';
  }
}

export class NetworkError extends AgentMemError {
  constructor(message: string = 'Network error') {
    super(message, 0, 'NETWORK_ERROR');
    this.name = 'NetworkError';
  }
}

export class NotFoundError extends AgentMemError {
  constructor(message: string = 'Resource not found') {
    super(message, 404, 'NOT_FOUND_ERROR');
    this.name = 'NotFoundError';
  }
}

export class RateLimitError extends AgentMemError {
  constructor(message: string = 'Rate limit exceeded') {
    super(message, 429, 'RATE_LIMIT_ERROR');
    this.name = 'RateLimitError';
  }
}

export class ServerError extends AgentMemError {
  constructor(message: string = 'Server error') {
    super(message, 500, 'SERVER_ERROR');
    this.name = 'ServerError';
  }
}

/**
 * Request options for API calls
 */
export interface RequestOptions {
  timeout?: number;
  retries?: number;
  useCache?: boolean;
  headers?: Record<string, string>;
}

// ============================================================================
// File-Centric Types (Phase D1)
// ============================================================================

/**
 * Lifecycle state for mounted resources
 */
export enum ResourceStatus {
  PENDING = 'pending',
  MOUNTED = 'mounted',
  FAILED = 'failed',
  UNMOUNTED = 'unmounted',
}

/**
 * Lifecycle state for categories
 */
export enum CategoryStatus {
  ACTIVE = 'active',
  ARCHIVED = 'archived',
  DELETED = 'deleted',
}

/**
 * Cross-language status model for async and long-running operations
 */
export enum OperationStatus {
  PENDING = 'pending',
  RUNNING = 'running',
  SUCCEEDED = 'succeeded',
  FAILED = 'failed',
  CANCELLED = 'cancelled',
}

/**
 * File-centric error code baseline for SDK alignment
 */
export enum PlatformErrorCode {
  VALIDATION_ERROR = 'validation_error',
  CATEGORY_NOT_FOUND = 'category_not_found',
  RESOURCE_URI_CONFLICT = 'resource_uri_conflict',
  MIGRATION_CONFLICT = 'migration_conflict',
  TASK_TIMEOUT = 'task_timeout',
  BACKGROUND_TASK_UNAVAILABLE = 'background_task_unavailable',
}

/**
 * Multi-tenant ownership scope
 */
export interface ScopeDescriptor {
  user_id: string;
  agent_id: string;
}

/**
 * Open metadata surface for resources
 */
export interface ResourceMetadataDescriptor {
  author?: string;
  tags?: string[];
  size_bytes?: number;
  modified_at?: string;
  attributes?: Record<string, string>;
}

/**
 * Open metadata surface for categories
 */
export interface CategoryMetadataDescriptor {
  tags?: string[];
  attributes?: Record<string, string>;
}

/**
 * Stable resource DTO for the file-centric public contract
 */
export interface ResourceDescriptor {
  id: string;
  uri: string;
  media_type: string;
  status: ResourceStatus;
  scope: ScopeDescriptor;
  metadata: ResourceMetadataDescriptor;
  created_at: string;
  updated_at: string;
}

/**
 * Stable category DTO for the file-centric public contract
 */
export interface CategoryDescriptor {
  id: string;
  path: string;
  name: string;
  parent_id?: string;
  children_ids: string[];
  summary?: string;
  item_count: number;
  status: CategoryStatus;
  scope: ScopeDescriptor;
  metadata: CategoryMetadataDescriptor;
  created_at: string;
  updated_at: string;
}

/**
 * Entity extracted from a resource
 */
export interface ExtractedEntity {
  id: string;
  name: string;
  entity_type: string;
  confidence: number;
  attributes?: Record<string, string>;
  span_start?: number;
  span_end?: number;
}

/**
 * Relation extracted from a resource
 */
export interface ExtractedRelation {
  id: string;
  subject_id: string;
  subject: string;
  predicate: string;
  object_id: string;
  object: string;
  relation_type: string;
  confidence: number;
  attributes?: Record<string, string>;
}

/**
 * Request to extract structured data from a mounted resource
 */
export interface ExtractionRequest {
  resource_id: string;
  scope: ScopeDescriptor;
  category_hint_paths?: string[];
  persist_output: boolean;
  include_entities: boolean;
  include_relations: boolean;
}

/**
 * Result of an extraction job
 */
export interface ExtractionResult {
  job_id: string;
  resource_id: string;
  status: OperationStatus;
  category_paths: string[];
  memory_ids: string[];
  entities: ExtractedEntity[];
  relations: ExtractedRelation[];
  warnings: string[];
  error_code?: PlatformErrorCode;
  error_message?: string;
  duration_ms: number;
  started_at: string;
  completed_at?: string;
}

/**
 * Plan for migrating legacy memories to file-centric surface
 */
export interface MigrationPlan {
  plan_id: string;
  scope: ScopeDescriptor;
  dry_run: boolean;
  source_surface: string;
  target_surface: string;
  legacy_memory_count: number;
  projected_resource_count: number;
  projected_category_count: number;
  warnings: string[];
  created_at: string;
}

/**
 * Report of a migration execution
 */
export interface MigrationReport {
  migration_id: string;
  plan_id: string;
  dry_run: boolean;
  status: OperationStatus;
  migrated_memories: number;
  mounted_resources: number;
  created_categories: number;
  conflicts: string[];
  warnings: string[];
  errors: string[];
  error_code?: PlatformErrorCode;
  rollback_available: boolean;
  started_at: string;
  completed_at?: string;
}

/**
 * Information about a proactive background task
 */
export interface ProactiveTaskInfo {
  id: string;
  task_type: string;
  status: OperationStatus;
  scope: ScopeDescriptor;
  schedule: string;
  pending_runs: number;
  running_count: number;
  last_started_at?: string;
  last_completed_at?: string;
  last_error_code?: PlatformErrorCode;
  last_error?: string;
}

/**
 * Statistics about the proactive task scheduler
 */
export interface SchedulerStats {
  state: string;
  total_tasks: number;
  running_tasks: number;
  completed_tasks: number;
  failed_tasks: number;
  cancelled_tasks: number;
  total_execution_time_ms: number;
  last_error?: string;
  updated_at: string;
}

/**
 * Standard error response structure
 */
export interface ErrorResponse {
  code: PlatformErrorCode;
  message: string;
  details: Record<string, any>;
  timestamp: string;
}
