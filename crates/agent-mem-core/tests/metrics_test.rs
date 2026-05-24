//! Metrics Module Tests

use std::{collections::HashMap, time::Duration};

use agent_mem_core::cognitive_memory::{MemoryMetrics, MemoryStatsByType, OperationTimer};

#[test]
fn test_memory_metrics_initialization() {
    let metrics = MemoryMetrics::new();
    assert_eq!(metrics.total_adds, 0);
    assert_eq!(metrics.total_retrieves, 0);
    assert_eq!(metrics.total_deletes, 0);
    assert_eq!(metrics.errors, 0);
}

#[test]
fn test_record_add_operations() {
    let mut metrics = MemoryMetrics::new();

    metrics.record_add(Duration::from_micros(100), 5);
    assert_eq!(metrics.total_adds, 5);
    assert!(metrics.avg_add_latency_us > 0.0);

    metrics.record_add(Duration::from_micros(200), 3);
    assert_eq!(metrics.total_adds, 8);
}

#[test]
fn test_record_retrieve_operations() {
    let mut metrics = MemoryMetrics::new();

    metrics.record_retrieve(Duration::from_micros(50));
    assert_eq!(metrics.total_retrieves, 1);
    assert_eq!(metrics.avg_retrieve_latency_us, 50.0);

    metrics.record_retrieve(Duration::from_micros(100));
    assert_eq!(metrics.total_retrieves, 2);
}

#[test]
fn test_record_delete_operations() {
    let mut metrics = MemoryMetrics::new();

    metrics.record_delete(Duration::from_micros(30));
    assert_eq!(metrics.total_deletes, 1);
    assert_eq!(metrics.avg_delete_latency_us, 30.0);
}

#[test]
fn test_record_errors() {
    let mut metrics = MemoryMetrics::new();

    metrics.record_error();
    metrics.record_error();
    assert_eq!(metrics.errors, 2);
}

#[test]
fn test_peak_memory_update() {
    let mut metrics = MemoryMetrics::new();

    metrics.update_peak(100);
    assert_eq!(metrics.peak_memory_count, 100);

    metrics.update_peak(50);
    assert_eq!(metrics.peak_memory_count, 100); // Should not decrease
}

#[test]
fn test_throughput_calculation() {
    let mut metrics = MemoryMetrics::new();

    metrics.record_add(Duration::from_micros(100), 10);
    metrics.record_retrieve(Duration::from_micros(50));
    metrics.record_delete(Duration::from_micros(30));
    metrics.record_get();

    let throughput = metrics.throughput(1.0);
    assert_eq!(throughput, 13.0); // 10 + 1 + 1 + 1
}

#[test]
fn test_memory_stats_by_type_from_map() {
    let mut by_type = HashMap::new();
    by_type.insert("semantic".to_string(), 10);
    by_type.insert("episodic".to_string(), 5);
    by_type.insert("procedural".to_string(), 3);
    by_type.insert("core".to_string(), 2);
    by_type.insert("working".to_string(), 1);

    let stats = MemoryStatsByType::from_map(&by_type);
    assert_eq!(stats.semantic_count, 10);
    assert_eq!(stats.episodic_count, 5);
    assert_eq!(stats.procedural_count, 3);
    assert_eq!(stats.core_count, 2);
    assert_eq!(stats.working_count, 1);
    assert_eq!(stats.total(), 21);
}

#[test]
fn test_operation_timer() {
    let timer = OperationTimer::new();

    std::thread::sleep(Duration::from_micros(500));

    let elapsed = timer.elapsed();
    assert!(elapsed.as_micros() >= 400); // Allow some tolerance
}

#[test]
fn test_empty_by_type() {
    let by_type = HashMap::new();
    let stats = MemoryStatsByType::from_map(&by_type);
    assert_eq!(stats.total(), 0);
}
