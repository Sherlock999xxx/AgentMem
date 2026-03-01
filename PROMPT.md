# AgentMem 文件核心改造计划 - 开发指南

**版本**: 2.6
**日期**: 2026-03-01
**状态**: 🎯 待实施
**预计工期**: 14-19 周（5-6 个月）
**团队**: AgentMem 核心开发团队

---

## 📋 一、改造目标与愿景

### 1.1 核心目标

将 **AgentMem** 从"基于类型"的记忆平台转型为"**文件核心**"的记忆系统，融合 memU 的直观文件系统隐喻与 AgentMem 的生产级性能引擎。

### 1.2 设计哲学对比

| 维度 | 当前 AgentMem（类型核心） | 目标设计（文件核心） |
|------|-------------------------|-------------------|
| **组织方式** | 按类型分类（Episodic, Semantic, Procedural） | 按类别分类（类似文件夹层级） |
| **记忆来源** | 直接插入 MemoryItem | 从 Resource 提取（挂载→提取→索引） |
| **导航方式** | 类型 + 属性过滤 | 类别路径浏览（如 `/偏好/沟通/风格`） |
| **检索策略** | 5 种搜索引擎并行 | 类别召回 + 充足度检查 + 7 阶段检索 |
| **主动整理** | 手动组织 | 24/7 后台代理自动整理 |

### 1.3 文件系统隐喻

```
记忆系统 = 文件系统
├── Categories（类别）= 文件夹（自动组织的主题）
│   ├── /偏好/沟通/风格
│   ├── /知识/编程/Rust
│   └── /技能/分析/调试
├── MemoryItems（记忆项）= 文件（事实、偏好、技能）
│   ├── 用户喜欢 Rust 编程.cj
│   ├── 偏好简洁代码风格.py
│   └── 擅长调试性能问题.rs
└── Resources（资源）= 挂载点（对话、文档、图片）
    ├── conv://chat-2025-02-28
    ├── doc://README.md
    └── file://screenshot.png
```

---

## 🎯 二、为什么需要这次改造？

### 2.1 memU 的设计优势（学习目标）

#### 1. 资源抽象层
- **URI 统一标识符**：`file://`, `http://`, `conv://`, `doc://`
- **MediaType 自动检测**：`text/`, `image/`, `audio/`, `video/`, `application/`
- **资源元数据管理**：作者、创建时间、标签、大小
- **核心价值**：所有记忆源自可追踪的来源，支持来源召回

#### 2. 类别层级系统
- **路径导航**：`/偏好/沟通/风格`, `/知识/编程/Rust`
- **LLM 生成摘要**：每个类别自动生成总结
- **类别嵌入搜索**：快速定位相关类别
- **核心价值**：按主题浏览，而非按类型分类

#### 3. 提取管道
- **7 阶段 memorize 工作流**：
  1. ingest_resource（挂载资源）
  2. preprocess_multimodal（多模态预处理）
  3. extract_items（提取记忆项）
  4. dedupe_merge（去重合并）
  5. categorize_items（自动分类）
  6. persist_index（持久化索引）
  7. build_response（构建响应）
- **核心价值**：系统化的记忆生成流程

#### 4. 充足度检查
- **类别充足度**：检查类别是否包含足够信息
- **资源充足度**：检查是否需要召回原始资源
- **LLM 驱动判断**：智能决策是否继续检索
- **核心价值**：早期退出避免过度检索，降低成本

#### 5. 主动智能代理
- **自动分类**：新记忆自动归入合适类别
- **去重合并**：识别并合并重复记忆
- **摘要生成**：定期为类别生成新摘要
- **核心价值**：24/7 后台整理，保持记忆系统健康

### 2.2 AgentMem 的技术优势（保留基础）

#### 1. 高性能引擎
- **216K ops/sec** 插件吞吐量
- **<100ms** P95 语义搜索延迟
- **93,000x** 缓存加速比
- **5,000 ops/s** 记忆添加吞吐量
- **异步、无锁架构**

#### 2. 8 个专业代理
1. **CoreAgent**：核心记忆管理
2. **EpisodicAgent**：情景记忆（事件、经历）
3. **SemanticAgent**：语义记忆（事实、知识）
4. **ProceduralAgent**：程序记忆（技能、操作）
5. **WorkingMemoryAgent**：工作记忆（临时上下文）
6. **ResourceAgent**：资源管理
7. **KnowledgeAgent**：知识图谱
8. **ContextualAgent**：上下文感知

#### 3. 5 种搜索引擎
- **Vector**：语义向量搜索（FastEmbed, OpenAI, VoyageAI）
- **BM25**：关键词搜索（TF-IDF 变种）
- **Full-Text**：全文搜索（FTS5）
- **Fuzzy**：模糊匹配（Levenshtein 距离）
- **RRF**：倒数排名融合（多引擎结果合并）

#### 4. 30+ 存储后端
- **关系型**：LibSQL, PostgreSQL, MySQL
- **向量数据库**：Qdrant, Pinecone, Milvus, Chroma, Weaviate, LanceDB
- **NoSQL**：MongoDB, Redis, Elasticsearch
- **本地**：FAISS, SQLite

#### 5. 20+ LLM 提供商
OpenAI, Anthropic, DeepSeek, Google, Azure, Zhipu, Baichuan, Qwen, MiniMax, Xingchen, Moonshot, LemonAI, 01.AI, SiliconFlow, Together, Groq, Perplexity, HuggingFace, Cohere, Mistral, Jina

#### 6. 企业特性
- **RBAC**：基于角色的访问控制
- **JWT**：JSON Web Token 认证
- **审计日志**：所有操作可追溯
- **多租户**：数据隔离
- **Prometheus 指标 + OpenTelemetry 追踪**

