# Memories

## Patterns

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

### mem-1772345139-b340
> AgentMem review: Conditional approval with Phase 0 validation required. Key findings: (1) Add validation PoC before implementation, (2) Resolve critical decisions: backwards compat (dual model), resource storage (blobs vs refs), multi-tenancy for categories, (3) Enhance testing strategy with regression/migration tests, (4) Performance baseline needed before resource layer. Confidence: 75/100. Strengths: comprehensive analysis, clear vision. Gaps: no validation phase, unresolved decisions.
<!-- tags: agentmem, review, architecture, planning | created: 2026-03-01 -->

### mem-1772345039-1227
> AgentMem reform vision: Transform from type-based to file-centric memory platform. Core changes: (1) Add Resource abstraction (file-like entities with URIs), (2) Implement Category hierarchy (folder-like organization), (3) Build ExtractionPipeline (Resource → MemoryItems), (4) Enhance search with category/resource awareness, (5) Add ProactiveAgent for 24/7 organization.
<!-- tags: agentmem, reform, architecture, planning | created: 2026-03-01 -->

## Fixes

## Context

### mem-1772346019-410a
> AgentMem 文件核心改造分析完成: 创建了 todo2.md (670行英文详细版) 和 TODO_CN.md (360行中文完整版),包含6阶段实施路线图(14-19周)。核心改造:Resource资源抽象层、Category类别层级、Extraction提取管道、Enhanced增强检索、Proactive主动代理、Integration集成迁移。保留了AgentMem的高性能引擎(216K ops/sec)、8个专业代理、企业特性和多语言SDK。参考memU的文件系统隐喻、资源抽象、类别组织和主动智能。已创建8个顺序Ralph任务跟踪实施。下一步:等待团队审查批准后启动第一阶段Resource设计。
<!-- tags: agentmem, reform, analysis, memU, planning | created: 2026-03-01 -->

### mem-1772345396-67e4
> AgentMem 文件核心改造完成：创建 TODO_CN.md (中文完整版 360行), todo2.md (英文详细版 670行), agentmem-reform-summary.md (执行摘要)。包含6阶段路线图 (14-19周), 资源层/类别层/提取管道/增强检索/主动代理完整设计, 成功指标和风险缓解。下一步: 等待团队审查批准后启动第一阶段。
<!-- tags: agentmem, reform, planning, chinese | created: 2026-03-01 -->

### mem-1772345039-99fe
> AgentMem architecture gap vs memU: AgentMem lacks (1) Resource abstraction layer - no file-like entity before memory items, (2) Hierarchical categories - only flat type-based organization, (3) Resource recall in search - returns only memory items not sources, (4) Sufficiency checks - no early exit during retrieval.
<!-- tags: agentmem, memU, comparison, architecture | created: 2026-03-01 -->
