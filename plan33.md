# AgentMem 核心AI记忆融合架构与发展计划 v6.0

**日期**: 2026-05-24
**版本**: v6.0 (聚焦版)
**目标**: 打造精简高效的AI Agent记忆平台，保持高内聚低耦合

---

## 一、核心融合架构

### 1.1 架构设计原则

```
┌─────────────────────────────────────────────────────────────────┐
│                    核心设计原则                                   │
├─────────────────────────────────────────────────────────────────┤
│ 1. 高内聚: 每个模块职责单一，聚焦核心功能                       │
│ 2. 低耦合: 模块间通过Trait接口交互                             │
│ 3. 可组合: 核心模块可灵活组合                                  │
│ 4. 可测试: 每个模块独立可测试                                   │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 核心模块融合

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                         AgentMem 核心融合架构 (精简版)                                   │
├────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                        │
│  ┌──────────────────────────────────────────────────────────────────────────────┐      │
│  │                          对话入口 (ChatRequest)                            │      │
│  │                          Orchestrator                                       │      │
│  │                          ┌─────────────────────────────────────────┐        │      │
│  │                          │ Memory Integration (记忆融合)              │        │      │
│  │                          │ ┌───────────┬───────────┬───────────┐ │        │      │
│  │                          │ │  CoreMem │ Contextual│  Episodic│ │        │      │
│  │                          │ │  (身份)   │  (上下文)  │  (事件)  │ │        │      │
│  │                          │ └───────────┴───────────┴───────────┘ │        │      │
│  │                          │ ┌───────────┬───────────┬───────────┐ │        │      │
│  │                          │ │SemanticMem│Procedural│ Resource │ │        │      │
│  │                          │ │  (语义)   │  (程序)   │  (资源)   │ │        │      │
│  │                          │ └───────────┴───────────┴───────────┘ │        │      │
│  │                          └─────────────────────────────────────────┘        │      │
│  └──────────────────────────────────────────────────────────────────────────────┘      │
│                                          │                                            │
│                                          ▼                                            │
│  ┌──────────────────────────────────────────────────────────────────────────────┐      │
│  │                         搜索融合层 (Search Fusion)                            │      │
│  │                         EnhancedHybridSearchEngineV2                            │      │
│  │                         ┌───────────┬───────────┬───────────┐                 │      │
│  │                         │  Vector   │   BM25    │   RRF    │                 │      │
│  │                         │  (语义)    │  (关键词) │  (融合)  │                 │      │
│  │                         └───────────┴───────────┴───────────┘                 │      │
│  └──────────────────────────────────────────────────────────────────────────────┘      │
│                                          │                                            │
│                                          ▼                                            │
│  ┌──────────────────────────────────────────────────────────────────────────────┐      │
│  │                         存储层 (Storage)                                    │      │
│  │                         UnifiedStorageCoordinator                             │      │
│  │                         ┌───────────────┬───────────────┐                    │      │
│  │                         │    LibSQL     │   LanceDB     │                    │      │
│  │                         │  (结构化)      │   (向量)       │                    │      │
│  │                         └───────────────┴───────────────┘                    │      │
│  └──────────────────────────────────────────────────────────────────────────────┘      │
│                                                                                        │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

### 1.3 核心模块职责

| 模块 | 职责 | 内聚度 | 耦合度 |
|------|------|--------|--------|
| **Orchestrator** | 对话编排，记忆提取，响应生成 | ⭐⭐⭐⭐⭐ | 低 |
| **MemoryEngine** | 记忆管理，生命周期，层级管理 | ⭐⭐⭐⭐⭐ | 低 |
| **8 Cognitive Agents** | 分类型处理记忆 (Core/Contextual/Episodic等) | ⭐⭐⭐⭐⭐ | 中 |
| **EnhancedSearchV2** | 混合搜索，Query分类，自适应阈值 | ⭐⭐⭐⭐ | 中 |
| **StorageCoordinator** | SQL+向量存储，多级缓存 | ⭐⭐⭐⭐ | 低 |

---

## 二、当前架构问题

### 2.1 模块耦合问题

```
问题1: MemoryEngine 耦合过重
├── 包含 hierarchy_manager
├── 包含 importance_scorer
├── 包含 conflict_resolver
├── 包含 memory_repository
└── 包含 enhanced_search_engine

