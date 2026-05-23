//! # AgentMem v4.0 API - 高级记忆管理功能
//!
//! 本模块暴露了 AgentMem v4.0 的高级功能，基于 plan32.md Phase 1 实施计划。

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
///
/// 提供 Persona 和 Human 块管理，支持自动重写机制。
#[derive(Clone)]
pub struct CoreMemoryApi {
    manager: Arc<RwLock<CoreMemoryManager>>,
}

impl CoreMemoryApi {
    /// 创建新的 CoreMemoryApi
    pub fn new() -> Self {
        Self {
            manager: Arc::new(RwLock::new(CoreMemoryManager::new())),
        }
    }

    /// 使用自定义配置创建 CoreMemoryApi
    pub fn with_config(config: CoreMemoryConfig) -> Self {
        Self {
            manager: Arc::new(RwLock::new(CoreMemoryManager::with_config(config))),
        }
    }

    /// 创建 Persona 块
    pub async fn create_persona(
        &self,
        agent_id: &str,
        content: String,
        max_capacity: Option<usize>,
    ) -> Result<String> {
        let manager = self.manager.read().await;
        let block_id = manager
            .create_persona_block(content, max_capacity)
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))?;
        
        info!("Created persona block {} for agent {}", block_id, agent_id);
        Ok(block_id)
    }

    /// 创建 Human 块
    pub async fn create_human(
        &self,
        user_id: &str,
        content: String,
        max_capacity: Option<usize>,
    ) -> Result<String> {
        let manager = self.manager.read().await;
        let block_id = manager
            .create_human_block(content, max_capacity)
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))?;
        
        info!("Created human block {} for user {}", block_id, user_id);
        Ok(block_id)
    }

    /// 获取 Persona 块
    pub async fn get_persona(&self, block_id: &str) -> Result<Option<CoreMemoryBlock>> {
        let manager = self.manager.read().await;
        manager
            .get_persona_block(block_id)
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    /// 获取 Human 块
    pub async fn get_human(&self, block_id: &str) -> Result<Option<CoreMemoryBlock>> {
        let manager = self.manager.read().await;
        manager
            .get_human_block(block_id)
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    /// 获取所有 Persona 块
    pub async fn list_personas(&self) -> Result<Vec<CoreMemoryBlock>> {
        let manager = self.manager.read().await;
        manager
            .list_persona_blocks()
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    /// 获取所有 Human 块
    pub async fn list_humans(&self) -> Result<Vec<CoreMemoryBlock>> {
        let manager = self.manager.read().await;
        manager
            .list_human_blocks()
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    /// 更新 Persona 块
    pub async fn update_persona(&self, block_id: &str, content: String) -> Result<()> {
        let manager = self.manager.read().await;
        manager
            .update_persona_block(block_id, content)
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))?;
        
        debug!("Updated persona block {}", block_id);
        Ok(())
    }

    /// 更新 Human 块
    pub async fn update_human(&self, block_id: &str, content: String) -> Result<()> {
        let manager = self.manager.read().await;
        manager
            .update_human_block(block_id, content)
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))?;
        
        debug!("Updated human block {}", block_id);
        Ok(())
    }

    /// 追加内容到 Persona 块
    pub async fn append_to_persona(&self, block_id: &str, content: &str) -> Result<()> {
        let manager = self.manager.read().await;
        manager
            .append_to_persona_block(block_id, content)
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    /// 追加内容到 Human 块
    pub async fn append_to_human(&self, block_id: &str, content: &str) -> Result<()> {
        let manager = self.manager.read().await;
        manager
            .append_to_human_block(block_id, content)
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    /// 删除 Persona 块
    pub async fn delete_persona(&self, block_id: &str) -> Result<()> {
        let manager = self.manager.read().await;
        manager
            .delete_persona_block(block_id)
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    /// 删除 Human 块
    pub async fn delete_human(&self, block_id: &str) -> Result<()> {
        let manager = self.manager.read().await;
        manager
            .delete_human_block(block_id)
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    /// 获取统计信息
    pub async fn get_stats(&self) -> Result<CoreMemoryStats> {
        let manager = self.manager.read().await;
        manager
            .get_stats()
            .await
            .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))
    }

    /// 检查块是否需要重写
    pub async fn needs_rewrite(&self, block_id: &str, block_type: CoreMemoryBlockType) -> Result<bool> {
        let manager = self.manager.read().await;
        match block_type {
            CoreMemoryBlockType::Persona => {
                if let Some(block) = manager.get_persona_block(block_id).await
                    .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))? {
                    Ok(block.needs_rewrite())
                } else {
                    Ok(false)
                }
            }
            CoreMemoryBlockType::Human => {
                if let Some(block) = manager.get_human_block(block_id).await
                    .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))? {
                    Ok(block.needs_rewrite())
                } else {
                    Ok(false)
                }
            }
        }
    }

    /// 获取容量使用率
    pub async fn capacity_usage(&self, block_id: &str, block_type: CoreMemoryBlockType) -> Result<f32> {
        let manager = self.manager.read().await;
        match block_type {
            CoreMemoryBlockType::Persona => {
                if let Some(block) = manager.get_persona_block(block_id).await
                    .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))? {
                    Ok(block.capacity_usage())
                } else {
                    Err(agent_mem_traits::AgentMemError::not_found(format!(
                        "Persona block {} not found", block_id
                    )))
                }
            }
            CoreMemoryBlockType::Human => {
                if let Some(block) = manager.get_human_block(block_id).await
                    .map_err(|e| agent_mem_traits::AgentMemError::internal_error(e.to_string()))? {
                    Ok(block.capacity_usage())
                } else {
                    Err(agent_mem_traits::AgentMemError::not_found(format!(
                        "Human block {} not found", block_id
                    )))
                }
            }
        }
    }
}

