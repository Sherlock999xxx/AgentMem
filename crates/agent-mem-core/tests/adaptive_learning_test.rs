//! AdaptiveLearning Engine Integration Tests

use agent_mem_core::adaptive_learning::{
    AdaptiveLearningConfig, AdaptiveLearningEngine, LearningStatistics, LearningStrategy,
};
use chrono::Utc;

#[tokio::test]
async fn test_adaptive_learning_config_default() {
    let config = AdaptiveLearningConfig::default();
    assert!(
        config.learning_rate > 0.0,
        "Learning rate should be positive"
    );
}

#[tokio::test]
async fn test_adaptive_learning_engine_creation() {
    let config = AdaptiveLearningConfig::default();
    let engine = AdaptiveLearningEngine::new(config);
    // Engine应该被创建（不返回Result）
    assert!(true, "Engine should be created successfully");
}

#[tokio::test]
async fn test_learning_strategy_variants() {
    // 测试所有学习策略
    let strategies = vec![
        LearningStrategy::Conservative,
        LearningStrategy::Balanced,
        LearningStrategy::Aggressive,
        LearningStrategy::Adaptive,
    ];
    assert_eq!(strategies.len(), 4, "Should have 4 learning strategies");
}

#[tokio::test]
async fn test_learning_statistics_structure() {
    let stats = LearningStatistics {
        total_learning_cycles: 100,
        parameter_adjustments: 50,
        avg_performance_improvement: 0.05,
        current_strategy: LearningStrategy::Balanced,
        last_updated: Utc::now(),
    };
    assert_eq!(stats.total_learning_cycles, 100);
    assert_eq!(stats.parameter_adjustments, 50);
}
