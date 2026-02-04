//! Real MemVid API integration using memvid-core 2.0

use crate::error::{MemvidError, Result};
use agent_mem_traits::{Memory, MemoryId, Content, AttributeSet, MetadataV4};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
use std::io;

/// Re-export memvid-core types
pub use memvid_core::{
    Memvid, PutOptions, SearchRequest, SearchResponse, TimelineQuery,
    OpenReadOptions, Frame, SearchHit, Stats
};

use memvid_core::types::FrameStatus;

/// Real MemVid store implementation using memvid-core 2.0
pub struct RealMemvidStore {
    /// Path to the .mv2 file
    path: String,

    /// In-memory cache for hot data
    cache: Arc<RwLock<lru::LruCache<String, Memory>>>,
}

impl RealMemvidStore {
    /// Create a new MemVid file
    pub async fn create(path: impl Into<String>) -> Result<Self> {
        let path = path.into();
        info!("Creating MemVid store: {}", path);

        // Create the MemVid file
        let _mem = Memvid::create(Path::new(&path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to create: {}", e)))?;

        // Initialize cache
        let cache_size = std::num::NonZeroUsize::new(1000).unwrap();
        let cache = Arc::new(RwLock::new(lru::LruCache::new(cache_size)));

        Ok(Self { path, cache })
    }

    /// Open an existing MemVid file
    pub async fn open(path: impl Into<String>) -> Result<Self> {
        let path_str = path.into();
        info!("Opening MemVid store: {}", path_str);

        if !Path::new(&path_str).exists() {
            return Err(MemvidError::Io(io::Error::new(io::ErrorKind::NotFound, path_str)));
        }

        let _mem = Memvid::open(&path_str)
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        let cache_size = std::num::NonZeroUsize::new(1000).unwrap();
        let cache = Arc::new(RwLock::new(lru::LruCache::new(cache_size)));

        Ok(Self { path: path_str, cache })
    }

    /// Add a memory
    pub async fn add(&self, memory: &Memory) -> Result<()> {
        debug!("Adding memory: {}", memory.id);

        let mut mem = Memvid::open(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        // Convert content to bytes
        let content = self.memory_to_bytes(memory)?;

        let uri = format!("mv2://memory/{}", memory.id.as_str());
        let search_text = format!("{}", memory.content);

        let options = PutOptions {
            uri: Some(uri.clone()),
            title: Some(format!("Memory: {}", memory.id.as_str())),
            search_text: Some(search_text),
            ..Default::default()
        };

        mem.put_bytes_with_options(&content, options)
            .map_err(|e| MemvidError::Memvid(format!("Failed to write: {}", e)))?;

        mem.commit()
            .map_err(|e| MemvidError::Memvid(format!("Failed to commit: {}", e)))?;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.put(memory.id.as_str().to_string(), memory.clone());

        Ok(())
    }

    /// Get a memory
    pub async fn get(&self, id: &MemoryId) -> Result<Option<Memory>> {
        debug!("Getting memory: {}", id);

        // Check cache first, but validate it's not stale
        {
            let mut cache = self.cache.write().await;
            if let Some(_memory) = cache.get(id.as_str()) {
                // Found in cache - but we need to verify it's still valid
                // Drop cache lock before loading from MemVid
            }
        }

        // Load from MemVid
        let mut mem = Memvid::open_read_only(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        let uri = format!("mv2://memory/{}", id.as_str());
        let frame = mem.frame_by_uri(&uri);

        if let Ok(frame) = frame {
            // Check if frame is deleted
            if frame.status != FrameStatus::Active {
                // Frame is deleted, remove from cache
                let mut cache = self.cache.write().await;
                cache.pop(id.as_str());
                return Ok(None);
            }

            // Try to get the text
            match mem.frame_text_by_id(frame.id) {
                Ok(text) => {
                    let memory = Memory {
                        id: id.clone(),
                        content: Content::text(text),
                        attributes: AttributeSet::new(),
                        relations: Default::default(),
                        metadata: MetadataV4::default(),
                    };

                    // Update cache
                    let mut cache = self.cache.write().await;
                    cache.put(id.as_str().to_string(), memory.clone());

                    Ok(Some(memory))
                }
                Err(_) => {
                    // Frame was deleted or text not available
                    // Remove from cache if present
                    let mut cache = self.cache.write().await;
                    cache.pop(id.as_str());
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }

    /// Update a memory
    pub async fn update(&self, memory: &Memory) -> Result<()> {
        debug!("Updating memory: {}", memory.id);

        let mut mem = Memvid::open(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        // Find existing frame
        let uri = format!("mv2://memory/{}", memory.id.as_str());
        let existing = mem.frame_by_uri(&uri);

        if let Ok(frame) = existing {
            let content = self.memory_to_bytes(memory)?;

            let search_text = format!("{}", memory.content);
            let options = PutOptions {
                uri: Some(uri),
                title: Some(format!("Memory: {}", memory.id.as_str())),
                search_text: Some(search_text),
                ..Default::default()
            };

            mem.update_frame(frame.id, Some(content), options, None)
                .map_err(|e| MemvidError::Memvid(format!("Failed to update: {}", e)))?;

            mem.commit()
                .map_err(|e| MemvidError::Memvid(format!("Failed to commit: {}", e)))?;

            // Update cache
            let mut cache = self.cache.write().await;
            cache.put(memory.id.as_str().to_string(), memory.clone());

            Ok(())
        } else {
            Err(MemvidError::MemoryNotFound(format!("Memory not found: {}", memory.id)))
        }
    }

    /// Delete a memory
    pub async fn delete(&self, id: &MemoryId) -> Result<()> {
        debug!("Deleting memory: {}", id);

        let mut mem = Memvid::open(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        let uri = format!("mv2://memory/{}", id.as_str());
        let frame = mem.frame_by_uri(&uri);

        if let Ok(frame) = frame {
            mem.delete_frame(frame.id)
                .map_err(|e| MemvidError::Memvid(format!("Failed to delete: {}", e)))?;

            mem.commit()
                .map_err(|e| MemvidError::Memvid(format!("Failed to commit: {}", e)))?;

            // Remove from cache
            let mut cache = self.cache.write().await;
            cache.pop(id.as_str());

            Ok(())
        } else {
            // Frame not found - might already be deleted
            // Remove from cache anyway
            let mut cache = self.cache.write().await;
            cache.pop(id.as_str());

            Err(MemvidError::MemoryNotFound(format!("Memory not found: {}", id)))
        }
    }

    /// List all memories
    pub async fn list(&self) -> Result<Vec<Memory>> {
        debug!("Listing memories");

        let mut mem = Memvid::open_read_only(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        let stats = mem.stats()
            .map_err(|e| MemvidError::Memvid(format!("Failed to get stats: {}", e)))?;

        let mut memories = Vec::new();

        for frame_id in 0..stats.frame_count {
            if let Ok(frame) = mem.frame_by_id(frame_id) {
                // Only include active frames
                if frame.status != FrameStatus::Active {
                    continue;
                }

                if let Ok(text) = mem.frame_text_by_id(frame_id) {
                    // Extract memory ID from URI
                    if let Some(uri) = &frame.uri {
                        if let Some(memory_id) = uri.strip_prefix("mv2://memory/") {
                            let memory = Memory {
                                id: MemoryId::from_string(memory_id.to_string()),
                                content: Content::text(text),
                                attributes: AttributeSet::new(),
                                relations: Default::default(),
                                metadata: MetadataV4::default(),
                            };
                            memories.push(memory);
                        }
                    }
                }
            }
        }

        Ok(memories)
    }

    /// Count memories
    pub async fn count(&self) -> Result<usize> {
        let mut mem = Memvid::open_read_only(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        let stats = mem.stats()
            .map_err(|e| MemvidError::Memvid(format!("Failed to get stats: {}", e)))?;

        // Count only active frames with mv2://memory/ URIs
        let mut count = 0;
        for frame_id in 0..stats.frame_count {
            if let Ok(frame) = mem.frame_by_id(frame_id) {
                // Check if frame is active and has a memory URI
                if frame.status == FrameStatus::Active {
                    if let Some(uri) = &frame.uri {
                        if uri.starts_with("mv2://memory/") {
                            count += 1;
                        }
                    }
                }
            }
        }

        Ok(count)
    }

    /// Search memories
    pub async fn search(&self, query: &str, top_k: usize) -> Result<Vec<SearchHit>> {
        debug!("Searching: query='{}', top_k={}", query, top_k);

        let mut mem = Memvid::open_read_only(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        let response = mem.search(SearchRequest {
            query: query.to_string(),
            top_k,
            snippet_chars: 200,
            uri: Some("mv2://memory/".to_string()),
            scope: None,
            cursor: None,
            no_sketch: false,
            as_of_frame: None,
            as_of_ts: None,
        }).map_err(|e| MemvidError::Memvid(format!("Search failed: {}", e)))?;

        Ok(response.hits)
    }

    /// Fuzzy search for approximate matching
    pub async fn search_fuzzy(&self, query: &str, top_k: usize) -> Result<Vec<SearchHit>> {
        debug!("Fuzzy search: query='{}', top_k={}", query, top_k);

        let mut mem = Memvid::open_read_only(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        // Add fuzzy operator
        let fuzzy_query = format!("{}~", query);

        let response = mem.search(SearchRequest {
            query: fuzzy_query,
            top_k,
            snippet_chars: 200,
            uri: Some("mv2://memory/".to_string()),
            scope: None,
            cursor: None,
            no_sketch: false,
            as_of_frame: None,
            as_of_ts: None,
        }).map_err(|e| MemvidError::Memvid(format!("Fuzzy search failed: {}", e)))?;

        Ok(response.hits)
    }

    /// Phrase search for exact matching
    pub async fn search_phrase(&self, phrase: &str, top_k: usize) -> Result<Vec<SearchHit>> {
        debug!("Phrase search: phrase='{}', top_k={}", phrase, top_k);

        let mut mem = Memvid::open_read_only(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        // Wrap in quotes for exact phrase matching
        let phrase_query = format!("\"{}\"", phrase);

        let response = mem.search(SearchRequest {
            query: phrase_query,
            top_k,
            snippet_chars: 200,
            uri: Some("mv2://memory/".to_string()),
            scope: None,
            cursor: None,
            no_sketch: false,
            as_of_frame: None,
            as_of_ts: None,
        }).map_err(|e| MemvidError::Memvid(format!("Phrase search failed: {}", e)))?;

        Ok(response.hits)
    }

    /// Multi-term search (combines terms with OR)
    pub async fn search_multi(&self, terms: Vec<&str>, top_k: usize) -> Result<Vec<SearchHit>> {
        debug!("Multi-term search: terms={:?}, top_k={}", terms, top_k);

        let mut mem = Memvid::open_read_only(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        // Combine queries with OR
        let combined_query = terms.join(" OR ");

        let response = mem.search(SearchRequest {
            query: combined_query,
            top_k,
            snippet_chars: 200,
            uri: Some("mv2://memory/".to_string()),
            scope: None,
            cursor: None,
            no_sketch: false,
            as_of_frame: None,
            as_of_ts: None,
        }).map_err(|e| MemvidError::Memvid(format!("Multi-term search failed: {}", e)))?;

        Ok(response.hits)
    }

    /// Get statistics
    pub async fn stats(&self) -> Result<Stats> {
        let mem = Memvid::open_read_only(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        mem.stats()
            .map_err(|e| MemvidError::Memvid(format!("Failed to get stats: {}", e)))
    }

    // ============================================================================
    // Batch Operations
    // ============================================================================

    /// Add multiple memories in a single transaction
    pub async fn batch_add(&self, memories: &[Memory]) -> Result<Vec<MemoryId>> {
        if memories.is_empty() {
            return Ok(Vec::new());
        }

        debug!("Batch adding {} memories", memories.len());

        let mut mem = Memvid::open(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        let mut ids = Vec::with_capacity(memories.len());
        let mut cache = self.cache.write().await;

        for memory in memories {
            let content = self.memory_to_bytes(memory)?;
            let uri = format!("mv2://memory/{}", memory.id.as_str());
            let search_text = format!("{}", memory.content);

            let options = PutOptions {
                uri: Some(uri.clone()),
                title: Some(format!("Memory: {}", memory.id.as_str())),
                search_text: Some(search_text),
                ..Default::default()
            };

            mem.put_bytes_with_options(&content, options)
                .map_err(|e| MemvidError::Memvid(format!("Failed to write: {}", e)))?;

            // Update cache
            cache.put(memory.id.as_str().to_string(), memory.clone());
            ids.push(memory.id.clone());
        }

        // Single commit for all operations
        mem.commit()
            .map_err(|e| MemvidError::Memvid(format!("Failed to commit: {}", e)))?;

        info!("Batch added {} memories successfully", ids.len());
        Ok(ids)
    }

    /// Get multiple memories by their IDs
    pub async fn batch_get(&self, ids: &[MemoryId]) -> Result<Vec<Option<Memory>>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        debug!("Batch getting {} memories", ids.len());

        let mut results = Vec::with_capacity(ids.len());
        let mut mem = Memvid::open_read_only(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        // First pass: check cache
        let mut cache = self.cache.write().await;
        let mut uncached_ids = Vec::new();
        let mut uncached_indices = Vec::new();

        for (index, id) in ids.iter().enumerate() {
            if let Some(memory) = cache.get(id.as_str()) {
                results.push(Some(memory.clone()));
            } else {
                results.push(None);
                uncached_ids.push(id.clone());
                uncached_indices.push(index);
            }
        }

        // Second pass: load uncached from MemVid
        drop(cache); // Release cache lock before MemVid operations

        for (id, index) in uncached_ids.into_iter().zip(uncached_indices.into_iter()) {
            let uri = format!("mv2://memory/{}", id.as_str());
            let frame = mem.frame_by_uri(&uri);

            if let Ok(frame) = frame {
                if frame.status != FrameStatus::Active {
                    continue;
                }

                if let Ok(text) = mem.frame_text_by_id(frame.id) {
                    let memory = Memory {
                        id: id.clone(),
                        content: Content::text(text),
                        attributes: AttributeSet::new(),
                        relations: Default::default(),
                        metadata: MetadataV4::default(),
                    };

                    // Update cache
                    let mut cache = self.cache.write().await;
                    cache.put(id.as_str().to_string(), memory.clone());
                    results[index] = Some(memory);
                }
            }
        }

        Ok(results)
    }

    /// Delete multiple memories in a single transaction
    pub async fn batch_delete(&self, ids: &[MemoryId]) -> Result<usize> {
        if ids.is_empty() {
            return Ok(0);
        }

        debug!("Batch deleting {} memories", ids.len());

        let mut mem = Memvid::open(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        let mut deleted_count = 0;
        let mut cache = self.cache.write().await;

        for id in ids {
            let uri = format!("mv2://memory/{}", id.as_str());
            let frame = mem.frame_by_uri(&uri);

            if let Ok(frame) = frame {
                mem.delete_frame(frame.id)
                    .map_err(|e| MemvidError::Memvid(format!("Failed to delete: {}", e)))?;

                // Remove from cache
                cache.pop(id.as_str());
                deleted_count += 1;
            }
        }

        // Single commit for all deletions
        if deleted_count > 0 {
            mem.commit()
                .map_err(|e| MemvidError::Memvid(format!("Failed to commit: {}", e)))?;
        }

        info!("Batch deleted {} memories successfully", deleted_count);
        Ok(deleted_count)
    }

    /// Update multiple memories in a single transaction
    pub async fn batch_update(&self, memories: &[Memory]) -> Result<Vec<MemoryId>> {
        if memories.is_empty() {
            return Ok(Vec::new());
        }

        debug!("Batch updating {} memories", memories.len());

        let mut mem = Memvid::open(Path::new(&self.path))
            .map_err(|e| MemvidError::Memvid(format!("Failed to open: {}", e)))?;

        let mut ids = Vec::with_capacity(memories.len());
        let mut cache = self.cache.write().await;

        for memory in memories {
            let uri = format!("mv2://memory/{}", memory.id.as_str());

            // Delete old version if it exists
            if let Ok(old_frame) = mem.frame_by_uri(&uri) {
                let _ = mem.delete_frame(old_frame.id);
            }

            let content = self.memory_to_bytes(memory)?;
            let search_text = format!("{}", memory.content);

            let options = PutOptions {
                uri: Some(uri.clone()),
                title: Some(format!("Memory: {}", memory.id.as_str())),
                search_text: Some(search_text),
                ..Default::default()
            };

            // Write new version
            mem.put_bytes_with_options(&content, options)
                .map_err(|e| MemvidError::Memvid(format!("Failed to write: {}", e)))?;

            // Update cache
            cache.put(memory.id.as_str().to_string(), memory.clone());
            ids.push(memory.id.clone());
        }

        // Single commit for all updates
        mem.commit()
            .map_err(|e| MemvidError::Memvid(format!("Failed to commit: {}", e)))?;

        info!("Batch updated {} memories successfully", ids.len());
        Ok(ids)
    }

    // Helper: Convert Memory to bytes
    fn memory_to_bytes(&self, memory: &Memory) -> Result<Vec<u8>> {
        // For now, just serialize the content
        match &memory.content {
            Content::Text(text) => Ok(text.as_bytes().to_vec()),
            Content::Structured(data) => serde_json::to_vec(data)
                .map_err(|e| MemvidError::Serialization(format!("{}", e))),
            Content::Vector(_vec) => Ok(b"vector".to_vec()), // Placeholder
            Content::Multimodal(_) => Ok(b"multimodal".to_vec()), // Placeholder
            Content::Binary(data) => Ok(data.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_real_memvid_basic() {
        let path = "test_real_basic.mv2";

        // Create store
        let store = RealMemvidStore::create(path).await.unwrap();

        // Add a memory
        let memory = Memory {
            id: MemoryId::from_string("test-1".to_string()),
            content: Content::text("Hello from real MemVid!"),
            attributes: AttributeSet::new(),
            relations: Default::default(),
            metadata: MetadataV4::default(),
        };

        store.add(&memory).await.unwrap();

        // Get it back
        let retrieved = store.get(&memory.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id.as_str(), "test-1");

        // Cleanup
        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn test_real_memvid_count() {
        let path = "test_real_count.mv2";

        let store = RealMemvidStore::create(path).await.unwrap();

        // Add 5 memories
        for i in 0..5 {
            let memory = Memory {
                id: MemoryId::from_string(format!("count-{}", i)),
                content: Content::text(&format!("Memory {}", i)),
                attributes: AttributeSet::new(),
                relations: Default::default(),
                metadata: MetadataV4::default(),
            };
            store.add(&memory).await.unwrap();
        }

        // Count
        let count = store.count().await.unwrap();
        assert_eq!(count, 5);

        // Cleanup
        let _ = std::fs::remove_file(path);
    }
}
