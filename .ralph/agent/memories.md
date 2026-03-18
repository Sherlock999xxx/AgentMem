# Memories

## Patterns

### mem-1773804355-b9e7
> pattern: when multiple cargo runs contend during Ralph loops, switch verification to a fresh per-task --target-dir instead of waiting on the shared artifact lock
<!-- tags: cargo, testing, ralph | created: 2026-03-18 -->

### mem-1773803537-5f5b
> pattern: when concurrent Ralph loops contend for the workspace target directory, run cargo verification with an isolated --target-dir to avoid artifact-lock stalls
<!-- tags: cargo, testing, ralph | created: 2026-03-18 -->

### mem-1772349435-8f74
> AgentMem 阶段2完成: Category类别层级系统实现完成 (agent-mem-category crate ~2,100行代码)。核心特性: (1)Category数据模型支持path/name/parent_id/children_ids/summary/embedding/item_count, (2)CategoryPath支持层级路径解析和验证(/偏好/沟通/风格), (3)CategoryTreeNode支持树形结构可视化, (4)CategoryManager trait定义完整CRUD操作, (5)InMemoryCategoryManager实现HashMap存储。技术亮点: 自动父类别创建、多租户支持(user_id+agent_id)、38个单元测试全部通过。下一步: 阶段3 Extraction提取管道框架。
<!-- tags: agentmem, category, implementation | created: 2026-03-01 -->

### mem-1772346713-06b6
> AgentMem vs memU 差距: (1)无资源抽象层-直接插入MemoryItem无来源追踪, (2)无层级类别-只能按类型过滤不能按主题浏览, (3)搜索无类别上下文-只能搜索记忆不能搜索类别, (4)无充足度检查-无早期退出机制, (5)无主动代理-无24/7后台整理。
<!-- tags: agentmem, memU, comparison, gap-analysis | created: 2026-03-01 -->

### mem-1772345038-5b5e
> memU retrieval strategy: Two pipelines (retrieve_rag with embedding ranking, retrieve_llm with LLM ranking). 7-stage: route intention → category recall → sufficiency check → item recall → resource recall → sufficiency check → build response. Category-based organization enables hierarchical navigation.
<!-- tags: memU, retrieval, search | created: 2026-03-01 -->

### mem-1772345037-6ac5
> memU ingestion pipeline: 7-stage memorize workflow: ingest_resource → preprocess_multimodal → extract_items → dedupe_merge → categorize_items → persist_index → build_response. Resources mounted first, then extracted into structured memory items with embeddings.
<!-- tags: memU, ingestion, pipeline | created: 2026-03-01 -->

### mem-1772345036-80e3
> memU file-centric philosophy: Memory as file system - Categories=folders (auto-organized topics), MemoryItems=files (facts/preferences/skills), Resources=mount points (conversations/documents/images). Hierarchical navigation, persistent, portable, 24/7 proactive memory agent.
<!-- tags: memU, architecture, design-pattern | created: 2026-03-01 -->

### mem-1772343795-74a3
> Cleanup pattern: Root directory should contain only essential project docs (README, INSTALL, CONTRIBUTING, CHANGELOG, CODE_OF_CONDUCT). Remove intermediate analysis/evaluation files. Archive reports to claudedocs/archived/ following the pattern from logs/archived/
<!-- tags: cleanup, organization, documentation | created: 2026-03-01 -->

### mem-1772342953-503b
> Large-scale cleanup: use claudedocs/archived/ for intermediate reports and analysis files. Keep only essential docs (README, CHANGELOG, CONTRIBUTING, INSTALL) in root. Git recognizes bulk file moves as renames.
<!-- tags: cleanup, organization, git | created: 2026-03-01 -->

### mem-1772342886-1d4f
> Git rename detection automatically identifies moved files when using 'git mv'. Files moved with mv are shown as renames in git status and preserve file history.
<!-- tags: git, version-control | created: 2026-03-01 -->

