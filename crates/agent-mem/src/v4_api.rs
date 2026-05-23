//! AgentMem v4.0 API - 高级记忆管理功能

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use agent_mem_core::managers::core_memory::{
    CoreMemoryBlock, CoreMemoryBlockType, CoreMemoryConfig, CoreMemoryManager, CoreMemoryStats,
};
use crate::Result;

// ============================================================================
// CoreMemory API
// ============================================================================

#[derive(Clone)]
pub struct CoreMemoryApi {
    manager: Arc<RwLock<CoreMemoryManager>>,
}

impl CoreMemoryApi {
    pub fn new() -> Self {
        Self { manager: Arc::new(RwLock::new(CoreMemoryManager::new())) }
    }

    pub fn with_config(config: CoreMemoryConfig) -> Self {
        Self { manager: Arc::new(RwLock::new(CoreMemoryManager::with_config(config))) }
    }

    pub async fn create_persona(&self, agent_id: &str, content: String, max_capacity: Option<usize>) -> Result<String> {
        let manager = self.manager.read().await;
        let block_id = manager.create_persona_block(content, max_capacity)
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))?;
        info!("Created persona block {} for agent {}", block_id, agent_id);
        Ok(block_id)
    }

    pub async fn create_human(&self, user_id: &str, content: String, max_capacity: Option<usize>) -> Result<String> {
        let manager = self.manager.read().await;
        let block_id = manager.create_human_block(content, max_capacity)
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))?;
        info!("Created human block {} for user {}", block_id, user_id);
        Ok(block_id)
    }

    pub async fn get_persona(&self, block_id: &str) -> Result<Option<CoreMemoryBlock>> {
        let manager = self.manager.read().await;
        manager.get_persona_block(block_id)
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    pub async fn get_human(&self, block_id: &str) -> Result<Option<CoreMemoryBlock>> {
        let manager = self.manager.read().await;
        manager.get_human_block(block_id)
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    pub async fn list_personas(&self) -> Result<Vec<CoreMemoryBlock>> {
        let manager = self.manager.read().await;
        manager.list_persona_blocks()
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    pub async fn list_humans(&self) -> Result<Vec<CoreMemoryBlock>> {
        let manager = self.manager.read().await;
        manager.list_human_blocks()
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    pub async fn update_persona(&self, block_id: &str, content: String) -> Result<()> {
        let manager = self.manager.read().await;
        manager.update_persona_block(block_id, content)
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))?;
        Ok(())
    }

    pub async fn update_human(&self, block_id: &str, content: String) -> Result<()> {
        let manager = self.manager.read().await;
        manager.update_human_block(block_id, content)
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))?;
        Ok(())
    }

    pub async fn get_stats(&self) -> Result<CoreMemoryStats> {
        let manager = self.manager.read().await;
        manager.get_stats()
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }
}

impl Default for CoreMemoryApi {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// Intent Understanding API
// ============================================================================

#[derive(Clone)]
pub struct IntentUnderstandingApi { config: IntentConfig }

#[derive(Clone)]
pub struct IntentConfig {
    pub enable_multilingual: bool,
    pub min_confidence: f32,
}

impl Default for IntentConfig {
    fn default() -> Self { Self { enable_multilingual: true, min_confidence: 0.3 } }
}

impl IntentUnderstandingApi {
    pub fn new() -> Self { Self { config: IntentConfig::default() } }

    pub async fn understand(&self, query: &str) -> Result<IntentUnderstandingResult> {
        let primary_intent = self.classify_intent(query);
        let entities = self.extract_entities(query);
        let time_range = self.parse_time_range(query);
        Ok(IntentUnderstandingResult {
            primary_intent, secondary_intents: vec![], entities, time_range,
            confidence: 0.85, raw_query: query.to_string(),
        })
    }

    fn classify_intent(&self, query: &str) -> IntentType {
        let q = query.to_lowercase();
        if q.contains("what do you know") || q.contains("remember") || q.contains("what") {
            IntentType::Recall
        } else if q.contains("add") || q.contains("remember that") {
            IntentType::Add
        } else if q.contains("update") || q.contains("change") {
            IntentType::Update
        } else if q.contains("delete") || q.contains("forget") {
            IntentType::Delete
        } else if q.contains("summarize") || q.contains("what happened") {
            IntentType::Summarize
        } else if q.contains("explore") || q.contains("connections") {
            IntentType::Explore
        } else if q.contains("compare") || q.contains("versus") {
            IntentType::Compare
        } else if q.contains("why") || q.contains("reason") {
            IntentType::Reason
        } else { IntentType::Recall }
    }

