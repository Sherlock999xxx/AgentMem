//! V4Api Benchmark Suite
//! 
//! Benchmark tests for all V4Api modules including:
//! - CoreMemoryApi
//! - IntentUnderstandingApi
//! - MultiSignalSearchApi
//! - EntityLinkingApi
//! - ReasoningApi
//! - AdaptiveLearningApi
//! - MemoryTraceApi
//! - AuditLogApi
//! - QuotaApi
//! - MultiTenantApi
//! - CodeSandboxApi
//! - FleetApi
//! - MentalModelApi
//! - SchemaEvolutionApi

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;

#[path = "../crates/agent-mem/src/v4_api.rs"]
mod v4_api;

use v4_api::*;

/// CoreMemoryApi Benchmarks
fn bench_core_memory_api(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("v4_api_core_memory_create_persona", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.core_memory.create_persona(
                black_box("agent-1"),
                black_box("Test persona content".to_string()),
                black_box(None),
            ).await
        });
    });
    
    c.bench_function("v4_api_core_memory_list_personas", |b| {
        let api = V4Api::new();
        rt.block_on(async {
            // Pre-populate data
            for i in 0..10 {
                api.core_memory.create_persona(
                    &format!("agent-{}", i),
                    format!("Persona {} content", i),
                    None,
                ).await.ok();
            }
        });
        
        b.to_async(&rt).iter(|| async {
            api.core_memory.list_personas().await
        });
    });
    
    c.bench_function("v4_api_core_memory_get_stats", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.core_memory.get_stats().await
        });
    });
}

/// IntentUnderstandingApi Benchmarks
fn bench_intent_api(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let queries = vec![
        "What did John tell me about restaurants?",
        "Remember that I prefer Italian food",
        "Update my email address to new@example.com",
        "Forget what I said yesterday",
        "Summarize my recent conversations",
    ];
    
    c.bench_function("v4_api_intent_understand", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.intent.understand(black_box(queries[0])).await
        });
    });
    
    let mut group = c.benchmark_group("v4_api_intent_by_query_type");
    for (i, query) in queries.iter().enumerate() {
        group.bench_with_input(BenchmarkId::from_parameter(i), query, |b, q| {
            let api = V4Api::new();
            b.to_async(&rt).iter(|| async {
                api.intent.understand(black_box(*q)).await
            });
        });
    }
    group.finish();
}

/// MultiSignalSearchApi Benchmarks
fn bench_multi_signal_search_api(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("v4_api_multi_signal_search", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.search.search_with_signals(
                black_box("artificial intelligence"),
                black_box(None),
            ).await
        });
    });
    
    c.bench_function("v4_api_multi_signal_search_with_config", |b| {
        let api = V4Api::new();
        let config = MultiSignalConfig {
            semantic_weight: 0.6,
            bm25_weight: 0.3,
            entity_weight: 0.1,
            fusion_method: "rrf".to_string(),
            enable_time_decay: true,
            time_decay_factor: 0.95,
        };
        b.to_async(&rt).iter(|| async {
            api.search.search_with_signals(
                black_box("machine learning"),
                black_box(Some(config)),
            ).await
        });
    });
}

/// EntityLinkingApi Benchmarks
fn bench_entity_linking_api(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let memory_ids = vec!["memory-1", "memory-2", "memory-3", "memory-4", "memory-5"];
    
    c.bench_function("v4_api_entity_linking", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.entity_linking.link_entities(black_box(&memory_ids)).await
        });
    });
    
    c.bench_function("v4_api_entity_graph", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.entity_linking.get_entity_graph(black_box("John")).await
        });
    });
}

/// ReasoningApi Benchmarks
fn bench_reasoning_api(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("v4_api_causal_reasoning", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.reasoning.causal_reasoning(
                black_box("If it rains, the ground gets wet"),
                black_box("It rained"),
            ).await
        });
    });
    
    c.bench_function("v4_api_temporal_reasoning", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.reasoning.temporal_reasoning(
                black_box("Meeting scheduled for 3pm"),
                black_box("Now is 4pm"),
            ).await
        });
    });
    
    c.bench_function("v4_api_semantic_reasoning", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.reasoning.semantic_reasoning(
                black_box("All cats are mammals"),
                black_box("Whiskers is a cat"),
            ).await
        });
    });
}

/// AdaptiveLearningApi Benchmarks
fn bench_adaptive_learning_api(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("v4_api_adaptive_improve", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.adaptive.improve_from_feedback(
                black_box("query"),
                black_box("result"),
                black_box(true),
            ).await
        });
    });
    
    c.bench_function("v4_api_adaptive_get_strategy", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.adaptive.get_strategy(black_box("query")).await
        });
    });
    
    c.bench_function("v4_api_adaptive_metrics", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.adaptive.get_performance_metrics().await
        });
    });
}

/// MemoryTraceApi Benchmarks
fn bench_memory_trace_api(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("v4_api_trace_add", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.memory_trace.add_trace(
                black_box("user-1"),
                black_box("memory-1"),
                black_box("add"),
                black_box("Test memory content"),
            ).await
        });
    });
    
    c.bench_function("v4_api_trace_list", |b| {
        let api = V4Api::new();
        rt.block_on(async {
            // Pre-populate traces
            for i in 0..50 {
                api.memory_trace.add_trace(
                    &format!("user-{}", i % 5),
                    &format!("memory-{}", i),
                    "add",
                    &format!("Trace {}", i),
                ).await.ok();
            }
        });
        
        b.to_async(&rt).iter(|| async {
            api.memory_trace.list_traces(black_box(10)).await
        });
    });
}

