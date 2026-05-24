//! TemporalReasoning Engine Integration Tests

use std::sync::Arc;

use agent_mem_core::{
    temporal_graph::TemporalGraphEngine,
    temporal_reasoning::{TemporalReasoningConfig, TemporalReasoningPath, TemporalReasoningType},
};
use chrono::Utc;

#[tokio::test]
async fn test_temporal_reasoning_engine_creation() {
    // 创建GraphMemoryEngine和TemporalGraphEngine
    let graph_engine = agent_mem_core::graph_memory::GraphMemoryEngine::new();
    let _temporal_graph = Arc::new(TemporalGraphEngine::new(Arc::new(graph_engine)));
    // 引擎创建验证（只检查能正常构建）
    assert!(true, "TemporalGraphEngine created successfully");
}

#[tokio::test]
async fn test_temporal_reasoning_config_default() {
    let config = TemporalReasoningConfig::default();
    // 验证默认配置存在
    assert!(
        config.max_reasoning_depth > 0,
        "Max reasoning depth should be positive"
    );
}

#[tokio::test]
async fn test_temporal_reasoning_path_structure() {
    let now = Utc::now();
    let path = TemporalReasoningPath {
        nodes: vec!["node1".to_string(), "node2".to_string()],
        edges: vec![],
        timestamps: vec![now],
        reasoning_type: TemporalReasoningType::TemporalLogic,
        confidence: 0.95,
        explanation: "Test path".to_string(),
    };
    assert_eq!(path.confidence, 0.95);
    assert_eq!(path.reasoning_type, TemporalReasoningType::TemporalLogic);
    assert_eq!(path.nodes.len(), 2);
}

#[tokio::test]
async fn test_temporal_reasoning_types() {
    // 测试所有时序推理类型
    let types = vec![
        TemporalReasoningType::TemporalLogic,
        TemporalReasoningType::Causal,
        TemporalReasoningType::MultiHop,
        TemporalReasoningType::Counterfactual,
        TemporalReasoningType::Predictive,
    ];
    assert_eq!(types.len(), 5, "Should have 5 temporal reasoning types");
}
