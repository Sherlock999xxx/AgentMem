//! End-to-End Memory Workflow Test
//!
//! Tests the complete memory lifecycle:
//! 1. Create memories of different types
//! 2. Search and retrieve with various queries
//! 3. Filter by memory type
//! 4. Delete and verify

use agent_mem_core::{
    cognitive_memory::CognitiveMemoryManager,
    types::{Memory, MemoryType},
};

#[tokio::test]
async fn test_complete_memory_lifecycle() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    // 1. Create memories of different types
    let memories = vec![
        ("User likes Rust programming", MemoryType::Semantic, 0.9),
        (
            "User completed onboarding yesterday",
            MemoryType::Episodic,
            0.8,
        ),
        ("How to deploy: run deploy.sh", MemoryType::Procedural, 0.85),
        ("User critical: prefers dark mode", MemoryType::Core, 1.0),
        ("Working on feature X", MemoryType::Working, 0.95),
    ];

    let mut ids = vec![];
    for (content, mem_type, importance) in memories {
        let memory = Memory::new(
            "test-agent".to_string(),
            Some("test-user".to_string()),
            mem_type,
            content.to_string(),
            importance,
        );
        let id = manager.add_memory(memory).await.unwrap();
        ids.push(id);
    }

    // 2. Verify all memories added
    let stats = manager.get_stats().await.unwrap();
    assert_eq!(stats.total_memories, 5, "Should have 5 memories");

    // 3. Search with different queries
    let rust_results = manager.retrieve("rust", None, 10).await.unwrap();
    assert!(rust_results.len() >= 1, "Should find Rust related memory");

    let deploy_results = manager.retrieve("deploy", None, 10).await.unwrap();
    assert!(
        deploy_results.len() >= 1,
        "Should find deploy related memory"
    );

    // 4. Filter by memory type
    let semantic_only = manager
        .retrieve("", Some(vec![MemoryType::Semantic]), 10)
        .await
        .unwrap();
    assert!(semantic_only
        .iter()
        .all(|m| m.memory_type() == MemoryType::Semantic));

    // 5. Delete one memory
    let deleted = manager.delete_memory(&ids[0]).await.unwrap();
    assert!(deleted, "Should delete memory");

    // 6. Verify deletion
    let remaining = manager.get_stats().await.unwrap();
    assert_eq!(
        remaining.total_memories, 4,
        "Should have 4 memories remaining"
    );
}

#[tokio::test]
async fn test_multi_type_search_effectiveness() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    // Add memories with overlapping concepts
    let memories = vec![
        ("Python is great for AI", MemoryType::Semantic, 0.9),
        ("Python for web development", MemoryType::Semantic, 0.8),
        ("Rust for systems programming", MemoryType::Semantic, 0.85),
        ("JavaScript for frontend", MemoryType::Semantic, 0.75),
        ("Yesterday I learned Python", MemoryType::Episodic, 0.8),
    ];

    for (content, mem_type, importance) in memories {
        let memory = Memory::new(
            "test-agent".to_string(),
            None,
            mem_type,
            content.to_string(),
            importance,
        );
        let _ = manager.add_memory(memory).await;
    }

    // Search for "python" - should find multiple results
    let results = manager.retrieve("python", None, 10).await.unwrap();
    assert!(
        results.len() >= 2,
        "Should find at least 2 Python related memories"
    );

    // Verify results are ranked by relevance
    for result in &results {
        let content_str = format!("{:?}", result.content);
        assert!(
            content_str.to_lowercase().contains("python"),
            "All results should contain 'python'"
        );
    }
}

#[tokio::test]
async fn test_memory_type_filtering_accuracy() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    // Add memories of each type
    let type_memories = vec![
        ("Core fact 1", MemoryType::Core, 0.9),
        ("Core fact 2", MemoryType::Core, 0.85),
        ("Semantic fact 1", MemoryType::Semantic, 0.8),
        ("Semantic fact 2", MemoryType::Semantic, 0.75),
        ("Episodic event 1", MemoryType::Episodic, 0.7),
        ("Procedural step 1", MemoryType::Procedural, 0.8),
        ("Working task 1", MemoryType::Working, 0.95),
    ];

    for (content, mem_type, importance) in type_memories {
        let memory = Memory::new(
            "test-agent".to_string(),
            None,
            mem_type,
            content.to_string(),
            importance,
        );
        let _ = manager.add_memory(memory).await;
    }

    // Test filtering for each type
    let core_results = manager
        .retrieve("", Some(vec![MemoryType::Core]), 10)
        .await
        .unwrap();
    assert_eq!(core_results.len(), 2, "Should find 2 Core memories");

    let semantic_results = manager
        .retrieve("", Some(vec![MemoryType::Semantic]), 10)
        .await
        .unwrap();
    assert_eq!(semantic_results.len(), 2, "Should find 2 Semantic memories");

    let episodic_results = manager
        .retrieve("", Some(vec![MemoryType::Episodic]), 10)
        .await
        .unwrap();
    assert_eq!(episodic_results.len(), 1, "Should find 1 Episodic memory");
}

#[tokio::test]
async fn test_importance_based_ranking() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    // Add memories with different importance levels
    let memories = vec![
        ("Low priority memory", MemoryType::Semantic, 0.2),
        ("Medium priority memory", MemoryType::Semantic, 0.5),
        ("High priority memory", MemoryType::Semantic, 0.8),
        ("Critical priority memory", MemoryType::Semantic, 1.0),
    ];

    for (content, mem_type, importance) in memories {
        let memory = Memory::new(
            "test-agent".to_string(),
            None,
            mem_type,
            content.to_string(),
            importance,
        );
        let _ = manager.add_memory(memory).await;
    }

    // Retrieve without text query - should return by importance
    let results = manager.retrieve("", None, 10).await.unwrap();
    assert_eq!(results.len(), 4, "Should return all 4 memories");

    // Verify ordering (highest importance first)
    for i in 0..results.len() - 1 {
        assert!(
            results[i].importance() >= results[i + 1].importance(),
            "Results should be ordered by importance (descending)"
        );
    }
}

#[tokio::test]
async fn test_batch_operations_consistency() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    // Batch add memories
    let batch: Vec<Memory> = (0..20)
        .map(|i| {
            Memory::new(
                "batch-agent".to_string(),
                None,
                if i % 2 == 0 {
                    MemoryType::Semantic
                } else {
                    MemoryType::Episodic
                },
                format!("Batch memory {}", i),
                0.5 + (i as f32 * 0.02),
            )
        })
        .collect();

    let ids = manager.add_memories(batch).await.unwrap();
    assert_eq!(ids.len(), 20, "Should return 20 IDs");

    // Verify all added
    let stats = manager.get_stats().await.unwrap();
    assert_eq!(stats.total_memories, 20, "Should have 20 memories");

    // Verify type distribution
    let semantic_count = *stats.by_type.get("semantic").unwrap_or(&0);
    let episodic_count = *stats.by_type.get("episodic").unwrap_or(&0);

    assert_eq!(semantic_count, 10, "Should have 10 semantic memories");
    assert_eq!(episodic_count, 10, "Should have 10 episodic memories");
}
