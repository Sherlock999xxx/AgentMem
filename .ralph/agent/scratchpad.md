# AgentMem 改造计划 - Scratchpad

## 日期: 2026-03-01

## 当前理解

### 目标
全面分析 AgentMem 记忆平台，制定完善改造计划，并写入 PROMPT.md（中文说明）。

### 已完成的工作
1. ✅ **代码分析完成** - 772 个 Rust 文件，101K LOC 核心引擎
2. ✅ **差距分析完成** - AgentMem vs memU 对比（mem-1772346713-06b6）
3. ✅ **改造计划完成** - 6阶段 14-19周详细路线图
4. ✅ **详细文档创建**：
   - `todo3.md` (1331行，中文深度代码分析版)
   - `TODO_CN.md` (360行，中文完整版)
   - `todo2.md` (670行，英文详细版)
   - `agentmem-reform-summary.md` (执行摘要)

### 待完成任务
1. ❌ **创建 PROMPT.md** - 将改造计划整合为中文开发指南
2. ⏳ **Resource 数据模型设计** - 下一个实施任务

### 关键发现

#### memU 的设计优势（需要采纳）
- 文件系统隐喻（Categories=文件夹，MemoryItems=文件，Resources=挂载点）
- 资源抽象层（所有记忆源自可挂载资源）
- 类别层级（按主题浏览，而非按类型分类）
- 充足度检查（早期退出避免过度检索）
- 主动智能（24/7后台代理自动整理）

#### AgentMem 的技术优势（必须保留）
- 高性能引擎（216K ops/sec）
- 8个专业代理（Core, Episodic, Semantic, Procedural, Working, Resource, Knowledge, Contextual）
- 5种搜索引擎（Vector, BM25, Full-Text, Fuzzy, RRF）
- 30+ 存储后端支持
- 20+ LLM 提供商集成
- 企业特性（RBAC, JWT, 审计日志）
- 多语言 SDK（Python, JavaScript, Go, Cangjie）

#### 改造策略
**保留 85% 代码**（核心引擎、专业代理、搜索引擎、存储后端、LLM 集成、SDK）
**新增 4 个 crates**（~5K LOC）：
- agent-mem-resource（资源抽象）
- agent-mem-category（类别系统）
- agent-mem-extraction（提取管道）
- agent-mem-proactive（主动代理）

**重构 15% 代码**（MemoryType → Category，类型分发 → 类别路由，5阶段检索 → 7阶段检索）

### 6 阶段实施路线图

#### 阶段 1: Resource 资源抽象层（第 1-3 周）
- 设计 Resource 数据模型
- 实现 MediaType 检测和 URI 解析
- 创建 ResourceManager 组件

#### 阶段 2: Category 类别层级系统（第 4-6 周）
- 创建 Category 数据模型和层级结构
- 实现 CategoryManager 和路径导航
- 开发类别摘要生成

#### 阶段 3: Extraction 提取管道框架（第 7-10 周）
- 构建 ExtractionPipeline 框架
- 实现内容提取器（对话/文档/图片/音频）
- 开发去重与合并逻辑

#### 阶段 4: Enhanced 增强检索系统（第 11-13 周）
- 实现类别召回（category_recall）
- 添加资源召回（resource_recall）
- 开发充足度检查（sufficiency_check）

#### 阶段 5: Proactive 主动代理（第 14-16 周）
- 开发 ProactiveAgent 核心引擎
- 实现自动分类和去重合并
- 创建摘要生成和定时任务

#### 阶段 6: Integration 集成与迁移（第 17-19 周）
- 新旧 API 双兼容层
- SDK 迁移和文档更新
- 测试和性能验证

### 下一步行动
创建 PROMPT.md 文件，整合所有分析内容为简洁的中文开发指南。