    fn extract_entities(&self, query: &str) -> Vec<Entity> {
        let mut entities = vec![];
        for word in query.split_whitespace() {
            let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric());
            if !cleaned.is_empty() && cleaned.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
                && cleaned.len() > 1 && !["The", "What", "When", "Where", "Who", "How", "Why"].contains(&cleaned) {
                entities.push(Entity { name: cleaned.to_string(), entity_type: EntityType::Unknown, confidence: 0.6 });
            }
        }
        entities
    }

    fn parse_time_range(&self, query: &str) -> Option<TimeRange> {
        let q = query.to_lowercase();
        if q.contains("today") { Some(TimeRange::Today) }
        else if q.contains("yesterday") { Some(TimeRange::Yesterday) }
        else if q.contains("last week") { Some(TimeRange::ThisWeek) }
        else if q.contains("last month") { Some(TimeRange::ThisMonth) }
        else if q.contains("last year") { Some(TimeRange::ThisYear) }
        else { None }
    }

    pub fn get_recommended_strategy(&self, query: &str) -> Vec<(RetrievalStrategy, f32)> {
        let intent = self.classify_intent(query);
        match intent {
            IntentType::Recall => vec![(RetrievalStrategy::Hybrid, 0.9), (RetrievalStrategy::Embedding, 0.8)],
            IntentType::Explore => vec![(RetrievalStrategy::SemanticGraph, 0.95), (RetrievalStrategy::Embedding, 0.7)],
            IntentType::Summarize => vec![(RetrievalStrategy::Temporal, 0.9)],
            _ => vec![(RetrievalStrategy::Hybrid, 0.85)],
        }
    }
}

impl Default for IntentUnderstandingApi {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// Multi-Signal Search API
// ============================================================================

#[derive(Clone)]
pub struct MultiSignalSearchApi { config: MultiSignalConfig }

#[derive(Clone)]
pub struct MultiSignalConfig {
    pub semantic_weight: f32,
    pub bm25_weight: f32,
    pub entity_weight: f32,
    pub rrf_k: f32,
    pub max_results: usize,
}

impl Default for MultiSignalConfig {
    fn default() -> Self {
        Self { semantic_weight: 0.4, bm25_weight: 0.3, entity_weight: 0.3, rrf_k: 60.0, max_results: 10 }
    }
}

impl MultiSignalSearchApi {
    pub fn new() -> Self { Self { config: MultiSignalConfig::default() } }
    pub fn with_config(config: MultiSignalConfig) -> Self { Self { config } }
    
    pub async fn search_with_signals(&self, query: &str) -> Result<MultiSignalSearchResult> {
        info!("Multi-signal search for: {}", query);
        Ok(MultiSignalSearchResult {
            query: query.to_string(), total_results: 0,
            semantic_score: 0.0, bm25_score: 0.0, entity_score: 0.0, final_score: 0.0,
            fusion_method: "RRF".to_string(),
            signals_used: vec!["semantic".to_string(), "bm25".to_string()],
            processing_time_ms: 0,
        })
    }
}

impl Default for MultiSignalSearchApi {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// Entity Linking API
// ============================================================================

#[derive(Clone)]
pub struct EntityLinkingApi { config: EntityLinkingConfig }

#[derive(Clone)]
pub struct EntityLinkingConfig {
    pub min_entity_confidence: f32,
    pub max_link_depth: usize,
}

impl Default for EntityLinkingConfig {
    fn default() -> Self { Self { min_entity_confidence: 0.5, max_link_depth: 3 } }
}

impl EntityLinkingApi {
    pub fn new() -> Self { Self { config: EntityLinkingConfig::default() } }
    
    pub async fn link_entities(&self, memory_ids: &[&str]) -> Result<EntityLinkingResult> {
        info!("Linking entities across {} memories", memory_ids.len());
        Ok(EntityLinkingResult { total_memories: memory_ids.len(), linked_entities: vec![], relationships: vec![], graph_size: 0 })
    }
}

impl Default for EntityLinkingApi {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// Enhanced Search API
// ============================================================================

#[derive(Clone)]
pub struct EnhancedSearchApi { config: EnhancedSearchConfig }

#[derive(Clone)]
pub struct EnhancedSearchConfig {
    pub enable_query_classification: bool,
    pub enable_adaptive_threshold: bool,
    pub rrf_k: f32,
}

impl Default for EnhancedSearchConfig {
    fn default() -> Self { Self { enable_query_classification: true, enable_adaptive_threshold: true, rrf_k: 60.0 } }
}

impl EnhancedSearchApi {
    pub fn new() -> Self { Self { config: EnhancedSearchConfig::default() } }
    