/// AuditLogApi Benchmarks
fn bench_audit_log_api(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("v4_api_audit_log", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.audit_log.log_action(
                black_box("user-1"),
                black_box("memory"),
                black_box("create"),
                black_box("Created memory"),
            ).await
        });
    });
    
    c.bench_function("v4_api_audit_query", |b| {
        let api = V4Api::new();
        rt.block_on(async {
            // Pre-populate logs
            for i in 0..100 {
                api.audit_log.log_action(
                    &format!("user-{}", i % 10),
                    "memory",
                    "create",
                    &format!("Action {}", i),
                ).await.ok();
            }
        });
        
        b.to_async(&rt).iter(|| async {
            api.audit_log.query_logs(black_box(50)).await
        });
    });
}

/// QuotaApi Benchmarks
fn bench_quota_api(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("v4_api_quota_set", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.quota.set_quota(
                black_box("user-1"),
                black_box(1000),
                black_box(100),
            ).await
        });
    });
    
    c.bench_function("v4_api_quota_check", |b| {
        let api = V4Api::new();
        rt.block_on(async {
            api.quota.set_quota("user-1", 1000, 100).await.ok();
        });
        
        b.to_async(&rt).iter(|| async {
            api.quota.check_quota(black_box("user-1")).await
        });
    });
    
    c.bench_function("v4_api_quota_usage", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.quota.get_quota_usage(black_box("user-1")).await
        });
    });
}

/// MultiTenantApi Benchmarks
fn bench_multi_tenant_api(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("v4_api_tenant_create", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.multi_tenant.create_tenant(
                black_box(&format!("tenant-{}", uuid::Uuid::new_v4())),
                black_box("Test Tenant"),
            ).await
        });
    });
    
    c.bench_function("v4_api_tenant_switch", |b| {
        let api = V4Api::new();
        rt.block_on(async {
            api.multi_tenant.create_tenant("tenant-1", "Tenant 1").await.ok();
        });
        
        b.to_async(&rt).iter(|| async {
            api.multi_tenant.switch_tenant(black_box("tenant-1")).await
        });
    });
}

/// Health Check Benchmark
fn bench_health_check(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("v4_api_health_check", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            api.health_check().await
        });
    });
}

/// Phase 4 API Benchmarks
fn bench_phase4_apis(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let api = V4Api::new().with_phase4();
    
    // CodeSandboxApi
    c.bench_function("v4_api_phase4_code_sandbox_create", |b| {
        b.to_async(&rt).iter(|| async {
            api.code_sandbox.create_sandbox(
                black_box("python"),
                black_box(60),
            ).await
        });
    });
    
    // FleetApi
    c.bench_function("v4_api_phase4_fleet_create_agent", |b| {
        b.to_async(&rt).iter(|| async {
            api.fleet.create_agent(
                black_box(&format!("agent-{}", uuid::Uuid::new_v4())),
                black_box("helper"),
            ).await
        });
    });
    
    c.bench_function("v4_api_phase4_fleet_status", |b| {
        b.to_async(&rt).iter(|| async {
            api.fleet.get_fleet_status().await
        });
    });
    
    // MentalModelApi
    c.bench_function("v4_api_phase4_mental_model_create", |b| {
        b.to_async(&rt).iter(|| async {
            api.mental_model.create_persona_model(
                black_box(&format!("persona-{}", uuid::Uuid::new_v4())),
                black_box("Helpful assistant".to_string()),
            ).await
        });
    });
    
    // SchemaEvolutionApi
    c.bench_function("v4_api_phase4_schema_register", |b| {
        b.to_async(&rt).iter(|| async {
            api.schema_evolution.register_schema(
                black_box("test-schema"),
                black_box("user".to_string()),
                black_box(serde_json::json!({"name": "string"})),
            ).await
        });
    });
    
    // Phase 4 Health Check
    c.bench_function("v4_api_phase4_health_check", |b| {
        b.to_async(&rt).iter(|| async {
            api.health_check().await
        });
    });
}

/// Concurrent Operations Benchmark
fn bench_concurrent_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("v4_api_concurrent_persona_creates", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            let mut handles = vec![];
            for i in 0..10 {
                let api = api.clone();
                handles.push(tokio::spawn(async move {
                    api.core_memory.create_persona(
                        &format!("agent-{}", i),
                        format!("Persona {} content", i),
                        None,
                    ).await
                }));
            }
            futures::future::join_all(handles).await
        });
    });
    
    c.bench_function("v4_api_concurrent_searches", |b| {
        let api = V4Api::new();
        b.to_async(&rt).iter(|| async {
            let mut handles = vec![];
            let queries = vec![
                "artificial intelligence",
                "machine learning",
                "deep learning",
                "neural networks",
                "transformers",
            ];
            for q in queries {
                let api = api.clone();
                handles.push(tokio::spawn(async move {
                    api.search.search_with_signals(q, None).await
                }));
            }
            futures::future::join_all(handles).await
        });
    });
}

criterion_group! {
    name = v4_api_benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(5))
        .sample_size(100);
    targets = 
        bench_core_memory_api,
        bench_intent_api,
        bench_multi_signal_search_api,
        bench_entity_linking_api,
        bench_reasoning_api,
        bench_adaptive_learning_api,
        bench_memory_trace_api,
        bench_audit_log_api,
        bench_quota_api,
        bench_multi_tenant_api,
        bench_health_check,
        bench_phase4_apis,
        bench_concurrent_operations
}

criterion_main!(v4_api_benches);
