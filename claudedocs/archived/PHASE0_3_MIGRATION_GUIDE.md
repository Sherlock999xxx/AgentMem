# Phase 0.3 错误处理迁移指南

> **创建日期**: 2026-01-23
> **目的**: 提供系统化的 unwrap/expect 替换指南

---

## 📋 概述

本指南提供了如何系统化地替换 AgentMem 代码库中的 unwrap/expect 调用的详细说明和示例。

### 迁移策略

我们将使用新的 `error_handling` 模块中提供的辅助函数,逐步替换所有不安全的 unwrap/expect 调用。

---

## 🎯 迁移模式

### 模式 1: Mutex/RwLock 锁操作

**❌ Before (Unsafe)**

```rust
// 直接使用 unwrap(),可能 panic
let data = self.mutex.lock().unwrap();
let data = self.rwlock.read().unwrap();
let mut data = self.rwlock.write().unwrap();
```

**✅ After (Safe)**

```rust
// 使用 error_handling 模块的辅助函数
use crate::error_handling::{safe_lock, safe_read, safe_write};

let data = safe_lock(&self.mutex, "data_cache")?;
let data = safe_read(&self.rwlock, "config_data")?;
let mut data = safe_write(&self.rwlock, "shared_state")?;
```

**优势**:
- ✅ 自动处理 poisoning 错误
- ✅ 提供清晰的上下文信息
- ✅ 返回 CoreError 而非 panic

---

### 模式 2: 必需配置字段

**❌ Before (Unsafe)**

```rust
// 配置字段缺失时 panic
let api_key = self.config.api_key.as_ref().unwrap();
let db_url = self.config.database_url.as_ref().unwrap();
```

**✅ After (Safe)**

```rust
// 使用 require_config 或 require_some
use crate::error_handling::{require_config, require_some};

let api_key = require_config(self.config.api_key.clone(), "api_key")?;
let db_url = require_some(self.config.database_url.as_ref(), "database_url")?;
```

**优势**:
- ✅ 明确的错误类型
- ✅ 友好的错误消息
- ✅ 编译时检查

---

### 模式 3: Option 解包(带默认值)

**❌ Before (Unsafe)**

```rust
// 使用 unwrap_or
let timeout = config.timeout.unwrap_or(30);
let retries = config.retries.unwrap_or(3);
```

**✅ After (Safe)**

```rust
// 使用 error_handling 辅助函数
use crate::error_handling::unwrap_or_default;

let timeout = unwrap_or_default(config.timeout, 30);
let retries = unwrap_or_default(config.retries, 3);
```

或者使用 `unwrap_or_else`:

```rust
let timeout = unwrap_or_else(config.timeout, || calculate_default_timeout());
```

**优势**:
- ✅ 语义更清晰
- ✅ 支持延迟计算默认值
- ✅ 代码一致性更好

---

### 模式 4: Regex 编译

**❌ Before (Unsafe)**

```rust
// 静态模式使用 unwrap()
let regex = Regex::new(r"^\d+$").unwrap();
let email_regex = Regex::new(EMAIL_PATTERN).unwrap();
```

**✅ After (Safe - 静态模式)**

```rust
// 对于静态、已知有效的模式,使用 unwrap() 是可接受的
// 但添加注释说明安全性

// SAFETY: Static pattern verified at compile time
let regex = Regex::new(r"^\d+$").unwrap();
```

**✅ After (Safe - 动态模式)**

```rust
// 对于动态模式,使用 compile_regex
use crate::error_handling::compile_regex;

let user_pattern = get_user_pattern();
let regex = compile_regex(&user_pattern)?;
```

**优势**:
- ✅ 静态模式保持性能
- ✅ 动态模式有错误处理
- ✅ 代码注释说明安全性

---

### 模式 5: 测试代码

**✅ Acceptable (测试中)**

```rust
#[test]
fn test_something() {
    let value = some_function().unwrap();  // ✅ OK in tests
    assert_eq!(value, expected);
}
```

**策略**: 测试代码中的 unwrap() 是可接受的,因为:
- 测试失败应该 panic
- 简化测试代码
- 不影响生产代码

---

## 📊 迁移清单