#### 7. 多语言 SDK
- **Python**：PyPI 包
- **JavaScript/TypeScript**：NPM 包
- **Go**：Go module
- **Cangjie**：中文原生支持
- **LlamaIndex**：集成

#### 8. LLM 成本优化
- **连接池**：复用 LLM 连接
- **重试逻辑**：指数退避
- **提示压缩**：减少 token 使用
- **KV-cache 优化**：缓存 KV 对
- **成本降低 90%**

### 2.3 改造策略 = 最佳组合

**memU 的直观性 + AgentMem 的性能**

```
保留（85% 代码）：
✅ 核心高性能引擎
✅ 8 个专业代理
✅ 5 种搜索引擎
✅ 30+ 存储后端
✅ 20+ LLM 提供商
✅ 企业特性（RBAC, JWT, 审计日志）
✅ 多语言 SDK
✅ MCP 集成

新增（4 个 crates，~5K LOC）：
🆕 agent-mem-resource（资源抽象层）
🆕 agent-mem-category（类别层级系统）
🆕 agent-mem-extraction（提取管道框架）
🆕 agent-mem-proactive（主动代理）

重构（15% 代码）：
🔄 MemoryType → Category（类型到类别）
🔄 类型分发 → 类别路由（路由机制）
🔄 5 阶段检索 → 7 阶段检索（增强检索）
```

---

## 🏗️ 三、技术架构设计

### 3.1 改造前架构（当前）

```
Memory API
    ↓
MemoryOrchestrator
    ↓
8 个专业代理（Core, Episodic, Knowledge 等）
    ↓
存储后端（LibSQL, PostgreSQL 等）
```

### 3.2 改造后架构（目标）

```
FileCentricMemory API（新增高层 API）
    ↓
FileCentricOrchestrator（新增编排器）
    ↓
┌─────────────────────────────────┐
│ 资源层（新增）                     │
│ - ResourceManager（资源管理器）    │
│ - MediaTypeDetector（类型检测器）  │
│ - URIResolver（URI 解析器）       │
└─────────────────────────────────┘
    ↓
┌─────────────────────────────────┐
│ 提取管道（新增）                   │
│ - ExtractionPipeline（管道框架）  │
│ - ContentExtractor（内容提取器）  │
│ - DedupeMerger（去重合并器）      │
│ - AutoCategorizer（自动分类器）   │
└─────────────────────────────────┘
    ↓
┌─────────────────────────────────┐
│ 类别层级（新增）                   │
│ - CategoryManager（类别管理器）   │
│ - PathNavigator（路径导航器）     │
│ - SummaryGenerator（摘要生成器）  │
└─────────────────────────────────┘
    ↓
8 个增强的专业代理（保留核心，增加类别/资源感知）
    ↓
存储后端（不变 + 新增 Resource/Category 表）
```

### 3.3 新增 Crate 详细设计

#### Crate 1: agent-mem-resource（资源抽象层）

**职责**：管理所有外部资源的挂载、解析和元数据

**核心组件**：
```rust
// Resource 数据模型
pub struct Resource {
    pub id: String,                    // 唯一标识符
    pub uri: String,                   // 统一资源标识符
    pub media_type: MediaType,         // 媒体类型（text/, image/ 等）
    pub metadata: ResourceMetadata,    // 元数据（作者、时间、标签）
    pub status: ResourceStatus,        // 挂载状态
    pub scope: MemoryScope,            // 用户/代理范围
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// MediaType 枚举
pub enum MediaType {
    Text(TextType),          // text/plain, text/markdown, text/code
    Image(ImageType),        // image/png, image/jpeg, image/svg+xml
    Audio(AudioType),        // audio/mp3, audio/wav
    Video(VideoType),        // video/mp4, video/webm
    Application(AppType),    // application/pdf, application/json
}

// URI 解析器
pub trait URIResolver {
    async fn resolve(&self, uri: &str) -> Result<ResourceContent>;
    async fn validate(&self, uri: &str) -> Result<bool>;
}

// 媒体类型检测器
pub trait MediaTypeDetector {
    async fn detect(&self, content: &[u8]) -> Result<MediaType>;
    async fn from_uri(&self, uri: &str) -> Result<MediaType>;
}
```

**关键功能**：
- 资源挂载：`mount_resource(uri, scope) -> ResourceId`
- 资源解析：`resolve_resource(resource_id) -> ResourceContent`
- 类型检测：自动检测文件类型（magic bytes + URI 扩展名）
- 元数据提取：自动提取作者、创建时间、标签

#### Crate 2: agent-mem-category（类别层级系统）

**职责**：管理类别层级、路径导航和摘要生成

**核心组件**：
```rust
// Category 数据模型
pub struct Category {
    pub id: String,                    // 唯一标识符
    pub path: CategoryPath,            // 路径（如 /偏好/沟通/风格）
    pub name: String,                  // 类别名称
    pub parent_id: Option<String>,     // 父类别 ID
    pub children_ids: Vec<String>,     // 子类别 ID 列表
    pub summary: Option<String>,       // LLM 生成的摘要
    pub embedding: Option<Vec<f32>>,   // 类别嵌入向量
    pub item_count: usize,             // 包含的记忆项数量
    pub scope: MemoryScope,            // 用户/代理范围
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// CategoryPath 路径
pub struct CategoryPath {
    pub segments: Vec<String>,         // 路径段：["偏好", "沟通", "风格"]
}

// CategoryManager
pub trait CategoryManager {
    // 类别管理
    async fn create_category(&self, path: &CategoryPath, scope: &MemoryScope) -> Result<Category>;
    async fn get_category(&self, id: &str) -> Result<Category>;
    async fn list_categories(&self, parent_id: Option<&str>) -> Result<Vec<Category>>;

    // 路径导航
    async fn navigate_path(&self, path: &CategoryPath) -> Result<Category>;
    async fn browse_children(&self, category_id: &str) -> Result<Vec<Category>>;

    // 摘要生成
    async fn generate_summary(&self, category_id: &str) -> Result<String>;
    async fn update_summary(&self, category_id: &str) -> Result<()>;
}
```

