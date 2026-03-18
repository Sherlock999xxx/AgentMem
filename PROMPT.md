# plan1.1.1：基于 `mem111.md` 的 AgentMem file-centric 穿透实施计划

> 日期：2026-03-18
> 输入依据：`mem111.md`、`PROMPT.md`、当前仓库公开代码表面抽样
> 计划范围：把已经存在的 `resource/category/extraction/proactive` 能力穿透到 Rust 顶层 API、server/client 协议、8 个 agents 协作主链路和多语言 SDK

## 1. 计划目标

本计划不是重新发明新的底层 crate，而是完成下面这件事：

> 把已经实现的 file-centric 基础设施，收敛成用户可直接感知、可迁移、可观测的默认平台体验。

本轮计划的直接目标有四个：

1. 统一公共模型，让 `Resource / Category / Extraction / Migration / Proactive` 成为一等平台语言。
2. 把现有 agent 协作从 `MemoryType` 主轴逐步切换为 `resource -> extraction -> category -> retrieval -> proactive` 主链路。
3. 让 server、Rust client 和多语言 SDK 共享同一套合同，而不是各自维护一套 memory CRUD 语义。
4. 为 legacy `MemoryItem / MemoryType` 保留兼容层，但把默认文档和新入口切换到 file-centric surface。

## 2. 当前代码基线

下列判断直接来自当前仓库代码，不是抽象推测：

| 层面 | 代码证据 | 当前状态 | 结论 |
|---|---|---|---|
| Rust 顶层 API | `crates/agent-mem/src/lib.rs` | 快速开始仍围绕 `Memory::add()` / `Memory::search()`，并继续导出 `MemoryItem` / `MemoryType` | 顶层 facade 仍是 legacy-first |
| Specialized agents | `crates/agent-mem-core/src/agents/mod.rs` | 8 个 agents 仍按 `MemoryType` 分工 | 主链路还没切到 resource/category |
| Server DTO | `crates/agent-mem-server/src/models.rs` | 只有 `MemoryRequest` / `SearchRequest` 等 memory CRUD 模型 | 协议层没有 file-centric 一等对象 |
| Rust client DTO | `crates/agent-mem-client/src/models.rs` | 仍是 `AddMemoryRequest` / `SearchMemoriesRequest` | 客户端合同仍旧模型优先 |
| Python SDK | `sdks/python/agentmem/types.py` | 只公开 `MemoryType`、`Memory`、`SearchQuery` | 适合当 Beta 先行层，但当前仍是 legacy-only |
| JavaScript SDK | `sdks/javascript/src/types.ts` | 以 `CreateMemoryParams` 和 `SearchQuery` 为中心 | 需要跟随 server 合同一起升级 |
| Go SDK | `sdks/go/types.go` | 强类型 DTO 仍围绕 `MemoryType` | 更适合在合同稳定后做收口验证 |
| 仓颉 HTTP SDK | `sdks/cangjie/src/http_new/memory.cj` | 仍只暴露 memory CRUD，搜索解析也较简化 | 应放在最后一波对齐 |

## 3. 规划原则

1. 先统一公共合同，再迁移 SDK。
2. 先做 dual-surface，不做一次性替换。
3. 旧接口可继续保留至少一个次版本周期，但默认文档必须转向 file-centric API。
4. SDK 迁移必须 contract-first，并复用共享 fixtures。
5. Proactive 不再作为孤立 crate 演进，必须接到资源摄取、提取完成和检索闭环。
6. 旧的 umbrella 任务 `task-1772345012-d328` 不再作为一个实现单元推进，应拆成阶段任务执行。

## 4. 阶段路线图

整体建议按 6 个阶段推进，预计覆盖当前剩余改造缺口的 6 到 9 周。

### 阶段 A：统一公共模型

目标：先让所有平台表面说同一套 file-centric 语言。

核心产出：

- 稳定 `ResourceDescriptor`
- 稳定 `CategoryDescriptor`
- 稳定 `ExtractionRequest / ExtractionResult`
- 稳定 `MigrationPlan / MigrationReport`
- 稳定 `ProactiveTaskInfo / SchedulerStats`
- 为这些模型生成共享 OpenAPI 或 JSON Schema 合同

优先改动面：

