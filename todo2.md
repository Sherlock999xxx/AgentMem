# AgentMem 文件核心改造 TODO 清单

> **中文说明**: 本文档为详细技术实现计划 (英文)。如需中文概览和决策说明，请参阅 **[TODO_CN.md](./TODO_CN.md)** (中文完整版)。

## Overview

将 AgentMem 从基于类型的记忆平台改造为受 memU"内存即文件系统"哲学启发的文件核心记忆系统。

**分析文档**: 详见 `.ralph/agent/scratchpad.md` 获取完整的差距分析
**时间线**: 14-19 周 (5-6 个月)
**优先级**: 高

---

## 第一阶段：基础架构 (第 1-3 周)

### 1.1 Resource 资源抽象设计
- [ ] **设计 Resource 数据模型**
  - ResourceID, URI, content_type, metadata, status
  - MediaType 枚举 (文本、图片、音频、视频、对话、文档)
  - ResourceStatus 生命周期 (pending → mounted → indexed → archived)
  - 与现有 AttributeSet (V4) 集成
  - **交付物**: 数据模型文档 + Rust 结构体
  - **预估**: 2-3 天

- [ ] **设计 ResourceManager 接口**
  - `mount(uri: &str) -> Result<Resource>` - 挂载资源
  - `unmount(resource_id: ResourceID) -> Result<()>` - 卸载资源
  - `get(resource_id: ResourceID) -> Result<Resource>` - 获取资源
  - `list(filter: ResourceFilter) -> Result<Vec<Resource>>` - 列出资源
  - **交付物**: Trait 定义 + 集成测试
  - **预估**: 2 天

