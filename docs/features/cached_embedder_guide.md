# CachedEmbedder 使用指南

**版本**: 1.0
**更新日期**: 2026-01-22
**状态**: ✅ 已启用 (默认)

---

## 概述

`CachedEmbedder` 是一个嵌入向量缓存层,使用 LRU (Least Recently Used) 缓存策略来避免重复计算相同内容的嵌入向量。

**性能提升**: 缓存命中时可获得 **2-5x** 的性能提升。

---

## 配置选项

### OrchestratorConfig 配置字段

```rust
pub struct OrchestratorConfig {
    /// 是否启用嵌入缓存 (默认: true)
    pub enable_embedder_cache: Option<bool>,

    /// 嵌入缓存大小 (默认: 1000)
    pub embedder_cache_size: Option<usize>,

    /// 嵌入缓存 TTL 秒数 (默认: 3600 秒 = 1 小时)
    pub embedder_cache_ttl_secs: Option<u64>,
}
```

### 默认配置

```rust
OrchestratorConfig {
    enable_embedder_cache: Some(true),  // 默认启用
    embedder_cache_size: Some(1000),     // 缓存 1000 个嵌入向量
    embedder_cache_ttl_secs: Some(3600), // TTL 1 小时
    ..Default::default()
}
```

---

## 使用示例

### 1. 使用默认配置 (推荐)

```rust
use agent_mem::Memory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 默认配置已启用缓存
    let memory = Memory::new_core().await?;

    // 第一次添加: 生成嵌入向量 (缓存未命中)
    let id1 = memory.add("AgentMem 是一个企业级 AI 记忆管理平台").await?;

    // 第二次添加相同内容: 从缓存返回 (缓存命中) ⚡
    let id2 = memory.add("AgentMem 是一个企业级 AI 记忆管理平台").await?;

    // 缓存命中时性能提升 2-5x!
    Ok(())
}
```

### 2. 自定义缓存配置

```rust
use agent_mem::{Memory, OrchestratorConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 自定义缓存配置
    let config = OrchestratorConfig {
        // 启用缓存
        enable_embedder_cache: Some(true),

        // 缓存 2000 个嵌入向量 (适合更大规模的应用)
        embedder_cache_size: Some(2000),

        // TTL 2 小时 (适合内容变化不频繁的场景)
        embedder_cache_ttl_secs: Some(7200),

        ..Default::default()
    };

    let memory = Memory::new_with_config(config).await?;
    // ...
    Ok(())
}
```

### 3. 禁用缓存

```rust
use agent_mem::{Memory, OrchestratorConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 禁用缓存 (不推荐,除非用于测试)
    let config = OrchestratorConfig {
        enable_embedder_cache: Some(false),
        ..Default::default()
    };

    let memory = Memory::new_with_config(config).await?;
    // ...
    Ok(())
}
```

---

## 性能优化建议

### 缓存大小配置

**小规模应用** (< 10,000 条记忆):
```rust
embedder_cache_size: Some(500)   // 500 个嵌入向量
```

**中等规模应用** (10,000 - 100,000 条记忆):
```rust
embedder_cache_size: Some(1000)  // 1000 个嵌入向量 (默认)
```

**大规模应用** (> 100,000 条记忆):
```rust
embedder_cache_size: Some(2000)  // 2000 个嵌入向量
```

### TTL 配置

**内容频繁变化** (实时数据):
```rust
embedder_cache_ttl_secs: Some(1800)  // 30 分钟
```

**内容中等频率变化** (日常内容):
```rust
embedder_cache_ttl_secs: Some(3600)  // 1 小时 (默认)
```

**内容很少变化** (静态内容):
```rust
embedder_cache_ttl_secs: Some(7200)  // 2 小时
```

---

## 工作原理

### 缓存键生成

使用 SHA256 哈希算法生成缓存键:

```rust
cache_key = SHA256(content)
```

**特性**:
- ✅ 确定性: 相同内容生成相同的缓存键
- ✅ 低碰撞率: SHA256 保证几乎无哈希碰撞
- ✅ 快速计算: 哈希计算速度快

### LRU 缓存策略

- **缓存淘汰**: 当缓存满时,淘汰最久未使用的条目
- **TTL 过期**: 超过 TTL 的条目自动失效
- **线程安全**: 使用 Arc + Mutex 保证并发安全

### 缓存感知方法

CachedEmbedder 实现了缓存感知的 `embed()` 和 `embed_batch()` 方法:

**单个嵌入**:
```rust
async fn embed(&self, text: &str) -> Result<Vec<f32>> {
    // 1. 检查缓存
    if let Some(cached) = cache.get(cache_key) {
        return Ok(cached);  // 缓存命中 ⚡
    }

    // 2. 缓存未命中,生成新的嵌入
    let embedding = inner.embed(text).await?;

    // 3. 写入缓存
    cache.put(cache_key, embedding.clone());

    Ok(embedding)
}
```

