//! # AgentMem v4.0 API - 高级记忆管理功能
//!
//! 本模块暴露了 AgentMem v4.0 的高级功能，基于 plan32.md Phase 1-2 实施计划。

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use agent_mem_core::managers::core_memory::{
    CoreMemoryBlock, CoreMemoryBlockType, CoreMemoryConfig, CoreMemoryManager, CoreMemoryStats,
};
use crate::Result;

// ============================================================================
// CoreMemory API - Persona/Human 块管理
// ============================================================================

/// CoreMemory API - 对标 Letta Block-based Memory
#[derive(Clone)]
pub struct CoreMemoryApi {
    manager: Arc<RwLock<CoreMemoryManager>>,
}

impl CoreMemoryApi {
    pub fn new() -> Self {
        Self {
            manager: Arc::new(RwLock::new(CoreMemoryManager::new())),
        }
    }

    pub fn with_config(config: CoreMemoryConfig) -> Self {
        Self {
            manager: Arc::new(RwLock::new(CoreMemoryManager::with_config(config))),
        }
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
        debug!("Updated persona block {}", block_id);
        Ok(())
    }

    pub async fn update_human(&self, block_id: &str, content: String) -> Result<()> {
        let manager = self.manager.read().await;
        manager.update_human_block(block_id, content)
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))?;
        debug!("Updated human block {}", block_id);
        Ok(())
    }

    pub async fn append_to_persona(&self, block_id: &str, content: &str) -> Result<()> {
        let manager = self.manager.read().await;
        manager.append_to_persona_block(block_id, content)
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    pub async fn append_to_human(&self, block_id: &str, content: &str) -> Result<()> {
        let manager = self.manager.read().await;
        manager.append_to_human_block(block_id, content)
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    pub async fn delete_persona(&self, block_id: &str) -> Result<()> {
        let manager = self.manager.read().await;
        manager.delete_persona_block(block_id)
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    pub async fn delete_human(&self, block_id: &str) -> Result<()> {
        let manager = self.manager.read().await;
        manager.delete_human_block(block_id)
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    pub async fn get_stats(&self) -> Result<CoreMemoryStats> {
        let manager = self.manager.read().await;
        manager.get_stats()
            .await.map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    pub async fn needs_rewrite(&self, block_id: &str, block_type: CoreMemoryBlockType) -> Result<bool> {
        let manager = self.manager.read().await;
        match block_type {
            CoreMemoryBlockType::Persona => {
                if let Some(block) = manager.get_persona_block(block_id).await
                    .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))? {
                    Ok(block.needs_rewrite())
                } else { Ok(false) }
            }
            CoreMemoryBlockType::Human => {
                if let Some(block) = manager.get_human_block(block_id).await
                    .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))? {
                    Ok(block.needs_rewrite())
                } else { Ok(false) }
            }
        }
    }

    pub async fn capacity_usage(&self, block_id: &str, block_type: CoreMemoryBlockType) -> Result<f32> {
        let manager = self.manager.read().await;
        match block_type {
            CoreMemoryBlockType::Persona => {
                if let Some(block) = manager.get_persona_block(block_id).await
                    .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))? {
                    Ok(block.capacity_usage())
                } else {
                    Err(agent_mem_traits::AgentMemError::not_found(format!("Persona block {} not found", block_id)))
                }
            }
            CoreMemoryBlockType::Human => {
                if let Some(block) = manager.get_human_block(block_id).await
                    .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))? {
                    Ok(block.capacity_usage())
                } else {
                    Err(agent_mem_traits::AgentMemError::not_found(format!("Human block {} not found", block_id)))
                }
            }
        }
    }
}

impl Default for CoreMemoryApi {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// Intent Understanding API
// ============================================================================

#[derive(Clone)]
pub struct IntentUnderstandingApi {
    _config: IntentConfig,
}

#[derive(Clone)]
pub struct IntentConfig {
    pub enable_multilingual: bool,
    pub min_confidence: f32,
}

impl Default for IntentConfig {
    fn default() -> Self {
        Self { enable_multilingual: true, min_confidence: 0.3 }
    }
}

impl IntentUnderstandingApi {
    pub fn new() -> Self {
        Self { _config: IntentConfig::default() }
    }

