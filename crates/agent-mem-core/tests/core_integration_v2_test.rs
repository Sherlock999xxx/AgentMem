//! Core Integration v2 - 全面验证所有核心模块
//!
//! 验证核心模块的集成工作

use agent_mem_core::{
    cognitive_memory::CognitiveMemoryManager,
    graph_memory::{GraphMemoryEngine, NodeType},
    causal_reasoning::CausalReasoningEngine,
    types::{Memory, MemoryType},
};

#[tokio::test]
async fn test_cognitive_memory_manager_integration() {
    let manager = CognitiveMemoryManager::with_default_config().await;
    assert!(manager.is_ok(), "CognitiveMemoryManager should create successfully");
    
    let manager = manager.unwrap();
    
    // 添加不同类型的记忆
    for i in 0..10 {
        let memory = Memory::new(
            "test-agent".to_string(),
            Some("test-user".to_string()),
            match i % 4 {
                0 => MemoryType::Semantic,
                1 => MemoryType::Episodic,
                2 => MemoryType::Procedural,
                _ => MemoryType::Core,
            },
            format!("Test content {}", i),
            0.5 + (i as f32 * 0.05),
        );
        let _ = manager.add_memory(memory).await;
    }
    
    let stats = manager.get_stats().await.unwrap();
    assert_eq!(stats.total_memories, 10, "Should have 10 memories");
}

#[tokio::test]
async fn test_graph_memory_engine_integration() {
    let engine = GraphMemoryEngine::new();
    
    // 测试添加节点
    let memory = Memory::new(
        "test-agent".to_string(),
        None,
        MemoryType::Semantic,
        "Graph test content".to_string(),
        0.8,
    );
    
    let result = engine.add_node(memory, NodeType::Entity).await;
    assert!(result.is_ok(), "Graph node addition should succeed");
}

#[tokio::test]
async fn test_causal_reasoning_engine_integration() {
    let engine = CausalReasoningEngine::with_defaults();
    
    // 测试添加因果节点
    let node = agent_mem_core::causal_reasoning::CausalNode {
        id: "test-node-1".to_string(),
        content: "Test event content".to_string(),
        node_type: agent_mem_core::causal_reasoning::CausalNodeType::Event,
        timestamp: chrono::Utc::now(),
        properties: std::collections::HashMap::new(),
    };
    
    let result = engine.add_node(node).await;
    assert!(result.is_ok(), "Causal node addition should succeed");
}

#[tokio::test]
async fn test_all_engines_integration() {
    // 验证所有引擎可以同时存在
    let cognitive = CognitiveMemoryManager::with_default_config().await.unwrap();
    let graph = GraphMemoryEngine::new();
    let _causal = CausalReasoningEngine::with_defaults();
    
    let mem = Memory::new(
        "integration-test".to_string(),
        None,
        MemoryType::Episodic,
        "Integration test content".to_string(),
        0.9,
    );
    
    // 添加到认知记忆
    let id = cognitive.add_memory(mem.clone()).await.unwrap();
    assert!(!id.is_empty());
    
    // 添加到图记忆
    let _ = graph.add_node(mem, NodeType::Event).await;
    
    // 验证统计
    let stats = cognitive.get_stats().await.unwrap();
    assert_eq!(stats.total_memories, 1);
}

#[tokio::test]
async fn test_memory_type_filtering_integration() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();
    
    // 添加不同类型的记忆
    let types = vec![
        MemoryType::Semantic,
        MemoryType::Episodic,
        MemoryType::Procedural,
        MemoryType::Core,
    ];
    
    for (i, mem_type) in types.iter().enumerate() {
        let mem = Memory::new(
            "test-agent".to_string(),
            None,
            mem_type.clone(),
            format!("Type {} content", i),
            0.7,
        );
        let _ = manager.add_memory(mem).await;
    }
    
    // 检索并验证
    let results = manager.retrieve("Type", None, 10).await.unwrap();
    assert_eq!(results.len(), 4, "Should find all 4 memory types");
}

#[tokio::test]
async fn test_graph_node_types() {
    let engine = GraphMemoryEngine::new();
    
    // 测试所有节点类型
    let node_types = [
        NodeType::Entity,
        NodeType::Concept,
        NodeType::Event,
        NodeType::Relation,
        NodeType::Context,
    ];
    
    for node_type in node_types {
        let memory = Memory::new(
            "test-agent".to_string(),
            None,
            MemoryType::Semantic,
            format!("Node type test"),
            0.7,
        );
        
        let result = engine.add_node(memory, node_type.clone()).await;
        assert!(result.is_ok(), "Should support node type: {:?}", node_type);
    }
}

#[tokio::test]
async fn test_memory_importance_ranking() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();
    
    // 添加不同重要性的记忆
    for i in 0..5 {
        let mem = Memory::new(
            "test-agent".to_string(),
            None,
            MemoryType::Semantic,
            format!("Importance {}", i),
            0.5 + (i as f32 * 0.1), // 0.5, 0.6, 0.7, 0.8, 0.9
        );
        let _ = manager.add_memory(mem).await;
    }
    
    let stats = manager.get_stats().await.unwrap();
    assert_eq!(stats.total_memories, 5);
    
    // 验证重要性排序
    let results = manager.retrieve("Importance", None, 5).await.unwrap();
    assert_eq!(results.len(), 5);
    
    // 验证排序（重要性高的在前）
    for i in 0..results.len() - 1 {
        let curr = results.get(i).unwrap();
        let next = results.get(i + 1).unwrap();
        assert!(
            curr.importance() >= next.importance(),
            "Results should be sorted by importance descending"
        );
    }
}

#[tokio::test]
async fn test_memory_stats_by_type() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();
    
    // 添加不同类型的记忆
    for i in 0..12 {
        let mem = Memory::new(
            "test-agent".to_string(),
            None,
            match i % 4 {
                0 => MemoryType::Semantic,
                1 => MemoryType::Episodic,
                2 => MemoryType::Procedural,
                _ => MemoryType::Core,
            },
            format!("Stats test {}", i),
            0.8,
        );
        let _ = manager.add_memory(mem).await;
    }
    
    let stats = manager.get_stats().await.unwrap();
    assert_eq!(stats.total_memories, 12);
    assert!(!stats.by_type.is_empty(), "Should have stats by type");
}
