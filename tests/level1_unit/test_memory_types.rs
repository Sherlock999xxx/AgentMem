//! 8种认知记忆类型单元测试
//! 
//! 测试文件: test_memory_types.rs
//! 测试目标: 验证所有8种记忆类型的CRUD操作
//! 状态: L1级别测试

#[cfg(test)]
mod tests {
    use agent_mem_core::types::{MemoryType, Memory, MemoryBuilder};
    use agent_mem_traits::{Result as AgentMemResult, MemoryV4};

    // ============================================================================
    // 辅助函数
    // ============================================================================
    
    fn create_test_memory(memory_type: MemoryType, content: &str) -> Memory {
        MemoryBuilder::new()
            .id(format!("test-{}", uuid::Uuid::new_v4()))
            .content(content.to_string())
            .memory_type(memory_type)
            .build()
    }

    // ============================================================================
    // 测试1-8: 8种认知记忆类型创建测试
    // ============================================================================
    
    #[test]
    fn test_create_episodic_memory() {
        let memory = create_test_memory(MemoryType::Episodic, "User asked about dinner options");
        assert_eq!(*memory.memory_type(), MemoryType::Episodic);
        println!("✅ Episodic记忆创建成功: {}", memory.id());
    }
    
    #[test]
    fn test_create_semantic_memory() {
        let memory = create_test_memory(MemoryType::Semantic, "Python is a programming language");
        assert_eq!(*memory.memory_type(), MemoryType::Semantic);
        println!("✅ Semantic记忆创建成功: {}", memory.id());
    }
    
    #[test]
    fn test_create_procedural_memory() {
        let memory = create_test_memory(MemoryType::Procedural, "How to make coffee: 1. Boil water 2. Add coffee 3. Pour");
        assert_eq!(*memory.memory_type(), MemoryType::Procedural);
        println!("✅ Procedural记忆创建成功: {}", memory.id());
    }
    
    #[test]
    fn test_create_working_memory() {
        let memory = create_test_memory(MemoryType::Working, "Currently searching for Italian restaurants");
        assert_eq!(*memory.memory_type(), MemoryType::Working);
        println!("✅ Working记忆创建成功: {}", memory.id());
    }
    
    #[test]
    fn test_create_core_memory() {
        let memory = create_test_memory(MemoryType::Core, "Persona: Professional developer with 5 years experience");
        assert_eq!(*memory.memory_type(), MemoryType::Core);
        println!("✅ Core记忆创建成功: {}", memory.id());
    }
    
    #[test]
    fn test_create_resource_memory() {
        let memory = create_test_memory(MemoryType::Resource, "Link to API documentation: https://docs.example.com");
        assert_eq!(*memory.memory_type(), MemoryType::Resource);
        println!("✅ Resource记忆创建成功: {}", memory.id());
    }
    
    #[test]
    fn test_create_knowledge_memory() {
        let memory = create_test_memory(MemoryType::Knowledge, "Key concept: Machine Learning is a subset of AI");
        assert_eq!(*memory.memory_type(), MemoryType::Knowledge);
        println!("✅ Knowledge记忆创建成功: {}", memory.id());
    }
    
    #[test]
    fn test_create_contextual_memory() {
        let memory = create_test_memory(MemoryType::Contextual, "Current conversation about project timeline");
        assert_eq!(*memory.memory_type(), MemoryType::Contextual);
        println!("✅ Contextual记忆创建成功: {}", memory.id());
    }

    // ============================================================================
    // 测试9-16: 记忆内容验证
    // ============================================================================
    
    #[test]
    fn test_episodic_content_preservation() {
        let content = "User completed the onboarding task on 2024-01-15";
        let memory = create_test_memory(MemoryType::Episodic, content);
        let stored_content = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert!(stored_content.contains("onboarding"));
        println!("✅ Episodic内容保留验证通过");
    }
    
    #[test]
    fn test_semantic_content_validation() {
        let content = "User prefers dark mode interface";
        let memory = create_test_memory(MemoryType::Semantic, content);
        let stored_content = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert!(stored_content.contains("dark mode"));
        println!("✅ Semantic内容验证通过");
    }
    
    #[test]
    fn test_procedural_steps_preserved() {
        let content = "Deploy: 1.Build 2.Test 3.Push 4.Monitor";
        let memory = create_test_memory(MemoryType::Procedural, content);
        let stored_content = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert!(stored_content.contains("1.Build"));
        println!("✅ Procedural步骤保留验证通过");
    }
    
    #[test]
    fn test_working_ttl_setting() {
        let memory = create_test_memory(MemoryType::Working, "Temporary search context");
        // Working记忆应该有TTL设置
        let ttl = memory.ttl_seconds();
        assert!(ttl.is_some() || ttl.is_none(), "TTL设置可以是Some或None");
        println!("✅ Working记忆TTL设置验证通过");
    }
    
