# AgentMem V4 API 文档

## 概述

AgentMem V4 API 是统一的高级记忆管理 API，提供 24+ 个功能模块，涵盖从核心记忆管理到企业级功能的完整能力。

## 快速开始

```rust
use agent_mem::v4_api::V4Api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let v4 = V4Api::new();
    
    // CoreMemory - 对标 Letta
    let persona_id = v4.core_memory.create_persona(
        "agent-1",
        "I am a helpful assistant".to_string(),
        None,
    ).await?;
    
    // Intent - 对标 Mem0
    let intent = v4.intent.understand("What did John tell me?").await?;
    
    // Multi-Signal Search - 对标 Mem0 v3
    let result = v4.search.search_with_signals("restaurants", None).await?;
    
    println!("✓ V4 API working!");
    Ok(())
}
```

## API 模块列表

### Phase 1: 核心 API

| API | 功能 | 对标 |
|-----|------|------|
| `CoreMemoryApi` | Persona/Human 块管理 | Letta |
| `IntentUnderstandingApi` | 查询意图理解 | Mem0 |
| `MultiSignalSearchApi` | 多信号混合搜索 | Mem0 v3 |
| `EntityLinkingApi` | 跨记忆实体链接 | Mem0 |

### Phase 2: 扩展 API

| API | 功能 | 对标 |
|-----|------|------|
| `EnhancedSearchApi` | 增强混合搜索 | - |
| `ReasoningApi` | 因果/时序推理 | - |
| `AdaptiveLearningApi` | 自适应学习 | Mem0 |

### Phase 3: 企业级 API

| API | 功能 | 对标 |
|-----|------|------|
| `MemoryTraceApi` | 记忆轨迹追踪 | - |
| `AuditLogApi` | 审计日志 | - |
| `QuotaApi` | 配额管理 | - |
| `MultiTenantApi` | 多租户隔离 | - |

### Phase 4: 高级 API

| API | 功能 | 对标 |
|-----|------|------|
| `CodeSandboxApi` | 代码执行沙箱 | Letta |
| `FleetApi` | 多 Agent 舰队管理 | Agno |
| `MentalModelApi` | 心智模型 | Letta |
| `SchemaEvolutionApi` | Schema 自动演进 | - |

### Phase 5: 分布式 API

| API | 功能 | 对标 |
|-----|------|------|
| `DecentralizedArchitectureApi` | 去中心化架构 | - |

## 详细 API 文档

### CoreMemoryApi

对标 Letta 的 Block-based Memory 系统。

```rust
// 创建 Persona 块
let persona_id = v4.core_memory.create_persona(
    "agent-1",
    "I am a Rust expert".to_string(),
    Some(10000), // max_capacity
).await?;

// 创建 Human 块
let human_id = v4.core_memory.create_human(
    "user-123",
    "Name: John, likes: pizza".to_string(),
    None,
).await?;

// 获取块
let persona = v4.core_memory.get_persona(&persona_id).await?;

// 列出所有块
let personas = v4.core_memory.list_personas().await?;

// 更新块
v4.core_memory.update_persona(&persona_id, "Updated content".to_string()).await?;

// 追加内容
v4.core_memory.append_to_persona(&persona_id, " More content".to_string()).await?;

// 获取统计
let stats = v4.core_memory.get_stats().await?;
println!("{} personas, {} humans", stats.persona_blocks, stats.human_blocks);
```

### IntentUnderstandingApi

对标 Mem0 的意图理解系统。

```rust
// 理解查询意图
let intent = v4.intent.understand(
    "What did John tell me about restaurants last week?"
).await?;

match intent.primary_intent {
    IntentType::Recall => println!("User wants to recall information"),
    IntentType::Add => println!("User wants to add memory"),
    IntentType::Update => println!("User wants to update memory"),
    IntentType::Delete => println!("User wants to delete memory"),
    IntentType::Summarize => println!("User wants summary"),
    IntentType::Explore => println!("User wants to explore"),
    IntentType::Compare => println!("User wants comparison"),
    IntentType::Reason => println!("User wants reasoning"),
}

// 提取的实体
for entity in &intent.entities {
    println!("Entity: {} ({:?})", entity.name, entity.entity_type);
}

// 时间范围
if let Some(time_range) = &intent.time_range {
    println!("Time range: {:?}", time_range);
}
```

### MultiSignalSearchApi

对标 Mem0 v3 的多信号检索。

```rust
// 配置搜索参数
let config = MultiSignalConfig {
    semantic_weight: 0.5,
    bm25_weight: 0.3,
    entity_weight: 0.2,
    fusion_method: "rrf".to_string(), // "rrf" or "weighted"
    enable_time_decay: true,
    time_decay_factor: 0.95,
};

// 多信号搜索
let result = v4.search.search_with_signals(
    "machine learning",
    Some(config),
).await?;

println!("Found {} results", result.total_results);
println!("Fusion method: {}", result.fusion_method);
println!("Processing time: {}ms", result.processing_time_ms);
```

### EntityLinkingApi

跨记忆实体链接。

```rust
// 链接实体
let result = v4.entity_linking.link_entities(&[
    "memory-1",
    "memory-2", 
    "memory-3",
]).await?;

println!("Linked {} entities", result.linked_entities.len());
println!("Found {} relationships", result.relationships.len());

// 获取实体图
let graph = v4.entity_linking.get_entity_graph("John").await?;
```

