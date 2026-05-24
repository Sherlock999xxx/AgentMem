// Package agentmem provides the official Go SDK for AgentMem API
// Enterprise-grade memory management for AI agents.
package agentmem

import (
	"time"
)

// MemoryType represents the type of memory
type MemoryType string

const (
	// MemoryTypeEpisodic represents event-based memories
	MemoryTypeEpisodic MemoryType = "episodic"
	// MemoryTypeSemantic represents factual knowledge
	MemoryTypeSemantic MemoryType = "semantic"
	// MemoryTypeProcedural represents how-to knowledge
	MemoryTypeProcedural MemoryType = "procedural"
	// MemoryTypeUntyped represents unclassified memories
	MemoryTypeUntyped MemoryType = "untyped"
)

// ImportanceLevel represents memory importance levels
type ImportanceLevel int

const (
	ImportanceLow ImportanceLevel = iota + 1
	ImportanceMedium
	ImportanceHigh
	ImportanceCritical
)

// MatchType represents search match types
type MatchType string

const (
	MatchTypeExactText   MatchType = "exact_text"
	MatchTypePartialText MatchType = "partial_text"
	MatchTypeSemantic    MatchType = "semantic"
	MatchTypeMetadata    MatchType = "metadata"
)

// Memory represents a memory record
type Memory struct {
	ID           string                 `json:"id"`
	Content      string                 `json:"content"`
	MemoryType   MemoryType             `json:"memory_type"`
	AgentID      string                 `json:"agent_id"`
	UserID       *string                `json:"user_id,omitempty"`
	SessionID    *string                `json:"session_id,omitempty"`
	Importance   float64                `json:"importance"`
	Metadata     map[string]interface{} `json:"metadata,omitempty"`
	CreatedAt    *time.Time             `json:"created_at,omitempty"`
	UpdatedAt    *time.Time             `json:"updated_at,omitempty"`
	AccessCount  int                    `json:"access_count"`
	LastAccessed *time.Time             `json:"last_accessed,omitempty"`
	Embedding    []float64              `json:"embedding,omitempty"`
}

// SearchQuery represents search parameters
type SearchQuery struct {
	AgentID         string                 `json:"agent_id"`
	TextQuery       *string                `json:"text_query,omitempty"`
	VectorQuery     []float64              `json:"vector_query,omitempty"`
	MemoryType      *MemoryType            `json:"memory_type,omitempty"`
	UserID          *string                `json:"user_id,omitempty"`
	MinImportance   *float64               `json:"min_importance,omitempty"`
	MaxAgeSeconds   *int                   `json:"max_age_seconds,omitempty"`
	Limit           int                    `json:"limit"`
	MetadataFilters map[string]interface{} `json:"metadata_filters,omitempty"`
}

// SearchResult represents a search result with score and match type
type SearchResult struct {
	Memory    Memory    `json:"memory"`
	Score     float64   `json:"score"`
	MatchType MatchType `json:"match_type"`
}

// MemoryStats represents memory statistics
type MemoryStats struct {
	TotalMemories           int            `json:"total_memories"`
	MemoriesByType          map[string]int `json:"memories_by_type"`
	MemoriesByAgent         map[string]int `json:"memories_by_agent"`
	AverageImportance       float64        `json:"average_importance"`
	OldestMemoryAgeDays     float64        `json:"oldest_memory_age_days"`
	MostAccessedMemoryID    *string        `json:"most_accessed_memory_id,omitempty"`
	TotalAccessCount        int            `json:"total_access_count"`
}

// CreateMemoryParams represents parameters for creating a memory
type CreateMemoryParams struct {
	Content    string                 `json:"content"`
	AgentID    string                 `json:"agent_id"`
	MemoryType *MemoryType            `json:"memory_type,omitempty"`
	UserID     *string                `json:"user_id,omitempty"`
	SessionID  *string                `json:"session_id,omitempty"`
	Importance *float64               `json:"importance,omitempty"`
	Metadata   map[string]interface{} `json:"metadata,omitempty"`
}

// UpdateMemoryParams represents parameters for updating a memory
type UpdateMemoryParams struct {
	Content    *string                `json:"content,omitempty"`
	Importance *float64               `json:"importance,omitempty"`
	Metadata   map[string]interface{} `json:"metadata,omitempty"`
}