- `crates/agent-mem/src/`
- `crates/agent-mem-client/src/models.rs`
- `crates/agent-mem-server/src/models.rs`
- `docs/` 下新增合同说明和迁移指南

验收标准：

- Rust 顶层 API 能公开 file-centric 类型而不破坏现有 `MemoryItem / MemoryType`
- server 和 Rust client DTO 对同一套 file-centric 字段达成一致
- 共享合同可被 Python/JavaScript/Go/仓颉 SDK 消费

### 阶段 B：重构 agent 协作主链路

目标：把“资源进入系统后的默认路径”从 memory CRUD 变成 file-centric 主链路。

重点改造：

1. `ResourceAgent` 从并列 agent 升级为资源挂载和预处理入口。
2. `SemanticAgent` / `ProceduralAgent` 直接消费 extraction 输出和 category 上下文。
3. `KnowledgeAgent` / `ContextualAgent` 接入 category-aware retrieval。
4. retrieval router 从 `MemoryType` 映射转向 `resource/category` 感知调度。

优先改动面：

- `crates/agent-mem-core/src/agents/`
- `crates/agent-mem-core/src/retrieval/`
- `crates/agent-mem-core/src/orchestrator/`

验收标准：

- 至少一条资源摄取路径默认走 `mount -> extract -> categorize -> store`
- 检索入口能显式消费 category/resource 上下文
- `MemoryType` 不再是唯一的 agent 路由键

### 阶段 C：把 server / client / Rust unified API 升级为 dual-surface

目标：在不破坏旧接口的前提下，让 file-centric surface 成为平台默认入口。

新增公共接口建议：

- `mount_resource`
- `get_resource`
- `extract_resource`
- `list_categories`
- `search_categories`
- `plan_legacy_migration`
- `apply_legacy_migration`
- `rollback_migration`
- `list_proactive_tasks`
- `run_proactive_task`
- `cancel_proactive_task`
- `get_scheduler_stats`

兼容策略：

- 保留 `add_memory / search_memories` 等 legacy surface
- 旧接口在可行时内部复用新合同
- README、示例和 API 文档以 file-centric 用法为主，legacy API 放入兼容章节

验收标准：

- server 路由、Rust client 和顶层 `agent-mem` API 均能完成同一组 file-centric 示例
- legacy surface 仍可用
- 文档主叙事完成切换

### 阶段 D：按波次迁移 SDK

目标：在稳定合同基础上，把多语言 SDK 从 memory CRUD 升级到 file-centric surface。

#### D0：冻结跨语言合同

产出：

- 共享 DTO 字段基线
- 长任务状态模型：`pending / running / succeeded / failed / cancelled`
- 错误码基线：参数错误、分类不存在、迁移冲突、任务超时、后台任务不可用
- 共享 contract fixtures

#### D1：Python + JavaScript Beta 先行

原因：

- Python 最适合快速验证抽象是否顺手
- JavaScript 最适合验证 REST surface 是否适合前端和 runtime

最低能力面：

- 数据模型：`Resource`、`Category`、`ExtractionJob`、`MigrationPlan`、`MigrationReport`、`ProactiveTask`
- 同步接口：`mount_resource`、`get_resource`、`create_category`、`list_categories`、`search_categories`
- 异步接口：`extract_resource`、`run_proactive_task`、`cancel_proactive_task`
- 迁移接口：`plan_legacy_migration`、`apply_legacy_migration`、`rollback_migration`
- 观测接口：`get_scheduler_stats`、`get_migration_status`

#### D2：Go 稳定化收口

目标：

- 用强类型结构体验证 DTO 是否已经稳定
- 验证长任务轮询和取消语义
- 验证迁移报告和错误码是否适合服务端集成

#### D3：仓颉最终对齐

目标：

- 消费已经稳定的 HTTP 合同
- 补齐资源、类别、迁移、后台任务最小可用表面
- 用较少但完整的 E2E 示例保证功能对等

阶段 D 验收标准：

- 四套 SDK 均能完成资源挂载 -> 提取 -> 分类 -> 检索 -> 主动任务的共享示例
- 四套 SDK 均支持 migration dry-run 并返回结构化报告
- 四套 SDK 共享同一套 contract fixtures 和任务状态语义

### 阶段 E：补齐迁移工具和回归验证

