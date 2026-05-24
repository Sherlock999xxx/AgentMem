# AgentMem 1.1 快速开始指南

**版本**: 1.1
**更新日期**: 2026-01-22
**状态**: P0 阶段已完成 ✅

---

## 🚀 快速开始

### 1. 安装

```bash
# 添加依赖到 Cargo.toml
[dependencies]
agent-mem = "2.0"
```

### 2. 基本使用

```rust
use agent_mem::Memory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 Memory 实例 (自动启用所有 P0 优化)
    let memory = Memory::new_core().await?;

    // 添加记忆
    let id = memory.add("AgentMem 是一个企业级 AI 记忆管理平台").await?;
    println!("记忆 ID: {}", id);

    // 搜索记忆
    let results = memory.search("企业级 AI").await?;
    println!("找到 {} 条相关记忆", results.len());

    Ok(())
}
```

---

## ✅ P0 阶段优化 (已启用)

### 1. 批量数据库插入 ✅

**性能提升**: 2-3x

使用多行 SQL INSERT,单次事务提交:

```rust
// 批量添加 (自动使用优化后的批量插入)
let contents = vec![
    "记忆 1".to_string(),
    "记忆 2".to_string(),
    "记忆 3".to_string(),
];

let results = memory.add_batch(contents, Default::default()).await?;
println!("批量添加 {} 条记忆", results.len());
```

### 2. 批量嵌入生成 ✅

**性能提升**: 5-10x

使用 `embed_batch()` API,一次性生成所有嵌入:

```rust
// 自动使用批量嵌入生成
let ids = memory.add_batch(vec![...], Default::default()).await?;
// 内部调用: embedder.embed_batch(&contents)
```

### 3. 嵌入缓存 ✅ 🆕

**性能提升**: 2-5x (缓存命中时)

LRU 缓存 + TTL,自动缓存重复内容:

```rust
// 默认启用缓存,无需额外配置
let memory = Memory::new_core().await?;

// 第一次: 生成嵌入 (缓存未命中)
memory.add("重复内容").await?;

// 第二次: 从缓存返回 (缓存命中) ⚡
memory.add("重复内容").await?;
```

**自定义缓存配置**:

```rust
use agent_mem::{Memory, OrchestratorConfig};

let config = OrchestratorConfig {
    enable_embedder_cache: Some(true),     // 启用缓存
    embedder_cache_size: Some(2000),        // 缓存 2000 个嵌入
    embedder_cache_ttl_secs: Some(7200),    // TTL 2 小时
    ..Default::default()
};

let memory = Memory::new_with_config(config).await?;
```

### 4. 连接池 ✅

**性能提升**: 3-5x

PostgreSQL 和 LibSQL 连接池,自动管理连接:

```rust
// PostgreSQL 连接池 (自动启用)
let config = OrchestratorConfig {
    storage_url: Some("postgresql://user:pass@localhost/db".to_string()),
    ..Default::default()
};

let memory = Memory::new_with_config(config).await?;
// 内部使用: PgPoolOptions::new().max_connections(100)
```

---

## 📊 性能对比

| 优化项 | 基准性能 | 优化后性能 | 提升倍数 |
|-------|---------|-----------|---------|
| **基准** | 54.95 ops/s | - | - |
| **批量插入** | - | 136.84 items/s | 2.5x |
| **批量嵌入** | - | - | 5-10x |
| **连接池** | - | - | 3-5x |
| **嵌入缓存** | - | - | 2-5x |
| **综合效果** | 54.95 ops/s | **404.5 ops/s** | **7.36x** |

**预期性能** (启用所有优化 + 缓存命中):
- 保守: 809 ops/s (2x 缓存提升)
- 乐观: 2,022.5 ops/s (5x 缓存提升)

---

## 🔧 高级配置

### 完整配置示例