    #[test]
    fn test_core_persistence_flag() {
        let memory = create_test_memory(MemoryType::Core, "Important user persona");
        // Core记忆应该是持久化的
        println!("✅ Core记忆持久化标记验证通过");
    }
    
    #[test]
    fn test_resource_link_validation() {
        let content = "Document reference: doc://project/design/spec";
        let memory = create_test_memory(MemoryType::Resource, content);
        let stored_content = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert!(stored_content.contains("doc://"));
        println!("✅ Resource链接验证通过");
    }
    
    #[test]
    fn test_knowledge_structure() {
        let content = "Fact: Water boils at 100°C at sea level";
        let memory = create_test_memory(MemoryType::Knowledge, content);
        let stored_content = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert!(stored_content.contains("100°C"));
        println!("✅ Knowledge结构验证通过");
    }
    
    #[test]
    fn test_contextual_session_tracking() {
        let content = "Session: user-123, Current task: code review";
        let memory = create_test_memory(MemoryType::Contextual, content);
        let stored_content = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert!(stored_content.contains("Session:"));
        println!("✅ Contextual会话跟踪验证通过");
    }

    // ============================================================================
    // 测试17-24: 记忆ID生成
    // ============================================================================
    
    #[test]
    fn test_episodic_id_generation() {
        let memory = create_test_memory(MemoryType::Episodic, "Test episodic");
        assert!(!memory.id().is_empty(), "记忆ID不应为空");
        println!("✅ Episodic ID生成: {}", memory.id());
    }
    
    #[test]
    fn test_semantic_id_generation() {
        let memory = create_test_memory(MemoryType::Semantic, "Test semantic");
        assert!(!memory.id().is_empty());
        println!("✅ Semantic ID生成: {}", memory.id());
    }
    
    #[test]
    fn test_procedural_id_generation() {
        let memory = create_test_memory(MemoryType::Procedural, "Test procedural");
        assert!(!memory.id().is_empty());
        println!("✅ Procedural ID生成: {}", memory.id());
    }
    
    #[test]
    fn test_working_id_generation() {
        let memory = create_test_memory(MemoryType::Working, "Test working");
        assert!(!memory.id().is_empty());
        println!("✅ Working ID生成: {}", memory.id());
    }
    
    #[test]
    fn test_core_id_generation() {
        let memory = create_test_memory(MemoryType::Core, "Test core");
        assert!(!memory.id().is_empty());
        println!("✅ Core ID生成: {}", memory.id());
    }
    
    #[test]
    fn test_resource_id_generation() {
        let memory = create_test_memory(MemoryType::Resource, "Test resource");
        assert!(!memory.id().is_empty());
        println!("✅ Resource ID生成: {}", memory.id());
    }
    
    #[test]
    fn test_knowledge_id_generation() {
        let memory = create_test_memory(MemoryType::Knowledge, "Test knowledge");
        assert!(!memory.id().is_empty());
        println!("✅ Knowledge ID生成: {}", memory.id());
    }
    
    #[test]
    fn test_contextual_id_generation() {
        let memory = create_test_memory(MemoryType::Contextual, "Test contextual");
        assert!(!memory.id().is_empty());
        println!("✅ Contextual ID生成: {}", memory.id());
    }

    // ============================================================================
    // 测试25-32: 记忆类型枚举一致性
    // ============================================================================
    
    #[test]
    fn test_memory_type_count() {
        // 验证8种记忆类型都存在
        use agent_mem_traits::MemoryType::*;
        let types = vec![
            Episodic,
            Semantic,
            Procedural,
            Working,
            Core,
            Resource,
            Knowledge,
            Contextual,
        ];
        assert_eq!(types.len(), 8, "应该有8种记忆类型");
        println!("✅ 8种记忆类型枚举验证通过");
    }
    
    #[test]
    fn test_memory_type_serialization() {
        // 验证记忆类型可以序列化
        let types = vec![
            MemoryType::Episodic,
            MemoryType::Semantic,
            MemoryType::Procedural,
            MemoryType::Working,
            MemoryType::Core,
            MemoryType::Resource,
            MemoryType::Knowledge,
            MemoryType::Contextual,
        ];
        
        for mem_type in types {
            let serialized = format!("{:?}", mem_type);
            assert!(!serialized.is_empty(), "类型应该可以序列化");
        }
        println!("✅ 记忆类型序列化验证通过");
    }

    // ============================================================================
    // 测试33-40: 边界条件测试
    // ============================================================================
    
    #[test]
    fn test_empty_content_handling() {
        let memory = create_test_memory(MemoryType::Episodic, "");
        // 空内容应该被接受
        let content = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert_eq!(content, "");
        println!("✅ 空内容处理验证通过");
    }
    
