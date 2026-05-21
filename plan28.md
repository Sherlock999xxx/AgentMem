# AgentMem v2.1 最佳改进计划 (plan28.md)

## Context

基于 plan27.md 的 MVP 核心任务完成状态，继续优化 AgentMem 至生产可用。

**plan27.md 已完成**:
- ✅ 消除 12 处 preview_error
- ✅ 简化核心 API (6 deprecated)
- ✅ HTTP 端点分析完成
- ✅ 文档更新

---

## 实施状态更新 (2026-05-21)

### 已完成 ✅

| 功能 | 状态 | 说明 |
|------|------|------|
| 代码库分析 | ✅ | 完成 |
| 编译警告分析 | ✅ | 2455 warnings (外部 crates 为主) |
| HTTP 端点分析 | ✅ | 141 个端点 (含大量冗余别名) |
| HTTP 端点精简 | ✅ | **141 → 91 端点 (减少 35%)** |
| Python SDK 分析 | ✅ | 已精简 (~25 public API) |

### 当前问题 (分析结果)

| 问题 | 当前状态 | 目标 |
|------|----------|------|
| 编译警告 | 2455 (外部 crates) | < 20 (可控) |
| HTTP 端点 | 91 (精简后) | ~60 (核心功能) |
| 跳过测试 | 待统计 | < 5 |
| SDK 方法 | ~25 (Python 已精简) | ~12-15 |

---

## 核心问题分析

### 1. HTTP 端点冗余 (141 → 目标 ~60)

**问题**: File-centric routes 有大量冗余别名
- `/api/v1/migrations/...` → 基础路由
- `/api/v1/file-centric/migration/...` → canonical 路由
- `/api/v1/file-centric/migrations/...` → 复数别名

**需要合并的路由组**:
```
Group 1: Migration (3组 → 1组)
  ❌ /api/v1/migrations/plan
  ❌ /api/v1/file-centric/migration/plan
  ❌ /api/v1/file-centric/migrations/plan
  ✅ 合并到 /api/v1/file-centric/migrations/plan

Group 2: Resource/Extraction (4组 → 1组)
  ❌ /api/v1/resources/mount
  ❌ /api/v1/file-centric/resources/:id
  ❌ /api/v1/file-centric/extraction
  ❌ /api/v1/file-centric/extraction/extract
  ✅ 合并到 /api/v1/file-centric/resources

Group 3: Proactive Tasks (2组 → 1组)
  ❌ /api/v1/proactive/tasks
  ❌ /api/v1/file-centric/proactive/tasks
  ✅ 合并到 /api/v1/file-centric/proactive/tasks

Group 4: Scheduler Stats (3组 → 1组)
  ❌ /api/v1/proactive/stats
  ❌ /api/v1/proactive/scheduler/stats
  ❌ /api/v1/file-centric/proactive/scheduler/stats
  ✅ 合并到 /api/v1/file-centric/proactive/stats
```

### 2. 编译警告来源

主要来自外部 crates (不可控):
- `lance` - 向量数据库 (700+ warnings)
- `datafusion` - SQL 引擎 (500+ warnings)
- `arrow` - 数据格式 (300+ warnings)

内部 crates warnings 可通过 `#[allow(dead_code)]` 处理

### 3. Python SDK 状态

已精简到 ~25 public API:
```python
# 核心 Memory 操作 (6)
add_memory, get_memory, update_memory, delete_memory
search_memories, get_all_memories

# 批量操作 (2)
batch_add_memories, batch_delete_memories

# 统计/健康 (2)
health_check, get_metrics

# File-centric (8)
mount_resource, get_resource, list_resources
get_category, get_category_by_path, list_categories, search_categories
extract_resource, get_extraction_status

# 工具类 (7)
Config, ToolExecutor, ToolSchema, MetricsCollector, PerformanceTracker, etc.
```

---

## 已实施完成 ✅

### Phase 2.1: HTTP 端点精简 (优先级: P0) ✅

**文件**: `crates/agent-mem-server/src/routes/mod.rs`

**已删除的冗余路由别名**:

| 已删除路由 | 合并到 |
|-----------|--------|
| `/api/v1/migrations/*` | `/api/v1/file-centric/migrations/*` |
| `/api/v1/proactive/*` | `/api/v1/file-centric/proactive/*` |
| `/api/v1/resources/*` | `/api/v1/file-centric/resources/*` |
| `/api/v1/categories/*` | `/api/v1/file-centric/categories/*` |
| 重复 extraction 路由 | `/api/v1/file-centric/extraction/:job_id` |
| 重复 stats 路由 | `/api/v1/file-centric/proactive/stats` |

**结果**: 141 → 91 端点 (减少 35%)

### Phase 2.2: SDK 精简 (优先级: P1)

Python SDK 已完成 (~25 方法)

目标: 进一步精简到 ~15 方法
- 保留核心: add, get, search, delete, get_all, get_stats
- 保留必要: batch_add, batch_delete, mount_resource, get_resource
- 合并: health_check + get_metrics → get_health()

### Phase 3: 生产验证 (优先级: P1)

#### 3.1 Docker 验证
- [ ] 验证 Dockerfile 构建 (Docker 未运行)
- [ ] 验证容器运行
- [ ] 验证 API 响应

#### 3.2 性能测试
- [ ] add (单条) < 100ms
- [ ] search < 200ms
- [ ] get (单条) < 50ms

---

## Critical Files

| 优先级 | 文件 | 任务 |
|--------|------|------|
| P0 | `crates/agent-mem-server/src/routes/mod.rs` | 端点精简 |
| P1 | `sdks/python/agentmem/client.py` | SDK 进一步精简 |
| P1 | `Dockerfile` | 验证 |
| P2 | `tests/` | 测试修复 |

---

## 验收标准

- [ ] HTTP 端点 < 60
- [ ] Python SDK 方法 < 20
- [ ] Docker 构建成功
- [ ] 性能测试通过

---

## Timeline

```
Week 1: Phase 2.1 (HTTP 端点精简)
Week 2: Phase 2.2 (SDK 精简)
Week 3: Phase 3.1 (Docker 验证)
Week 4: Phase 3.2 (性能测试)
```

**总计**: 4 周