```rust
use agent_mem::{Memory, OrchestratorConfig};

let config = OrchestratorConfig {
    // 存储配置
    storage_url: Some("postgresql://user:pass@localhost/db".to_string()),

    // 嵌入器配置
    embedder_provider: Some("fastembed".to_string()),
    embedder_model: Some("multilingual-e5-small".to_string()),

    // 向量存储配置
    vector_store_url: Some("./data/vectors".to_string()),

    // P0 优化配置
    enable_embedding_queue: Some(true),           // 批量嵌入队列
    embedding_batch_size: Some(64),               // 批处理大小
    embedding_batch_interval_ms: Some(20),        // 批处理间隔

    // P0 缓存配置 (默认启用)
    enable_embedder_cache: Some(true),            // 启用嵌入缓存
    embedder_cache_size: Some(1000),              // 缓存大小
    embedder_cache_ttl_secs: Some(3600),          // TTL (秒)

    // 智能功能
    enable_intelligent_features: false,           // 禁用智能功能 (更快)

    ..Default::default()
};

let memory = Memory::new_with_config(config).await?;
```

### 性能模式配置

**最高性能** (适合高吞吐场景):

```rust
let config = OrchestratorConfig {
    enable_intelligent_features: false,           // 禁用智能功能
    enable_embedder_cache: Some(true),            // 启用缓存
    embedder_cache_size: Some(2000),              // 更大缓存
    embedding_batch_size: Some(128),              // 更大批处理
    ..Default::default()
};
```

**平衡模式** (默认配置):

```rust
let config = OrchestratorConfig::default();
```

**功能完整** (适合需要智能分析):

```rust
let config = OrchestratorConfig {
    enable_intelligent_features: true,            // 启用智能功能
    enable_embedder_cache: Some(true),            // 启用缓存
    ..Default::default()
};
```

---

## 📈 性能测试

### 运行性能测试

```bash
# CachedEmbedder 性能测试
cargo run --example cached_embedder_perf_test

# 批量操作性能测试
cargo run --example batch_mode_benchmark

# 完整性能基准
cargo bench --bench memory_operations
```

### 预期结果

**CachedEmbedder 测试**:
- 单条嵌入 (缓存命中): < 5ms
- 批量嵌入 (100 条,缓存命中): < 100ms
- 性能提升: 2-5x

---

## 🎯 下一步

### 学习资源

- [CachedEmbedder 使用指南](./cached_embedder_guide.md)
- [API 参考文档](../api/)
- [性能优化文档](../performance/)

### 常见用例

```rust
// 1. FAQ 系统 (高重复内容)
let faq_memory = Memory::new_core().await?;
faq_memory.add("如何重置密码?").await?;
// 缓存命中率: 90%+, 性能提升: 5x

// 2. 聊天机器人 (中等重复)
let chat_memory = Memory::new_core().await?;
// 缓存命中率: 60-70%, 性能提升: 2-3x

// 3. 实时数据处理 (低重复)
let stream_memory = Memory::new_core().await?;
// 缓存命中率: 20-30%, 性能提升: 1.3-1.5x
```

---

## ⚠️ 注意事项

1. **内存使用**: 缓存会占用额外内存,默认 1000 个嵌入向量约 4-8 MB

2. **TTL 设置**: 根据内容变化频率调整 TTL,避免返回过时数据

3. **并发安全**: 缓存是线程安全的,可以安全地在多线程环境中使用

4. **测试环境**: 禁用智能功能可以获得更纯粹的性能测试结果

---

## 🐛 故障排查

### 问题 1: 性能提升不明显

**可能原因**:
- 缓存命中率低 (< 30%)
- 内容重复度低

**解决方案**:
- 分析内容重复度
- 增加缓存大小
- 延长 TTL

### 问题 2: 内存占用过高

**解决方案**:
```rust
embedder_cache_size: Some(500),  // 减小缓存
embedder_cache_ttl_secs: Some(1800),  // 缩短 TTL
```

### 问题 3: 编译错误

**确保启用了必要的 features**:
```toml
[dependencies]
agent-mem = { version = "2.0", features = ["fastembed"] }
```

---

## 📚 相关文档

- [完整 API 文档](../api/)
- [性能优化指南](../performance/)
- [部署指南](../deployment/)

---

**文档维护**: AgentMem Team
**最后更新**: 2026-01-22