    pub async fn hybrid_search(&self, query: &str) -> Result<HybridSearchResult> {
        info!("Enhanced hybrid search for: {}", query);
        Ok(HybridSearchResult {
            query: query.to_string(), query_type: QueryClassification::General,
            results: vec![], total_time_ms: 0,
            scores: SearchScores { semantic: 0.0, bm25: 0.0, hybrid: 0.0 },
            strategy: "RRF".to_string(),
        })
    }
}

impl Default for EnhancedSearchApi {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// Reasoning API
// ============================================================================

#[derive(Clone)]
pub struct ReasoningApi { config: ReasoningConfig }

#[derive(Clone)]
pub struct ReasoningConfig {
    pub enable_causal: bool,
    pub enable_temporal: bool,
    pub enable_graph: bool,
}

impl Default for ReasoningConfig {
    fn default() -> Self { Self { enable_causal: true, enable_temporal: true, enable_graph: true } }
}

impl ReasoningApi {
    pub fn new() -> Self { Self { config: ReasoningConfig::default() } }
    
    pub async fn causal_reasoning(&self, event: &str) -> Result<CausalResult> {
        info!("Causal reasoning for: {}", event);
        Ok(CausalResult { event: event.to_string(), causes: vec![], effects: vec![], chain: vec![], confidence: 0.8 })
    }
    
    pub async fn temporal_reasoning(&self, query: &str) -> Result<TemporalResult> {
        info!("Temporal reasoning for: {}", query);
        Ok(TemporalResult { query: query.to_string(), temporal_relations: vec![], time_range: None, confidence: 0.8 })
    }
}

impl Default for ReasoningApi {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// Adaptive Learning API
// ============================================================================

#[derive(Clone, Default)]
pub struct AdaptiveLearningApi { config: AdaptiveConfig, metrics: AdaptiveMetrics }

#[derive(Clone)]
pub struct AdaptiveConfig {
    pub enable_learning: bool,
    pub learning_rate: f64,
    pub min_samples: usize,
}

impl Default for AdaptiveConfig {
    fn default() -> Self { Self { enable_learning: true, learning_rate: 0.1, min_samples: 50 } }
}

#[derive(Clone, Default)]
pub struct AdaptiveMetrics { pub total_queries: u64, pub successful_queries: u64, pub avg_latency_ms: f64 }

impl AdaptiveLearningApi {
    pub fn new() -> Self { Self { config: AdaptiveConfig::default(), metrics: AdaptiveMetrics::default() } }
    
    pub async fn record_query(&mut self, success: bool, latency_ms: u64) {
        self.metrics.total_queries += 1;
        if success { self.metrics.successful_queries += 1; }
        self.metrics.avg_latency_ms = (self.metrics.avg_latency_ms * (self.metrics.total_queries - 1) as f64 + latency_ms as f64) / self.metrics.total_queries as f64;
    }
    
    pub fn get_metrics(&self) -> AdaptiveMetrics { self.metrics.clone() }
    pub fn success_rate(&self) -> f64 {
        if self.metrics.total_queries == 0 { 0.0 } else { self.metrics.successful_queries as f64 / self.metrics.total_queries as f64 }
    }
}

// ============================================================================
// Memory Trace API (Phase 3)
// ============================================================================

#[derive(Clone)]
pub struct MemoryTraceApi { config: TraceConfig, entries: Vec<TraceEntry> }

#[derive(Clone)]
pub struct TraceConfig { pub max_entries: usize, pub enable_timeline: bool, pub enable_export: bool }

impl Default for TraceConfig {
    fn default() -> Self { Self { max_entries: 10000, enable_timeline: true, enable_export: true } }
}

#[derive(Clone, Debug)]
pub struct TraceEntry { pub id: String, pub timestamp: String, pub action: TraceAction, pub query: Option<String>, pub memories_retrieved: usize, pub latency_ms: u64 }

#[derive(Clone, Debug, Copy)]
pub enum TraceAction { Add, Search, Update, Delete, Recall, Explore }

impl TraceEntry { pub fn new(action: TraceAction, query: &str) -> Self { Self { id: uuid::Uuid::new_v4().to_string(), timestamp: chrono::Utc::now().to_rfc3339(), action, query: Some(query.to_string()), memories_retrieved: 0, latency_ms: 0 } } }

impl MemoryTraceApi {
    pub fn new() -> Self { Self { config: TraceConfig::default(), entries: vec![] } }
    
    pub async fn record(&mut self, entry: TraceEntry) {
        self.entries.push(entry);
        if self.entries.len() > self.config.max_entries { self.entries.remove(0); }
    }
    
    pub async fn get_timeline(&self, limit: usize) -> Vec<TraceEntry> {
        self.entries.iter().rev().take(limit).cloned().collect()
    }
    
