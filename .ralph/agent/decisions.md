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
