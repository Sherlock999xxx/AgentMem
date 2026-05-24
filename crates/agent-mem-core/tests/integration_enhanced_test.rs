//! Integration Tests for Enhanced Memory System
//!
//! Tests for:
//! 1. CategoryRecallEngine integration
//! 2. ResourceRecallEngine integration
//! 3. CognitiveMemoryManager end-to-end

use agent_mem_core::{
    cognitive_memory::CognitiveMemoryManager,
    search::{
        CategoryRecallConfig, CategoryRecallEngine, CategoryScope, CategorySearchResult,
        InMemoryCategoryRecall, InMemoryResourceRecall, ResourceContext, ResourceRecallConfig,
        ResourceRecallEngine, ResourceType,
    },
    types::{Memory, MemoryType},
};

#[tokio::test]
async fn test_category_recall_engine_basic() {
    let config = CategoryRecallConfig::default();
    let engine = InMemoryCategoryRecall::new(config);

    // 添加一些类别
    let categories = vec![
        CategorySearchResult {
            id: "rust-1".to_string(),
            path: "/tech/rust".to_string(),
            name: "rust".to_string(),
            score: 1.0,
            parent_id: None,
            item_count: 10,
            summary: Some("Rust programming".to_string()),
        },
        CategorySearchResult {
            id: "python-1".to_string(),
            path: "/tech/python".to_string(),
            name: "python".to_string(),
            score: 1.0,
            parent_id: None,
            item_count: 15,
            summary: Some("Python programming".to_string()),
        },
    ];

    for category in categories {
        engine.add_category(category).await;
    }

    // 搜索类别
    let scope = CategoryScope::new("global".to_string());
    let results = engine.search_categories("rust", &scope, 10).await;
    assert!(results.is_ok(), "Should search categories");
    let result = results.unwrap();
    assert_eq!(result.categories.len(), 1, "Should find 'rust' category");
}

#[tokio::test]
async fn test_category_recall_with_related() {
    let config = CategoryRecallConfig::default();
    let engine = InMemoryCategoryRecall::new(config);

    // 添加不同类型的类别
    let categories = vec![
        CategorySearchResult {
            id: "tech-1".to_string(),
            path: "/tech".to_string(),
            name: "tech".to_string(),
            score: 1.0,
            parent_id: None,
            item_count: 20,
            summary: Some("Technology category".to_string()),
        },
        CategorySearchResult {
            id: "rust-2".to_string(),
            path: "/tech/rust".to_string(),
            name: "rust".to_string(),
            score: 1.0,
            parent_id: Some("tech-1".to_string()),
            item_count: 10,
            summary: Some("Rust programming".to_string()),
        },
    ];

    for category in categories {
        engine.add_category(category).await;
    }

    // 获取相关类别
    let scope = CategoryScope::new("global".to_string());
    let results = engine.get_related("tech-1", &scope, 10).await;
    assert!(results.is_ok(), "Should get related categories");
}

#[tokio::test]
async fn test_resource_recall_engine_basic() {
    let config = ResourceRecallConfig::default();
    let engine = InMemoryResourceRecall::new(config);

    // 添加资源
    let resources = vec![
        ResourceContext {
            id: "res-1".to_string(),
            uri: "https://rust-lang.org".to_string(),
            resource_type: ResourceType::Http,
            media_type: "text/html".to_string(),
            summary: Some("Rust official site".to_string()),
            created_at: None,
            accessed_at: None,
            metadata: None,
        },
        ResourceContext {
            id: "res-2".to_string(),
            uri: "https://python.org".to_string(),
            resource_type: ResourceType::Http,
            media_type: "text/html".to_string(),
            summary: Some("Python official site".to_string()),
            created_at: None,
            accessed_at: None,
            metadata: None,
        },
    ];

    for resource in resources {
        engine.add_resource(resource).await;
    }

    // 搜索资源
    let results = engine.search_resources("rust", 10).await;
    assert!(results.is_ok(), "Should search resources");
    let result = results.unwrap();
    assert!(
        result.resources.len() >= 1,
        "Should find at least 1 resource"
    );
}

#[tokio::test]
async fn test_cognitive_memory_with_category_recall() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    // 添加记忆
    let memories = vec![
        ("Rust is a systems language", MemoryType::Semantic, 0.9),
        ("Python is great for data", MemoryType::Semantic, 0.8),
        ("Web development with React", MemoryType::Semantic, 0.85),
    ];

    for (content, mem_type, importance) in memories {
        let memory = Memory::new(
            "test-agent".to_string(),
            None,
            mem_type,
            content.to_string(),
            importance,
        );
        let _ = manager.add_memory(memory).await.unwrap();
    }

    // 获取统计
    let stats = manager.get_stats().await.unwrap();
    assert_eq!(stats.total_memories, 3, "Should have 3 memories");
}