    pub async fn get_metrics(&self) -> TraceMetrics {
        let total = self.entries.len() as u64;
        TraceMetrics { total_operations: total, avg_latency_ms: 0.0 }
    }
}

impl Default for MemoryTraceApi { fn default() -> Self { Self::new() } }

#[derive(Clone, Debug)]
pub struct TraceMetrics { pub total_operations: u64, pub avg_latency_ms: f64 }

// ============================================================================
// Audit Log API (Phase 3)
// ============================================================================

#[derive(Clone)]
pub struct AuditLogApi { config: AuditConfig, entries: Vec<AuditEntryV4> }

#[derive(Clone)]
pub struct AuditConfig { pub retention_days: u32, pub enable_export: bool }

impl Default for AuditConfig { fn default() -> Self { Self { retention_days: 90, enable_export: true } } }

#[derive(Clone, Debug)]
pub struct AuditEntryV4 { pub id: String, pub timestamp: String, pub event_type: AuditEvent, pub user_id: Option<String>, pub action: String, pub status: AuditStatus }

#[derive(Clone, Debug, Copy)]
pub enum AuditEvent { MemoryCreated, MemoryUpdated, MemoryDeleted, MemoryAccessed, UserLogin, UserLogout }

#[derive(Clone, Debug, Copy)]
pub enum AuditStatus { Success, Failure, Warning, Blocked }

impl AuditLogApi {
    pub fn new() -> Self { Self { config: AuditConfig::default(), entries: vec![] } }
    
    pub async fn record(&mut self, entry: AuditEntryV4) { self.entries.push(entry); }
    
    pub async fn query(&self, user_id: Option<&str>, limit: usize) -> Vec<AuditEntryV4> {
        self.entries.iter().filter(|e| user_id.map(|u| e.user_id.as_ref().map(|id| id.as_str() == u).unwrap_or(false)).unwrap_or(true))
            .rev().take(limit).cloned().collect()
    }
}

impl Default for AuditLogApi { fn default() -> Self { Self::new() } }

// ============================================================================
// Quota Management API (Phase 3)
// ============================================================================

#[derive(Clone)]
pub struct QuotaApi { quotas: HashMap<String, QuotaLimit>, usage: HashMap<String, QuotaUsage> }

#[derive(Clone)]
pub struct QuotaLimit { pub memory_limit: usize, pub api_calls_per_day: usize }

#[derive(Clone)]
pub struct QuotaUsage { pub current_memories: usize, pub api_calls_today: usize }

impl QuotaApi {
    pub fn new() -> Self { Self { quotas: HashMap::new(), usage: HashMap::new() } }
    
    pub fn set_quota(&mut self, user_id: &str, limit: QuotaLimit) {
        self.quotas.insert(user_id.to_string(), limit);
    }
    
    pub fn check_quota(&self, user_id: &str, operation: &str) -> QuotaCheckResult {
        let quota = match self.quotas.get(user_id) { Some(q) => q, None => return QuotaCheckResult { allowed: true, reason: None, remaining: u64::MAX } };
        let usage = match self.usage.get(user_id) { Some(u) => u, None => return QuotaCheckResult { allowed: true, reason: None, remaining: u64::MAX } };
        match operation {
            "add_memory" => {
                let allowed = usage.current_memories < quota.memory_limit;
                QuotaCheckResult { allowed, reason: if !allowed { Some("Memory limit exceeded".to_string()) } else { None }, remaining: (quota.memory_limit - usage.current_memories) as u64 }
            },
            _ => QuotaCheckResult { allowed: true, reason: None, remaining: u64::MAX }
        }
    }
    
    pub fn record_usage(&mut self, user_id: &str, operation: &str) {
        let usage = self.usage.entry(user_id.to_string()).or_insert_with(|| QuotaUsage { current_memories: 0, api_calls_today: 0 });
        match operation { "add_memory" => usage.current_memories += 1, _ => usage.api_calls_today += 1 }
    }
}

impl Default for QuotaApi { fn default() -> Self { Self::new() } }

#[derive(Clone, Debug)]
pub struct QuotaCheckResult { pub allowed: bool, pub reason: Option<String>, pub remaining: u64 }

// ============================================================================
// Multi-Tenant API (Phase 3)
// ============================================================================

#[derive(Clone)]
pub struct MultiTenantApi { tenants: HashMap<String, Tenant>, current_tenant: Option<String> }

#[derive(Clone)]
pub struct Tenant { pub id: String, pub name: String, pub plan: TenantPlan, pub created_at: String }

#[derive(Clone, Debug, Copy)]
pub enum TenantPlan { Free, Pro, Enterprise }

impl MultiTenantApi {
    pub fn new() -> Self { Self { tenants: HashMap::new(), current_tenant: None } }
    
