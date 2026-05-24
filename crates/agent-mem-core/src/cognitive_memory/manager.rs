//! CognitiveMemoryManager - 统一认知记忆管理器
//!
//! 融合8种认知记忆的统一管理接口：
//! - CoreMemory: 核心身份和角色记忆
//! - ContextualMemory: 上下文情境记忆
//! - SemanticMemory: 语义知识记忆
//! - EpisodicMemory: 事件情景记忆
//! - ProceduralMemory: 程序性步骤记忆
//! - WorkingMemory: 工作短期记忆
//! - ResourceMemory: 资源引用记忆
//! - KnowledgeMemory: 知识库记忆

use crate::managers::{
    ContextualMemoryManager, CoreMemoryManager, KnowledgeVaultManager, ResourceMemoryManager,
};
use crate::types::{Memory, MemoryType};
use crate::CoreResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 认知记忆操作类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CognitiveOperation {
    /// 添加记忆
    Add {
        content: String,
        memory_type: MemoryType,
        importance: Option<f32>,
        metadata: Option<HashMap<String, String>>,
    },
    /// 检索记忆
    Retrieve {
        query: String,
        memory_types: Option<Vec<MemoryType>>,
        limit: Option<usize>,
    },
    /// 更新记忆
    Update {
        id: String,
        content: Option<String>,
        importance: Option<f32>,
    },
    /// 删除记忆
    Delete { id: String },
    /// 提取模式
    ExtractPattern {
        memory_type: Option<MemoryType>,
        time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    },
}

/// 认知记忆结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveResult {
    /// 记忆列表
    pub memories: Vec<Memory>,
    /// 操作统计
    pub stats: CognitiveStats,
}

/// 认知统计
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CognitiveStats {
    /// 总记忆数
    pub total_memories: usize,
    /// 按类型统计
    pub by_type: HashMap<String, usize>,
    /// 操作耗时(ms)
    pub operation_time_ms: u64,
}

/// CognitiveMemory配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveMemoryConfig {
    /// 启用CoreMemory
    pub enable_core_memory: bool,
    /// 启用ContextualMemory
    pub enable_contextual_memory: bool,
    /// 启用ResourceMemory
    pub enable_resource_memory: bool,
    /// 启用KnowledgeVault
    pub enable_knowledge_vault: bool,
    /// 默认重要性
    pub default_importance: f32,
    /// 最大记忆数
    pub max_memories: usize,
}

impl Default for CognitiveMemoryConfig {
    fn default() -> Self {
        Self {
            enable_core_memory: true,
            enable_contextual_memory: true,
            enable_resource_memory: true,
            enable_knowledge_vault: true,
            default_importance: 0.5,
            max_memories: 1000,
        }
    }
}

/// CognitiveMemoryManager - 统一认知记忆管理器
///
/// 提供统一的接口来管理所有类型的认知记忆
pub struct CognitiveMemoryManager {
    /// 配置
    config: CognitiveMemoryConfig,
    /// 核心记忆管理器
    core_memory: Arc<RwLock<CoreMemoryManager>>,
    /// 上下文记忆管理器
    contextual_memory: Arc<RwLock<ContextualMemoryManager>>,
    /// 资源记忆管理器
    resource_memory: Arc<RwLock<ResourceMemoryManager>>,
    /// 知识库管理器
    knowledge_vault: Arc<RwLock<KnowledgeVaultManager>>,
    /// 记忆存储
    memories: Arc<RwLock<HashMap<String, Memory>>>,
}