#[tokio::test]
async fn test_memory_importance_ranking() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    // 添加不同重要性的记忆
    let memories = vec![
        ("Critical system info", MemoryType::Core, 1.0),
        ("Important fact", MemoryType::Semantic, 0.7),
        ("Minor detail", MemoryType::Episodic, 0.3),
        ("Another critical", MemoryType::Core, 0.95),
    ];

    for (content, mem_type, importance) in memories {
        let memory = Memory::new(
            "test-agent".to_string(),
            None,
            mem_type,
            content.to_string(),
            importance,
        );
        let _ = manager.add_memory(memory).await.unwrap();
    }

    // 检索并验证排序
    let results = manager.retrieve("", None, 10).await.unwrap();

    // 结果应该按重要性排序
    if results.len() >= 2 {
        assert!(
            results[0].importance() >= results[1].importance(),
            "Results should be sorted by importance"
        );
    }
}

#[tokio::test]
async fn test_memory_type_filtering() {
    // 创建新的管理器实例
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    // 添加多种类型的记忆 - 使用高重要性确保被检索
    let memories = vec![
        ("Core type content", MemoryType::Core, 1.0),
        ("Semantic type content", MemoryType::Semantic, 0.9),
        ("Episodic type content", MemoryType::Episodic, 0.8),
    ];

    for (content, mem_type, importance) in memories {
        let memory = Memory::new(
            "test-agent".to_string(),
            None,
            mem_type,
            content.to_string(),
            importance,
        );
        let _ = manager.add_memory(memory).await.unwrap();
    }

    // 验证添加成功
    let all_stats = manager.get_stats().await.unwrap();
    assert_eq!(all_stats.total_memories, 3, "Should have 3 memories");

    // 获取所有类型验证类型
    let all_results = manager.retrieve("", None, 10).await.unwrap();
    println!("All results count: {}", all_results.len());

    // 按类型过滤测试
    let core_results = manager
        .retrieve("", Some(vec![MemoryType::Core]), 10)
        .await
        .unwrap();
    println!("Core filter results: {}", core_results.len());

    // 验证至少有1个Core记忆
    assert!(
        core_results.len() >= 1,
        "Should find at least 1 Core memory, got {}",
        core_results.len()
    );
}

#[tokio::test]
async fn test_batch_operations() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    // 批量添加
    let memories: Vec<Memory> = (0..10)
        .map(|i| {
            Memory::new(
                "test-agent".to_string(),
                None,
                MemoryType::Semantic,
                format!("Batch memory {}", i),
                0.5,
            )
        })
        .collect();

    let results = manager.add_memories(memories).await.unwrap();
    assert_eq!(results.len(), 10, "Should add 10 memories");

    // 验证总数
    let stats = manager.get_stats().await.unwrap();
    assert_eq!(stats.total_memories, 10, "Should have 10 memories");
}

#[tokio::test]
async fn test_delete_and_verify() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    // 添加记忆
    let memory = Memory::new(
        "test-agent".to_string(),
        None,
        MemoryType::Semantic,
        "To be deleted".to_string(),
        0.5,
    );
    let id = manager.add_memory(memory).await.unwrap();

    // 验证存在
    let retrieved = manager.get_memory(&id).await.unwrap();
    assert!(retrieved.is_some(), "Memory should exist");

    // 删除
    let deleted = manager.delete_memory(&id).await.unwrap();
    assert!(deleted, "Should delete successfully");

    // 验证不存在
    let retrieved = manager.get_memory(&id).await.unwrap();
    assert!(
        retrieved.is_none(),
        "Memory should not exist after deletion"
    );
}

#[tokio::test]
async fn test_stats_by_type() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();

    // 添加多种类型的记忆
    for i in 0..3 {
        manager
            .add_memory(Memory::new(
                "test-agent".to_string(),
                None,
                MemoryType::Semantic,
                format!("StatsSemantic{}", i),
                0.5,
            ))
            .await
            .unwrap();
    }
    for i in 0..2 {
        manager
            .add_memory(Memory::new(
                "test-agent".to_string(),
                None,
                MemoryType::Episodic,
                format!("StatsEpisodic{}", i),
                0.5,
            ))
            .await
            .unwrap();
    }

    let stats = manager.get_stats().await.unwrap();
    println!("Total memories: {}", stats.total_memories);
    println!("Stats by type: {:?}", stats.by_type);

    // 验证总数
    assert_eq!(stats.total_memories, 5, "Should have exactly 5 memories");

    // 验证类型
    let semantic_count = *stats.by_type.get("semantic").unwrap_or(&0);
    let episodic_count = *stats.by_type.get("episodic").unwrap_or(&0);

    assert_eq!(semantic_count, 3, "Should have 3 semantic memories");
    assert_eq!(episodic_count, 2, "Should have 2 episodic memories");
}

#[tokio::test]
async fn test_resource_recall_by_id() {
    let config = ResourceRecallConfig::default();
    let engine = InMemoryResourceRecall::new(config);

    // 添加一个资源
    let context = ResourceContext {
        id: "test-resource-1".to_string(),
        uri: "https://example.com".to_string(),
        resource_type: ResourceType::Http,
        media_type: "text/html".to_string(),
        summary: Some("Example resource".to_string()),
        created_at: None,
        accessed_at: None,
        metadata: None,
    };

    engine.add_resource(context).await;

    // 获取资源
    let result = engine.get_resource("test-resource-1").await;
    assert!(result.is_ok(), "Should get resource");
    let resource = result.unwrap();
    assert!(resource.is_some(), "Resource should exist");
}