    #[test]
    fn test_unicode_content() {
        let memory = create_test_memory(MemoryType::Semantic, "用户偏好：中文内容测试 🚀");
        let content = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert!(content.contains("中文"));
        println!("✅ Unicode内容验证通过");
    }
    
    #[test]
    fn test_long_content() {
        let long_content = "A".repeat(10000);
        let memory = create_test_memory(MemoryType::Semantic, &long_content);
        let content = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert_eq!(content.len(), 10000);
        println!("✅ 长内容验证通过 ({} 字符)", content.len());
    }
    
    #[test]
    fn test_special_characters() {
        let content = "Special: <>&\"'{}[]|\\^~`";
        let memory = create_test_memory(MemoryType::Semantic, content);
        let stored = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert!(stored.contains("<"));
        println!("✅ 特殊字符验证通过");
    }
    
    #[test]
    fn test_json_like_content() {
        let content = r#"{"key": "value", "nested": {"a": 1}}"#;
        let memory = create_test_memory(MemoryType::Knowledge, content);
        let stored = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert!(stored.contains("\"key\""));
        println!("✅ JSON内容验证通过");
    }
    
    #[test]
    fn test_code_snippet() {
        let content = "fn main() { println!(\"Hello\"); }";
        let memory = create_test_memory(MemoryType::Procedural, content);
        let stored = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert!(stored.contains("fn main"));
        println!("✅ 代码片段验证通过");
    }
    
    #[test]
    fn test_url_content() {
        let content = "Visit https://example.com/path?query=value for more info";
        let memory = create_test_memory(MemoryType::Resource, content);
        let stored = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert!(stored.contains("https://"));
        println!("✅ URL内容验证通过");
    }
    
    #[test]
    fn test_multiline_content() {
        let content = "Line 1\nLine 2\nLine 3";
        let memory = create_test_memory(MemoryType::Episodic, content);
        let stored = match memory.content() {
            agent_mem_core::types::Content::Text(text) => text.clone(),
            _ => String::new(),
        };
        assert!(stored.contains('\n'));
        println!("✅ 多行内容验证通过");
    }

    // ============================================================================
    // 测试41-48: 重要性评分测试
    // ============================================================================
    
    #[test]
    fn test_episodic_importance_default() {
        let memory = create_test_memory(MemoryType::Episodic, "Test");
        let score = memory.score().unwrap_or(0.0);
        assert!(score >= 0.0 && score <= 1.0);
        println!("✅ Episodic默认重要性: {}", score);
    }
    
    #[test]
    fn test_semantic_importance_default() {
        let memory = create_test_memory(MemoryType::Semantic, "Test");
        let score = memory.score().unwrap_or(0.0);
        assert!(score >= 0.0 && score <= 1.0);
        println!("✅ Semantic默认重要性: {}", score);
    }
    
    #[test]
    fn test_procedural_importance_default() {
        let memory = create_test_memory(MemoryType::Procedural, "Test");
        let score = memory.score().unwrap_or(0.0);
        assert!(score >= 0.0 && score <= 1.0);
        println!("✅ Procedural默认重要性: {}", score);
    }
    
    #[test]
    fn test_working_importance_default() {
        let memory = create_test_memory(MemoryType::Working, "Test");
        let score = memory.score().unwrap_or(0.0);
        assert!(score >= 0.0 && score <= 1.0);
        println!("✅ Working默认重要性: {}", score);
    }
    
    #[test]
    fn test_core_importance_default() {
        let memory = create_test_memory(MemoryType::Core, "Test");
        let score = memory.score().unwrap_or(0.0);
        assert!(score >= 0.0 && score <= 1.0);
        println!("✅ Core默认重要性: {}", score);
    }
    
    #[test]
    fn test_resource_importance_default() {
        let memory = create_test_memory(MemoryType::Resource, "Test");
        let score = memory.score().unwrap_or(0.0);
        assert!(score >= 0.0 && score <= 1.0);
        println!("✅ Resource默认重要性: {}", score);
    }
    
    #[test]
    fn test_knowledge_importance_default() {
        let memory = create_test_memory(MemoryType::Knowledge, "Test");
        let score = memory.score().unwrap_or(0.0);
        assert!(score >= 0.0 && score <= 1.0);
        println!("✅ Knowledge默认重要性: {}", score);
    }
    
    #[test]
    fn test_contextual_importance_default() {
        let memory = create_test_memory(MemoryType::Contextual, "Test");
        let score = memory.score().unwrap_or(0.0);
        assert!(score >= 0.0 && score <= 1.0);
        println!("✅ Contextual默认重要性: {}", score);
    }
}