问题2: 8种记忆Agent分散
├── episodic_memory.rs
├── semantic_memory.rs
├── procedural_memory.rs
├── contextual_memory.rs
├── core_memory.rs
├── resource_memory.rs
└── knowledge_vault.rs
```

### 2.2 需要精简的模块

| 模块 | 问题 | 优先级 |
|------|------|--------|
| agents/ 模块 | 分散且功能重复 | P0 |
| managers/ 模块 | 大且复杂 | P0 |
| search/ 模块 | 27个子模块过多 | P1 |
| graph_memory | 未激活但存在 | P1 |
| causal_reasoning | 独立未集成 | P2 |
| temporal_reasoning | 独立未集成 | P2 |

---

## 三、精简融合方案

### 3.1 核心融合架构 (目标)

```
目标架构:

Orchestrator
    │
    ├── MemoryEngine (核心引擎)
    │       │
    │       ├── HierarchyManager (层级管理)
    │       ├── ImportanceScorer (重要性评分)
    │       └── ConflictResolver (冲突解决)
    │
    ├── CognitiveMemoryManager (认知记忆融合)
    │       │
    │       ├── CoreMemory (身份/偏好)
    │       ├── ContextualMemory (上下文)
    │       ├── EpisodicMemory (事件)
    │       ├── SemanticMemory (语义)
    │       ├── ProceduralMemory (程序)
    │       └── ResourceMemory (资源)
    │
    ├── EnhancedSearchEngine (增强搜索)
    │       │
    │       ├── VectorSearch (向量)
    │       ├── BM25Search (关键词)
    │       └── RRFusion (融合)
    │
    └── StorageCoordinator (统一存储)
            │
            ├── LibSQLRepository (结构化)
            └── VectorStore (向量)
```

### 3.2 融合步骤

#### Phase 1: 核心融合 (1周)

```
任务:
├── [ ] 融合 CognitiveMemoryManager
│       └── 将8个Manager合并为1个统一管理器
│
├── [ ] 简化 MemoryEngine
│       ├── 提取公共接口
│       └── 减少依赖
│
└── [ ] 清理 Agents 模块
        ├── 保留 BaseAgent Trait
        └── 移除重复实现
```

#### Phase 2: 搜索优化 (1周)

```
任务:
├── [ ] 精简 EnhancedHybridSearchEngineV2
│       ├── 移除未使用的子模块
│       └── 保留核心: Vector + BM25 + RRF
│
├── [ ] 集成 CausalReasoning
│       └── 作为可选的增强模块
│
└── [ ] 集成 TemporalReasoning
        └── 作为可选的增强模块
```

#### Phase 3: 存储优化 (1周)

```
任务:
├── [ ] 简化 StorageCoordinator
│       ├── 统一接口
│       └── 移除冗余缓存
│
├── [ ] 集成 KnowledgeVault
│       └── 作为安全存储模块
│
└── [ ] 集成 Forgetting
        └── 作为自动清理模块
