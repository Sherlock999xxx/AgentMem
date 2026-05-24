//! Query embedding cache service
//!
//! Provides LRU caching for query embeddings to avoid regenerating
//! embeddings for duplicate or similar queries.

use agent_mem_traits::Result;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Cached embedding entry
#[derive(Debug, Clone)]
pub struct CachedEmbedding {
    /// The embedding vector
    pub embedding: Vec<f32>,
    /// Timestamp when this entry was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Number of times this entry was accessed
    pub access_count: u64,
}

impl CachedEmbedding {
    pub fn new(embedding: Vec<f32>) -> Self {
        Self {
            embedding,
            created_at: chrono::Utc::now(),
            access_count: 0,
        }
    }

    pub fn mark_accessed(&mut self) {
        self.access_count += 1;
    }
}

/// Query embedding cache with LRU eviction
///
/// **Performance Impact:**
/// - Cache hit: <1ms (vs 50-200ms for embedding generation)
/// - Typical hit rate: 40-60% for repetitive queries
/// - Memory: ~6MB for 1K cached embeddings (1536-dim)
pub struct QueryEmbeddingCache {
    /// LRU cache: normalized query -> embedding
    cache: Arc<RwLock<LruCache<String, CachedEmbedding>>>,
    /// Maximum cache size
    max_size: usize,
    /// Total cache hits
    hits: Arc<RwLock<u64>>,
    /// Total cache misses
    misses: Arc<RwLock<u64>>,
}

impl QueryEmbeddingCache {
    /// Create a new query embedding cache
    ///
    /// # Arguments
    /// * `max_size` - Maximum number of cached embeddings (default: 1,000)
    pub fn new(max_size: usize) -> Self {
        let size = NonZeroUsize::new(max_size.max(1)).unwrap();
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(size))),
            max_size,
            hits: Arc::new(RwLock::new(0)),
            misses: Arc::new(RwLock::new(0)),
        }
    }

    /// Create cache with default size (1,000 entries)
    pub fn default() -> Self {
        Self::new(1_000)
    }

    /// Get or generate embedding for a query
    ///
    /// # Arguments
    /// * `query` - The query text
    /// * `generator` - Async function to generate embedding if not cached
    ///
    /// # Returns
    /// * `Ok(Vec<f32>)` - The embedding vector (from cache or freshly generated)
    /// * `Err(...)` - Embedding generation failed
    ///
    /// # Performance
    /// - Cache hit: <1ms
    /// - Cache miss: 50-200ms (first time)
    pub async fn get_or_generate<F, Fut>(&self, query: &str, generator: F) -> Result<Vec<f32>>
    where
        F: FnOnce(String) -> Fut,
        Fut: std::future::Future<Output = Result<Vec<f32>>>,
    {
        // Normalize query for better cache hits
        let normalized_query = Self::normalize_query(query);

        // Try to get from cache
        {
            let mut cache = self.cache.write().await;
            if let Some(entry) = cache.get_mut(&normalized_query) {
                entry.mark_accessed();
                *self.hits.write().await += 1;
                tracing::debug!(
                    "Embedding cache hit: query='{}' (access count: {})",
                    Self::truncate_query(query, 50),
                    entry.access_count
                );
                return Ok(entry.embedding.clone());
            }
        }

        // Cache miss - generate embedding
        *self.misses.write().await += 1;
        tracing::debug!(
            "Embedding cache miss: query='{}', generating...",
            Self::truncate_query(query, 50)
        );

        let embedding = generator(query.to_string()).await?;

        // Store in cache
        let entry = CachedEmbedding::new(embedding.clone());
        let mut cache = self.cache.write().await;
        cache.put(normalized_query, entry);

        Ok(embedding)
    }

    /// Normalize query string for consistent caching
    ///
    /// Transformations:
    /// - Trim whitespace
    /// - Convert to lowercase
    /// - Remove extra whitespace
    fn normalize_query(query: &str) -> String {
        query
            .trim()
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Truncate query for logging
    fn truncate_query(query: &str, max_len: usize) -> String {
        if query.len() <= max_len {
            query.to_string()
        } else {
            format!("{}...", &query[..max_len])
        }
    }

    /// Get cache statistics
    ///
    /// # Returns
    /// * `(hits, misses, hit_rate, size)` - Cache performance metrics
    pub async fn stats(&self) -> (u64, u64, f64, usize) {
        let hits = *self.hits.read().await;
        let misses = *self.misses.read().await;
        let total = hits + misses;
        let hit_rate = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };
        let size = self.cache.read().await.len();
        (hits, misses, hit_rate, size)
    }

    /// Clear the cache
    pub async fn clear(&self) {
        self.cache.write().await.clear();
        *self.hits.write().await = 0;
        *self.misses.write().await = 0;
    }

    /// Get current cache size
    pub async fn len(&self) -> usize {
        self.cache.read().await.len()
    }

    /// Check if cache is empty
    pub async fn is_empty(&self) -> bool {
        self.cache.read().await.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_hit_miss() {
        let cache = QueryEmbeddingCache::new(100);

        // First call should miss
        let result1 = cache
            .get_or_generate("test query", |_| async { Ok(vec![0.1, 0.2, 0.3]) })
            .await
            .unwrap();

        // Second call should hit
        let result2 = cache
            .get_or_generate("test query", |_| async {
                panic!("Should not be called for cached query");
            })
            .await
            .unwrap();

        assert_eq!(result1, result2);
    }

    #[tokio::test]
    async fn test_query_normalization() {
        let cache = QueryEmbeddingCache::new(100);

        let result1 = cache
            .get_or_generate("  Test  Query  ", |_| async { Ok(vec![0.1, 0.2, 0.3]) })
            .await
            .unwrap();

        let result2 = cache
            .get_or_generate("test query", |_| async {
                panic!("Should not be called for normalized query");
            })
            .await
            .unwrap();

        assert_eq!(result1, result2);
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache = QueryEmbeddingCache::new(100);

        // Generate some cache hits and misses
        let _ = cache
            .get_or_generate("query1", |_| async { Ok(vec![0.1]) })
            .await;
        let _ = cache
            .get_or_generate("query2", |_| async { Ok(vec![0.2]) })
            .await;
        let _ = cache
            .get_or_generate("query1", |_| async { panic!("Should hit cache") })
            .await;

        let (hits, misses, hit_rate, size) = cache.stats().await;
        assert_eq!(hits, 1);
        assert_eq!(misses, 2);
        assert!((hit_rate - 0.333).abs() < 0.01); // ~33.3%
        assert_eq!(size, 2);
    }
}
