//! Cache modules for AgentMem
//!
//! Provides various caching mechanisms for improving performance:
//! - Query embedding cache (LRU)
//! - Vector result cache (LRU)
//! - Semantic caching for vector search results

pub mod embedding_cache;

pub use embedding_cache::{CachedEmbedding, QueryEmbeddingCache};