**关键功能**：
- 类别创建：`create_category("/偏好/沟通/风格")`
- 路径导航：`navigate_path("/知识/编程")`
- 浏览子类别：`browse_children(category_id)`
- 摘要生成：LLM 自动生成类别摘要
- 类别搜索：基于嵌入向量的语义搜索

#### Crate 3: agent-mem-extraction（提取管道框架）

**职责**：从资源中提取结构化记忆项

**核心组件**：
```rust
// ExtractionPipeline 管道
pub struct ExtractionPipeline {
    stages: Vec<Box<dyn ExtractionStage>>,
}

// ExtractionStage 阶段 trait
pub trait ExtractionStage {
    async fn process(&self, input: ExtractionInput) -> Result<ExtractionOutput>;
}

// 7 个标准阶段
pub struct ResourceIngestor;           // 1. 挂载资源
pub struct MultimodalPreprocessor;     // 2. 多模态预处理
pub struct ItemExtractor;              // 3. 提取记忆项
pub struct DedupeMerger;               // 4. 去重合并
pub struct AutoCategorizer;            // 5. 自动分类
pub struct IndexPersistor;             // 6. 持久化索引
pub struct ResponseBuilder;            // 7. 构建响应

// ContentExtractor 内容提取器
pub trait ContentExtractor {
    async fn extract(&self, resource: &Resource) -> Result<Vec<MemoryItem>>;
}

// 具体提取器
pub struct ConversationExtractor;      // 对话提取
pub struct DocumentExtractor;          // 文档提取
pub struct ImageExtractor;             // 图片提取（OCR）
pub struct AudioExtractor;             // 音频提取（ASR）
```

**关键功能**：
- 流水线处理：`pipeline.execute(resource) -> Vec<MemoryItem>`
- 内容提取：从对话、文档、图片、音频提取记忆
- 去重合并：识别并合并重复记忆
- 自动分类：LLM 自动为新记忆分配类别

#### Crate 4: agent-mem-proactive（主动代理）

**职责**：24/7 后台自动整理记忆系统

**核心组件**：
```rust
// ProactiveAgent 主动代理
pub struct ProactiveAgent {
    orchestrator: Arc<FileCentricOrchestrator>,
    scheduler: TaskScheduler,
}

// ProactiveTask 主动任务
pub enum ProactiveTask {
    AutoCategorize,                    // 自动分类新记忆
    DedupeMerge,                       // 去重合并重复记忆
    GenerateSummaries,                 // 生成类别摘要
    IndexOptimization,                 // 优化搜索索引
    ResourceArchival,                  // 归档旧资源
}

// TaskScheduler 任务调度器
pub trait TaskScheduler {
    async fn schedule(&self, task: ProactiveTask, interval: Duration) -> Result<()>;
    async fn run_once(&self, task: ProactiveTask) -> Result<()>;
}
```

**关键功能**：
- 定时任务：每隔 N 分钟执行自动整理
- 触发式任务：新记忆到达时触发分类
- 批处理任务：深夜批量处理摘要生成
- 健康检查：定期检查记忆系统健康度

### 3.4 增强的检索流程（7 阶段）

```
用户查询："用户偏好什么编程语言？"
    ↓
1. 路由意图（route_intention）
   → 判断查询类型：偏好类查询
    ↓
2. 类别召回（category_recall）
   → 搜索相关类别：/偏好/编程/*
   → 召回类别：["/偏好/编程/语言", "/偏好/编程/工具"]
    ↓
3. 充足度检查（sufficiency_check）
   → LLM 判断：类别信息是否充足？
   → 决策：需要继续检索记忆项
    ↓
4. 记忆项召回（item_recall）
   → 在选定类别中搜索记忆项
   → 5 种搜索引擎并行：Vector + BM25 + FTS + Fuzzy + RRF
    ↓
5. 资源召回（resource_recall）
   → 召回记忆项的来源资源
   → 提供上下文：用户在对话中提到喜欢 Rust
    ↓
6. 充足度检查（sufficiency_check）
   → LLM 判断：信息是否充足？
   → 决策：充足，可以回答
    ↓
7. 构建响应（build_response）
   → 整合类别、记忆项、资源信息
   → 生成最终答案："用户偏好 Rust 编程语言，并在多个对话中强调其性能和安全性优势。"
```

### 3.5 数据库 Schema 变更

#### 新增表：resources
```sql
CREATE TABLE resources (
    id TEXT PRIMARY KEY,
    uri TEXT NOT NULL UNIQUE,
    media_type TEXT NOT NULL,          -- "text/plain", "image/png" 等
    metadata JSON,                     -- {author, created_at, tags, size}
    status TEXT NOT NULL,              -- "mounted", "pending", "failed"
    user_id TEXT NOT NULL,
    agent_id TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id),
    INDEX idx_uri (uri),
    INDEX idx_user_id (user_id),
    INDEX idx_status (status)
);
```