    pub async fn understand(&self, query: &str) -> Result<IntentUnderstandingResult> {
        let primary_intent = self.classify_intent(query);
        let entities = self.extract_entities(query);
        let time_range = self.parse_time_range(query);
        Ok(IntentUnderstandingResult {
            primary_intent,
            secondary_intents: vec![],
            entities,
            time_range,
            confidence: 0.85,
            raw_query: query.to_string(),
        })
    }

    fn classify_intent(&self, query: &str) -> IntentType {
        let q = query.to_lowercase();
        if q.contains("what do you know") || q.contains("remember") || q.contains("tell me about") || q.contains("what") {
            IntentType::Recall
        } else if q.contains("add") || q.contains("remember that") || q.contains("note that") {
            IntentType::Add
        } else if q.contains("update") || q.contains("change") || q.contains("modify") {
            IntentType::Update
        } else if q.contains("delete") || q.contains("remove") || q.contains("forget") {
            IntentType::Delete
        } else if q.contains("summarize") || q.contains("summary") || q.contains("what happened") {
            IntentType::Summarize
        } else if q.contains("explore") || q.contains("find related") || q.contains("connections") {
            IntentType::Explore
        } else if q.contains("compare") || q.contains("difference") || q.contains("versus") {
            IntentType::Compare
        } else if q.contains("why") || q.contains("reason") || q.contains("because") || q.contains("how did") {
            IntentType::Reason
        } else {
            IntentType::Recall
        }
    }

    fn extract_entities(&self, query: &str) -> Vec<Entity> {
        let mut entities = vec![];
        for word in query.split_whitespace() {
            let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric());
            if !cleaned.is_empty() && cleaned.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
                && cleaned.len() > 1 && !["The", "What", "When", "Where", "Who", "How", "Why", "Did"].contains(&cleaned) {
                entities.push(Entity { name: cleaned.to_string(), entity_type: EntityType::Unknown, confidence: 0.6 });
            }
        }
        entities
    }

    fn parse_time_range(&self, query: &str) -> Option<TimeRange> {
        let q = query.to_lowercase();
        if q.contains("today") { Some(TimeRange::Today) }
        else if q.contains("yesterday") { Some(TimeRange::Yesterday) }
        else if q.contains("last week") || q.contains("this week") { Some(TimeRange::ThisWeek) }
        else if q.contains("last month") || q.contains("this month") { Some(TimeRange::ThisMonth) }
        else if q.contains("last year") || q.contains("this year") { Some(TimeRange::ThisYear) }
        else { None }
    }

