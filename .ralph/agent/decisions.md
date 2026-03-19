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

## DEC-006
- Decision: `task-1773924455-9358` 是否应直接提交当前 JavaScript / Go / 仓颉 file-centric SDK 改动
- Chosen Option: 不提交，先将本任务标记为 blocked/failed，并为下一轮创建“对齐 preview server route contract 与 SDK surface”的原子任务
- Confidence: 79
- Alternatives Considered: 1) 直接按当前改动提交，接受 SDK 先于 server 的 route 漂移 2) 在本轮同时大改 Rust server/client 路由以追平 18 个 SDK 方法 3) 仅修复仓颉语法/编译问题后提交剩余 SDK 改动
- Reasoning: 代码实证表明当前 Rust preview surface 只暴露 `/api/v1/resources/*`、`/api/v1/categories/*`、`/api/v1/migrations/*`、`/api/v1/proactive/*` 的子集；而待提交 SDK 改动普遍假设 `/api/v1/file-centric/*` 路由，并暴露 `get_category_by_path`、`get_migration_status`、`get_proactive_task` 等 server 当前不存在的接口。此时提交会把跨语言 SDK 固化到一个并不存在的公共合同上，后续返工成本更高。先把阻塞显式化，再拆出 route/contract 对齐任务，风险更低。
- Reversibility: 高。下一轮既可以扩 server 追平 SDK 合同，也可以收缩 SDK 到当前 preview surface；本次保留未提交状态不会扩大用户影响面。
- Timestamp (UTC ISO 8601): 2026-03-19T14:15:00Z

## DEC-007
- Decision: `task-1773924797-863f` 的 route-contract 对齐是直接替换旧 preview 路径，还是叠加新的 canonical file-centric 路由层
- Chosen Option: 保留现有 `/api/v1/resources|categories|migrations|proactive/*` preview 路由不变，并新增 `/api/v1/file-centric/*` canonical 路由与 collection-style 响应 envelope，缺失的 get/status 端点以轻量 stub 或现有 handler 复用方式补齐
- Confidence: 78
- Alternatives Considered: 1) 直接把现有 preview 路由整体重命名为 `/api/v1/file-centric/*` 2) 只修 SDK，不扩 server 3) 一次性把所有 SDK 分支差异路径也全部纳入 server 兼容层
- Reasoning: 现有 Rust client 和已有 preview 测试仍依赖未加前缀的路径，直接替换会制造不必要的回归；而完全不扩 server 会继续阻塞已经进入 SDK wave 的 file-centric surface。叠加 canonical 路由层可以用最小改动把 Python/JS 目标合同落到真实 server 上，同时把旧 preview surface 继续保留为兼容层。对 Go/Cangjie 的个别路径偏差，后续再在各 SDK 内收敛更稳妥。
- Reversibility: 高。后续可在文档和客户端完成迁移后逐步废弃旧 preview 路由，或继续补充少量 alias，而不影响已新增的 canonical surface。
- Timestamp (UTC ISO 8601): 2026-03-19T15:05:00Z

## DEC-008
- Decision: `task-1773924797-9514` 中 `http_new` 包重复定义 `ExtractionRequest` 时，是否通过重命名 API helper 保持旧签名，还是统一到已存在的 file-centric `ExtractionRequest`
- Chosen Option: 删除 `api.cj` 中重复的 helper 定义，并让 `FileCentricApi.extractResource` 直接消费 `file_centric.cj` 里已有的 `ExtractionRequest`
- Confidence: 74
- Alternatives Considered: 1) 把 `api.cj` 的 helper 重命名为另一个请求类型，仅为通过编译保留旧字段形状 2) 暂时移除 `extractResource` API，等后续 parity 任务再补回 3) 同时大改整个仓颉 file-centric DTO 以完全追平其它 SDK
- Reasoning: 当前任务目标是恢复 `http_new` 包对现有 `cjc` 的可编译性，而不是重新设计整个 Cangjie SDK。保留两个同名请求类型会继续阻断编译，也会让公共表面更分裂。直接统一到现有 file-centric `ExtractionRequest` 至少保证“一个概念一个类型”，并把改动范围控制在当前包内；如果后续还需调整字段与路由合同，可以在此基础上继续收敛，而不必先处理命名冲突。
- Reversibility: 高。后续可以继续演进 `ExtractionRequest` 字段或为 `extractResource` 增加适配层，但不需要再处理重复类型冲突。
- Timestamp (UTC ISO 8601): 2026-03-19T16:10:00Z