#### 新增表：categories
```sql
CREATE TABLE categories (
    id TEXT PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,         -- "/偏好/沟通/风格"
    name TEXT NOT NULL,                -- "风格"
    parent_id TEXT,                    -- NULL 表示根类别
    summary TEXT,                      -- LLM 生成的摘要
    embedding BLOB,                    -- 类别嵌入向量
    item_count INTEGER DEFAULT 0,
    user_id TEXT NOT NULL,
    agent_id TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (parent_id) REFERENCES categories(id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    INDEX idx_path (path),
    INDEX idx_parent_id (parent_id),
    INDEX idx_user_id (user_id)
);
```

#### 修改表：memory_items
```sql
-- 新增字段
ALTER TABLE memory_items ADD COLUMN resource_id TEXT;
ALTER TABLE memory_items ADD COLUMN category_id TEXT;
ALTER TABLE memory_items ADD COLUMN extracted_at DATETIME;

-- 新增外键
ALTER TABLE memory_items ADD FOREIGN KEY (resource_id) REFERENCES resources(id);
ALTER TABLE memory_items ADD FOREIGN KEY (category_id) REFERENCES categories(id);

-- 新增索引
CREATE INDEX idx_resource_id ON memory_items(resource_id);
CREATE INDEX idx_category_id ON memory_items(category_id);
```

---

## 🗺️ 四、六阶段实施路线图

### 阶段 1：Resource 资源抽象层（第 1-3 周）

**目标**：建立资源抽象层，实现资源的挂载、解析和元数据管理

#### Week 1: 数据模型设计
- [ ] **Task 1.1**：设计 Resource 数据结构
  - 定义 `Resource` 结构体（id, uri, media_type, metadata, status）
  - 定义 `MediaType` 枚举（Text, Image, Audio, Video, Application）
  - 定义 `ResourceMetadata` 结构体（author, created_at, tags, size）
  - 编写单元测试（~200 行）

- [ ] **Task 1.2**：设计数据库 Schema
  - 创建 `resources` 表（参考 3.5 节）
  - 设计索引策略（uri, user_id, status）
  - 编写迁移脚本（SQLite, PostgreSQL, LibSQL）

- [ ] **Task 1.3**：设计 API 接口
  - `ResourceManager` trait 定义
  - `mount_resource(uri, scope) -> ResourceId`
  - `resolve_resource(resource_id) -> ResourceContent`
  - `list_resources(scope) -> Vec<Resource>`

#### Week 2-3: 核心实现
- [ ] **Task 1.4**：实现 MediaTypeDetector
  - Magic bytes 检测（文件头识别）
  - URI 扩展名检测（.png, .pdf, .md）
  - 支持常见类型：text/plain, text/markdown, image/png, application/pdf（~500 行）

- [ ] **Task 1.5**：实现 URIResolver
  - `file://` 协议：本地文件系统
  - `http://` 协议：HTTP 下载
  - `conv://` 协议：对话历史
  - `doc://` 协议：文档引用（~600 行）

- [ ] **Task 1.6**：实现 ResourceManager
  - 资源挂载逻辑
  - 资源状态管理（mounted, pending, failed）
  - 资源元数据提取（作者、时间、标签）（~800 行）

- [ ] **Task 1.7**：集成测试
  - 端到端测试：挂载 → 解析 → 查询
  - 性能测试：挂载 1000 个资源 < 5 秒
  - 错误处理测试：无效 URI、权限错误

**交付物**：
- ✅ `agent-mem-resource` crate（~2,100 行代码）
- ✅ 数据库迁移脚本
- ✅ API 文档和集成测试
- ✅ 性能基准测试报告

---

### 阶段 2：Category 类别层级系统（第 4-6 周）

**目标**：建立类别层级系统，实现路径导航和摘要生成

#### Week 4: 数据模型设计
- [ ] **Task 2.1**：设计 Category 数据结构
  - 定义 `Category` 结构体（id, path, name, parent_id, children_ids, summary）
  - 定义 `CategoryPath` 结构体（segments 向量）
  - 定义 `CategoryTreeNode` 结构体（树形结构）
  - 编写单元测试（~200 行）

- [ ] **Task 2.2**：设计数据库 Schema
  - 创建 `categories` 表（参考 3.5 节）
  - 设计层级查询策略（递归 CTE vs 物化路径）
  - 编写迁移脚本

- [ ] **Task 2.3**：设计 API 接口
  - `CategoryManager` trait 定义
  - `create_category(path, scope) -> CategoryId`
  - `navigate_path(path) -> Category`
  - `browse_children(category_id) -> Vec<Category>`

#### Week 5-6: 核心实现
- [ ] **Task 2.4**：实现 CategoryManager
  - 类别创建逻辑（自动创建父类别）
  - 路径导航逻辑（路径解析、验证）
  - 层级查询逻辑（父子关系遍历）（~800 行）

- [ ] **Task 2.5**：实现 SummaryGenerator
  - LLM 驱动的摘要生成（GPT-4, DeepSeek）
  - 摘要更新策略（增量更新 vs 全量重生成）
  - 摘要缓存机制（~600 行）

- [ ] **Task 2.6**：实现类别搜索
  - 类别嵌入生成（使用现有 embedding 模型）
  - 类别语义搜索（Vector + BM25）
  - 类别推荐（基于用户查询）（~500 行）

- [ ] **Task 2.7**：集成测试
  - 端到端测试：创建 → 导航 → 搜索 → 摘要
  - 性能测试：创建 100 个类别 < 2 秒
  - 并发测试：多用户同时创建类别

**交付物**：
- ✅ `agent-mem-category` crate（~2,100 行代码）
- ✅ 数据库迁移脚本
- ✅ API 文档和集成测试
- ✅ 性能基准测试报告

---