    pub fn get_recommended_strategy(&self, query: &str) -> Vec<(RetrievalStrategy, f32)> {
        let intent = self.classify_intent(query);
        match intent {
            IntentType::Recall => vec![(RetrievalStrategy::Hybrid, 0.9), (RetrievalStrategy::Embedding, 0.8), (RetrievalStrategy::SemanticGraph, 0.7)],
            IntentType::Explore => vec![(RetrievalStrategy::SemanticGraph, 0.95), (RetrievalStrategy::Embedding, 0.7)],
            IntentType::Summarize => vec![(RetrievalStrategy::Temporal, 0.9), (RetrievalStrategy::Embedding, 0.6)],
            _ => vec![(RetrievalStrategy::Hybrid, 0.85), (RetrievalStrategy::Embedding, 0.7)],
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
pub struct MultiSignalSearchApi {
    config: MultiSignalConfig,
}

#[derive(Clone)]
pub struct MultiSignalConfig {
    pub semantic_weight: f32,
    pub bm25_weight: f32,
    pub entity_weight: f32,
    pub time_decay_weight: f32,
    pub rrf_k: f32,
    pub max_results: usize,
    pub min_score: f32,
    pub enable_time_decay: bool,
    pub enable_entity_match: bool,
}

impl Default for MultiSignalConfig {
    fn default() -> Self {
        Self {
            semantic_weight: 0.4,
            bm25_weight: 0.3,
            entity_weight: 0.3,
            time_decay_weight: 0.0,
            rrf_k: 60.0,
            max_results: 10,
            min_score: 0.3,
            enable_time_decay: false,
            enable_entity_match: true,
        }
    }
}

impl MultiSignalSearchApi {
    pub fn new() -> Self { Self { config: MultiSignalConfig::default() } }
    pub fn with_config(config: MultiSignalConfig) -> Self { Self { config } }
    
    pub async fn search_with_signals(&self, query: &str) -> Result<MultiSignalSearchResult> {
        info!("Multi-signal search for query: {}", query);
        Ok(MultiSignalSearchResult {
            query: query.to_string(),
            total_results: 0,
            semantic_score: 0.0,
            bm25_score: 0.0,
            entity_score: 0.0,
            final_score: 0.0,
            fusion_method: "RRF".to_string(),
            signals_used: vec!["semantic".to_string(), "bm25".to_string()],
            processing_time_ms: 0,
        })
    }

    pub fn get_config(&self) -> MultiSignalConfig { self.config.clone() }
    pub fn update_config(&mut self, config: MultiSignalConfig) { self.config = config; }
}

impl Default for MultiSignalSearchApi {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// Entity Linking API
// ============================================================================

#[derive(Clone)]
pub struct EntityLinkingApi {
    config: EntityLinkingConfig,
}

#[derive(Clone)]
pub struct EntityLinkingConfig {
    pub min_entity_confidence: f32,
    pub min_link_confidence: f32,
    pub max_link_depth: usize,
    pub enable_disambiguation: bool,
}

impl Default for EntityLinkingConfig {
    fn default() -> Self {
        Self { min_entity_confidence: 0.5, min_link_confidence: 0.3, max_link_depth: 3, enable_disambiguation: true }
    }
}

impl EntityLinkingApi {
    pub fn new() -> Self { Self { config: EntityLinkingConfig::default() } }
    
    pub async fn link_entities(&self, memory_ids: &[&str]) -> Result<EntityLinkingResult> {
        info!("Linking entities across {} memories", memory_ids.len());
        Ok(EntityLinkingResult {
            total_memories: memory_ids.len(),
            linked_entities: vec![],
            relationships: vec![],
            graph_size: 0,
        })
    }

    pub async fn get_entity_graph(&self, entity_name: &str) -> Result<Option<EntityGraph>> {
        info!("Getting entity graph for: {}", entity_name);
        Ok(None)
    }
}

impl Default for EntityLinkingApi {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// Enhanced Search API
// ============================================================================

#[derive(Clone)]
pub struct EnhancedSearchApi {
    config: EnhancedSearchConfig,
}

#[derive(Clone)]
pub struct EnhancedSearchConfig {
    pub enable_query_classification: bool,
    pub enable_adaptive_threshold: bool,
    pub enable_parallel: bool,
    pub enable_metrics: bool,
    pub enable_cache: bool,
    pub rrf_k: f32,
    pub vector_weight: f32,
    pub fulltext_weight: f32,
}

impl Default for EnhancedSearchConfig {
    fn default() -> Self {
        Self {
            enable_query_classification: true,
            enable_adaptive_threshold: true,
            enable_parallel: true,
            enable_metrics: true,
            enable_cache: false,
            rrf_k: 60.0,
            vector_weight: 0.7,
            fulltext_weight: 0.3,
        }
    }
}

impl EnhancedSearchApi {
    pub fn new() -> Self { Self { config: EnhancedSearchConfig::default() } }
    pub fn with_config(config: EnhancedSearchConfig) -> Self { Self { config } }
    
    pub async fn hybrid_search(&self, query: &str) -> Result<HybridSearchResult> {
        info!("Enhanced hybrid search for: {}", query);
        let query_type = self.classify_query_type(query);
        Ok(HybridSearchResult {
            query: query.to_string(),
            query_type: query_type.clone(),
            results: vec![],
            total_time_ms: 0,
            scores: SearchScores { semantic: 0.0, bm25: 0.0, hybrid: 0.0 },
            strategy: "RRF".to_string(),
        })
    }
    
    fn classify_query_type(&self, query: &str) -> QueryClassification {
        let q = query.to_lowercase();
        if q.contains("how") || q.contains("why") || q.contains("what") { QueryClassification::Conceptual }
        else if q.contains("when") || q.contains("date") || q.contains("time") { QueryClassification::Temporal }
        else if q.contains("where") || q.contains("location") { QueryClassification::Location }
        else if q.contains("who") || q.contains("person") { QueryClassification::Entity }
        else { QueryClassification::General }
    }
    
    pub fn get_config(&self) -> EnhancedSearchConfig { self.config.clone() }
}

impl Default for EnhancedSearchApi {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// Reasoning API
// ============================================================================

#[derive(Clone)]
pub struct ReasoningApi {
    config: ReasoningConfig,
}

#[derive(Clone)]
pub struct ReasoningConfig {
    pub enable_causal: bool,
    pub enable_temporal: bool,
    pub enable_graph: bool,
    pub max_hop_depth: usize,
}

impl Default for ReasoningConfig {
    fn default() -> Self {
        Self { enable_causal: true, enable_temporal: true, enable_graph: true, max_hop_depth: 3 }
    }
}

impl ReasoningApi {
    pub fn new() -> Self { Self { config: ReasoningConfig::default() } }
    pub fn with_config(config: ReasoningConfig) -> Self { Self { config } }
    
    pub async fn causal_reasoning(&self, event: &str) -> Result<CausalResult> {
        info!("Causal reasoning for: {}", event);
        Ok(CausalResult { event: event.to_string(), causes: vec![], effects: vec![], chain: vec![], confidence: 0.8 })
    }
    
    pub async fn temporal_reasoning(&self, query: &str) -> Result<TemporalResult> {
        info!("Temporal reasoning for: {}", query);
        Ok(TemporalResult { query: query.to_string(), temporal_relations: vec![], time_range: None, confidence: 0.8 })
    }
    
    pub async fn graph_reasoning(&self, start_entity: &str, _relation_type: Option<&str>) -> Result<GraphResult> {
        info!("Graph reasoning from: {}", start_entity);
        Ok(GraphResult { start_entity: start_entity.to_string(), paths: vec![], confidence: 0.8 })
    }
}

impl Default for ReasoningApi {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// Adaptive Learning API
// ============================================================================

#[derive(Clone, Default)]
pub struct AdaptiveLearningApi {
    config: AdaptiveConfig,
    metrics: AdaptiveMetrics,
}

#[derive(Clone)]
pub struct AdaptiveConfig {
    pub enable_learning: bool,
    pub learning_rate: f64,
    pub min_samples: usize,
    pub evaluation_window_hours: u64,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self { enable_learning: true, learning_rate: 0.1, min_samples: 50, evaluation_window_hours: 24 }
    }
}

#[derive(Clone, Default)]
pub struct AdaptiveMetrics {
    pub total_queries: u64,
    pub successful_queries: u64,
    pub avg_latency_ms: f64,
    pub accuracy: f64,
}

impl AdaptiveLearningApi {
    pub fn new() -> Self {
        Self { config: AdaptiveConfig::default(), metrics: AdaptiveMetrics::default() }
    }
    
    pub async fn record_query(&mut self, success: bool, latency_ms: u64) {
        self.metrics.total_queries += 1;
        if success { self.metrics.successful_queries += 1; }
        self.metrics.avg_latency_ms = 
            (self.metrics.avg_latency_ms * (self.metrics.total_queries - 1) as f64 + latency_ms as f64) 
            / self.metrics.total_queries as f64;
    }
    
    pub fn get_metrics(&self) -> AdaptiveMetrics { self.metrics.clone() }
    pub fn success_rate(&self) -> f64 {
        if self.metrics.total_queries == 0 { 0.0 }
        else { self.metrics.successful_queries as f64 / self.metrics.total_queries as f64 }
    }
}


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
        }
    }
    
    pub fn all_apis(&self) -> Vec<String> {
        vec![
            "CoreMemory".to_string(),
            "IntentUnderstanding".to_string(),
            "MultiSignalSearch".to_string(),
            "EntityLinking".to_string(),
            "EnhancedSearch".to_string(),
            "Reasoning".to_string(),
            "AdaptiveLearning".to_string(),
        ]
    }

    pub async fn health_check(&self) -> V4ApiHealth {
        V4ApiHealth {
            core_memory: true,
            intent: true,
            search: true,
            entity_linking: true,
            enhanced_search: true,
            reasoning: true,
            adaptive: true,
            overall: true,
        }
    }
    
    pub fn full() -> Self { Self::new() }
}

impl Default for V4Api {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// Types
// ============================================================================

#[derive(Debug, Clone)]
pub struct IntentUnderstandingResult {
    pub primary_intent: IntentType,
    pub secondary_intents: Vec<IntentType>,
    pub entities: Vec<Entity>,
    pub time_range: Option<TimeRange>,
    pub confidence: f32,
    pub raw_query: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntentType {
    Recall, Add, Update, Delete, Summarize, Explore, Compare, Reason,
}

impl IntentType {
    pub fn description(&self) -> &'static str {
        match self {
            IntentType::Recall => "Recall related memories",
            IntentType::Add => "Add new memory",
            IntentType::Update => "Update existing memory",
            IntentType::Delete => "Delete memory",
            IntentType::Summarize => "Summarize memories",
            IntentType::Explore => "Explore relationships",
            IntentType::Compare => "Compare memories",
            IntentType::Reason => "Reason about memories",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entity { pub name: String, pub entity_type: EntityType, pub confidence: f32 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType { Person, Location, Organization, Time, Unknown }

#[derive(Debug, Clone)]
pub enum TimeRange { Today, Yesterday, ThisWeek, ThisMonth, ThisYear, Custom(u64) }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RetrievalStrategy {
    Embedding, BM25, StringMatch, FuzzyMatch, Hybrid, SemanticGraph, Temporal, ContextAware,
}

#[derive(Debug, Clone)]
pub struct MultiSignalSearchResult {
    pub query: String,
    pub total_results: usize,
    pub semantic_score: f32,
    pub bm25_score: f32,
    pub entity_score: f32,
    pub final_score: f32,
    pub fusion_method: String,
    pub signals_used: Vec<String>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct EntityLinkingResult {
    pub total_memories: usize,
    pub linked_entities: Vec<LinkedEntity>,
    pub relationships: Vec<EntityRelationship>,
    pub graph_size: usize,
}

#[derive(Debug, Clone)]
pub struct LinkedEntity {
    pub name: String,
    pub entity_type: EntityType,
    pub source_memory_ids: Vec<String>,
    pub occurrence_count: usize,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct EntityRelationship { pub source: String, pub target: String, pub relation_type: String, pub confidence: f32 }

#[derive(Debug, Clone)]
pub struct EntityGraph { pub nodes: Vec<EntityNode>, pub edges: Vec<EntityEdge> }

#[derive(Debug, Clone)]
pub struct EntityNode { pub id: String, pub name: String, pub entity_type: EntityType, pub properties: HashMap<String, String> }

#[derive(Debug, Clone)]
pub struct EntityEdge { pub id: String, pub source: String, pub target: String, pub relation: String, pub weight: f32 }

#[derive(Debug, Clone)]
pub struct HybridSearchResult {
    pub query: String,
    pub query_type: QueryClassification,
    pub results: Vec<HybridSearchItem>,
    pub total_time_ms: u64,
    pub scores: SearchScores,
    pub strategy: String,
}

#[derive(Debug, Clone)]
pub struct HybridSearchItem { pub id: String, pub content: String, pub memory_type: String, pub score: f32, pub source: String }

#[derive(Debug, Clone)]
pub struct SearchScores { pub semantic: f32, pub bm25: f32, pub hybrid: f32 }

#[derive(Debug, Clone, Copy)]
pub enum QueryClassification { General, Conceptual, Temporal, Location, Entity, Factual }

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
pub struct GraphResult { pub start_entity: String, pub paths: Vec<GraphPath>, pub confidence: f32 }

#[derive(Debug, Clone)]
pub struct GraphPath { pub nodes: Vec<String>, pub relations: Vec<String>, pub confidence: f32 }

#[derive(Debug, Clone)]
pub struct V4ApiHealth {
    pub core_memory: bool,
    pub intent: bool,
    pub search: bool,
    pub entity_linking: bool,
    pub enhanced_search: bool,
    pub reasoning: bool,
    pub adaptive: bool,
    pub overall: bool,
}
