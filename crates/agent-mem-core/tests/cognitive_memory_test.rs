//! CognitiveMemoryManager 单元测试

use agent_mem_core::cognitive_memory::CognitiveMemoryManager;
use agent_mem_core::types::Memory;

#[tokio::test]
async fn test_cognitive_manager_creation() {
    let manager = CognitiveMemoryManager::with_default_config().await;
    assert!(manager.is_ok(), "Should create CognitiveMemoryManager");
}

#[tokio::test]
async fn test_add_and_retrieve_memory() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    let memory = Memory::new(
        "test-agent".to_string(),
        None,
        agent_mem_core::types::MemoryType::Semantic,
        "Test content".to_string(),
        0.5,
    );

    let id = manager.add_memory(memory).await.unwrap();
    assert!(!id.is_empty());

    let retrieved = manager.get_memory(&id).await.unwrap();
    assert!(retrieved.is_some());
}

#[tokio::test]
async fn test_delete_memory() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    let memory = Memory::new(
        "test-agent".to_string(),
        None,
        agent_mem_core::types::MemoryType::Episodic,
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
            agent_mem_core::types::MemoryType::Semantic,
            format!("Stats {}", i),
            0.5,
        );
        let _ = manager.add_memory(memory).await;
    }

    let stats = manager.get_stats().await.unwrap();
    assert_eq!(stats.total_memories, 3);
}