### 阶段 3：Extraction 提取管道框架（第 7-10 周）

**目标**：构建提取管道框架，实现从资源到记忆项的自动提取

#### Week 7-8: 管道框架
- [ ] **Task 3.1**：设计 ExtractionPipeline 架构
  - 定义 `ExtractionStage` trait
  - 定义 `ExtractionInput` 和 `ExtractionOutput`
  - 定义管道编排逻辑（顺序、并行、条件）
  - 编写单元测试（~200 行）

- [ ] **Task 3.2**：实现 7 个标准阶段
  - `ResourceIngestor`：挂载资源
  - `MultimodalPreprocessor`：预处理（OCR、ASR）
  - `ItemExtractor`：提取记忆项
  - `DedupeMerger`：去重合并
  - `AutoCategorizer`：自动分类
  - `IndexPersistor`：持久化索引
  - `ResponseBuilder`：构建响应（~1,500 行）

- [ ] **Task 3.3**：实现管道编排器
  - 顺序执行：stage1 → stage2 → stage3
  - 并行执行：stage2 和 stage3 并行
  - 错误处理：失败重试、跳过、回滚
  - 性能监控：每个阶段的耗时统计（~600 行）

#### Week 9-10: 内容提取器
- [ ] **Task 3.4**：实现 ConversationExtractor
  - 对话格式解析（JSON, Markdown, Transcript）
  - 说话人识别（user, assistant, system）
  - 关键信息提取（事实、偏好、事件）（~800 行）

- [ ] **Task 3.5**：实现 DocumentExtractor
  - 文档格式支持（Markdown, PDF, TXT, DOCX）
  - 分段策略（按段落、按章节、按 token）
  - 关键句子提取（TF-IDF + LLM）（~800 行）

- [ ] **Task 3.6**：实现 ImageExtractor 和 AudioExtractor
  - ImageExtractor：OCR（Tesseract, Azure Vision）
  - AudioExtractor：ASR（Whisper, Azure Speech）
  - 多模态内容整合（~600 行）

- [ ] **Task 3.7**：实现去重合并逻辑
  - 相似度计算（余弦相似度 + Jaccard 相似度）
  - 重复检测策略（精确匹配、语义相似）
  - 记忆合并逻辑（时间戳优先、投票机制）（~500 行）

- [ ] **Task 3.8**：实现自动分类器
  - LLM 分类（Zero-shot, Few-shot）
  - 规则分类（关键词匹配、正则表达式）
  - 类别推荐（Top-K 类别推荐）（~600 行）

- [ ] **Task 3.9**：集成测试
  - 端到端测试：挂载 → 提取 → 去重 → 分类 → 索引
  - 性能测试：处理 100 个对话 < 30 秒
  - 准确性测试：去重准确率 > 90%，分类准确率 > 85%

**交付物**：
- ✅ `agent-mem-extraction` crate（~5,600 行代码）
- ✅ API 文档和集成测试
- ✅ 性能基准测试报告
- ✅ 准确性评估报告

---

### 阶段 4：Enhanced 增强检索系统（第 11-13 周）

**目标**：实现 7 阶段增强检索流程

#### Week 11-12: 类别和资源召回
- [ ] **Task 4.1**：实现 category_recall
  - 类别嵌入搜索（Vector search）
  - 类别路径匹配（模糊匹配）
  - 类别推荐（Top-K 相关类别）（~600 行）

- [ ] **Task 4.2**：实现 resource_recall
  - 资源元数据搜索（BM25）
  - 资源内容搜索（Full-Text Search）
  - 资源摘要生成（LLM）（~500 行）

- [ ] **Task 4.3**：实现 sufficiency_check
  - LLM 驱动的充足度判断
  - 阈值策略（信息量评分）
  - 早期退出机制（~400 行）

#### Week 13: 检索流程整合
- [ ] **Task 4.4**：实现 7 阶段检索流程
  - 路由意图（route_intention）
  - 类别召回（category_recall）
  - 充足度检查（sufficiency_check）
  - 记忆项召回（item_recall）
  - 资源召回（resource_recall）
  - 充足度检查（sufficiency_check）
  - 构建响应（build_response）（~800 行）

- [ ] **Task 4.5**：优化检索性能
  - 并行执行（类别召回 + 记忆项召回）
  - 缓存策略（类别缓存、资源缓存）
  - 批处理（批量嵌入生成）（~400 行）

- [ ] **Task 4.6**：集成测试
  - 端到端测试：查询 → 7 阶段检索 → 答案
  - 性能测试：平均检索延迟 < 200ms
  - 准确性测试：检索准确率 > 85%

**交付物**：
- ✅ 增强的检索系统（~2,700 行代码）
- ✅ API 文档和集成测试
- ✅ 性能基准测试报告
- ✅ 准确性评估报告

---

### 阶段 5：Proactive 主动代理（第 14-16 周）

**目标**：开发 24/7 后台主动代理

#### Week 14-15: 代理核心
- [ ] **Task 5.1**：设计 ProactiveAgent 架构
  - 定义 `ProactiveTask` 枚举
  - 定义 `TaskScheduler` trait
  - 定义任务执行逻辑（定时、触发式、批处理）
  - 编写单元测试（~200 行）

- [ ] **Task 5.2**：实现 TaskScheduler
  - 定时任务：每隔 N 分钟执行
  - 触发式任务：事件驱动（新记忆到达）
  - 批处理任务：定时批处理（~800 行）

- [ ] **Task 5.3**：实现自动分类任务
  - 新记忆自动分类（LLM + 规则）
  - 类别推荐（Top-K 类别）
  - 分类反馈学习（~600 行）