```

---

## 四、精简后的架构

### 4.1 核心模块清单

| 模块 | 行数 | 职责 | 状态 |
|------|------|------|------|
| **Orchestrator** | ~1K | 对话编排 | ✅ 保留 |
| **MemoryEngine** | ~500 | 核心引擎 | 🔄 精简 |
| **CognitiveMemory** | ~2K | 8种记忆融合 | 🆕 新建 |
| **EnhancedSearch** | ~500 | 混合搜索 | 🔄 精简 |
| **StorageCoordinator** | ~300 | 统一存储 | 🔄 精简 |
| **CoreMemory** | ~25K | 核心记忆 | ✅ 保留 |
| **ContextualMemory** | ~48K | 上下文 | ✅ 保留 |

### 4.2 移除/合并的模块

| 模块 | 操作 | 原因 |
|------|------|------|
| agents/ | 合并到CognitiveMemory | 职责重叠 |
| managers/ episodic | 合并到CognitiveMemory | 功能重复 |
| managers/ semantic | 合并到CognitiveMemory | 功能重复 |
| managers/ procedural | 合并到CognitiveMemory | 功能重复 |
| managers/ knowledge_vault | 保留为独立模块 | 安全相关 |
| managers/ resource | 合并到CognitiveMemory | 功能重复 |
| search/ adaptive_router | 移除 | 未使用 |
| search/ cached_adaptive | 移除 | 未使用 |
| search/ learning | 合并到AdaptiveSearch | 功能重叠 |

---

## 五、融合后的接口设计

### 5.1 CognitiveMemory Trait

```rust
/// 认知记忆融合管理器
pub trait CognitiveMemory: Send + Sync {
    /// 添加记忆
    async fn add(&self, memory: Memory) -> Result<String>;
    
    /// 获取记忆
    async fn get(&self, id: &str) -> Result<Option<Memory>>;
    
    /// 更新记忆
    async fn update(&self, memory: Memory) -> Result<Memory>;
    
    /// 删除记忆
    async fn delete(&self, id: &str) -> Result<bool>;
    
    /// 搜索记忆
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<Memory>>;
    
    /// 获取特定类型记忆
    async fn get_by_type(&self, memory_type: MemoryType) -> Result<Vec<Memory>>;
}
```

### 5.2 MemoryEngine 接口

```rust
/// 精简后的核心引擎
pub struct MemoryEngine {
    cognitive_memory: Arc<dyn CognitiveMemory>,
    hierarchy_manager: Arc<dyn HierarchyManager>,
    importance_scorer: Arc<dyn ImportanceScorer>,
    search_engine: Arc<dyn EnhancedSearch>,
    storage: Arc<StorageCoordinator>,
}

impl MemoryEngine {
    /// 创建引擎
    pub fn new(config: EngineConfig) -> Self { ... }
    
    /// 添加记忆 (自动分类到合适类型)
    pub async fn add(&self, content: &str) -> Result<String> { ... }
    
    /// 智能搜索 (融合所有记忆类型)
    pub async fn search(&self, query: &str) -> Result<SearchResponse> { ... }
    
    /// 获取记忆上下文 (为Agent提供上下文)
    pub async fn get_context(&self, session_id: &str) -> Result<AgentContext> { ... }
}
```

---

## 六、实施计划

### 6.1 Week 1: 核心融合

```
Day 1-2:
├── [ ] 设计 CognitiveMemory Trait
├── [ ] 实现 CognitiveMemoryManager
└── [ ] 迁移 CoreMemory 功能

Day 3-4:
├── [ ] 迁移 ContextualMemory 功能
├── [ ] 迁移 EpisodicMemory 功能
└── [ ] 迁移 SemanticMemory 功能

Day 5:
├── [ ] 迁移 ProceduralMemory 功能
├── [ ] 迁移 ResourceMemory 功能
└── [ ] 编写迁移测试
```

### 6.2 Week 2: 搜索优化

```
Day 1-2:
├── [ ] 精简 EnhancedHybridSearchEngineV2
├── [ ] 移除未使用子模块
└── [ ] 保留核心: Vector + BM25 + RRF

Day 3-4:
├── [ ] 集成 QueryClassifier
├── [ ] 集成 AdaptiveThreshold
└── [ ] 性能测试