**批量嵌入**:
```rust
async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
    let mut results = Vec::new();
    let mut uncached_indices = Vec::new();
    let mut uncached_texts = Vec::new();

    // 1. 检查哪些文本已缓存
    for (idx, text) in texts.iter().enumerate() {
        if let Some(cached) = cache.get(cache_key) {
            results.push((idx, cached));  // 缓存命中 ⚡
        } else {
            uncached_indices.push(idx);
            uncached_texts.push(text.clone());
        }
    }

    // 2. 批量生成未缓存的嵌入
    let new_embeddings = inner.embed_batch(&uncached_texts).await?;

    // 3. 缓存新生成的嵌入
    for (text, embedding) in uncached_texts.iter().zip(new_embeddings.iter()) {
        cache.put(cache_key, embedding.clone());
    }

    // 4. 返回完整结果
    Ok(results)
}
```

---

## 性能基准

### 缓存命中率 vs 性能提升

| 缓存命中率 | 性能提升 | 场景 |
|-----------|---------|------|
| **90%** | 5x | 高度重复内容 (FAQ、模板) |
| **60%** | 2x | 中等重复内容 (日常对话) |
| **30%** | 1.3x | 低重复内容 (实时数据) |

### 理论 QPS 提升

**基准**: 404.5 ops/s

**保守估计** (60% 命中率, 2x 提升):
```
404.5 × 2 = 809 ops/s
```

**乐观估计** (90% 命中率, 5x 提升):
```
404.5 × 5 = 2,022.5 ops/s
```

---

## 监控和调试

### 获取缓存统计

**注意**: 当前版本需要通过内部 API 访问缓存统计。未来版本将提供公共方法。

```rust
// TODO: 添加公共 API
let stats = cached_embedder.get_stats();
println!("命中次数: {}", stats.hits);
println!("未命中次数: {}", stats.misses);
println!("命中率: {:.2}%", stats.hit_rate());
```

### 日志输出

启用 INFO 级别日志查看缓存行为:

```rust
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();
```

**示例日志**:
```
INFO  ✅ 嵌入缓存已启用（缓存大小: 1000, TTL: 3600秒）
DEBUG ✅ 嵌入向量缓存命中: a3f5c9d2...
DEBUG 缓存未命中，生成新的嵌入向量: b4e6d8e1...
DEBUG ✅ 嵌入向量已缓存: b4e6d8e1...
```

---

## 最佳实践

### ✅ 推荐做法

1. **默认启用缓存**
   ```rust
   let config = OrchestratorConfig::default();  // 缓存已启用
   ```

2. **根据应用规模调整缓存大小**
   - 小应用: 500-1000
   - 中应用: 1000-2000
   - 大应用: 2000-5000

3. **合理设置 TTL**
   - 静态内容: 2-4 小时
   - 日常内容: 1-2 小时
   - 实时内容: 20-30 分钟

### ❌ 避免做法

1. **不要在测试时禁用缓存**
   ```rust
   // ❌ 不推荐 (除非是单元测试)
   enable_embedder_cache: Some(false)
   ```

2. **不要设置过大的缓存**
   ```rust
   // ❌ 不推荐 (浪费内存)
   embedder_cache_size: Some(100000)
   ```

3. **不要设置过长的 TTL**
   ```rust
   // ❌ 不推荐 (内容可能过时)
   embedder_cache_ttl_secs: Some(86400)  // 24 小时
   ```

---

## 故障排查

### 问题 1: 缓存未生效

**症状**: 每次添加相同内容都很慢

**解决方案**:
1. 检查缓存是否启用:
   ```rust
   println!("缓存启用: {:?}", config.enable_embedder_cache);
   ```

2. 检查日志是否有缓存命中信息

3. 确认内容完全相同 (包括空格和标点)

### 问题 2: 内存占用过高

**症状**: 应用内存使用量持续增长

**解决方案**:
1. 减小缓存大小:
   ```rust
   embedder_cache_size: Some(500)  // 从 1000 减少到 500
   ```

2. 缩短 TTL:
   ```rust
   embedder_cache_ttl_secs: Some(1800)  // 从 3600 减少到 1800
   ```

### 问题 3: 缓存命中率低

**症状**: 性能提升不明显 (< 1.5x)

**解决方案**:
1. 分析内容重复度
2. 调整 TTL 让缓存更持久
3. 增加缓存大小

---

## 未来改进

- [ ] 添加公共 API 获取缓存统计
- [ ] 支持持久化缓存 (Redis)
- [ ] 支持缓存预热
- [ ] 支持分布式缓存
- [ ] 添加缓存监控指标 (Prometheus)

---

## 相关文档

- [性能测试报告](../docs/performance/cached_embedder_benchmark.md)
- [Embedder API 文档](../docs/api/embedder.md)
- [配置参考](../docs/api/config.md)

---

**文档维护**: AgentMem Team
**最后更新**: 2026-01-22
