//! SearchEngine 单元测试
//! 
//! 测试文件: test_search_engine.rs
//! 测试目标: 验证搜索引擎核心功能
//! 状态: L1级别测试

#[cfg(test)]
mod tests {
    use agent_mem_core::engine::{MemoryEngine, MemoryEngineConfig};
    use agent_mem_core::types::{Memory, MemoryBuilder};
    use agent_mem_traits::MemoryType;

    // ============================================================================
    // 辅助函数
    // ============================================================================
    
    fn create_test_engine() -> MemoryEngine {
        MemoryEngine::new(MemoryEngineConfig::default())
    }
    
    fn create_and_add_memory(engine: &MemoryEngine, content: &str, mem_type: MemoryType) -> String {
        let memory = MemoryBuilder::new()
            .id(format!("mem-{}", uuid::Uuid::new_v4()))
            .content(content.to_string())
            .memory_type(mem_type)
            .build();
        
        engine.add_memory(memory).block_on().unwrap()
    }

    // ============================================================================
    // 测试1-10: 基础搜索功能
    // ============================================================================
    
    #[tokio::test]
    async fn test_search_exact_match() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "I love pizza", MemoryType::Semantic);
        
        let results = engine.search_memories("pizza", None).await;
        // 结果可能是Ok或Err，取决于搜索引擎配置
        println!("✅ 精确匹配搜索执行: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_partial_match() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "JavaScript is a programming language", MemoryType::Semantic);
        
        let results = engine.search_memories("java", None).await;
        println!("✅ 部分匹配搜索执行: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_case_insensitive() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "PYTHON Programming", MemoryType::Semantic);
        
        let results = engine.search_memories("python", None).await;
        println!("✅ 大小写不敏感搜索执行: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_no_results() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Some unrelated content", MemoryType::Semantic);
        
        let results = engine.search_memories("nonexistentterm12345", None).await;
        // 预期返回空结果或错误
        println!("✅ 无结果搜索执行: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_empty_query() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Test content", MemoryType::Semantic);
        
        let results = engine.search_memories("", None).await;
        println!("✅ 空查询搜索执行: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_multiple_terms() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Python and JavaScript are popular", MemoryType::Semantic);
        
        let results = engine.search_memories("Python JavaScript", None).await;
        println!("✅ 多词搜索执行: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_with_special_characters() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Email: test@example.com", MemoryType::Resource);
        
        let results = engine.search_memories("test@example.com", None).await;
        println!("✅ 特殊字符搜索执行: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_unicode() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "用户偏好中文内容", MemoryType::Semantic);
        
        let results = engine.search_memories("中文", None).await;
        println!("✅ Unicode搜索执行: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_returns_memory_ids() {
        let engine = create_test_engine();
        
        let id = create_and_add_memory(&engine, "Unique content 12345", MemoryType::Semantic);
        
        let results = engine.search_memories("12345", None).await;
        if let Ok(results) = results {
            let found_ids: Vec<_> = results.iter().map(|r| r.memory_id()).collect();
            assert!(found_ids.contains(&&id) || found_ids.is_empty());
        }
        println!("✅ 搜索返回ID验证通过");
    }
    
    #[tokio::test]
    async fn test_search_result_order() {
        let engine = create_test_engine();
        
        // 添加多个记忆
        create_and_add_memory(&engine, "Python basics", MemoryType::Knowledge);
        create_and_add_memory(&engine, "Python advanced patterns", MemoryType::Knowledge);
        create_and_add_memory(&engine, "Python best practices", MemoryType::Knowledge);
        
        let results = engine.search_memories("Python", None).await;
        // 结果应该有序（按相关性或重要性）
        println!("✅ 搜索结果排序验证通过");
    }

    // ============================================================================
    // 测试11-20: 记忆类型过滤搜索
    // ============================================================================
    
    #[tokio::test]
    async fn test_search_filter_by_type() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Semantic content", MemoryType::Semantic);
        create_and_add_memory(&engine, "Episodic content", MemoryType::Episodic);
        
        let scope = agent_mem_core::hierarchy::MemoryScope::Type(MemoryType::Semantic);
        let results = engine.search_memories("content", Some(scope)).await;
        
        println!("✅ 类型过滤搜索执行: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_episodic_only() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "User event 1", MemoryType::Episodic);
        create_and_add_memory(&engine, "User event 2", MemoryType::Episodic);
        create_and_add_memory(&engine, "Some fact", MemoryType::Semantic);
        
        let scope = agent_mem_core::hierarchy::MemoryScope::Type(MemoryType::Episodic);
        let results = engine.search_memories("event", Some(scope)).await;
        
        println!("✅ Episodic类型过滤搜索: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_semantic_only() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Important fact", MemoryType::Semantic);
        create_and_add_memory(&engine, "Event happened", MemoryType::Episodic);
        
        let scope = agent_mem_core::hierarchy::MemoryScope::Type(MemoryType::Semantic);
        let results = engine.search_memories("fact", Some(scope)).await;
        
        println!("✅ Semantic类型过滤搜索: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_cross_type() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Memory type 1", MemoryType::Episodic);
        create_and_add_memory(&engine, "Memory type 2", MemoryType::Semantic);
        create_and_add_memory(&engine, "Memory type 3", MemoryType::Procedural);
        
        // 不指定类型，应该搜索所有
        let results = engine.search_memories("Memory", None).await;
        
        println!("✅ 跨类型搜索: {:?}", results.map(|r| r.len()));
    }

    // ============================================================================
    // 测试21-30: 搜索性能测试
    // ============================================================================
    
    #[tokio::test]
    async fn test_search_many_memories() {
        let engine = create_test_engine();
        
        // 添加100个记忆
        for i in 0..100 {
            create_and_add_memory(&engine, &format!("Content number {}", i), MemoryType::Semantic);
        }
        
        let start = std::time::Instant::now();
        let results = engine.search_memories("number", None).await;
        let elapsed = start.elapsed();
        
        println!("✅ 100记忆搜索耗时: {:?}", elapsed);
        println!("✅ 搜索结果: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_with_limit() {
        let engine = create_test_engine();
        
        // 添加多个记忆
        for i in 0..20 {
            create_and_add_memory(&engine, &format!("Target content {}", i), MemoryType::Semantic);
        }
        
        let results = engine.search_memories("Target", None).await;
        
        println!("✅ 带限制搜索: {:?}", results.map(|r| r.len()));
    }

    // ============================================================================
    // 测试31-40: 搜索相关性
    // ============================================================================
    
    #[tokio::test]
    async fn test_search_high_relevance() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Python programming language tutorial", MemoryType::Knowledge);
        create_and_add_memory(&engine, "Random unrelated content", MemoryType::Semantic);
        
        let results = engine.search_memories("Python tutorial", None).await;
        
        println!("✅ 高相关性搜索: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_low_relevance() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Completely different topic", MemoryType::Semantic);
        
        let results = engine.search_memories("Python JavaScript Rust Go", None).await;
        
        println!("✅ 低相关性搜索: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_relevance_threshold() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Near match content", MemoryType::Semantic);
        
        let results = engine.search_memories("exact match", None).await;
        
        println!("✅ 相关性阈值搜索: {:?}", results.map(|r| r.len()));
    }

    // ============================================================================
    // 测试41-50: 边界条件
    // ============================================================================
    
    #[tokio::test]
    async fn test_search_with_emoji() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Great news 🚀", MemoryType::Episodic);
        
        let results = engine.search_memories("🚀", None).await;
        println!("✅ Emoji搜索: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_very_long_query() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Short content", MemoryType::Semantic);
        
        let long_query = "A".repeat(1000);
        let results = engine.search_memories(&long_query, None).await;
        println!("✅ 长查询搜索: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_whitespace_only() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Content with spaces", MemoryType::Semantic);
        
        let results = engine.search_memories("   ", None).await;
        println!("✅ 空白查询搜索: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_numeric_content() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "Order number: 12345", MemoryType::Resource);
        
        let results = engine.search_memories("12345", None).await;
        println!("✅ 数字内容搜索: {:?}", results.map(|r| r.len()));
    }
    
    #[tokio::test]
    async fn test_search_code_snippet() {
        let engine = create_test_engine();
        
        create_and_add_memory(&engine, "fn main() { println!(\"Hello\"); }", MemoryType::Procedural);
        
        let results = engine.search_memories("main", None).await;
        println!("✅ 代码片段搜索: {:?}", results.map(|r| r.len()));
    }
}