Day 5:
├── [ ] 可选: 集成 CausalReasoning
└── [ ] 可选: 集成 TemporalReasoning
```

### 6.3 Week 3: 存储与测试

```
Day 1-2:
├── [ ] 简化 StorageCoordinator
├── [ ] 集成 KnowledgeVault
└── [ ] 集成 Forgetting

Day 3-4:
├── [ ] 端到端测试
├── [ ] 性能基准测试
└── [ ] 文档更新

Day 5:
├── [ ] 清理旧代码
├── [ ] 发布 v7.0
└── [ ] 提交PR
```

---

## 七、验证指标

### 7.1 架构指标

| 指标 | 当前 | 目标 | 状态 |
|------|------|------|------|
| 模块数量 | 31 Crates | 20 Crates | 🔄 |
| 核心耦合 | 高 | 低 | 🔄 |
| 代码行数 | ~315K | ~200K | 🔄 |
| 接口一致性 | 低 | 高 | 🔄 |

### 7.2 功能指标

| 指标 | 当前 | 目标 | Mem0 |
|------|------|------|------|
| Precision@K | 85% | 92% | 90% |
| Recall@K | 80% | 88% | 85% |
| P95延迟 | 200ms | 120ms | 150ms |
| QPS | 600 | 800 | 800 |

### 7.3 代码质量

| 指标 | 当前 | 目标 |
|------|------|------|
| 测试覆盖率 | 60% | 80% |
| 文档覆盖率 | 50% | 80% |
| 编译警告 | 22个 | 0个 |

---

## 八、行动清单

### 立即行动 (Day 1)

- [ ] 创建 CognitiveMemory Trait 设计文档
- [ ] 设计模块合并计划
- [ ] 创建分支 feature/fusion-v6

### Week 1 行动

- [ ] 实现 CognitiveMemoryManager
- [ ] 迁移 8 种记忆类型
- [ ] 编写单元测试

### Week 2 行动

- [ ] 精简 EnhancedSearchEngineV2
- [ ] 性能测试
- [ ] 集成可选推理模块

### Week 3 行动

- [ ] 简化存储层
- [ ] 端到端测试
- [ ] 发布 v7.0

---

## 九、架构对比

### 9.1 当前 vs 目标

```
当前架构 (问题):
┌────────────────────────────────────────────────────────────┐
│ 31 Crates → 功能分散                                      │
│ 8个独立Manager → 耦合高                                    │
│ 27个搜索子模块 → 过于复杂                                    │
│ 多个未激活功能 → 维护负担                                    │
└────────────────────────────────────────────────────────────┘

目标架构 (精简):
┌────────────────────────────────────────────────────────────┐
│ 20 Crates → 职责清晰                                       │
│ CognitiveMemory → 统一记忆管理                                │
│ EnhancedSearch → 核心搜索融合                                 │
│ StorageCoordinator → 统一存储                                 │
└────────────────────────────────────────────────────────────┘
```

### 9.2 与Mem0对比

| 维度 | AgentMem (目标) | Mem0 | 评估 |
|------|-----------------|------|------|
| 记忆类型 | 6种融合 | 4种 | ✅ 领先 |
| 搜索融合 | Vector+BM25+RRF | 向量+关键词 | ✅ 持平 |
| 存储架构 | 统一Coordinator | Qdrant | ✅ 领先 |
| 复杂度 | 精简 | 简单 | 🔄 |

---

## 十、风险与缓解

### 10.1 技术风险

| 风险 | 影响 | 缓解 |
|------|------|------|
| 融合破坏现有功能 | 高 | 充分测试 |
| 性能下降 | 中 | 性能基准测试 |
| 接口不兼容 | 高 | 向后兼容 |

### 10.2 进度风险

| 风险 | 影响 | 缓解 |
|------|------|------|
| 时间不足 | 高 | 优先核心融合 |
| 测试不足 | 中 | 增加测试时间 |

---

**计划版本**: v6.0
**特点**: 聚焦核心、高内聚低耦合、精简可执行
**目标**: 3周内完成融合，发布v7.0