- [ ] **Task 5.4**：实现去重合并任务
  - 定期扫描重复记忆
  - 自动合并重复记忆
  - 合并冲突解决（~500 行）

#### Week 16: 摘要和优化
- [ ] **Task 5.5**：实现摘要生成任务
  - 类别摘要生成（LLM）
  - 摘要更新策略（增量更新）
  - 摘要质量评估（~400 行）

- [ ] **Task 5.6**：实现索引优化任务
  - 定期重建索引
  - 嵌入向量化（批量）
  - 查询性能优化（~300 行）

- [ ] **Task 5.7**：实现资源归档任务
  - 旧资源归档（压缩、冷存储）
  - 资源生命周期管理
  - 归档资源检索（~400 行）

- [ ] **Task 5.8**：集成测试
  - 端到端测试：主动任务执行
  - 性能测试：处理 10,000 个记忆 < 1 小时
  - 稳定性测试：连续运行 7 天无崩溃

**交付物**：
- ✅ `agent-mem-proactive` crate（~3,200 行代码）
- ✅ API 文档和集成测试
- ✅ 性能基准测试报告
- ✅ 稳定性测试报告

---

### 阶段 6：Integration 集成与迁移（第 17-19 周）

**目标**：新旧 API 双兼容，SDK 迁移，测试和文档

#### Week 17: API 兼容层
- [ ] **Task 6.1**：设计双 API 策略
  - 保留旧 API（`Memory::add`, `Memory::search`）
  - 新增新 API（`FileCentricMemory::mount`, `FileCentricMemory::navigate`）
  - 内部适配器层（旧 API → 新实现）（~400 行）

- [ ] **Task 6.2**：实现 API 适配器
  - `Memory::add` → `FileCentricMemory::add_memory`（自动创建虚拟 Resource）
  - `Memory::search` → `FileCentricMemory::search`（7 阶段检索）
  - 向后兼容性保证（~600 行）

- [ ] **Task 6.3**：API 迁移指南
  - 编写迁移指南文档
  - 提供迁移脚本
  - 提供最佳实践（~300 行文档）

#### Week 18: SDK 迁移
- [ ] **Task 6.4**：更新 Python SDK
  - 新增 Resource 和 Category API
  - 保持向后兼容
  - 更新示例代码（~400 行）

- [ ] **Task 6.5**：更新 JavaScript SDK
  - 新增 Resource 和 Category API
  - 保持向后兼容
  - 更新示例代码（~400 行）

- [ ] **Task 6.6**：更新 Go 和 Cangjie SDK
  - 新增 Resource 和 Category API
  - 保持向后兼容（~400 行）

#### Week 19: 测试和文档
- [ ] **Task 6.7**：全面集成测试
  - 回归测试：所有旧 API 测试通过
  - 新功能测试：所有新 API 测试通过
  - 性能测试：性能不降低（~1,000 行测试）

- [ ] **Task 6.8**：更新文档
  - 更新 README.md
  - 更新 API 文档
  - 更新用户指南
  - 更新开发者指南（~2,000 行文档）

- [ ] **Task 6.9**：性能验证
  - 基准测试：216K ops/sec 性能保持
  - 负载测试：10,000 并发用户
  - 稳定性测试：99.9% 可用性（性能报告）

**交付物**：
- ✅ 双 API 兼容层（~1,000 行代码）
- ✅ 更新的 SDK（Python, JS, Go, Cangjie）
- ✅ 完整的测试套件（~1,000 行测试）
- ✅ 完整的文档（~2,000 行文档）
- ✅ 性能验证报告

---

## 📊 五、成功指标与风险缓解

### 5.1 成功指标

#### 技术指标
| 指标 | 当前值 | 目标值 | 测量方法 |
|------|--------|--------|----------|
| **代码复用率** | - | ≥85% | 代码分析工具（Tokei） |
| **性能吞吐量** | 216K ops/sec | 保持不变 | 基准测试（Criterion.rs） |
| **检索延迟 P95** | <100ms | <150ms | 性能测试（Apache Bench） |
| **去重准确率** | - | ≥90% | 测试集评估 |
| **分类准确率** | - | ≥85% | 测试集评估 |
| **检索准确率** | - | ≥85% | 测试集评估 |
| **测试覆盖率** | - | ≥80% | Tarpaulin |

#### 功能指标
| 功能 | 状态 | 验收标准 |
|------|------|----------|
| **资源挂载** | 新增 | 支持文件、HTTP、对话、文档 4 种 URI |
| **类别导航** | 新增 | 支持 3 层以上类别层级 |
| **路径浏览** | 新增 | 支持路径解析、验证、遍历 |
| **摘要生成** | 新增 | LLM 生成摘要质量 > 4/5 |
| **主动整理** | 新增 | 24/7 自动分类、去重、摘要 |
| **双 API** | 新增 | 100% 向后兼容 |
| **SDK 更新** | 更新 | Python, JS, Go, Cangjie 全部支持 |

#### 用户体验指标
| 指标 | 当前值 | 目标值 | 测量方法 |
|------|--------|--------|----------|
| **学习曲线** | 中等 | 降低 30% | 用户调研 |
| **开发效率** | - | 提升 40% | 用户反馈 |
| **API 直观性** | - | 提升 50% | A/B 测试 |

### 5.2 风险识别与缓解

#### 风险 1：性能降低
**概率**: 中（30%）
**影响**: 高

**缓解措施**:
1. **性能基准**：改造前建立性能基准（216K ops/sec）
2. **持续监控**：每个阶段结束后运行性能测试
3. **优化策略**：
   - 并行执行（类别召回 + 记忆项召回）
   - 缓存策略（类别缓存、资源缓存）
   - 批处理（批量嵌入生成）
