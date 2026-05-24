//! GraphMemory and CausalReasoning Tests
//!
//! Week 3 Optional Advanced Features

use agent_mem_core::graph_memory::GraphMemoryEngine;
use agent_mem_core::causal_reasoning::CausalReasoningEngine;
use agent_mem_core::types::{Memory, MemoryType};

#[tokio::test]
async fn test_graph_memory_engine_creation() {
    let _engine = GraphMemoryEngine::new();
    assert!(true); // Just verify creation
}

#[tokio::test]
async fn test_causal_reasoning_engine_creation() {
    let _engine = CausalReasoningEngine::with_defaults();
    assert!(true); // Just verify creation
}

#[tokio::test]
async fn test_memory_types() {
    let memory = Memory::new(
        "test-agent".to_string(),
        Some("test-user".to_string()),
        MemoryType::Semantic,
        "Test content".to_string(),
        0.8,
    );
    assert_eq!(memory.memory_type(), MemoryType::Semantic);
}
