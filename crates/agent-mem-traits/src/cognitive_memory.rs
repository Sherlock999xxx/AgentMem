//! CognitiveMemory Trait - 统一8种认知记忆的接口
//!
//! 这个trait定义了AgentMem的统一认知记忆接口，融合了：
//! - CoreMemory: 核心身份和角色记忆
//! - ContextualMemory: 上下文情境记忆
//! - SemanticMemory: 语义知识记忆
//! - EpisodicMemory: 事件情景记忆
//! - ProceduralMemory: 程序性步骤记忆
//! - WorkingMemory: 工作短期记忆
//! - ResourceMemory: 资源引用记忆
//! - KnowledgeMemory: 知识库记忆

use crate::{MemoryItem, Result, Session};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 认知记忆类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CognitiveMemoryType {
    /// 核心记忆 - Agent身份、角色、核心价值观
    Core,
    /// 上下文记忆 - 当前会话、环境、情境
    Contextual,
    /// 语义记忆 - 事实知识、概念、定义
    Semantic,
    /// 情景记忆 - 具体事件、经历、时间线
    Episodic,
    /// 程序记忆 - 操作步骤、工作流程、方法
    Procedural,
    /// 工作记忆 - 当前任务、临时信息、焦点
    Working,
    /// 资源记忆 - 链接、文档、参考资料
    Resource,
    /// 知识记忆 - 领域知识、规则、约束
    Knowledge,
}

impl Default for CognitiveMemoryType {
    fn default() -> Self {
        CognitiveMemoryType::Core
    }
}

impl std::fmt::Display for CognitiveMemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CognitiveMemoryType::Core => write!(f, "core"),
            CognitiveMemoryType::Contextual => write!(f, "contextual"),
            CognitiveMemoryType::Semantic => write!(f, "semantic"),
            CognitiveMemoryType::Episodic => write!(f, "episodic"),
            CognitiveMemoryType::Procedural => write!(f, "procedural"),
            CognitiveMemoryType::Working => write!(f, "working"),
            CognitiveMemoryType::Resource => write!(f, "resource"),
            CognitiveMemoryType::Knowledge => write!(f, "knowledge"),
        }
    }
}

/// 认知记忆项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveMemoryItem {
    /// 记忆ID
    pub id: String,
    /// 记忆类型
    pub memory_type: CognitiveMemoryType,
    /// 记忆内容
    pub content: String,
    /// 重要性评分 (0.0-1.0)
    pub importance: f32,
    /// 创建时间戳
    pub created_at: i64,
    /// 更新时间戳
    pub updated_at: i64,
    /// 访问时间戳
    pub accessed_at: i64,
    /// 访问次数
    pub access_count: u64,
    /// 标签
    pub tags: Vec<String>,
    /// 元数据
    pub metadata: HashMap<String, String>,
    /// 关联记忆ID列表
    pub related_ids: Vec<String>,
    /// 是否持久化
    pub persistent: bool,
    /// TTL (秒)，0表示无限制
    pub ttl_seconds: u64,
}

impl CognitiveMemoryItem {
    pub fn new(id: String, memory_type: CognitiveMemoryType, content: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id,
            memory_type,
            content,
            importance: 0.5,
            created_at: now,
            updated_at: now,
            accessed_at: now,
            access_count: 0,
            tags: Vec::new(),
            metadata: HashMap::new(),
            related_ids: Vec::new(),
            persistent: false,
            ttl_seconds: 0,
        }
    }
}

/// 检索选项
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CognitiveRecallOptions {
    /// 最大返回结果数
    pub limit: usize,
    /// 最小相关性分数 (0.0-1.0)
    pub min_relevance: f32,
    /// 时间范围过滤（从时间戳）
    pub from_timestamp: Option<i64>,
    /// 时间范围过滤（到时间戳）
    pub to_timestamp: Option<i64>,
    /// 标签过滤
    pub tags: Option<Vec<String>>,
    /// 元数据过滤
    pub metadata_filter: Option<HashMap<String, String>>,
    /// 是否包含已过期的Working记忆
    pub include_expired: bool,
}