目标：保证 legacy 数据能安全迁移，而不是只支持新项目。

必需能力：

- dry-run
- 结构化迁移报告
- 回滚
- 样本对比校验
- 检索质量回归

最小验证矩阵：

- 单用户 / 多用户
- 小数据集 / 大数据集
- 含资源附件 / 不含资源附件
- 含层级类别 / 无类别历史数据

验收标准：

- 迁移失败可回滚
- 迁移前后关键搜索结果和资源可追溯性可比对
- 回归测试能够覆盖 legacy-only、dual-surface、file-centric-first 三种模式

### 阶段 F：让 Proactive 成为平台默认后台平面

目标：把 `agent-mem-proactive` 从“有骨架的子系统”升级为平台默认后台平面。

核心工作：

- 对接 `agent-mem-event-bus`
- 资源挂载后自动触发提取
- 提取完成后自动分类
- 定期摘要刷新和去重整理
- server / SDK 暴露任务观测和任务控制能力

验收标准：

- 资源进入系统后可自动触发后台整理
- Proactive 结果能反哺检索和上下文构建
- 平台具备任务观测、取消和健康状态接口

## 5. 推荐拆分为原子任务的执行顺序

下面的任务粒度适合后续 Ralph 循环逐个关闭：

1. `contracts:file-centric-dto-spec`
   产出跨语言 DTO 字段基线和状态/错误码合同。
2. `rust:public-dual-surface-models`
   为 `agent-mem`、server、client 引入 file-centric DTO 和新入口。
3. `core:resource-first-ingest-path`
   把资源挂载到提取和分类链路串起来。
4. `core:category-aware-routing`
   让 retrieval router 和 agent registry 脱离 `MemoryType` 唯一路由。
5. `sdk:python-beta-file-centric`
   先在 Python 验证接口可用性和迁移体验。
6. `sdk:javascript-beta-file-centric`
   跟随共享合同验证 REST 和长任务语义。
7. `sdk:go-stabilization`
   在合同趋稳后做类型收敛。
8. `sdk:cangjie-parity`
   在 HTTP 合同稳定后做最终对齐。
9. `migration:dry-run-and-rollback`
   建立 legacy 迁移与回滚链路。
10. `proactive:platform-default-integration`
    将后台整理能力纳入平台默认平面。

## 6. 验证策略

每个阶段都必须满足 backpressure 约束，不能只完成代码合并而缺少真实验证。

### 合同层验证

- 共享 JSON fixtures 验证 DTO 兼容性
- OpenAPI/Schema 快照测试
- 错误码和长任务状态的一致性测试

### Rust 平台验证

- `cargo test` 覆盖 `agent-mem`、`agent-mem-client`、`agent-mem-server`、相关 core 模块
- 至少一组资源挂载 -> 提取 -> 分类 -> 检索 E2E 测试
- 至少一组 legacy surface 回归测试

### SDK 验证

- Python/JavaScript/Go/仓颉消费共享 fixtures
- 每套 SDK 至少保留一组 adversarial case：
  - 分类不存在
  - 资源 URI 冲突
  - 迁移冲突
  - 长任务取消

### 迁移与主动代理验证

- migration dry-run 与 rollback
- proactive 自动分类和摘要刷新结果检查
- scheduler 任务状态和错误传播检查

## 7. 风险与约束

1. 最大风险不是底层能力不足，而是对外模型继续分裂。
2. 如果不先冻结合同，四套 SDK 会各自漂移并反复返工。
3. 如果不保留 dual-surface，现有用户将承受不必要的破坏式升级。
4. 如果不做 migration dry-run 和 rollback，file-centric 改造无法安全进入已有部署。
5. 如果 Proactive 不接进主链路，平台仍会停留在“新增 crate 已存在，但默认体验没变化”的中间态。

## 8. 本计划的首要执行建议

如果下一轮只能先做一件事，应先完成下面这个原子任务：

> 冻结 file-centric 跨语言公共合同，并以 server + Rust client 为第一批实现对象。

原因很直接：

- 这是 SDK 迁移和 agent 主链路重构的共同依赖；
- 这是把 `mem111.md` 的“公共表面尚未穿透”结论转化为可执行工作的最短路径；
- 这是当前最能降低返工率的一步。