### mem-1772342777-22f7
> Git rename detection automatically identifies moved files when using 'git mv' or regular 'mv' followed by 'git add'. Files moved with mv are shown as renames in git status and preserve file history.
<!-- tags: git, version-control | created: 2026-03-01 -->

### mem-1772342307-b30a
> Old log files should be archived periodically to keep logs/ directory clean - separate active logs from historical ones in logs/archived/
<!-- tags: cleanup, logging, organization | created: 2026-03-01 -->

### mem-1772342093-83c7
> Project backup files (.bak) should be removed - they clutter the codebase and version control should be used instead
<!-- tags: cleanup, version-control | created: 2026-03-01 -->

## Decisions

### mem-1772348202-3d8c
> PROMPT.md 创建完成: 整合了所有 AgentMem 改造分析（todo3.md 1331行, TODO_CN.md 360行, todo2.md 670行）为简洁的中文开发指南（1177行）。包含9个主要章节: 1)改造目标与愿景, 2)为什么需要改造(memU vs AgentMem对比), 3)技术架构设计(4个新crates详细设计), 4)六阶段实施路线图(14-19周,每周拆解到具体任务), 5)成功指标与风险缓解(7大风险识别), 6)关键决策与理由(6个架构决策), 7)参考文档, 8)下一步行动, 9)联系信息。核心策略: 保留85%代码(高性能引擎、8个专业代理、5种搜索引擎、30+存储后端、20+LLM提供商), 新增4个crates(~5K LOC): agent-mem-resource(资源抽象)、agent-mem-category(类别系统)、agent-mem-extraction(提取管道)、agent-mem-proactive(主动代理), 重构15%代码(MemoryType→Category, 类型分发→类别路由, 5阶段检索→7阶段检索)。采用双API兼容策略确保零破坏性变更。
<!-- tags: agentmem, reform, planning, chinese, architecture, prompt-md | created: 2026-03-01 -->

### mem-1772346712-34a3
> AgentMem 改造分析完成: 创建 todo3.md (1331行中文详细版), 772个Rust文件分析完成, 101K LOC核心引擎, 改造计划6阶段14-19周。核心策略: 保留85%, 新增4个crates(~5K LOC), 重构15%。双API兼容, 渐进交付。
<!-- tags: agentmem, reform, planning, architecture | created: 2026-03-01 -->

### mem-1772345139-b340
> AgentMem review: Conditional approval with Phase 0 validation required. Key findings: (1) Add validation PoC before implementation, (2) Resolve critical decisions: backwards compat (dual model), resource storage (blobs vs refs), multi-tenancy for categories, (3) Enhance testing strategy with regression/migration tests, (4) Performance baseline needed before resource layer. Confidence: 75/100. Strengths: comprehensive analysis, clear vision. Gaps: no validation phase, unresolved decisions.
<!-- tags: agentmem, review, architecture, planning | created: 2026-03-01 -->

### mem-1772345039-1227
> AgentMem reform vision: Transform from type-based to file-centric memory platform. Core changes: (1) Add Resource abstraction (file-like entities with URIs), (2) Implement Category hierarchy (folder-like organization), (3) Build ExtractionPipeline (Resource → MemoryItems), (4) Enhance search with category/resource awareness, (5) Add ProactiveAgent for 24/7 organization.
<!-- tags: agentmem, reform, architecture, planning | created: 2026-03-01 -->

## Fixes

### mem-1773806393-5075
> failure: cmd=/Users/louloulin/.cargo/bin/ralph tools task ensure "Update mem111 with integration gap assessment and SDK migration roadmap" --key analysis:mem111-integration-roadmap -p 2 -d ..., exit=2, error=unrecognized subcommand ensure, next=use ralph tools task add when ensure is unavailable in the current CLI
<!-- tags: tooling, error-handling, ralph | created: 2026-03-18 -->