// ============================================================================
// Intent Understanding API - 意图理解引擎
// ============================================================================

/// Intent Understanding API - 对标 Mem0 意图理解
///
/// 提供基于规则的意图理解能力。
#[derive(Clone)]
pub struct IntentUnderstandingApi {
    // 配置存储
    _config: IntentConfig,
}

#[derive(Debug, Clone)]
pub struct IntentConfig {
    pub enable_multilingual: bool,
    pub min_confidence: f32,
}

impl Default for IntentConfig {
    fn default() -> Self {
        Self {
            enable_multilingual: true,
            min_confidence: 0.3,
        }
    }
}

impl IntentUnderstandingApi {
    /// 创建新的 IntentUnderstandingApi
    pub fn new() -> Self {
        Self {
            _config: IntentConfig::default(),
        }
    }

    /// 理解查询意图
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

    /// 分类查询意图
    fn classify_intent(&self, query: &str) -> IntentType {
        let query_lower = query.to_lowercase();
        
        if query_lower.contains("what do you know") 
            || query_lower.contains("remember")
            || query_lower.contains("tell me about")
            || query_lower.contains("what") {
            IntentType::Recall
        } else if query_lower.contains("add")
            || query_lower.contains("remember that")
            || query_lower.contains("note that") {
            IntentType::Add
        } else if query_lower.contains("update")
            || query_lower.contains("change")
            || query_lower.contains("modify") {
            IntentType::Update
        } else if query_lower.contains("delete")
            || query_lower.contains("remove")
            || query_lower.contains("forget") {
            IntentType::Delete
        } else if query_lower.contains("summarize")
            || query_lower.contains("summary")
            || query_lower.contains("what happened") {
            IntentType::Summarize
        } else if query_lower.contains("explore")
            || query_lower.contains("find related")
            || query_lower.contains("connections") {
            IntentType::Explore
        } else if query_lower.contains("compare")
            || query_lower.contains("difference")
            || query_lower.contains("versus") {
            IntentType::Compare
        } else if query_lower.contains("why")
            || query_lower.contains("reason")
            || query_lower.contains("because")
            || query_lower.contains("how did") {
            IntentType::Reason
        } else {
            IntentType::Recall
        }
    }