4. **回滚计划**：如果性能降低 >20%，回滚到旧版本

#### 风险 2：API 兼容性破坏
**概率**: 中（25%）
**影响**: 高

**缓解措施**:
1. **双 API 策略**：保留旧 API，新增新 API
2. **适配器层**：旧 API 内部调用新实现
3. **回归测试**：所有旧 API 测试必须通过
4. **版本管理**：使用语义化版本（SemVer）
5. **迁移指南**：提供详细的迁移指南和脚本

#### 风险 3：LLM 成本过高
**概率**: 中（30%）
**影响**: 中

**缓解措施**:
1. **充足度检查**：早期退出避免过度检索
2. **缓存策略**：缓存类别摘要、嵌入向量
3. **模型选择**：使用低成本模型（DeepSeek vs GPT-4）
4. **批处理**：批量调用 LLM 降低成本
5. **成本监控**：实时监控 LLM 成本，设置预算

#### 风险 4：类别层级爆炸
**概率**: 低（15%）
**影响**: 中

**缓解措施**:
1. **层级限制**：最多 5 层类别层级
2. **合并策略**：自动合并相似类别
3. **归档策略**：归档不活跃的类别
4. **限制创建**：限制用户创建类别数量（每用户最多 100 个）

#### 风险 5：主动代理失控
**概率**: 低（10%）
**影响**: 高

**缓解措施**:
1. **限流策略**：限制主动代理执行频率（最多 1 次/分钟）
2. **资源限制**：限制主动代理 CPU 和内存使用
3. **人工审核**：主动代理操作需要人工审核（可选）
4. **回滚机制**：主动代理操作可以回滚
5. **监控告警**：主动代理异常时立即告警

#### 风险 6：数据迁移失败
**概率**: 中（20%）
**影响**: 高

**缓解措施**:
1. **备份策略**：迁移前备份数据库
2. **灰度迁移**：先迁移 10% 用户测试
3. **迁移脚本**：编写幂等的迁移脚本
4. **回滚计划**：迁移失败时可以回滚
5. **验证测试**：迁移后验证数据完整性

#### 风险 7：开发延期
**概率**: 中（30%）
**影响**: 中

**缓解措施**:
1. **MVP 优先**：优先实现核心功能（资源、类别、提取）
2. **迭代交付**：每个阶段（2-3 周）交付可用版本
3. **并行开发**：4 个 crates 可以并行开发
4. **预留缓冲**：总工期 14-19 周，预留 5 周缓冲
5. **定期评审**：每周评审进度，及时调整

### 5.3 质量保证策略

#### 测试策略
1. **单元测试**：每个函数必须有单元测试（覆盖率 ≥80%）
2. **集成测试**：每个 crate 必须有集成测试
3. **端到端测试**：完整的用户场景测试
4. **性能测试**：每个阶段结束后运行性能测试
5. **回归测试**：所有旧 API 测试必须通过

#### 代码审查策略
1. **强制审查**：所有代码必须经过至少 1 人审查
2. **审查清单**：使用审查清单确保质量
3. **自动化检查**：使用 CI/CD 自动化检查（clippy, fmt, test）
4. **安全审查**：安全相关代码必须经过安全专家审查

#### 文档策略
1. **API 文档**：所有公开 API 必须有文档
2. **用户指南**：提供详细的用户指南
3. **开发者指南**：提供详细的开发者指南
4. **示例代码**：提供丰富的示例代码
5. **迁移指南**：提供详细的迁移指南

---

## 🔧 六、关键决策与理由

### 决策 1：双 API 策略

**问题**：是否完全抛弃旧 API，只提供新 API？

**选择**：**保留旧 API，新增新 API，内部适配器层**

**理由**：
1. **向后兼容**：现有用户无需修改代码
2. **渐进迁移**：用户可以逐步迁移到新 API
3. **降低风险**：如果新 API 有问题，旧 API 仍然可用
4. **维护成本**：适配器层代码 <1,000 行，维护成本可控

**替代方案**：
- ❌ 完全抛弃旧 API：破坏性变更，用户流失
- ❌ 只保留旧 API：无法提供文件核心能力

**置信度**：85/100

---

### 决策 2：Resource 存储 Blob vs 引用

**问题**：Resource 内容是否存储在数据库中（Blob）还是只存储引用？

**选择**：**小文件（<1MB）存储 Blob，大文件（≥1MB）存储引用 + 外部存储**

**理由**：
1. **性能**：小文件直接从数据库读取，避免 I/O
2. **成本**：大文件存储在对象存储（S3），降低数据库成本
3. **灵活性**：支持多种外部存储（本地文件系统、S3、Azure Blob）
4. **可扩展性**：大文件独立存储，易于扩展

**替代方案**：
- ❌ 全部存储 Blob：数据库膨胀，备份困难
- ❌ 全部存储引用：小文件性能差

**置信度**：80/100

---

### 决策 3：Category 多租户隔离

**问题**：Category 是否支持多租户（user_id, agent_id）？

**选择**：**支持多租户，Category 包含 user_id 和可选的 agent_id**

**理由**：
1. **数据隔离**：不同用户/代理的类别完全隔离
2. **个性化**：每个用户可以有自定义类别结构
3. **安全性**：防止跨用户/代理访问类别
4. **一致性**：与 MemoryItem 的多租户策略一致

**替代方案**：
- ❌ 全局共享类别：隐私问题，混乱
- ❌ 仅 user_id 隔离：代理无法有独立类别

**置信度**：90/100

---

### 决策 4：LLM 模型选择

