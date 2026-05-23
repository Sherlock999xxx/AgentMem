pub mod api_simplification;
pub mod v4_api;
pub mod auto_config;
pub mod builder;
pub mod cache;
pub mod chat;
pub mod history;
pub mod memory;
pub mod orchestrator;
pub mod platform;
pub mod types;
pub mod visualization;

pub use api_simplification::{EnhancedError, ErrorEnhancer, FluentMemory, SmartDefaults};
pub use builder::MemoryBuilder;
pub use memory::Memory;

// v4.0 API - 高级记忆管理功能 (Phase 1-4)
pub use v4_api::{
    // Core APIs (Phase 1)
    CoreMemoryApi, IntentUnderstandingApi, MultiSignalSearchApi, EntityLinkingApi,
    // Extended APIs (Phase 2)
    EnhancedSearchApi, ReasoningApi, AdaptiveLearningApi,
    // Enterprise APIs (Phase 3)
    MemoryTraceApi, AuditLogApi, QuotaApi, MultiTenantApi,
    // Advanced APIs (Phase 4)
    CodeSandboxApi, SandboxConfig, SandboxResult,
    FleetApi, FleetAgent, AgentTeam, AgentRole, AgentStatus, TeamStrategy,
    MentalModelApi, PersonaModel, PersonalityTrait, InteractionFeedback,
    SchemaEvolutionApi, SchemaDefinition,
    V4ApiPhase4, V4ApiPhase4Health,
    // Unified API
    V4Api, V4ApiHealth,
    // Types
    IntentUnderstandingResult, IntentType, Entity, EntityType, TimeRange,
    MultiSignalSearchResult, MultiSignalConfig, RetrievalStrategy,
    EntityLinkingResult, EntityLinkingConfig, LinkedEntity, EntityRelationship,
    HybridSearchResult, HybridSearchItem, SearchScores, QueryClassification,
    ReasoningConfig, CausalResult, CauseEffect, TemporalResult, TemporalRelation,
    TimeRangeResult, AdaptiveConfig, AdaptiveMetrics,
    TraceConfig, TraceEntry, TraceAction, TraceMetrics,
    AuditConfig, AuditEntryV4, AuditEvent, AuditStatus,
    QuotaLimit, QuotaUsage, QuotaCheckResult,
    Tenant, TenantPlan,
};

pub use platform::{
    ApplyMigrationRequest, CancelProactiveTaskRequest, CategoryDescriptor,
    CategoryMetadataDescriptor, CategoryStatus, ExtractedEntity, ExtractedRelation,
    ExtractionRequest, ExtractionResult, MountResourceRequest, MigrationPlan,
    MigrationReport, OperationStatus, PlanMigrationRequest, PlatformErrorCode,
    ProactiveTaskInfo, ResourceDescriptor, ResourceMetadataDescriptor, ResourceStatus,
    RollbackMigrationRequest, RunProactiveTaskRequest, SchedulerStats,
    SchedulerState, ScopeDescriptor, SearchCategoriesRequest,
};
pub use types::{
    AddMemoryOptions, AddResult, DeleteAllOptions, GetAllOptions, MemoryEvent, MemoryScope,
    MemoryStats, RelationEvent, SearchOptions,
};

pub use agent_mem_traits::{AgentMemError, Result};
pub use agent_mem_traits::abstractions::{
    AttributeKey, AttributeSet, AttributeValue, Content, Memory as MemoryV4, Metadata, Query,
    QueryIntent, RelationGraph,
};

#[allow(deprecated)]
pub use agent_mem_traits::{MemoryItem, MemoryType};

pub use agent_mem_core::{
    ContextualAgent, CoreAgent, EpisodicAgent, KnowledgeAgent, ProceduralAgent, ResourceAgent,
    SemanticAgent, WorkingAgent,
};

#[cfg(feature = "plugins")]
pub use agent_mem_plugins as plugins;

pub mod plugin_integration;
#[cfg(feature = "plugins")]
pub use plugin_integration::{PluginEnhancedMemory, PluginHooks};
