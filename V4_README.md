# AgentMem v4.0 - 顶级 AI Agent 记忆平台

![Version](https://img.shields.io/badge/version-v4.0.0-blue)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![License](https://img.shields.io/badge/license-Apache%202.0-green)

**AgentMem** 是一个生产级的 AI Agent 记忆管理系统，对标全球顶级记忆平台 **Mem0**、**Letta** 和 **Agno**。

## ✨ 核心特性

### 8 种认知记忆类型
```
┌─────────────────────────────────────────────────────────────┐
│  Episodic    │ 语义记忆 │ 程序记忆 │ 工作记忆 │ 核心记忆    │
│  (事件)       │ (事实)   │ (技能)   │ (临时)   │ (Persona)  │
├─────────────────────────────────────────────────────────────┤
│  Resource    │ Knowledge │ Contextual │                         │
│  (资源)       │ (知识库)  │ (上下文)  │                          │
└─────────────────────────────────────────────────────────────┘
```

### V4 API - 24+ 功能模块

| Phase | 模块 | 功能 | 对标 |
|-------|------|------|------|
| **P1** | CoreMemory | Persona/Human 块管理 | Letta |
| **P1** | Intent | 查询意图理解 | Mem0 |
| **P1** | MultiSignal | 多信号混合搜索 | Mem0 v3 |
| **P1** | EntityLinking | 跨记忆实体链接 | Mem0 |
| **P2** | EnhancedSearch | 增强混合搜索 | - |
| **P2** | Reasoning | 因果/时序推理 | - |
| **P2** | AdaptiveLearning | 自适应学习 | Mem0 |
| **P3** | MemoryTrace | 记忆轨迹追踪 | - |
| **P3** | AuditLog | 审计日志 | - |
| **P3** | Quota | 配额管理 | - |
| **P3** | MultiTenant | 多租户隔离 | - |
| **P4** | CodeSandbox | 代码执行沙箱 | Letta |
| **P4** | Fleet | 多 Agent 管理 | Agno |
| **P4** | MentalModel | 心智模型 | Letta |
| **P4** | SchemaEvolution | Schema 自动演进 | - |
| **P5** | Decentralized | 去中心化架构 | - |

## 🚀 快速开始

### 安装

```bash
# 添加到 Cargo.toml
[dependencies]
agent-mem = { git = "https://github.com/your-org/agentmem" }

# 或从 crates.io
agent-mem = "4.0"
```

### 基础使用

```rust
use agent_mem::v4_api::V4Api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let v4 = V4Api::new();
    
    // 1. 创建记忆
    let persona_id = v4.core_memory.create_persona(
        "agent-1",
        "I am a helpful AI assistant".to_string(),
        None,
    ).await?;
    
    // 2. 理解查询意图
    let intent = v4.intent.understand(
        "What did John tell me about restaurants?"
    ).await?;
    println!("Intent: {:?}", intent.primary_intent);
    
    // 3. 多信号搜索
    let results = v4.search.search_with_signals(
        "restaurants",
        None,
    ).await?;
    println!("Found {} results", results.total_results);
    
    // 4. 健康检查
    let health = v4.health_check().await;
    println!("System healthy: {}", health.overall);
    
    Ok(())
}
```

### 高级使用 - V4ApiPhase4

```rust
use agent_mem::v4_api::{V4Api, AgentRole, TeamStrategy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let v4 = V4Api::new().with_phase4();
    
    // 代码沙箱
    let sandbox = v4.code_sandbox.create_sandbox("python", 60).await?;
    let output = v4.code_sandbox.execute_code(&sandbox, "print('Hello!')").await?;
    
    // 多 Agent 舰队
    let agent = v4.fleet.create_agent("researcher", AgentRole::Researcher).await?;
    let team = v4.fleet.create_team("AI Team", TeamStrategy::Parallel).await?;
    
    // 心智模型
    let model = v4.mental_model.create_persona_model(
        "empathetic",
        "You are an empathetic assistant".to_string(),
    ).await?;
    
    Ok(())
}
```

## 📊 架构图

```
╔══════════════════════════════════════════════════════════════════════╗
║                      AgentMem v4.0 Architecture                      ║
╠══════════════════════════════════════════════════════════════════════╣
║                                                                       ║
║  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐  ║
║  │   REST   │  │   MCP    │  │   CLI    │  │ Python   │  │  WASM  │  ║
║  │   API    │  │ Server   │  │   Tool   │  │   SDK    │  │Plugins │  ║
║  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘  └───┬────┘  ║
║       └────────────┴────────────┴────────────┴────────────┘         ║
║                              │                                         ║
║                              ▼                                         ║
║  ┌─────────────────────────────────────────────────────────────────┐ ║
║  │                    V4Api / V4ApiPhase4                           │ ║
║  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐  │ ║
║  │  │  Core   │ │ Intent  │ │  Multi  │ │ Entity  │ │Enhanced │  │ ║
║  │  │ Memory  │ │         │ │ Signal  │ │ Linking │ │ Search  │  │ ║
║  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘  │ ║
║  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐  │ ║
║  │  │Reasoning│ │Adaptive │ │ Memory  │ │  Audit  │ │  Quota  │  │ ║
║  │  │         │ │Learning │ │ Trace   │ │  Log    │ │         │  │ ║
║  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘  │ ║
║  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐  │ ║
║  │  │Multi    │ │  Code   │ │  Fleet  │ │ Mental  │ │ Schema  │  │ ║
║  │  │Tenant   │ │Sandbox  │ │         │ │ Model   │ │Evolution│  │ ║
║  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘  │ ║
║  └─────────────────────────────────────────────────────────────────┘ ║
║                              │                                         ║
║                              ▼                                         ║
║  ┌─────────────────────────────────────────────────────────────────┐ ║
║  │                    MemoryManager (8种记忆)                        │ ║
║  │  Episodic │ Semantic │ Procedural │ Working │ Core │ Knowledge │  │ ║
║  └─────────────────────────────────────────────────────────────────┘ ║
║                              │                                         ║
║                              ▼                                         ║
║  ┌─────────────────────────────────────────────────────────────────┐ ║
║  │                       Search Engine                               │ ║
║  │    Vector │ BM25 │ Hybrid │ Adaptive │ Graph │ Neural           │  ║
║  └─────────────────────────────────────────────────────────────────┘ ║
║                              │                                         ║
║                              ▼                                         ║
║  ┌─────────────────────────────────────────────────────────────────┐ ║
║  │                        Storage Layer                             │ ║
║  │    LanceDB │ Qdrant │ Redis │ PostgreSQL │ S3                   │  ║
║  └─────────────────────────────────────────────────────────────────┘ ║
║                                                                       ║
╚══════════════════════════════════════════════════════════════════════╝
```

## 🔍 与竞品对比

| 特性 | AgentMem v4 | Mem0 | Letta | Agno |
|------|-------------|------|-------|------|
| 记忆类型 | 8 种 | 4 种 | 3 种 | 5 种 |
| 意图理解 | ✅ | ✅ | ❌ | ❌ |
| 多信号检索 | ✅ | ✅ | ❌ | ❌ |
| 实体链接 | ✅ | ✅ | ❌ | ❌ |
| 因果推理 | ✅ | ❌ | ❌ | ❌ |
| 自适应学习 | ✅ | ✅ | ❌ | ❌ |
| 代码沙箱 | ✅ | ❌ | ✅ | ❌ |
| Fleet 管理 | ✅ | ❌ | ❌ | ✅ |
| 多租户 | ✅ | ✅ | ✅ | ✅ |
| 去中心化 | ✅ | ❌ | ❌ | ❌ |
| 开源 | ✅ | 部分 | ✅ | ✅ |

## 📦 模块结构

```
agentmem/
├── crates/
│   ├── agent-mem/           # 统一 API (V4Api)
│   ├── agent-mem-core/      # 核心引擎
│   ├── agent-mem-traits/    # Trait 定义
│   ├── agent-mem-llm/       # LLM 提供商
│   ├── agent-mem-storage/   # 存储层
│   ├── agent-mem-embeddings/ # Embedding 服务
│   ├── agent-mem-intelligence/ # 智能功能
│   ├── agent-mem-config/    # 配置管理
│   ├── agent-mem-utils/     # 工具函数
│   └── agent-mem-server/    # REST API 服务器
├── examples/
│   └── v4-api-demo/        # V4 API 演示
├── benches/
│   └── v4_api_benchmark.rs # 基准测试
└── V4_API.md               # API 文档
```

## 🧪 测试

```bash
# 运行所有测试
cargo test --workspace

# 运行 V4 API 测试
cargo test --package agent-mem v4_api

# 运行基准测试
cargo bench --package agent-mem --bench v4_api_benchmark
```

## 📈 性能

| 操作 | 延迟 | 吞吐量 |
|------|------|--------|
| 记忆创建 | < 10ms | 10K/s |
| 意图理解 | < 50ms | 1K/s |
| 多信号搜索 | < 100ms | 500/s |
| 实体链接 | < 30ms | 2K/s |

## 📚 文档

- [V4 API 文档](./crates/agent-mem/V4_API.md)
- [基准测试](./benches/README.md)
- [示例代码](./examples/v4-api-demo/)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

Apache 2.0

---

**AgentMem v4.0** - 让 AI Agent 拥有真正的记忆能力 🚀