### mem-1773806042-a4b0
> failure: cmd=/Users/louloulin/.cargo/bin/ralph tools task start task-1772345012-d328, exit=2, error=unrecognized subcommand start, next=use supported task lifecycle commands only and rely on task state files/ready list for progression
<!-- tags: tooling, error-handling, ralph | created: 2026-03-18 -->

### mem-1773805925-bcb0
> fix: after '/Users/louloulin/.cargo/bin/ralph tools task close <id>' reports success, '/Users/louloulin/.cargo/bin/ralph tools task show <id>' may still print stale status; verify the real state via '/Users/louloulin/.cargo/bin/ralph tools task list' or '.ralph/agent/tasks.jsonl' instead
<!-- tags: ralph, tooling, error-handling | created: 2026-03-18 -->

### mem-1773805590-5f91
> failure: cmd=cargo test -p agent-mem-proactive --target-dir /tmp/agentmem-proactive-target-0b4b, exit=101, error=agent-mem-core failed to compile at crates/agent-mem-core/src/storage/coordinator.rs:197 due type annotations needed for Option<_>, next=avoid the unrelated core crate dependency for proactive executors or fix the coordinator inference bug in a separate task
<!-- tags: cargo, testing, agentmem, error-handling | created: 2026-03-18 -->

### mem-1773804560-b2ac
> failure: cmd=/Users/louloulin/.cargo/bin/ralph tools task start task-1772351699-0b4b, exit=2, error=unrecognized subcommand start, next=inspect task CLI help and use supported task lifecycle commands only
<!-- tags: tooling, error-handling, ralph | created: 2026-03-18 -->

### mem-1773803834-29ae
> failure: cmd=/Users/louloulin/.cargo/bin/ralph tools task start task-1772351685-e302, exit=2, error=unrecognized subcommand start, next=inspect task CLI help and use supported task lifecycle commands only
<!-- tags: tooling, error-handling, ralph | created: 2026-03-18 -->

### mem-1773803445-976f
> failure: cmd=cargo test -p agent-mem-proactive, exit=blocked, error=artifact directory lock held by stale cargo pid 12029 from an earlier run, next=terminate the stale cargo process and rerun the active verification
<!-- tags: tooling, error-handling, cargo, testing | created: 2026-03-18 -->

### mem-1773803445-8fb7
> failure: cmd=/Users/louloulin/.cargo/bin/ralph tools interact progress ..., exit=1, error=No bot token configured, next=skip interact progress unless RALPH_TELEGRAM_BOT_TOKEN is configured
<!-- tags: tooling, error-handling, ralph, robot | created: 2026-03-18 -->

### mem-1773803347-3751
> failure: cmd=cargo fmt -p agent-mem-proactive, exit=1, error=rustfmt config parse failed due duplicate use_try_shorthand key, next=run rustfmt with a temporary clean config-path until workspace rustfmt.toml is fixed
<!-- tags: tooling, error-handling, rustfmt | created: 2026-03-18 -->

### mem-1773803317-3a28
> failure: cmd=sed -n '1,220p' .ralph/agent/decisions.md, exit=1, error=.ralph/agent/decisions.md missing, next=create the decision journal only when a <=80 confidence architectural decision must be recorded
<!-- tags: tooling, error-handling, ralph | created: 2026-03-18 -->

### mem-1773802441-b5fc
> failure: cmd=/Users/louloulin/.cargo/bin/ralph tools task start task-1772351678-b1f8, exit=2, error=unrecognized subcommand start, next=inspect task CLI help and use supported task lifecycle commands only
<!-- tags: tooling, error-handling, ralph | created: 2026-03-18 -->

### mem-1773802413-9c24
> failure: cmd=sed -n '1,220p' .ralph/agent/scratchpad.md, exit=1, error=.ralph/agent/scratchpad.md missing, next=recreate scratchpad and append current loop notes
<!-- tags: tooling, error-handling, ralph | created: 2026-03-18 -->

## Context