**问题**：摘要生成和自动分类使用哪个 LLM？

**选择**：**DeepSeek-V3（主要）+ GPT-4o（备用）**

**理由**：
1. **成本**：DeepSeek 成本低（$0.14/1M tokens vs GPT-4 $30/1M tokens）
2. **性能**：DeepSeek 性能接近 GPT-4（基准测试 ~95%）
3. **中文支持**：DeepSeek 中文优化
4. **可用性**：DeepSeek API 稳定，限流宽松
5. **备用方案**：GPT-4o 作为备用，确保可用性

**替代方案**：
- ❌ 仅使用 GPT-4：成本过高（$300K/月 → $30K/月）
- ❌ 仅使用 DeepSeek：可用性风险

**置信度**：75/100

---

### 决策 5：类别层级表示

**问题**：类别层级使用什么数据结构？

**选择**：**物化路径（Materialized Path）+ 父指针（Parent Pointer）混合**

**理由**：
1. **物化路径**：路径如 `/1/2/3`，快速查询祖先和后代
2. **父指针**：parent_id 外键，快速查询父节点和子节点
3. **冗余设计**：两个字段冗余，性能和灵活性兼顾
4. **索引优化**：path 和 parent_id 都有索引

**数据结构**：
```sql
CREATE TABLE categories (
    id TEXT PRIMARY KEY,
    path TEXT NOT NULL,        -- "/1/2/3" (物化路径)
    parent_id TEXT,            -- "2" (父指针)
    INDEX idx_path (path),
    INDEX idx_parent_id (parent_id)
);
```

**替代方案**：
- ❌ 递归 CTE（Recursive CTE）：性能差（深度查询）
- ❌ 闭包表（Closure Table）：存储冗余，维护复杂
- ❌ 嵌套集（Nested Set）：插入/移动复杂

**置信度**：85/100

---

### 决策 6：主动代理触发策略

**问题**：主动代理何时执行整理任务？

**选择**：**混合策略：定时任务（每 5 分钟）+ 触发式任务（新记忆到达）+ 批处理任务（深夜）**

**理由**：
1. **定时任务**：定期整理，保持系统健康
2. **触发式任务**：新记忆立即分类，用户体验好
3. **批处理任务**：深夜批量处理摘要生成，降低负载
4. **灵活性**：不同任务使用不同触发策略

**任务分配**：
- 自动分类：触发式（新记忆到达时）
- 去重合并：定时（每 5 分钟）
- 摘要生成：批处理（深夜 2-4 点）
- 索引优化：定时（每天 3 点）
- 资源归档：定时（每周日凌晨）

**替代方案**：
- ❌ 仅定时任务：响应慢
- ❌ 仅触发式任务：负载高

**置信度**：80/100

---

## 📚 七、参考文档

### 内部文档
- `todo3.md` - AgentMem 文件核心改造详细计划（代码深度分析版，1331 行）
- `TODO_CN.md` - AgentMem 文件核心改造计划（中文完整版，360 行）
- `todo2.md` - AgentMem File-Centric Reform Plan（English Detailed Version, 670 lines）
- `README.md` - AgentMem 项目介绍
- `INSTALL.md` - 安装指南
- `CONTRIBUTING.md` - 贡献指南

### 外部参考
- **memU** - 内存即文件系统的记忆平台（设计灵感来源）
- **Mem0** - AgentMem 的兼容目标
- **LangChain Memory** - LLM 记忆管理最佳实践
- **ChromaDB/Qdrant** - 向量数据库设计参考

### 技术栈
- **Rust** - 核心语言（1.75+）
- **Tokio** - 异步运行时
- **SQLx** - 数据库 ORM
- **Serde** - 序列化/反序列化
- **Anyhow** - 错误处理
- **Tracing** - 日志和追踪

---

## 🎯 八、下一步行动

### 立即开始（本周）
1. ✅ **审查本计划**：团队审查 PROMPT.md，确认改造方向
2. ✅ **建立基准**：运行性能基准测试，建立当前性能基线
3. ✅ **设置环境**：创建开发分支 `feature-file-centric-memory`
4. ✅ **准备数据**：准备测试数据集（1000 个对话、100 个文档）

### 第 1-3 周（阶段 1）
1. **Task 1.1**：设计 Resource 数据结构（Week 1）
2. **Task 1.2**：设计数据库 Schema（Week 1）
3. **Task 1.3**：设计 API 接口（Week 1）
4. **Task 1.4**：实现 MediaTypeDetector（Week 2）
5. **Task 1.5**：实现 URIResolver（Week 2）
6. **Task 1.6**：实现 ResourceManager（Week 3）
7. **Task 1.7**：集成测试（Week 3）

### 关键里程碑
- **Week 3**：Resource 资源抽象层完成
- **Week 6**：Category 类别层级系统完成
- **Week 10**：Extraction 提取管道框架完成
- **Week 13**：Enhanced 增强检索系统完成
- **Week 16**：Proactive 主动代理完成
- **Week 19**：Integration 集成与迁移完成，发布 2.6.0

---

## 📞 九、联系信息

### 核心团队
- **项目负责人**：[待填写]
- **架构师**：[待填写]
- **开发团队**：[待填写]

### 沟通渠道
- **Slack**：#agentmem-development
- **邮件**：agentmem-team@example.com
- **会议**：每周三 10:00 AM 进度评审

### 问题反馈
- **GitHub Issues**：https://github.com/louloulin/agentmem/issues
- **GitHub Discussions**：https://github.com/louloulin/agentmem/discussions

---

**祝改造成功！🚀**

*AgentMem 核心开发团队*
*2026-03-01*