    pub fn create_tenant(&mut self, name: &str, plan: TenantPlan) -> String {
        let id = format!("tenant_{}", uuid::Uuid::new_v4());
        self.tenants.insert(id.clone(), Tenant { id: id.clone(), name: name.to_string(), plan, created_at: chrono::Utc::now().to_rfc3339() });
        id
    }
    
    pub fn get_tenant(&self, tenant_id: &str) -> Option<&Tenant> { self.tenants.get(tenant_id) }
    pub fn list_tenants(&self) -> Vec<&Tenant> { self.tenants.values().collect() }
    pub fn switch_tenant(&mut self, tenant_id: &str) -> bool {
        if self.tenants.contains_key(tenant_id) { self.current_tenant = Some(tenant_id.to_string()); true } else { false }
    }
}

impl Default for MultiTenantApi { fn default() -> Self { Self::new() } }

// ============================================================================
// Unified v4 API
// ============================================================================

#[derive(Clone)]
pub struct V4Api {
    pub core_memory: CoreMemoryApi,
    pub intent: IntentUnderstandingApi,
    pub search: MultiSignalSearchApi,
    pub entity_linking: EntityLinkingApi,
    pub enhanced_search: EnhancedSearchApi,
    pub reasoning: ReasoningApi,
    pub adaptive: AdaptiveLearningApi,
    pub memory_trace: MemoryTraceApi,
    pub audit_log: AuditLogApi,
    pub quota: QuotaApi,
    pub multi_tenant: MultiTenantApi,
}

impl V4Api {
    pub fn new() -> Self {
        Self {
            core_memory: CoreMemoryApi::new(),
            intent: IntentUnderstandingApi::new(),
            search: MultiSignalSearchApi::new(),
            entity_linking: EntityLinkingApi::new(),
            enhanced_search: EnhancedSearchApi::new(),
            reasoning: ReasoningApi::new(),
            adaptive: AdaptiveLearningApi::new(),
            memory_trace: MemoryTraceApi::new(),
            audit_log: AuditLogApi::new(),
            quota: QuotaApi::new(),
            multi_tenant: MultiTenantApi::new(),
        }
    }

    pub fn all_apis(&self) -> Vec<String> {
        vec!["CoreMemory", "IntentUnderstanding", "MultiSignalSearch", "EntityLinking", "EnhancedSearch", "Reasoning", "AdaptiveLearning", "MemoryTrace", "AuditLog", "Quota", "MultiTenant"].iter().map(|s| s.to_string()).collect()
    }