### mem-1773806361-0c4b
> context: AgentMem workspace now has resource/category/extraction/proactive crates, but public Rust/client/server/SDK surfaces remain MemoryType-first; the main remaining gap is platform-level file-centric model and migration API exposure
<!-- tags: agentmem, migration, sdk, integration | created: 2026-03-18 -->

### mem-1773805894-8178
> context: agent-mem-proactive executors now integrate via SemanticMemoryStore and CategoryManager with injectable handles, covering auto-categorize tree_path writeback, semantic dedupe deletion, and category summary refresh while keeping no-op fallback when backends are absent
<!-- tags: agentmem, proactive, category, integration | created: 2026-03-18 -->

### mem-1773804455-1c31
> context: TaskScheduler 现已消费结构化 TaskSchedule，支持 interval/cron next_run、event queue trigger、batch_window 门控以及运行中后台任务取消；ProactiveAgent 同步暴露 trigger_task/cancel_task 门面
<!-- tags: agentmem, proactive, scheduler | created: 2026-03-18 -->

### mem-1773803686-48eb
> context: agent-mem-proactive now boots through a ProactiveAgent facade that registers built-in executors, loads config-driven schedules, and falls back to six default proactive tasks for initialization
<!-- tags: agentmem, proactive, architecture | created: 2026-03-18 -->

### mem-1772346401-cf41
> AgentMem 文件核心改造分析完成: 创建了 todo3.md (中文代码深度分析版 ~500行)。基于对 772 个 Rust 文件的探索,制定了详细的复用和重构计划: 保留 85% 代码 (101K LOC 核心引擎, 8个专业代理, 5种搜索引擎, 30+ 存储后端, 20+ LLM 集成, 多语言 SDK), 重构 15% 代码 (MemoryType → Category, 类型分发 → 类别路由, 5阶段检索 → 7阶段检索), 新增 4 个 crate (~5K LOC): agent-mem-resource (资源抽象), agent-mem-category (类别系统), agent-mem-extraction (提取管道), agent-mem-proactive (主动代理)。制定了 6 阶段实施路线图 (14-19周), 每阶段拆解到天级别的任务清单。采用双 API 兼容性策略, 确保零破坏性变更。
<!-- tags: agentmem, reform, planning, chinese, code-analysis | created: 2026-03-01 -->

### mem-1772346019-410a
> AgentMem 文件核心改造分析完成: 创建了 todo2.md (670行英文详细版) 和 TODO_CN.md (360行中文完整版),包含6阶段实施路线图(14-19周)。核心改造:Resource资源抽象层、Category类别层级、Extraction提取管道、Enhanced增强检索、Proactive主动代理、Integration集成迁移。保留了AgentMem的高性能引擎(216K ops/sec)、8个专业代理、企业特性和多语言SDK。参考memU的文件系统隐喻、资源抽象、类别组织和主动智能。已创建8个顺序Ralph任务跟踪实施。下一步:等待团队审查批准后启动第一阶段Resource设计。
<!-- tags: agentmem, reform, analysis, memU, planning | created: 2026-03-01 -->

### mem-1772345396-67e4
> AgentMem 文件核心改造完成：创建 TODO_CN.md (中文完整版 360行), todo2.md (英文详细版 670行), agentmem-reform-summary.md (执行摘要)。包含6阶段路线图 (14-19周), 资源层/类别层/提取管道/增强检索/主动代理完整设计, 成功指标和风险缓解。下一步: 等待团队审查批准后启动第一阶段。
<!-- tags: agentmem, reform, planning, chinese | created: 2026-03-01 -->

### mem-1772345039-99fe
> AgentMem architecture gap vs memU: AgentMem lacks (1) Resource abstraction layer - no file-like entity before memory items, (2) Hierarchical categories - only flat type-based organization, (3) Resource recall in search - returns only memory items not sources, (4) Sufficiency checks - no early exit during retrieval.
<!-- tags: agentmem, memU, comparison, architecture | created: 2026-03-01 -->
