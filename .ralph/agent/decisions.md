# Decision Journal

Use this file to record consequential decisions when confidence is 80 or below.

## DEC-001
- Decision: `agent-mem-proactive` 的 `TaskScheduler` 如何在不重写整体架构的前提下补齐 event / batch / cancel 能力
- Chosen Option: 保持现有轮询调度器，扩展 `ScheduledTask` 持久状态，并在调度器内增加事件排队、批处理窗口门控、后台执行取消通道
- Confidence: 78
- Alternatives Considered: 1) 直接引入完整外部 job scheduler 重写执行流 2) 仅补 public API，不补真实调度语义 3) 把取消/触发逻辑下沉到各 executor
- Reasoning: 当前 crate 已经有可用的 scheduler/model 骨架和测试基础，最小增量方案可以复用已有状态机与 facade，同时把缺失的行为收敛到 `scheduler.rs`，避免把调度语义散落到执行器层。代价是批处理窗口和事件语义先以内存态实现，不做跨进程持久化。
- Reversibility: 高。后续如果需要替换为更完整的外部调度后端，可以沿用本次补上的 `trigger_task/cancel_task/schedule_config` 接口和状态字段。
- Timestamp (UTC ISO 8601): 2026-03-18T03:20:00Z

## DEC-002
- Decision: `task-1772351699-0b4b` 中三个 proactive executor 的真实实现是否直接依赖 `agent-mem-core`
- Chosen Option: 保留对 `agent_mem_traits::SemanticMemoryStore` 和 `agent_mem_category::CategoryManager` 的直接集成，但移除对 `agent-mem-core` crate 的编译期依赖，在 `agent-mem-proactive` 内内联轻量的摘要与去重算法
- Confidence: 76
- Alternatives Considered: 1) 继续依赖 `agent-mem-core` 并顺手修复其无关编译错误 2) 暂停本任务并新开 blocker task 先修 core 3) 维持 placeholder executor 不做真实逻辑
- Reasoning: 本轮验证时发现仅为复用 `MemoryDeduplicator` / `MemorySummarizer` 引入 `agent-mem-core` 会触发 `crates/agent-mem-core/src/storage/coordinator.rs` 的既有编译错误，阻断 `agent-mem-proactive` 的独立构建。当前任务的关键交付是让 proactive executor 真正接到现有 memory/category 抽象并可通过测试，因此优先保证本 crate 可编译、可验证；去重与摘要算法本身较小，内联实现的风险可控。
- Reversibility: 中高。后续若 `agent-mem-core` 编译问题修复，可以把内联 helper 替换回核心库实现，而不影响 executor 的 store/category manager 注入接口。
- Timestamp (UTC ISO 8601): 2026-03-18T04:10:00Z

## DEC-003
- Decision: file-centric 合同冻结阶段是否直接公开底层 `resource/category/proactive` crate 的内部结构
- Chosen Option: 先在 server/client 侧定义独立的 API-facing DTO，并用共享 fixtures 冻结字段基线，而不是直接重导出底层 crate 结构
- Confidence: 77
- Alternatives Considered: 1) 直接把底层 crate 类型提升为公共合同 2) 只写文档，不在 Rust server/client 中落地真实模型 3) 直接同时改顶层 facade、server、client 和 SDK
- Reasoning: 当前底层 crate 的结构主要面向内部实现，字段命名、状态值和多租户语义还没有经过跨语言合同收敛。先在 server/client 定义独立 DTO 可以冻结外部语义，减少对内部实现细节的泄漏，也避免为了重用类型额外引入耦合和依赖扩散。
- Reversibility: 高。后续如果内部类型稳定，可以为这些 API DTO 增加 `From/TryFrom` 适配，甚至逐步合并实现，但不会破坏已冻结的外部合同。
- Timestamp (UTC ISO 8601): 2026-03-18T10:11:00Z

## DEC-004
- Decision: `task-1773831045-6d1e` 的 dual-surface 入口是否先接临时内存实现，还是先发布 typed preview surface
- Chosen Option: 先在 `agent-mem` / server / client 三层引入 typed preview entrypoints，并让 server 返回明确的 `501 Not Implemented`、Rust facade 返回 `UnsupportedOperation`，等待下一任务把后端链路接到真实的 `resource -> extract -> categorize`
- Confidence: 79
- Alternatives Considered: 1) 直接在 server 内接一套临时 in-memory resource/category manager 伪实现 2) 继续只保留 DTO，不新增真实入口 3) 一次性把入口和 ingest 主链路同时做完
- Reasoning: 当前下一个 ready task 已经专门负责把 ingest 主链路接通。如果本轮为了"看起来可用"临时接一套独立 in-memory backend，会制造和真实持久化/编排链路不一致的行为，反而增加返工和歧义。先冻结路径、方法名、请求响应类型和错误语义，可以让 Rust/server/client 公开表面同步到位，同时把未完成的后端状态显式暴露出来。
- Reversibility: 高。下一轮只需要替换 handler/facade 内部实现，不需要再改外部路径、方法签名和客户端调用方式。
- Timestamp (UTC ISO 8601): 2026-03-18T11:34:00Z

## DEC-005
- Decision: `task-1773831045-7cb2` file-centric routes 如何接通后端 manager
- Chosen Option: 在 server 内新建 `FileCentricState` struct，持有 `Arc<dyn ResourceManagerTrait>`、`Arc<InMemoryCategoryManager>`、`Arc<RwLock<Option<ExtractionPipeline>>>`，并在 router 初始化时注入为 Extension layer
- Confidence: 78
- Alternatives Considered: 1) 把 resource/category/extraction crate 的类型直接提升为 public API 2) 用 trait object (`Arc<dyn Trait>`) 封装 manager 3) 每个 handler 内直接 new 一个 manager 实例
- Reasoning: 当前 resource/category/extraction crate 的内部结构尚未经过外部 API 收敛，使用 trait object 可以解耦接口，后续如需替换实现（如从 in-memory 到持久化）不影响 handler 签名。`InMemoryCategoryManager` 已有完整的 trait 实现，直接持有即可，无需额外包装。选择 RwLock 包裹 Option<ExtractionPipeline> 是因为 pipeline 可能未配置，用 `None` 表示 stub 行为。
- Reversibility: 高。后续可以替换 State 内部的 manager 实现，或改为持有 `Arc<dyn Trait + Send + Sync>` 统一接口。
- Timestamp (UTC ISO 8601): 2026-03-19T00:57:00Z