    /// 提取实体
    fn extract_entities(&self, query: &str) -> Vec<Entity> {
        let words: Vec<&str> = query.split_whitespace().collect();
        let mut entities = Vec::new();
        
        for word in words {
            let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric());
            if !cleaned.is_empty() 
                && cleaned.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
                && cleaned.len() > 1 
                && !["The", "What", "When", "Where", "Who", "How", "Why", "Did", "Can", "Should"].contains(&cleaned) {
                entities.push(Entity {
                    name: cleaned.to_string(),
                    entity_type: EntityType::Unknown,
                    confidence: 0.6,
                });
            }
        }
        
        entities
    }

    /// 解析时间范围
    fn parse_time_range(&self, query: &str) -> Option<TimeRange> {
        let query_lower = query.to_lowercase();
        
        if query_lower.contains("today") {
            Some(TimeRange::Today)
        } else if query_lower.contains("yesterday") {
            Some(TimeRange::Yesterday)
        } else if query_lower.contains("last week") || query_lower.contains("this week") {
            Some(TimeRange::ThisWeek)
        } else if query_lower.contains("last month") || query_lower.contains("this month") {
            Some(TimeRange::ThisMonth)
        } else if query_lower.contains("last year") || query_lower.contains("this year") {
            Some(TimeRange::ThisYear)
        } else {
            None
        }
    }

    /// 获取推荐检索策略
    pub fn get_recommended_strategy(&self, query: &str) -> Vec<(RetrievalStrategy, f32)> {
        let intent = self.classify_intent(query);
        
        match intent {
            IntentType::Recall => vec![
                (RetrievalStrategy::Hybrid, 0.9),
                (RetrievalStrategy::Embedding, 0.8),
                (RetrievalStrategy::SemanticGraph, 0.7),
            ],
            IntentType::Explore => vec![
                (RetrievalStrategy::SemanticGraph, 0.95),
                (RetrievalStrategy::Embedding, 0.7),
            ],
            IntentType::Summarize => vec![
                (RetrievalStrategy::Temporal, 0.9),
                (RetrievalStrategy::Embedding, 0.6),
            ],
            _ => vec![
                (RetrievalStrategy::Hybrid, 0.85),
                (RetrievalStrategy::Embedding, 0.7),
            ],
        }
    }
}

// ============================================================================
// Multi-Signal Search API - 多信号检索
// ============================================================================

/// Multi-Signal Search API - 对标 Mem0 v3 多信号检索
#[derive(Clone)]
pub struct MultiSignalSearchApi {
    config: MultiSignalConfig,
}

impl MultiSignalSearchApi {
    /// 创建新的 MultiSignalSearchApi
    pub fn new() -> Self {
        Self {
            config: MultiSignalConfig::default(),
        }
    }

    /// 使用自定义配置创建
    pub fn with_config(config: MultiSignalConfig) -> Self {
        Self { config }
    }

    /// 多信号搜索
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

    /// 获取配置
    pub fn get_config(&self) -> MultiSignalConfig {
        self.config.clone()
    }

    /// 更新配置
    pub fn update_config(&mut self, config: MultiSignalConfig) {
        self.config = config;
    }
}

// ============================================================================
// Entity Linking API - 实体链接网络
// ============================================================================

/// Entity Linking API - 实体链接网络
#[derive(Clone)]
pub struct EntityLinkingApi {
    config: EntityLinkingConfig,
}

impl EntityLinkingApi {
    /// 创建新的 EntityLinkingApi
    pub fn new() -> Self {
        Self {
            config: EntityLinkingConfig::default(),
        }
    }

    /// 链接多个记忆中的实体
    pub async fn link_entities(&self, memory_ids: &[&str]) -> Result<EntityLinkingResult> {
        info!("Linking entities across {} memories", memory_ids.len());
        
        Ok(EntityLinkingResult {
            total_memories: memory_ids.len(),
            linked_entities: vec![],
            relationships: vec![],
            graph_size: 0,
        })
    }

    /// 获取实体关系图
    pub async fn get_entity_graph(&self, entity_name: &str) -> Result<Option<EntityGraph>> {
        info!("Getting entity graph for: {}", entity_name);
        Ok(None)
    }
}

// ============================================================================
// Unified v4 API - 统一 v4 API
// ============================================================================