### 1.2 内容类型检测
- [ ] **实现 MediaType 检测器**
  - 文件扩展名映射
  - MIME 类型检测
  - 基于内容的嗅探 (处理模糊文件)
  - URI 协议检测 (file://, http://, conv://, doc://)
  - **交付物**: MediaTypeDetector crate/模块
  - **预估**: 2-3 天

- [ ] **添加 URI 解析**
  - file:// 路径 → LocalFS (本地文件系统)
  - http:// URL → HTTP fetcher (HTTP 获取器)
  - conv:// ID → Conversation store (对话存储)
  - doc:// ID → Document store (文档存储)
  - 可扩展的 URI 协议注册表
  - **交付物**: URIResolver 带协议插件
  - **预估**: 3 天

### 1.3 资源存储实现
- [ ] **扩展存储后端以支持资源**
  - 在现有模式中添加 `resources` 表
  - 存储二进制内容 (或引用)
  - 索引元数据字段
  - 支持 LibSQL, PostgreSQL, SQLite
  - **交付物**: 每个后端的 ResourceRepo 实现
  - **预估**: 4-5 天

- [ ] **实现 ResourceManager**
  - 从 URI 获取/存储资源
  - 内容验证
  - 状态跟踪
  - 频繁访问资源的缓存层
  - **交付物**: 可工作的 ResourceManager 带测试
  - **预估**: 5-7 天

### 1.4 测试与文档
- [ ] **ResourceManager 单元测试**
  - 挂载/卸载操作
  - URI 解析 (所有协议)
  - 内容类型检测
  - 错误处理
  - **交付物**: >80% 测试覆盖率
  - **预估**: 2 天

- [ ] **文档: 资源系统**
  - 架构概述
  - API 参考 (ResourceManager)
  - URI 协议指南
  - 示例 (挂载文件、URL、对话)
  - **交付物**: docs/resource-system.md
  - **预估**: 1-2 天

---

## 第二阶段: 类别层级系统 (第 4-6 周)

### 2.1 Category Model Design
- [ ] **Design Category data model**
  - CategoryID, name, parent_id (hierarchy)
  - summary (LLM-generated)
  - embedding (for category search)
  - mount_point (optional external source)
  - Path-based access ("/preferences/communication/style")
  - **Deliverable**: Category model + hierarchy tests
  - **Estimate**: 2-3 days

- [ ] **Design CategoryManager interface**
  - `create(path: &str, summary: &str) -> Result<Category>`
  - `get(path: &str) -> Result<Category>`
  - `list(path: &str) -> Result<Vec<Category>>`
  - `move(from: &str, to: &str) -> Result<()>`
  - `delete(path: &str) -> Result<()>`
  - **Deliverable**: Trait definition
  - **Estimate**: 1-2 days

### 2.2 Category Storage Implementation
- [ ] **Extend storage for categories**
  - Add `categories` table with self-referential parent_id
  - Index for path lookups
  - Support for all backends
  - **Deliverable**: CategoryRepo implementation
  - **Estimate**: 3-4 days

- [ ] **Implement CategoryManager**
  - Path parsing and resolution
  - Hierarchical queries (children, ancestors)
  - Path validation (no cycles, unique names)
  - Lazy loading of category trees
  - **Deliverable**: Working CategoryManager
  - **Estimate**: 5-6 days

### 2.3 Category Summarization
- [ ] **LLM-based category summarization**
  - Summarize items in a category
  - Update summary when items change
  - Trigger on add/remove/update
  - Batch updates for efficiency
  - **Deliverable**: CategorySummarizer service
  - **Estimate**: 3-4 days

- [ ] **Category embeddings**
  - Embed category summaries
  - Embed for semantic category search
  - Update embeddings on summary change
  - **Deliverable**: CategoryEmbedding service
  - **Estimate**: 2 days

### 2.4 Navigation API
- [ ] **Implement browse/navigate API**
  - `navigate(path: &str) -> Result<CategoryView>`
  - List categories at path
  - List items at path
  - Return metadata (item counts, sizes)
  - **Deliverable**: Navigation API + tests
  - **Estimate**: 3-4 days

### 2.5 Testing & Documentation
- [ ] **Unit tests for categories**
  - CRUD operations
  - Navigation
  - Hierarchy integrity
  - Path resolution
  - **Deliverable**: >80% coverage
  - **Estimate**: 2 days

- [ ] **Documentation: Category System**
  - File system metaphor explanation
  - Category API reference
  - Best practices for organizing memory
  - Examples (browse, navigate)
  - **Deliverable**: docs/category-system.md
  - **Estimate**: 2 days

---

## Phase 3: Extraction Pipeline (Weeks 7-10)

### 3.1 Pipeline Framework
- [ ] **Design ExtractionWorkflow (inspired by memU)**
  - Pipeline = sequence of ExtractionSteps
  - Each step declares required/produced state
  - Capability tags (llm, vector, db, io, vision)
  - Interceptor hooks (before/after/on_error)
  - **Deliverable**: Pipeline framework design
  - **Estimate**: 3-4 days

- [ ] **Implement ExtractionPipeline engine**
  - Step validation and dependency checking
  - State management between steps
  - Error handling and rollback
  - Interceptor execution
  - **Deliverable**: ExtractionPipeline with tests
  - **Estimate**: 5-6 days

### 3.2 Resource → MemoryItem Extractors
- [ ] **Conversation extractor**
  - Parse conversation messages
  - Extract facts, preferences, skills
  - Identify speakers and context
  - Handle multi-turn conversations
  - **Deliverable**: ConversationExtractor
  - **Estimate**: 5-7 days

- [ ] **Document extractor**
  - Parse PDF, DOCX, TXT, Markdown
  - Extract sections, headings, key points
  - Preserve document structure
  - Handle tables, lists, code blocks
  - **Deliverable**: DocumentExtractor
  - **Estimate**: 7-10 days

- [ ] **Image/Vision extractor**
  - Extract text from images (OCR)
  - Describe visual content
  - Detect objects, faces, scenes
  - Handle image metadata
  - **Deliverable**: VisionExtractor
  - **Estimate**: 5-7 days

- [ ] **Audio/Video extractor**
  - Transcribe audio (speech-to-text)
  - Extract video frames for vision
  - Detect speakers and segments
  - Handle temporal metadata
  - **Deliverable**: MultimediaExtractor
  - **Estimate**: 7-10 days

### 3.3 Intelligence Features
- [ ] **Deduplication and merging**
  - Detect duplicate memories (semantic + exact)
  - Merge conflicting information
  - Resolve contradictions (recency + confidence)
  - **Deliverable**: DedupeService
  - **Estimate**: 4-5 days

- [ ] **Auto-categorization**
  - LLM-based category suggestion
  - Assign items to categories on extraction
  - Learn from user corrections
  - **Deliverable**: AutoCategorizer
  - **Estimate**: 4-5 days

### 3.4 Pipeline Integration
- [ ] **Assemble extraction pipeline**
  - Step 1: Ingest resource
  - Step 2: Detect content type
  - Step 3: Route to extractor
  - Step 4: Extract memory items
  - Step 5: Dedupe and merge
  - Step 6: Categorize items
  - Step 7: Update category summaries
  - Step 8: Index for search
  - **Deliverable**: Complete extraction pipeline
  - **Estimate**: 3-4 days

- [ ] **Integration with ResourceManager**
  - Trigger extraction on mount
  - Update resource status
  - Handle extraction failures
  - **Deliverable**: ResourceManager → Extractor wiring
  - **Estimate**: 2-3 days

### 3.5 Testing & Documentation
- [ ] **End-to-end extraction tests**
  - Test all content types
  - Validate pipeline steps
  - Error scenarios
  - **Deliverable**: Integration test suite
  - **Estimate**: 3-4 days

- [ ] **Documentation: Extraction Pipeline**
  - Pipeline architecture
  - Extensibility guide (custom extractors)
  - Content type reference
  - Examples
  - **Deliverable**: docs/extraction-pipeline.md
  - **Estimate**: 2-3 days

---

## Phase 4: Enhanced Retrieval (Weeks 11-13)

### 4.1 Category-Aware Search
- [ ] **Implement category recall in search**
  - Browse categories matching query
  - Navigate hierarchy for relevant items
  - Combine with vector search
  - **Deliverable**: CategorySearch module
  - **Estimate**: 3-4 days

- [ ] **Add category embeddings to search**
  - Search categories semantically
  - Boost items in matching categories
  - Hierarchical scoring (parent/child)
  - **Deliverable**: CategoryEmbeddingSearch
  - **Estimate**: 2-3 days

### 4.2 Resource Recall
- [ ] **Include source resources in results**
  - Return Resource alongside MemoryItem
  - Link items back to sources
  - Provide resource context
  - **Deliverable**: SearchResult with Resource
  - **Estimate**: 2-3 days

- [ ] **Implement resource browsing**
  - Browse by resource type
  - Filter by mount point
  - Resource metadata search
  - **Deliverable**: ResourceBrowser
  - **Estimate**: 2 days

### 4.3 Sufficiency Checking
- [ ] **Design sufficiency check algorithm**
  - Evaluate context coverage for query
  - Check if current results are enough
  - Early exit in search pipeline
  - **Deliverable**: SufficiencyChecker design
  - **Estimate**: 2 days

- [ ] **Implement sufficiency in search pipeline**
  - After category recall
  - After item recall
  - Configurable thresholds
  - **Deliverable**: Sufficiency integration
  - **Estimate**: 3-4 days

### 4.4 Query V4 Enhancement
- [ ] **Add category filters to Query V4**
  - `query.with_category("/preferences")`
  - `query.with_category_recursive()`
  - `query.exclude_category()`
  - **Deliverable**: Enhanced Query V4
  - **Estimate**: 2 days

- [ ] **Add resource filters to Query V4**
  - `query.with_resource_type("conversation")`
  - `query.with_resource_uri("conv://*")`
  - `query.include_resources(true)`
  - **Deliverable**: Resource-aware Query V4
  - **Estimate**: 2 days

### 4.5 Search Pipeline Assembly
- [ ] **Assemble enhanced search pipeline**
  1. Route intention
  2. Query rewrite (optional)
  3. Category recall
  4. Sufficiency check (optional)
  5. Item recall (vector + BM25 + RRF)
  6. Resource recall
  7. Sufficiency check (optional)
  8. Build response
  - **Deliverable**: Complete search pipeline
  - **Estimate**: 4-5 days

### 4.6 Testing & Documentation
- [ ] **Search performance tests**
  - Latency (target P95 <150ms)
  - Accuracy (relevance metrics)
  - Sufficiency effectiveness
  - **Deliverable**: Performance benchmark suite
  - **Estimate**: 2-3 days

- [ ] **Documentation: Enhanced Search**
  - Search pipeline overview
  - Category search guide
  - Query V4 reference
  - Examples
  - **Deliverable**: docs/enhanced-search.md
  - **Estimate**: 2 days

---

## Phase 5: Proactive Agent (Weeks 14-16)

### 5.1 ProactiveAgent Design
- [ ] **Design ProactiveAgent architecture**
  - Background task runner
  - Event-driven triggers
  - Periodic maintenance jobs
  - **Deliverable**: ProactiveAgent design doc
  - **Estimate**: 2-3 days

- [ ] **Implement task scheduler**
  - Cron-like scheduling
  - Event triggers (on_add, on_update)
  - Task queue and executor
  - **Deliverable**: TaskScheduler
  - **Estimate**: 3-4 days

### 5.2 Proactive Tasks
- [ ] **Category summary updates**
  - Periodic re-summarization
  - Detect stale summaries
  - Update on significant changes
  - **Deliverable**: SummaryUpdateTask
  - **Estimate**: 3-4 days

- [ ] **Duplicate detection**
  - Periodic scan for duplicates
  - Merge or flag duplicates
  - User confirmation workflow
  - **Deliverable**: DuplicateDetectionTask
  - **Estimate**: 4-5 days

- [ ] **Memory consolidation**
  - Merge fragmented memories
  - Resolve contradictions
  - Archive old/irrelevant items
  - **Deliverable**: ConsolidationTask
  - **Estimate**: 5-6 days

- [ ] **Intent prediction**
  - Analyze user patterns
  - Pre-fetch likely context
  - Suggest actions
  - **Deliverable**: IntentPredictor
  - **Estimate**: 5-7 days

### 5.3 Proactive API
- [ ] **Implement proactive suggestions API**
  - `get_suggestions() -> Vec<Suggestion>`
  - `confirm_suggestion(id) -> Result<()>`
  - `dismiss_suggestion(id) -> Result<()>`
  - **Deliverable**: ProactiveAPI
  - **Estimate**: 2-3 days

- [ ] **Notification system**
  - Push notifications for suggestions
  - Webhook integration
  - User preferences (frequency, types)
  - **Deliverable**: NotificationService
  - **Estimate**: 3-4 days

### 5.4 Testing & Documentation
- [ ] **Proactive agent tests**
  - Task execution
  - Event triggers
  - Suggestion quality
  - **Deliverable**: Test suite
  - **Estimate**: 3 days

- [ ] **Documentation: Proactive Memory**
  - ProactiveAgent overview
  - Task configuration
  - API reference
  - Examples
  - **Deliverable**: docs/proactive-memory.md
  - **Estimate**: 2 days

---

## Phase 6: Integration & Migration (Weeks 17-19)

### 6.1 Agent Integration
- [ ] **Integrate with 8 specialized agents**
  - CoreAgent, EpisodicAgent, etc. use resources
  - Update agent logic for categories
  - Maintain backwards compatibility
  - **Deliverable**: Updated agents
  - **Estimate**: 5-7 days

- [ ] **Hybrid operation mode**
  - Support file-centric + type-based APIs
  - Unified Memory interface
  - Gradual migration path
  - **Deliverable**: HybridMemory wrapper
  - **Estimate**: 4-5 days

### 6.2 Data Migration
- [ ] **Design migration strategy**
  - Legacy MemoryItem → Resource-derived MemoryItem
  - Synthetic resources for existing items
  - Auto-categorization of legacy data
  - **Deliverable**: Migration design doc
  - **Estimate**: 2-3 days

- [ ] **Implement migration tools**
  - Migration CLI command
  - Progress tracking and rollback
  - Validation and verification
  - **Deliverable**: MigrationTool
  - **Estimate**: 5-7 days

### 6.3 SDK Updates
- [ ] **Update Python SDK**
  - File-centric API
  - Resource mounting
  - Category navigation
  - **Deliverable**: Python SDK v2.0
  - **Estimate**: 4-5 days

- [ ] **Update JavaScript SDK**
  - File-centric API
  - Async resource mounting
  - Category browsing
  - **Deliverable**: JS SDK v2.0
  - **Estimate**: 4-5 days

- [ ] **Update Go SDK**
  - File-centric API
  - Resource management
  - Category operations
  - **Deliverable**: Go SDK v2.0
  - **Estimate**: 3-4 days

- [ ] **Update Cangjie SDK**
  - File-centric API
  - Resource abstraction
  - Category navigation
  - **Deliverable**: Cangjie SDK v2.0
  - **Estimate**: 3-4 days

### 6.4 Documentation
- [ ] **Migration guide**
  - V3 → V4 → File-centric migration
  - Code examples
  - Common issues and solutions
  - **Deliverable**: docs/migration-guide.md
  - **Estimate**: 3-4 days

- [ ] **Update all examples**
  - File-centric examples
  - Resource mounting demos
  - Category navigation demos
  - **Deliverable**: Updated examples/
  - **Estimate**: 3-4 days

- [ ] **API reference update**
  - New APIs documented
  - Deprecated APIs marked
  - Quickstart updated
  - **Deliverable**: Complete API docs
  - **Estimate**: 2-3 days

### 6.5 Testing & Validation
- [ ] **End-to-end integration tests**
  - Full workflow: mount → extract → search → navigate
  - All SDKs tested
  - Migration verification
  - **Deliverable**: Integration test suite
  - **Estimate**: 4-5 days

- [ ] **Performance validation**
  - Benchmark vs baseline
  - Load testing
  - Memory profiling
  - **Deliverable**: Performance report
  - **Estimate**: 2-3 days

- [ ] **Beta testing program**
  - Recruit beta testers
  - Gather feedback
  - Bug fixes and iterations
  - **Deliverable**: Beta feedback report
  - **Estimate**: Ongoing during phase 6

---

## Open Questions / Decisions Needed

### Technical Decisions
- [ ] **Backwards Compatibility Strategy**
  - Option A: Migration tool (synthetic resources)
  - Option B: Dual model (support both APIs)
  - Option C: Breaking change (clear migration path)
  - **Decision needed by**: Week 2
  - **Owner**: Architecture team

- [ ] **Storage Standardization**
  - Keep multi-backend (LibSQL, Postgres, etc.)?
  - Or standardize on single backend?
  - **Decision needed by**: Week 2
  - **Owner**: Storage team

- [ ] **Performance Targets**
  - Acceptable overhead for resource layer?
  - Target P95 latency with file-centric model?
  - **Decision needed by**: Week 4
  - **Owner**: Performance team

### Product Decisions
- [ ] **Default Category Structure**
  - Pre-defined categories (memU-style)?
  - Or user-defined?
  - Hybrid (suggested + customizable)?
  - **Decision needed by**: Week 6
  - **Owner**: Product team

- [ ] **Proactive Feature Scope**
  - Full 24/7 agent or scheduled tasks?
  - Cloud-only or also self-hosted?
  - **Decision needed by**: Week 14
  - **Owner**: Product team

---

## Success Criteria

### Phase 1 Success
- ✅ ResourceManager handles 10K+ resources
- ✅ URI resolution supports 4+ schemes
- ✅ Performance degradation <10%

### Phase 2 Success
- ✅ Category hierarchy handles 1000+ categories
- ✅ Navigation latency <50ms P95
- ✅ Category summaries are useful (user rated >3/5)

### Phase 3 Success
- ✅ Extractors handle 5+ content types
- ✅ Extraction accuracy >80% (manual review)
- ✅ Pipeline extensibility proven (custom extractor demo)

### Phase 4 Success
- ✅ Search latency <150ms P95 (with resources)
- ✅ Category-aware search improves relevance >15%
- ✅ Sufficiency checks reduce LLM calls >30%

### Phase 5 Success
- ✅ Proactive suggestions useful >70% of time
- ✅ Background tasks <5% CPU overhead
- ✅ Intent prediction accuracy >60%

### Phase 6 Success
- ✅ 80%+ users migrate to new API within 3 months
- ✅ All SDKs support new features
- ✅ Migration tool has <1% data loss rate

---

## Risk Mitigation

### High-Risk Items
1. **Performance degradation from resource layer**
   - Mitigation: Aggressive caching, async pipelines
   - Contingency: Optional resource layer (opt-in)

2. **Complexity of category hierarchy**
   - Mitigation: Start flat, add hierarchy incrementally
   - Contingency: Keep flat type-based as fallback

3. **Migration challenges**
   - Mitigation: Comprehensive migration tools, rollback support
   - Contingency: Dual API support for 6+ months

### Medium-Risk Items
1. **Extractor quality varies by content type**
   - Mitigation: A/B testing, prompt iteration
   - Contingency: Allow manual correction

2. **Proactive agent resource usage**
   - Mitigation: Configurable task frequency
   - Contingency: Opt-in proactive features

---

## Glossary

- **Resource**: File-like entity (conversation, document, image, etc.)
- **Category**: Hierarchical folder-like organization
- **Mount**: Make a resource available as queryable memory
- **Extraction**: Transform resource → structured memory items
- **Sufficiency Check**: Determine if current context is enough for query
- **Proactive Agent**: Background agent that organizes memory autonomously

---

## References

- memU repository: `source/memU/`
- memU architecture: `source/memU/docs/architecture.md`
- AgentMem current: `crates/agent-mem/`
- AgentMem V4 traits: `crates/agent-mem-traits/`

---

**Last Updated**: 2026-03-01
**Status**: Planning Phase - Awaiting Review
**Next Milestone**: Phase 1 Kickoff (Pending approval)