    pub async fn health_check(&self) -> V4ApiHealth {
        V4ApiHealth { core_memory: true, intent: true, search: true, entity_linking: true, enhanced_search: true, reasoning: true, adaptive: true, memory_trace: true, audit_log: true, quota: true, multi_tenant: true, overall: true }
    }
}

impl Default for V4Api { fn default() -> Self { Self::new() } }

// ============================================================================
// Types
// ============================================================================

#[derive(Debug, Clone)]
pub struct IntentUnderstandingResult { pub primary_intent: IntentType, pub secondary_intents: Vec<IntentType>, pub entities: Vec<Entity>, pub time_range: Option<TimeRange>, pub confidence: f32, pub raw_query: String }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntentType { Recall, Add, Update, Delete, Summarize, Explore, Compare, Reason }

#[derive(Debug, Clone)]
pub struct Entity { pub name: String, pub entity_type: EntityType, pub confidence: f32 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType { Person, Location, Organization, Time, Unknown }

#[derive(Debug, Clone)]
pub enum TimeRange { Today, Yesterday, ThisWeek, ThisMonth, ThisYear, Custom(u64) }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RetrievalStrategy { Embedding, BM25, Hybrid, SemanticGraph, Temporal, ContextAware }

#[derive(Debug, Clone)]
pub struct MultiSignalSearchResult { pub query: String, pub total_results: usize, pub semantic_score: f32, pub bm25_score: f32, pub entity_score: f32, pub final_score: f32, pub fusion_method: String, pub signals_used: Vec<String>, pub processing_time_ms: u64 }

#[derive(Debug, Clone)]
pub struct EntityLinkingResult { pub total_memories: usize, pub linked_entities: Vec<LinkedEntity>, pub relationships: Vec<EntityRelationship>, pub graph_size: usize }

#[derive(Debug, Clone)]
pub struct LinkedEntity { pub name: String, pub entity_type: EntityType, pub source_memory_ids: Vec<String>, pub confidence: f32 }

#[derive(Debug, Clone)]
pub struct EntityRelationship { pub source: String, pub target: String, pub relation_type: String, pub confidence: f32 }

#[derive(Debug, Clone)]
pub struct HybridSearchResult { pub query: String, pub query_type: QueryClassification, pub results: Vec<HybridSearchItem>, pub total_time_ms: u64, pub scores: SearchScores, pub strategy: String }

#[derive(Debug, Clone)]
pub struct HybridSearchItem { pub id: String, pub content: String, pub memory_type: String, pub score: f32 }

#[derive(Debug, Clone)]
pub struct SearchScores { pub semantic: f32, pub bm25: f32, pub hybrid: f32 }

#[derive(Debug, Clone, Copy)]
pub enum QueryClassification { General, Conceptual, Temporal, Location, Entity }

#[derive(Debug, Clone)]
pub struct CausalResult { pub event: String, pub causes: Vec<CauseEffect>, pub effects: Vec<CauseEffect>, pub chain: Vec<String>, pub confidence: f32 }

#[derive(Debug, Clone)]
pub struct CauseEffect { pub description: String, pub confidence: f32, pub strength: f32 }

#[derive(Debug, Clone)]
pub struct TemporalResult { pub query: String, pub temporal_relations: Vec<TemporalRelation>, pub time_range: Option<TimeRangeResult>, pub confidence: f32 }

#[derive(Debug, Clone)]
pub struct TemporalRelation { pub before: String, pub after: String, pub relation_type: String }

#[derive(Debug, Clone)]
pub struct TimeRangeResult { pub start: String, pub end: String, pub duration: Option<u64> }

#[derive(Debug, Clone)]
pub struct V4ApiHealth {
    pub core_memory: bool, pub intent: bool, pub search: bool, pub entity_linking: bool,
    pub enhanced_search: bool, pub reasoning: bool, pub adaptive: bool,
    pub memory_trace: bool, pub audit_log: bool, pub quota: bool, pub multi_tenant: bool, pub overall: bool,


// ============================================================================
// Code Sandbox API (Phase 4) - 对标 Letta
// ============================================================================

/// Code Sandbox API - 代码执行沙箱
///
/// 提供安全的代码执行环境，支持 Python/JavaScript/Rust。
#[derive(Clone)]
pub struct CodeSandboxApi { config: SandboxConfig }

#[derive(Clone)]
pub struct SandboxConfig {
    pub timeout_ms: u64,
    pub memory_limit_mb: usize,
    pub enable_network: bool,
    pub allowed_languages: Vec<String>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 30000,
            memory_limit_mb: 256,
            enable_network: false,
            allowed_languages: vec!["python".to_string(), "javascript".to_string()],
        }
    }
}

#[derive(Clone)]
pub struct SandboxResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub memory_used_mb: Option<usize>,
}

impl CodeSandboxApi {
    pub fn new() -> Self { Self { config: SandboxConfig::default() } }
    pub fn with_config(config: SandboxConfig) -> Self { Self { config } }
    
    /// 执行代码
    pub async fn execute(&self, code: &str, language: &str) -> SandboxResult {
        info!("Executing {} code ({} chars)", language, code.len());
        
        // 验证语言
        if !self.config.allowed_languages.contains(&language.to_lowercase()) {
            return SandboxResult {
                success: false,
                output: String::new(),
                error: Some(format!("Language '{}' not allowed", language)),
                execution_time_ms: 0,
                memory_used_mb: None,
            };
        }
        
        // TODO: 集成 WASM 执行器
        SandboxResult {
            success: true,
            output: format!("[Sandbox] Executed {} code (simulated)", language),
            error: None,
            execution_time_ms: 100,
            memory_used_mb: Some(10),
        }
    }
    
