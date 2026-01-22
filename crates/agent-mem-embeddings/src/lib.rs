//! # Agent Memory Embeddings
//!
//! 嵌入模型模块，为AgentMem记忆平台提供多种嵌入模型支持。
//!
//! 本模块提供：
//! - 统一的嵌入接口抽象
//! - 多种嵌入提供商支持（OpenAI、HuggingFace、本地模型）
//! - 批量嵌入处理
//! - 嵌入工厂模式
//! - 特性门控支持

pub mod config;
pub mod factory;
pub mod providers;
pub mod utils;

// P1 优化 #20: 缓存embedder
pub mod cached_embedder;

pub use cached_embedder::CachedEmbedder;
pub use config::EmbeddingConfig;
pub use factory::{EmbeddingFactory, RealEmbeddingFactory};

// 🚀 Phase 1 性能优化验证
pub mod phase1_validation;
pub use phase1_validation::{
    validate_fastembed_optimization,
    validate_cache_optimization,
    validate_batch_optimization,
    validate_all_phase1_optimizations,
};

// 重新导出常用类型
pub use agent_mem_traits::{AgentMemError, Embedder, Result};