### ReasoningApi

因果和时序推理。

```rust
// 因果推理
let causal = v4.reasoning.causal_reasoning(
    "If it rains, the ground gets wet",
    "It rained",
).await?;

println!("Causes: {:?}", causal.causes);
println!("Effects: {:?}", causal.effects);
println!("Confidence: {}", causal.confidence);

// 时序推理
let temporal = v4.reasoning.temporal_reasoning(
    "Meeting at 3pm",
    "Current time is 4pm",
).await?;

println!("Temporal confidence: {}", temporal.confidence);
```

### AdaptiveLearningApi

自适应学习改进。

```rust
// 提供反馈改进
let improved = v4.adaptive.improve_from_feedback(
    "query",
    "result",
    true, // success
).await?;

// 获取当前策略
let strategy = v4.adaptive.get_strategy("complex query").await?;
println!("Using strategy: {:?}", strategy);

// 获取性能指标
let metrics = v4.adaptive.get_performance_metrics().await;
println!("Total queries: {}", metrics.total_queries);
println!("Success rate: {:.1}%", metrics.successful_queries as f64 / metrics.total_queries as f64 * 100.0);
```

### MemoryTraceApi

记忆操作轨迹追踪。

```rust
// 添加轨迹
v4.memory_trace.add_trace(
    "user-123",
    "memory-1", 
    "add",
    "User added a memory",
).await?;

// 列出轨迹
let traces = v4.memory_trace.list_traces(50).await?;
for trace in traces {
    println!("[{}] {} - {} ({})",
        trace.timestamp,
        trace.action,
        trace.query.as_deref().unwrap_or("-"),
        trace.latency_ms
    );
}
```

### AuditLogApi

审计日志记录。

```rust
// 记录操作
v4.audit_log.log_action(
    "user-123",
    "memory",
    "create",
    "Created memory about project X",
).await?;

// 查询日志
let logs = v4.audit_log.query_logs(100).await?;
for log in logs {
    println!("[{}] {}: {} - {:?}",
        log.timestamp,
        log.user_id.as_deref().unwrap_or("system"),
        log.action,
        log.status
    );
}
```

### QuotaApi

配额管理。

```rust
// 设置配额
v4.quota.set_quota("user-123", 1000, 100).await?;

// 检查配额
let check = v4.quota.check_quota("user-123").await?;
println!("Allowed: {}", check.allowed);

// 获取使用情况
let usage = v4.quota.get_quota_usage("user-123").await?;
println!("Memories: {}/{}", usage.current_memories, 1000);
```

### MultiTenantApi

多租户隔离。

```rust
// 创建租户
let tenant_id = v4.multi_tenant.create_tenant(
    "Enterprise Corp",
    TenantPlan::Enterprise,
);

// 切换租户
v4.multi_tenant.switch_tenant(&tenant_id);

// 获取当前租户
let current = v4.multi_tenant.get_current_tenant();
```

### DecentralizedArchitectureApi

去中心化分布式架构。

```rust
use agent_mem_core::decentralized_architecture::NodeStatus;

// 注册节点
let node_id = v4.decentralized.register_node(
    "192.168.1.100",
    8080,
    NodeStatus::Online,
).await?;

// 列出节点
let nodes = v4.decentralized.list_nodes().await?;
println!("Known nodes: {}", nodes.len());

// 获取同步状态
let sync_status = v4.decentralized.get_sync_status().await;
println!("Synced: {}/{}", sync_status.synced_nodes, sync_status.node_count);

// 同步数据
v4.decentralized.sync_data(
    "key",
    b"value".to_vec(),
    SyncOperationType::Create,
).await?;

// 获取冲突
let conflicts = v4.decentralized.get_conflicts(None).await?;
```

### V4ApiPhase4

完整的 Phase 4 API，包含所有高级功能。

```rust
let v4_phase4 = V4Api::new().with_phase4();

// Code Sandbox
let sandbox_id = v4_phase4.code_sandbox.create_sandbox("python", 60).await?;
let result = v4_phase4.code_sandbox.execute_code(
    &sandbox_id,
    "print('Hello, World!')",
).await?;

// Fleet Management
let agent_id = v4_phase4.fleet.create_agent(
    "researcher",
    AgentRole::Researcher,
).await?;

let team_id = v4_phase4.fleet.create_team(
    "AI Team",
    TeamStrategy::Parallel,
).await?;

v4_phase4.fleet.add_member_to_team(&team_id, &agent_id).await?;

// Mental Model
let model_id = v4_phase4.mental_model.create_persona_model(
    "empathetic",
    "You are an empathetic assistant",
).await?;

// Schema Evolution
let schema_id = v4_phase4.schema_evolution.register_schema(
    "user-profile",
    "User profile schema",
    serde_json::json!({
        "name": "string",
        "email": "string"
    }),
).await?;
```

## 健康检查

```rust
let health = v4.health_check().await;
println!("Overall: {}", health.overall);
println!("Core Memory: {}", health.core_memory);
println!("Intent: {}", health.intent);
println!("Search: {}", health.search);
// ...
```

## 完整示例

参见 `examples/v4-api-demo/main.rs`

## 基准测试

运行基准测试：

```bash
cargo bench --package agent-mem --bench v4_api_benchmark
```

## 许可证

Apache 2.0