impl CognitiveMemoryManager {
    /// 创建新的CognitiveMemoryManager
    pub async fn new(config: CognitiveMemoryConfig) -> CoreResult<Self> {
        let core_memory = Arc::new(RwLock::new(CoreMemoryManager::new()));
        let contextual_memory = Arc::new(RwLock::new(ContextualMemoryManager::new(
            crate::managers::ContextualMemoryConfig::default(),
        )));
        let resource_memory = Arc::new(RwLock::new(
            ResourceMemoryManager::new().map_err(|e| crate::CoreError::Internal("{e}".to_string()))?
        ));
        let knowledge_vault = Arc::new(RwLock::new(
            KnowledgeVaultManager::new(crate::managers::KnowledgeVaultConfig::default())
                .map_err(|e| crate::CoreError::Internal("{e}".to_string()))?
        ));

        Ok(Self {
            config,
            core_memory,
            contextual_memory,
            resource_memory,
            knowledge_vault,
            memories: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// 使用默认配置创建
    pub async fn with_default_config() -> CoreResult<Self> {
        Self::new(CognitiveMemoryConfig::default()).await
    }

    /// 添加记忆
    pub async fn add_memory(&self, memory: Memory) -> CoreResult<String> {
        let id = Uuid::new_v4().to_string();
        let mut memories = self.memories.write().await;
        memories.insert(id.clone(), memory);
        Ok(id)
    }

    /// 批量添加记忆
    pub async fn add_memories(&self, memories: Vec<Memory>) -> CoreResult<Vec<String>> {
        let mut ids = Vec::new();
        let mut mem_store = self.memories.write().await;
        for memory in memories {
            let id = Uuid::new_v4().to_string();
            mem_store.insert(id.clone(), memory);
            ids.push(id);
        }
        Ok(ids)
    }

    /// 获取记忆
    pub async fn get_memory(&self, id: &str) -> CoreResult<Option<Memory>> {
        let memories = self.memories.read().await;
        Ok(memories.get(id).cloned())
    }

    /// 检索记忆
    pub async fn retrieve(
        &self,
        query: &str,
        memory_types: Option<Vec<MemoryType>>,
        limit: usize,
    ) -> CoreResult<Vec<Memory>> {
        let _query = query; // Used for future search implementation
        let memories = self.memories.read().await;
        let mut results: Vec<Memory> = memories
            .values()
            .filter(|m| {
                // 按类型过滤
                if let Some(ref types) = memory_types {
                    let mem_type = m.memory_type();
                    return types.iter().any(|t| *t == mem_type);
                }
                true
            })
            .cloned()
            .collect();

        // 按相关性简单排序（这里用重要性作为代理）
        results.sort_by(|a, b| {
            b.importance()
                .partial_cmp(&a.importance())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // 限制数量
        results.truncate(limit);
        Ok(results)
    }

    /// 更新记忆
    #[allow(unused_variables)]
    pub async fn update_memory(
        &self,
        id: &str,
        content: Option<String>,
        importance: Option<f32>,
    ) -> CoreResult<bool> {
        let mut memories = self.memories.write().await;
        if memories.contains_key(id) {
            // Note: Memory更新需要通过builder或其他机制
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 删除记忆
    pub async fn delete_memory(&self, id: &str) -> CoreResult<bool> {
        let mut memories = self.memories.write().await;
        Ok(memories.remove(id).is_some())
    }

    /// 获取统计信息
    pub async fn get_stats(&self) -> CoreResult<CognitiveStats> {
        let memories = self.memories.read().await;
        let mut by_type: HashMap<String, usize> = HashMap::new();

        for memory in memories.values() {
            let mem_type = memory.memory_type();
            let type_str = mem_type.as_str();
            *by_type.entry(type_str.to_string()).or_insert(0) += 1;
        }

        Ok(CognitiveStats {
            total_memories: memories.len(),
            by_type,
            operation_time_ms: 0,
        })
    }

    /// 获取CoreMemoryManager
    pub fn core_memory_manager(&self) -> Arc<RwLock<CoreMemoryManager>> {
        self.core_memory.clone()
    }

    /// 获取ContextualMemoryManager
    pub fn contextual_memory_manager(&self) -> Arc<RwLock<ContextualMemoryManager>> {
        self.contextual_memory.clone()
    }

    /// 获取ResourceMemoryManager
    pub fn resource_memory_manager(&self) -> Arc<RwLock<ResourceMemoryManager>> {
        self.resource_memory.clone()
    }

    /// 获取KnowledgeVaultManager
    pub fn knowledge_vault_manager(&self) -> Arc<RwLock<KnowledgeVaultManager>> {
        self.knowledge_vault.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cognitive_memory_manager_creation() {
        let manager = CognitiveMemoryManager::with_default_config().await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_add_and_retrieve_memory() {
        let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

        let memory = Memory::new(
            "test-agent".to_string(),
            None,
            MemoryType::Semantic,
            "Test content".to_string(),
            0.5,
        );

        let id = manager.add_memory(memory.clone()).await.unwrap();
        assert!(!id.is_empty());

        let retrieved = manager.get_memory(&id).await.unwrap();
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_retrieve_with_filter() {
        let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

        // 添加多个记忆
        for i in 0..5 {
            let memory = Memory::new(
                "test-agent".to_string(),
                None,
                MemoryType::Semantic,
                format!("Content {}", i),
                0.5,
            );
            let _ = manager.add_memory(memory).await;
        }

        let results = manager.retrieve("Content", None, 10).await.unwrap();
        assert_eq!(results.len(), 5);
    }

    #[tokio::test]
    async fn test_delete_memory() {
        let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

        let memory = Memory::new(
            "test-agent".to_string(),
            None,
            MemoryType::Episodic,
            "To be deleted".to_string(),
            0.5,
        );
        let id = manager.add_memory(memory).await.unwrap();

        let deleted = manager.delete_memory(&id).await.unwrap();
        assert!(deleted);

        let result = manager.get_memory(&id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_stats() {
        let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

        // 添加一些记忆
        for i in 0..3 {
            let memory = Memory::new(
                "test-agent".to_string(),
                None,
                MemoryType::Semantic,
                format!("Stats {}", i),
                0.5,
            );
            let _ = manager.add_memory(memory).await;
        }

        let stats = manager.get_stats().await.unwrap();
        assert_eq!(stats.total_memories, 3);
    }
}