### Phase 0.3.1: P0 修复 (1 周)

**文件优先级**:

1. **API 层**
   - [ ] `client.rs` - 配置字段验证
   - [ ] `api/*.rs` - 错误处理

2. **存储层**
   - [ ] `storage/*.rs` - Lock 操作
   - [ ] `cache/*.rs` - Lock 操作

3. **配置**
   - [ ] `config.rs` - 配置验证
   - [ ] `config_env.rs` - 环境变量验证

**验收标准**:
- ✅ P0 模块无 unwrap/expect
- ✅ 编译通过
- ✅ 测试通过

### Phase 0.3.2: P1 修复 (1 周)

**文件优先级**:

1. **业务逻辑**
   - [ ] `manager.rs`
   - [ ] `engine.rs`
   - [ ] `operations.rs`

2. **数据处理**
   - [ ] `search/*.rs`
   - [ ] `retrieval/*.rs`

**验收标准**:
- ✅ P1 模块无 unwrap/expect
- ✅ 编译通过
- ✅ 测试通过

### Phase 0.3.3: P2 评估 (0.5 周)

**文件优先级**:

1. **工具函数**
   - [ ] 评估 Regex unwrap()
   - [ ] 添加注释说明

2. **测试代码**
   - [ ] 标记测试 unwrap()
   - [ ] 保持现状

**验收标准**:
- ✅ P2 unwrap/expect 已评估
- ✅ 代码注释完整

---

## 🛠️ 实施步骤

### 1. 添加依赖

确保 `error_handling` 模块已在 `lib.rs` 中导出:

```rust
pub mod error_handling;
```

### 2. 导入辅助函数

在需要使用的文件中:

```rust
use crate::error_handling::{
    safe_lock, safe_read, safe_write,
    require_some, require_config,
    unwrap_or_default, unwrap_or_else,
    compile_regex,
};
```

### 3. 替换 unwrap/expect

按照上述模式逐一替换:

```rust
// Before
let data = self.mutex.lock().unwrap();

// After
let data = safe_lock(&self.mutex, "context")?;
```

### 4. 编译和测试

```bash
# 编译检查
cargo build --package agent-mem-core

# 运行测试
cargo test --package agent-mem-core

# Clippy 检查
cargo clippy --package agent-mem-core -- -W clippy::unwrap_used
```

### 5. 提交代码

```bash
git add .
git commit -m "fix(security): Replace unwrap/expect with proper error handling

- Replace lock().unwrap() with safe_lock/safe_read/safe_write
- Replace config.unwrap() with require_config/require_some
- Add proper error messages and context
- Reduce unwrap/expect usage by ~200 instances

Phase 0.3.1: P0 error handling fixes"
```

---

## 📈 预期成果

### 统计目标

| 阶段 | 修复前 | 修复后 | 减少 |
|------|--------|--------|------|
| **Phase 0.3.1** | 356 | ~226 | -130 |
| **Phase 0.3.2** | ~226 | ~106 | -120 |
| **Phase 0.3.3** | ~106 | ~66 | -40 |
| **总计** | 356 | ~66 | **-81%** |

### 最终状态 (~66 处保留)

- Regex 编译: ~40 (静态模式,安全)
- 测试代码: ~26 (可接受)

---

## ✅ 验证标准

### 代码质量

- [ ] `cargo clippy -W clippy::unwrap_used` 在 P0/P1 代码无警告
- [ ] `cargo build` 成功
- [ ] `cargo test` 通过

### 安全性

- [ ] 零可能的生产 panic (P0/P1)
- [ ] 所有错误都有清晰的上下文
- [ ] 所有锁操作都有错误处理

### 文档

- [ ] 保留的 unwrap() 有注释说明
- [ ] 新代码有使用示例
- [ ] 迁移指南完整

---

## 🎯 下一步

1. **立即行动**: 开始 Phase 0.3.1 - P0 修复
2. **工具**: 使用 `cargo clippy -W clippy::unwrap_used` 定位问题
3. **参考**: 使用 `error_handling` 模块的辅助函数

---

**指南版本**: 1.0
**状态**: 准备实施
**预计完成**: 2-3 周