// BatchCreateMemoryParams represents parameters for batch memory creation
type BatchCreateMemoryParams struct {
	Memories []CreateMemoryParams `json:"memories"`
}

// HealthStatus represents API health status
type HealthStatus struct {
	Status    string            `json:"status"`
	Version   string            `json:"version"`
	Uptime    int64             `json:"uptime"`
	Timestamp string            `json:"timestamp"`
	Services  map[string]string `json:"services"`
}

// SystemMetrics represents system performance metrics
type SystemMetrics struct {
	RequestsPerSecond     float64 `json:"requests_per_second"`
	AverageResponseTime   float64 `json:"average_response_time"`
	ActiveConnections     int     `json:"active_connections"`
	MemoryUsage           float64 `json:"memory_usage"`
	CPUUsage              float64 `json:"cpu_usage"`
	CacheHitRate          float64 `json:"cache_hit_rate"`
}

// Config represents client configuration
type Config struct {
	// APIKey for authentication (required)
	APIKey string
	
	// BaseURL for the AgentMem API (default: https://api.agentmem.dev)
	BaseURL string
	
	// APIVersion (default: v1)
	APIVersion string
	
	// Timeout for requests (default: 30s)
	Timeout time.Duration
	
	// MaxRetries for failed requests (default: 3)
	MaxRetries int
	
	// RetryDelay between retries (default: 1s)
	RetryDelay time.Duration
	
	// EnableCompression for requests/responses (default: true)
	EnableCompression bool
	
	// EnableCaching for GET requests (default: true)
	EnableCaching bool
	
	// CacheTTL for cached responses (default: 5m)
	CacheTTL time.Duration
	
	// EnableLogging for debug output (default: false)
	EnableLogging bool
	
	// CustomHeaders to include in requests
	CustomHeaders map[string]string
}

// RequestOptions represents options for individual requests
type RequestOptions struct {
	Timeout    *time.Duration
	Retries    *int
	UseCache   *bool
	Headers    map[string]string
}

// APIResponse represents a generic API response
type APIResponse struct {
	Data    interface{} `json:"data,omitempty"`
	Error   *string     `json:"error,omitempty"`
	Message *string     `json:"message,omitempty"`
	Status  int         `json:"status"`
}

// SearchResponse represents search API response
type SearchResponse struct {
	Results []SearchResult `json:"results"`
}

// BatchCreateResponse represents batch create API response
type BatchCreateResponse struct {
	IDs []string `json:"ids"`
}

// CreateMemoryResponse represents create memory API response
type CreateMemoryResponse struct {
	ID string `json:"id"`
}

// ============================================================================
// File-Centric Types (Phase D2 - Go SDK Stabilization)
// ============================================================================

// ResourceStatus represents lifecycle state for mounted resources
type ResourceStatus string

const (
	ResourceStatusPending   ResourceStatus = "pending"
	ResourceStatusMounted   ResourceStatus = "mounted"
	ResourceStatusFailed    ResourceStatus = "failed"
	ResourceStatusUnmounted ResourceStatus = "unmounted"
)

// CategoryStatus represents lifecycle state for categories
type CategoryStatus string

const (
	CategoryStatusActive   CategoryStatus = "active"
	CategoryStatusArchived CategoryStatus = "archived"
	CategoryStatusDeleted  CategoryStatus = "deleted"
)

// OperationStatus represents cross-language status model for async and long-running operations
type OperationStatus string

const (
	OperationStatusPending   OperationStatus = "pending"
	OperationStatusRunning   OperationStatus = "running"
	OperationStatusSucceeded OperationStatus = "succeeded"
	OperationStatusFailed    OperationStatus = "failed"
	OperationStatusCancelled OperationStatus = "cancelled"
)

// PlatformErrorCode represents file-centric error code baseline for SDK alignment
type PlatformErrorCode string