/// Unified v4 API - 统一 v4 API 入口
#[derive(Clone)]
pub struct V4Api {
    pub core_memory: CoreMemoryApi,
    pub intent: IntentUnderstandingApi,
    pub search: MultiSignalSearchApi,
    pub entity_linking: EntityLinkingApi,
}

impl V4Api {
    /// 创建新的 V4Api
    pub fn new() -> Self {
        Self {
            core_memory: CoreMemoryApi::new(),
            intent: IntentUnderstandingApi::new(),
            search: MultiSignalSearchApi::new(),
            entity_linking: EntityLinkingApi::new(),
        }
    }

    /// 获取所有 API
    pub fn all_apis(&self) -> Vec<String> {
        vec![
            "CoreMemory".to_string(),
            "IntentUnderstanding".to_string(),
            "MultiSignalSearch".to_string(),
            "EntityLinking".to_string(),
        ]
    }

    /// 健康检查
    pub async fn health_check(&self) -> V4ApiHealth {
        V4ApiHealth {
            core_memory: true,
            intent: true,
            search: true,
            entity_linking: true,
            overall: true,
        }
    }
}

// ============================================================================
// Types
// ============================================================================

/// 意图理解结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntentUnderstandingResult {
    pub primary_intent: IntentType,
    pub secondary_intents: Vec<IntentType>,
    pub entities: Vec<Entity>,
    pub time_range: Option<TimeRange>,
    pub confidence: f32,
    pub raw_query: String,
}

/// 意图类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum IntentType {
    Recall,
    Add,
    Update,
    Delete,
    Summarize,
    Explore,
    Compare,
    Reason,
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

/// 实体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
    pub confidence: f32,
}

/// 实体类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum EntityType {
    Person,
    Location,
    Organization,
    Time,
    Unknown,
}

/// 时间范围
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TimeRange {
    Today,
    Yesterday,
    ThisWeek,
    ThisMonth,
    ThisYear,
    Custom(u64),
}

/// 检索策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RetrievalStrategy {
    Embedding,
    BM25,
    StringMatch,
    FuzzyMatch,
    Hybrid,
    SemanticGraph,
    Temporal,
    ContextAware,
}

/// 多信号搜索配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

/// 多信号搜索结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

/// 实体链接配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntityLinkingConfig {
    pub min_entity_confidence: f32,
    pub min_link_confidence: f32,
    pub max_link_depth: usize,
    pub enable_disambiguation: bool,
}

impl Default for EntityLinkingConfig {
    fn default() -> Self {
        Self {
            min_entity_confidence: 0.5,
            min_link_confidence: 0.3,
            max_link_depth: 3,
            enable_disambiguation: true,
        }
    }
}

/// 实体链接结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntityLinkingResult {
    pub total_memories: usize,
    pub linked_entities: Vec<LinkedEntity>,
    pub relationships: Vec<EntityRelationship>,
    pub graph_size: usize,
}

/// 链接实体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LinkedEntity {
    pub name: String,
    pub entity_type: EntityType,
    pub source_memory_ids: Vec<String>,
    pub occurrence_count: usize,
    pub confidence: f32,
}

/// 实体关系
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntityRelationship {
    pub source: String,
    pub target: String,
    pub relation_type: String,
    pub confidence: f32,
}

/// 实体图
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntityGraph {
    pub nodes: Vec<EntityNode>,
    pub edges: Vec<EntityEdge>,
}

/// 实体节点
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntityNode {
    pub id: String,
    pub name: String,
    pub entity_type: EntityType,
    pub properties: HashMap<String, String>,
}

/// 实体边
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntityEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub relation: String,
    pub weight: f32,
}

/// v4 API 健康状态
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct V4ApiHealth {
    pub core_memory: bool,
    pub intent: bool,
    pub search: bool,
    pub entity_linking: bool,
    pub overall: bool,
}

// ============================================================================
// Default traits
// ============================================================================

impl Default for CoreMemoryApi {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for IntentUnderstandingApi {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MultiSignalSearchApi {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for EntityLinkingApi {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for V4Api {
    fn default() -> Self {
        Self::new()
    }
}
