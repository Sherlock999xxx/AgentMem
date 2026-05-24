//! CognitiveMemory Manager - 统一8种认知记忆的管理器
//!
//! 本模块实现了CognitiveMemoryProvider Trait，融合了：
//! - CoreMemory: 核心身份和角色记忆
//! - ContextualMemory: 上下文情境记忆  
//! - SemanticMemory: 语义知识记忆
//! - EpisodicMemory: 事件情景记忆
//! - ProceduralMemory: 程序性步骤记忆
//! - WorkingMemory: 工作短期记忆
//! - ResourceMemory: 资源引用记忆
//! - KnowledgeMemory: 知识库记忆

pub mod manager;

pub use manager::CognitiveMemoryManager;