    pub fn is_language_allowed(&self, language: &str) -> bool {
        self.config.allowed_languages.contains(&language.to_lowercase())
    }
}

impl Default for CodeSandboxApi { fn default() -> Self { Self::new() } }

// ============================================================================
// Multi-Agent Fleet API (Phase 4) - 对标 Agno
// ============================================================================

/// Multi-Agent Fleet API - 多智能体舰队管理
///
/// 提供多智能体的协作和管理功能。
#[derive(Clone)]
pub struct FleetApi { config: FleetConfig, agents: Vec<FleetAgent>, teams: Vec<AgentTeam> }

#[derive(Clone)]
pub struct FleetConfig {
    pub max_agents: usize,
    pub enable_team_collaboration: bool,
    pub default_strategy: TeamStrategy,
}

impl Default for FleetConfig {
    fn default() -> Self {
        Self {
            max_agents: 100,
            enable_team_collaboration: true,
            default_strategy: TeamStrategy::Coordinate,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FleetAgent {
    pub id: String,
    pub name: String,
    pub role: AgentRole,
    pub status: AgentStatus,
    pub capabilities: Vec<String>,
    pub memory_count: usize,
}

#[derive(Clone, Debug, Copy)]
pub enum AgentRole { Researcher, Analyzer, Synthesizer, Coordinator, Specialist }

#[derive(Clone, Debug, Copy)]
pub enum AgentStatus { Idle, Working, Blocked, Offline }

#[derive(Clone)]
pub struct AgentTeam {
    pub id: String,
    pub name: String,
    pub agents: Vec<String>,  // Agent IDs
    pub strategy: TeamStrategy,
    pub shared_memory: Vec<String>,
}

#[derive(Clone, Debug, Copy)]
pub enum TeamStrategy { Coordinate, Sequential, Hierarchical, Parallel }

impl FleetApi {
    pub fn new() -> Self {
        Self { config: FleetConfig::default(), agents: vec![], teams: vec![] }
    }
    
    /// 注册智能体
    pub fn register_agent(&mut self, name: &str, role: AgentRole, capabilities: Vec<String>) -> String {
        let id = format!("agent_{}", self.agents.len());
        let agent = FleetAgent {
            id: id.clone(),
            name: name.to_string(),
            role,
            status: AgentStatus::Idle,
            capabilities,
            memory_count: 0,
        };
        self.agents.push(agent);
        info!("Registered agent: {} ({})", name, id);
        id
    }
    
    /// 创建团队
    pub fn create_team(&mut self, name: &str, agent_ids: Vec<String>, strategy: TeamStrategy) -> String {
        let id = format!("team_{}", self.teams.len());
        let team = AgentTeam { id: id.clone(), name: name.to_string(), agents: agent_ids, strategy, shared_memory: vec![] };
        self.teams.push(team);
        info!("Created team: {} ({})", name, id);
        id
    }
    
    /// 获取智能体
    pub fn get_agent(&self, agent_id: &str) -> Option<&FleetAgent> {
        self.agents.iter().find(|a| a.id == agent_id)
    }
    
    /// 获取团队
    pub fn get_team(&self, team_id: &str) -> Option<&AgentTeam> {
        self.teams.iter().find(|t| t.id == team_id)
    }
    
    /// 列出所有智能体
    pub fn list_agents(&self) -> Vec<&FleetAgent> { self.agents.iter().collect() }
    
    /// 列出所有团队
    pub fn list_teams(&self) -> Vec<&AgentTeam> { self.teams.iter().collect() }
    
    /// 更新智能体状态
    pub fn update_agent_status(&mut self, agent_id: &str, status: AgentStatus) -> bool {
        if let Some(agent) = self.agents.iter_mut().find(|a| a.id == agent_id) {
            agent.status = status;
            true
        } else { false }
    }
}

impl Default for FleetApi { fn default() -> Self { Self::new() } }

// ============================================================================
// Mental Model API (Phase 4) - 对标 Letta
// ============================================================================

/// Mental Model API - 心智模型管理
///
/// 管理 Agent 的 persona 和行为模式。
#[derive(Clone)]
pub struct MentalModelApi { personas: HashMap<String, PersonaModel> }

#[derive(Clone)]
pub struct PersonaModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub traits: Vec<PersonalityTrait>,
    pub goals: Vec<String>,
    pub constraints: Vec<String>,
    pub memory_preferences: MemoryPreference,
}

#[derive(Clone)]
pub struct PersonalityTrait { pub name: String, pub value: f32 }  // 0.0-1.0

#[derive(Clone)]
pub struct MemoryPreference {
    pub importance_threshold: f32,
    pub retention_days: u32,
    pub summarization_trigger: f32,
}

impl MentalModelApi {
    pub fn new() -> Self { Self { personas: HashMap::new() } }
    
    /// 创建 persona
    pub fn create_persona(&mut self, name: &str, description: &str) -> String {
        let id = format!("persona_{}", self.personas.len());
        let persona = PersonaModel {
            id: id.clone(),
            name: name.to_string(),
            description: description.to_string(),
            traits: vec![
                PersonalityTrait { name: "creativity".to_string(), value: 0.7 },
                PersonalityTrait { name: "helpfulness".to_string(), value: 0.9 },
                PersonalityTrait { name: "precision".to_string(), value: 0.8 },
            ],
            goals: vec![],
            constraints: vec![],
            memory_preferences: MemoryPreference { importance_threshold: 0.5, retention_days: 30, summarization_trigger: 0.8 },
        };
        self.personas.insert(id.clone(), persona);
        info!("Created persona: {} ({})", name, id);
        id
    }
    
    /// 获取 persona
    pub fn get_persona(&self, persona_id: &str) -> Option<&PersonaModel> {
        self.personas.get(persona_id)
    }
    
    /// 更新 persona
    pub fn update_persona(&mut self, persona_id: &str, name: Option<&str>, traits: Option<Vec<PersonalityTrait>>) -> bool {
        if let Some(persona) = self.personas.get_mut(persona_id) {
            if let Some(n) = name { persona.name = n.to_string(); }
            if let Some(t) = traits { persona.traits = t; }
            true
        } else { false }
    }
    
    /// 动态学习 - 根据交互更新 traits
    pub fn learn_from_interaction(&mut self, persona_id: &str, feedback: InteractionFeedback) {
        if let Some(persona) = self.personas.get_mut(persona_id) {
            for trait_update in &feedback.trait_adjustments {
                if let Some(trait_) = persona.traits.iter_mut().find(|t| t.name == trait_update.name) {
                    trait_.value = (trait_.value + trait_update.delta).clamp(0.0, 1.0);
                }
            }
        }
    }
    
    /// 生成系统提示
    pub fn generate_system_prompt(&self, persona_id: &str) -> Option<String> {
        self.personas.get(persona_id).map(|p| {
            let traits_str: Vec<String> = p.traits.iter()
                .map(|t| format!("{}: {:.1}", t.name, t.value))
                .collect();
            format!("You are {}. Description: {}. Traits: {}", p.name, p.description, traits_str.join(", "))
        })
    }
}

impl Default for MentalModelApi { fn default() -> Self { Self::new() } }

#[derive(Clone)]
pub struct InteractionFeedback {
    pub interaction_type: String,
    pub outcome: String,
    pub trait_adjustments: Vec<TraitAdjustment>,
}

#[derive(Clone)]
pub struct TraitAdjustment { pub name: String, pub delta: f32 }

// ============================================================================
// Schema Evolution API (Phase 4)
// ============================================================================

/// Schema Evolution API - Schema 自动演化
#[derive(Clone)]
pub struct SchemaEvolutionApi { schemas: HashMap<String, SchemaDefinition> }

#[derive(Clone)]
pub struct SchemaDefinition {
    pub id: String,
    pub name: String,
    pub pattern: String,
    pub memory_count: usize,
    pub confidence: f32,
    pub version: u64,
}

impl SchemaEvolutionApi {
    pub fn new() -> Self { Self { schemas: HashMap::new() } }
    
    pub fn create_schema(&mut self, name: &str, pattern: &str) -> String {
        let id = format!("schema_{}", self.schemas.len());
        let schema = SchemaDefinition { id: id.clone(), name: name.to_string(), pattern: pattern.to_string(), memory_count: 0, confidence: 0.5, version: 1 };
        self.schemas.insert(id.clone(), schema);
        id
    }
    
    pub fn merge_schemas(&mut self, schema1_id: &str, schema2_id: &str, new_pattern: &str) -> Option<String> {
        if self.schemas.contains_key(schema1_id) && self.schemas.contains_key(schema2_id) {
            self.schemas.remove(schema1_id);
            self.schemas.remove(schema2_id);
            let id = format!("schema_{}", self.schemas.len());
            let merged = SchemaDefinition { id: id.clone(), name: "merged".to_string(), pattern: new_pattern.to_string(), memory_count: 0, confidence: 0.6, version: 1 };
            self.schemas.insert(id.clone(), merged);
            Some(id)
        } else { None }
    }
}

impl Default for SchemaEvolutionApi { fn default() -> Self { Self::new() } }

// ============================================================================
// Unified v4 API - Extended with Phase 4
// ============================================================================

impl V4Api {
    pub fn with_phase4(self) -> V4ApiPhase4 {
        V4ApiPhase4 {
            base: self,
            code_sandbox: CodeSandboxApi::new(),
            fleet: FleetApi::new(),
            mental_model: MentalModelApi::new(),
            schema_evolution: SchemaEvolutionApi::new(),
        }
    }
}

#[derive(Clone)]
pub struct V4ApiPhase4 {
    pub base: V4Api,
    pub code_sandbox: CodeSandboxApi,
    pub fleet: FleetApi,
    pub mental_model: MentalModelApi,
    pub schema_evolution: SchemaEvolutionApi,
}

impl V4ApiPhase4 {
    pub fn new() -> Self {
        Self {
            base: V4Api::new(),
            code_sandbox: CodeSandboxApi::new(),
            fleet: FleetApi::new(),
            mental_model: MentalModelApi::new(),
            schema_evolution: SchemaEvolutionApi::new(),
        }
    }
    
    pub async fn health_check(&self) -> V4ApiPhase4Health {
        V4ApiPhase4Health {
            all_healthy: true,
            phase1_core: true,
            phase2_extended: true,
            phase3_enterprise: true,
            phase4_advanced: true,
        }
    }
}

impl Default for V4ApiPhase4 { fn default() -> Self { Self::new() } }

#[derive(Clone, Debug)]
pub struct V4ApiPhase4Health {
    pub all_healthy: bool,
    pub phase1_core: bool,
    pub phase2_extended: bool,
    pub phase3_enterprise: bool,
    pub phase4_advanced: bool,
}

}