const (
	PlatformErrorCodeValidationError            PlatformErrorCode = "validation_error"
	PlatformErrorCodeCategoryNotFound           PlatformErrorCode = "category_not_found"
	PlatformErrorCodeResourceUriConflict        PlatformErrorCode = "resource_uri_conflict"
	PlatformErrorCodeMigrationConflict          PlatformErrorCode = "migration_conflict"
	PlatformErrorCodeTaskTimeout                PlatformErrorCode = "task_timeout"
	PlatformErrorCodeBackgroundTaskUnavailable PlatformErrorCode = "background_task_unavailable"
)

// ScopeDescriptor represents user/agent scoping for resources and categories
type ScopeDescriptor struct {
	UserID  *string `json:"user_id,omitempty"`
	AgentID string  `json:"agent_id"`
}

// ResourceMetadataDescriptor represents open metadata surface for resources
type ResourceMetadataDescriptor struct {
	Author      *string           `json:"author,omitempty"`
	Tags        []string          `json:"tags,omitempty"`
	SizeBytes   *int64            `json:"size_bytes,omitempty"`
	ModifiedAt  *time.Time        `json:"modified_at,omitempty"`
	Attributes  map[string]string `json:"attributes,omitempty"`
}

// CategoryMetadataDescriptor represents open metadata surface for categories
type CategoryMetadataDescriptor struct {
	Tags       []string          `json:"tags,omitempty"`
	Attributes map[string]string `json:"attributes,omitempty"`
}

// ResourceDescriptor represents stable resource DTO for the file-centric public contract
type ResourceDescriptor struct {
	ID         string                    `json:"id"`
	URI        string                    `json:"uri"`
	MediaType  string                    `json:"media_type"`
	Status     ResourceStatus            `json:"status"`
	Scope      ScopeDescriptor           `json:"scope"`
	Metadata   *ResourceMetadataDescriptor `json:"metadata,omitempty"`
	CreatedAt  *time.Time                `json:"created_at,omitempty"`
	UpdatedAt  *time.Time                `json:"updated_at,omitempty"`
}

// CategoryDescriptor represents stable category DTO for the file-centric public contract
type CategoryDescriptor struct {
	ID          string                     `json:"id"`
	Path        string                     `json:"path"`
	Name        string                     `json:"name"`
	ParentID    *string                    `json:"parent_id,omitempty"`
	ChildrenIDs []string                   `json:"children_ids,omitempty"`
	Summary     *string                    `json:"summary,omitempty"`
	ItemCount   int                        `json:"item_count"`
	Status      CategoryStatus             `json:"status"`
	Scope       ScopeDescriptor            `json:"scope"`
	Metadata    *CategoryMetadataDescriptor `json:"metadata,omitempty"`
	CreatedAt   *time.Time                 `json:"created_at,omitempty"`
	UpdatedAt   *time.Time                 `json:"updated_at,omitempty"`
	Embedding   []float64                  `json:"embedding,omitempty"`
}

// ExtractionRequest represents request to extract structured data from a mounted resource
type ExtractionRequest struct {
	ResourceID          string          `json:"resource_id"`
	Scope               ScopeDescriptor `json:"scope"`
	CategoryHintPaths   []string        `json:"category_hint_paths,omitempty"`
	PersistOutput       *bool           `json:"persist_output,omitempty"`
	IncludeEntities     *bool           `json:"include_entities,omitempty"`
	IncludeRelations    *bool           `json:"include_relations,omitempty"`
}

// ExtractedEntity represents an entity extracted from a resource
type ExtractedEntity struct {
	ID         string            `json:"id"`
	Name       string            `json:"name"`
	EntityType string            `json:"entity_type"`
	Confidence float64           `json:"confidence"`
	Attributes map[string]string `json:"attributes,omitempty"`
	SpanStart  *int              `json:"span_start,omitempty"`
	SpanEnd    *int              `json:"span_end,omitempty"`
}

// ExtractedRelation represents a relation between entities
type ExtractedRelation struct {
	ID           string            `json:"id"`
	SubjectID    string            `json:"subject_id"`
	Subject      string            `json:"subject"`
	Predicate    string            `json:"predicate"`
	ObjectID     string            `json:"object_id"`
	Object       string            `json:"object"`
	RelationType string            `json:"relation_type"`
	Confidence   float64           `json:"confidence"`
	Attributes   map[string]string `json:"attributes,omitempty"`
}

