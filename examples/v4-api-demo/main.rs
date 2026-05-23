//! V4Api 完整示例
//!
//! 展示 AgentMem v4.0 所有 API 模块的使用方法

use agent_mem::v4_api::*;

/// V4Api 完整使用示例
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== AgentMem V4Api 完整示例 ===\n");

    // 创建 V4Api 实例
    let v4 = V4Api::new();
    
    // ===== Phase 1: 核心 API =====
    println!("--- Phase 1: 核心 API ---");
    
    // 1. CoreMemory API (对标 Letta)
    let persona_id = v4.core_memory.create_persona(
        "agent-1",
        "I am a helpful AI assistant specializing in Rust".to_string(),
        Some(10000),
    ).await?;
    println!("✓ Created persona: {}", persona_id);
    
    let human_id = v4.core_memory.create_human(
        "user-123",
        "Name: John, Interests: Rust, AI, Web3".to_string(),
        Some(5000),
    ).await?;
    println!("✓ Created human: {}", human_id);
    
    let stats = v4.core_memory.get_stats().await?;
    println!("✓ CoreMemory stats: {} personas, {} humans", 
        stats.total_blocks, stats.persona_blocks);
    
    // 2. Intent Understanding API (对标 Mem0)
    let intent = v4.intent.understand(
        "What did John tell me about Rust last week?"
    ).await?;
    println!("✓ Intent: {:?} (confidence: {:.2})", intent.primary_intent, intent.confidence);
    println!("  Entities: {:?}", intent.entities.iter().map(|e| &e.name).collect::<Vec<_>>());
    
    // 3. Multi-Signal Search API (对标 Mem0 v3)
    let search_config = MultiSignalConfig {
        semantic_weight: 0.5,
        bm25_weight: 0.3,
        entity_weight: 0.2,
        fusion_method: "rrf".to_string(),
        enable_time_decay: true,
        time_decay_factor: 0.95,
    };
    let search_result = v4.search.search_with_signals(
        "Rust programming",
        Some(search_config),
    ).await?;
    println!("✓ Multi-Signal Search: {} results (fusion: {})", 
        search_result.total_results, search_result.fusion_method);
    
    // 4. Entity Linking API
    let entity_result = v4.entity_linking.link_entities(
        &["memory-1", "memory-2", "memory-3"]
    ).await?;
    println!("✓ Entity Linking: {} entities, {} relationships", 
        entity_result.linked_entities.len(), entity_result.relationships.len());
    
    // ===== Phase 2: 扩展 API =====
    println!("\n--- Phase 2: 扩展 API ---");
    
    // 5. Enhanced Search API
    let hybrid_result = v4.enhanced_search.hybrid_search(
        "machine learning",
        10,
    ).await?;
    println!("✓ Hybrid Search: {} results (strategy: {})", 
        hybrid_result.results.len(), hybrid_result.strategy);
    
    // 6. Reasoning API
    let causal = v4.reasoning.causal_reasoning(
        "If it rains, the ground gets wet",
        "It rained",
    ).await?;
    println!("✓ Causal Reasoning: {} causes, {} effects", 
        causal.causes.len(), causal.effects.len());
    
    let temporal = v4.reasoning.temporal_reasoning(
        "Meeting scheduled for 3pm",
        "Now is 4pm",
    ).await?;
    println!("✓ Temporal Reasoning: confidence = {:.2}", temporal.confidence);
    
    // 7. Adaptive Learning API
    let improvement = v4.adaptive.improve_from_feedback(
        "query", "result", true,
    ).await?;
    println!("✓ Adaptive Learning: improved = {}", improvement);
    
    let metrics = v4.adaptive.get_performance_metrics().await;
    println!("✓ Performance metrics: {} total queries, {:.2}ms avg latency",
        metrics.total_queries, metrics.avg_latency_ms);
    
    // ===== Phase 3: 企业级 API =====
    println!("\n--- Phase 3: 企业级 API ---");
    
    // 8. Memory Trace API
    v4.memory_trace.add_trace(
        "user-123", "memory-1", "add", "Sample trace"
    ).await?;
    let traces = v4.memory_trace.list_traces(10).await?;
    println!("✓ Memory Trace: {} entries", traces.len());
    
    // 9. Audit Log API
    v4.audit_log.log_action(
        "user-123", "memory", "create", "Created memory"
    ).await?;
    let logs = v4.audit_log.query_logs(10).await?;
    println!("✓ Audit Log: {} entries", logs.len());
    
    // 10. Quota API
    v4.quota.set_quota("user-123", 1000, 100).await?;
    let usage = v4.quota.get_quota_usage("user-123").await?;
    println!("✓ Quota: {} / {} memories", usage.current_memories, 1000);
    
    // 11. Multi-Tenant API
    let tenant_id = v4.multi_tenant.create_tenant("Enterprise Corp", TenantPlan::Enterprise);
    println!("✓ Created tenant: {}", tenant_id);
    
    // ===== Decentralized API (Phase 5) =====
    println!("\n--- Phase 5: 去中心化 API ---");
    
    // 12. Decentralized Architecture API
    let node_id = v4.decentralized.register_node(
        "192.168.1.100", 
        8080, 
        agent_mem_core::decentralized_architecture::NodeStatus::Online
    ).await?;
    println!("✓ Registered node: {}", node_id);
    
    let nodes = v4.decentralized.list_nodes().await?;
    println!("✓ Known nodes: {}", nodes.len());
    
    let sync_status = v4.decentralized.get_sync_status().await;
    println!("✓ Sync status: {} nodes, {} synced", 
        sync_status.node_count, sync_status.synced_nodes);
    
    // ===== 健康检查 =====
    println!("\n--- 健康检查 ---");
    let health = v4.health_check().await;
    println!("✓ V4Api Health:");
    println!("  - Core Memory: {}", health.core_memory);
    println!("  - Intent: {}", health.intent);
    println!("  - Search: {}", health.search);
    println!("  - Decentralized: {}", health.decentralized);
    println!("  - Overall: {}", health.overall);
    
    // ===== Phase 4: 高级 API =====
    println!("\n--- Phase 4: 高级 API (V4ApiPhase4) ---");
    
    // 创建 Phase 4 API
    let v4_phase4 = V4Api::new().with_phase4();
    
    // Code Sandbox API
    let sandbox_id = v4_phase4.code_sandbox.create_sandbox(
        "python", 60
    ).await?;
    println!("✓ Created sandbox: {}", sandbox_id);
    
    // Fleet API
    let agent_id = v4_phase4.fleet.create_agent(
        "helper-agent", 
        AgentRole::Researcher
    ).await?;
    println!("✓ Created agent: {}", agent_id);
    
    let team_id = v4_phase4.fleet.create_team(
        "AI Team", 
        TeamStrategy::Parallel
    ).await?;
    println!("✓ Created team: {}", team_id);
    
    // Mental Model API
    let model_id = v4_phase4.mental_model.create_persona_model(
        "empathetic-assistant",
        "You are an empathetic assistant".to_string(),
    ).await?;
    println!("✓ Created mental model: {}", model_id);
    
    // Schema Evolution API
    let schema_id = v4_phase4.schema_evolution.register_schema(
        "user-profile",
        "User profile schema".to_string(),
        serde_json::json!({
            "name": "string",
            "email": "string",
            "age": "number"
        }),
    ).await?;
    println!("✓ Registered schema: {}", schema_id);
    
    // Phase 4 Health Check
    let health_phase4 = v4_phase4.health_check().await;
    println!("✓ V4ApiPhase4 Health: all_healthy = {}", health_phase4.all_healthy);
    
    println!("\n=== 全部 API 演示完成 ===");
    Ok(())
}