impl CognitiveRecallOptions {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            ..Default::default()
        }
    }
}

/// 检索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveRecallResult {
    /// 匹配的记忆项
    pub items: Vec<CognitiveMemoryItem>,
    /// 总匹配数
    pub total_count: usize,
    /// 检索时间（毫秒）
    pub retrieval_time_ms: u64,
    /// 平均相关性分数
    pub avg_relevance: f32,
}

/// 认知记忆Provider Trait
/// 
/// 统一的认知记忆接口，支持8种记忆类型的统一管理
#[async_trait]
pub trait CognitiveMemoryProvider: Send + Sync {
    // ========== 基础CRUD操作 ==========
    
    /// 添加认知记忆
    async fn add(&self, item: CognitiveMemoryItem) -> Result<String>;
    
    /// 批量添加认知记忆
    async fn add_batch(&self, items: Vec<CognitiveMemoryItem>) -> Result<Vec<String>>;
    
    /// 获取认知记忆
    async fn get(&self, id: &str) -> Result<Option<CognitiveMemoryItem>>;
    
    /// 更新认知记忆
    async fn update(&self, id: &str, content: &str) -> Result<()>;
    
    /// 删除认知记忆
    async fn delete(&self, id: &str) -> Result<()>;
    
    // ========== 检索操作 ==========
    
    /// 语义检索
    async fn search(&self, query: &str, session: &Session, options: CognitiveRecallOptions) -> Result<CognitiveRecallResult>;
    
    /// 按类型检索
    async fn get_by_type(&self, memory_type: CognitiveMemoryType, session: &Session, limit: usize) -> Result<Vec<CognitiveMemoryItem>>;
    
    /// 关联检索 - 获取与指定记忆相关的记忆
    async fn get_related(&self, id: &str, limit: usize) -> Result<Vec<CognitiveMemoryItem>>;
    
    // ========== 高级检索 ==========
    
    /// 时间范围检索
    async fn get_by_time_range(&self, from: i64, to: i64, session: &Session, limit: usize) -> Result<Vec<CognitiveMemoryItem>>;
    
    /// 标签检索
    async fn get_by_tags(&self, tags: &[String], session: &Session, limit: usize) -> Result<Vec<CognitiveMemoryItem>>;
    
    /// 精确内容检索
    async fn get_exact(&self, content: &str, session: &Session, limit: usize) -> Result<Vec<CognitiveMemoryItem>>;
    
    // ========== 特殊记忆类型操作 ==========
    
    /// 获取/设置Core记忆（Persona和Human块）
    async fn get_core_memory(&self, session: &Session) -> Result<HashMap<String, String>>;
    async fn set_core_memory(&self, session: &Session, block_type: &str, content: &str) -> Result<()>;
    
    /// 获取Working记忆
    async fn get_working_memory(&self, session: &Session) -> Result<Vec<CognitiveMemoryItem>>;
    
    /// 清除过期Working记忆
    async fn clear_expired_working(&self, session: &Session) -> Result<usize>;
    
    // ========== 统计和维护 ==========
    
    /// 获取记忆统计
    async fn get_stats(&self, session: &Session) -> Result<CognitiveMemoryStats>;
    
    /// 清理所有记忆
    async fn reset(&self) -> Result<()>;
}

/// 认知记忆统计
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CognitiveMemoryStats {
    /// 各类型记忆数量
    pub counts: HashMap<CognitiveMemoryType, usize>,
    /// 总记忆数
    pub total_count: usize,
    /// 总访问次数
    pub total_accesses: u64,
    /// 平均重要性
    pub avg_importance: f32,
    /// 最老记忆时间戳
    pub oldest_timestamp: Option<i64>,
    /// 最新记忆时间戳
    pub newest_timestamp: Option<i64>,
}