// ExtractionResult represents result of an extraction job
type ExtractionResult struct {
	JobID          string              `json:"job_id"`
	ResourceID     string              `json:"resource_id"`
	Status         OperationStatus     `json:"status"`
	CategoryPaths  []string            `json:"category_paths,omitempty"`
	MemoryIDs      []string            `json:"memory_ids,omitempty"`
	Entities       []ExtractedEntity   `json:"entities,omitempty"`
	Relations      []ExtractedRelation `json:"relations,omitempty"`
	Warnings       []string            `json:"warnings,omitempty"`
	ErrorCode      *PlatformErrorCode  `json:"error_code,omitempty"`
	ErrorMessage   *string             `json:"error_message,omitempty"`
	DurationMS     *int64              `json:"duration_ms,omitempty"`
	StartedAt      *time.Time          `json:"started_at,omitempty"`
	CompletedAt    *time.Time          `json:"completed_at,omitempty"`
}

// MigrationPlan represents plan for migrating legacy memories to file-centric surface
type MigrationPlan struct {
	PlanID                 string          `json:"plan_id"`
	Scope                  ScopeDescriptor `json:"scope"`
	DryRun                 bool            `json:"dry_run"`
	SourceSurface          string          `json:"source_surface"`
	TargetSurface          string          `json:"target_surface"`
	LegacyMemoryCount      int             `json:"legacy_memory_count"`
	ProjectedResourceCount int             `json:"projected_resource_count"`
	ProjectedCategoryCount int             `json:"projected_category_count"`
	Warnings               []string        `json:"warnings,omitempty"`
	CreatedAt              *time.Time      `json:"created_at,omitempty"`
}

// MigrationReport represents report of a migration execution
type MigrationReport struct {
	MigrationID       string             `json:"migration_id"`
	PlanID            string             `json:"plan_id"`
	DryRun            bool               `json:"dry_run"`
	Status            OperationStatus    `json:"status"`
	MigratedMemories  int                `json:"migrated_memories"`
	MountedResources  int                `json:"mounted_resources"`
	CreatedCategories int                `json:"created_categories"`
	Conflicts         []string           `json:"conflicts,omitempty"`
	Warnings          []string           `json:"warnings,omitempty"`
	Errors            []string           `json:"errors,omitempty"`
	ErrorCode         *PlatformErrorCode `json:"error_code,omitempty"`
	RollbackAvailable bool               `json:"rollback_available"`
	StartedAt         *time.Time         `json:"started_at,omitempty"`
	CompletedAt       *time.Time         `json:"completed_at,omitempty"`
}

// ProactiveTaskInfo represents information about a proactive background task
type ProactiveTaskInfo struct {
	ID              string          `json:"id"`
	TaskType        string          `json:"task_type"`
	Status          OperationStatus `json:"status"`
	Scope           ScopeDescriptor `json:"scope"`
	Schedule        string          `json:"schedule"`
	PendingRuns     int             `json:"pending_runs"`
	RunningCount    int             `json:"running_count"`
	LastStartedAt   *time.Time      `json:"last_started_at,omitempty"`
	LastCompletedAt *time.Time      `json:"last_completed_at,omitempty"`
	LastErrorCode   *PlatformErrorCode `json:"last_error_code,omitempty"`
	LastError       *string         `json:"last_error,omitempty"`
}

// SchedulerStats represents scheduler statistics
type SchedulerStats struct {
	State               string          `json:"state"`
	TotalTasks          int             `json:"total_tasks"`
	RunningTasks        int             `json:"running_tasks"`
	CompletedTasks      int             `json:"completed_tasks"`
	FailedTasks         int             `json:"failed_tasks"`
	CancelledTasks      int             `json:"cancelled_tasks"`
	TotalExecutionTimeMS int64          `json:"total_execution_time_ms"`
	LastError           *string         `json:"last_error,omitempty"`
	UpdatedAt           *time.Time      `json:"updated_at,omitempty"`
}

// ErrorResponse represents structured error response
type ErrorResponse struct {
	Code      PlatformErrorCode   `json:"code"`
	Message   string              `json:"message"`
	Details   map[string]interface{} `json:"details,omitempty"`
	Timestamp string              `json:"timestamp"`
}
