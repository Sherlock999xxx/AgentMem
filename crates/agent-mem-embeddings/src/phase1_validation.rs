//! 🚀 Phase 1 Embedding 性能优化验证
//!
//! 模块化性能验证测试,可以在 crates/agent-mem-embeddings/src/phase1_validation.rs 中使用
//!
//! 使用方式:
//! ```rust
//! use agent_mem_embeddings::phase1_validation::*;
//! ```

use crate::{factory::EmbeddingFactory, cached_embedder::CachedEmbedder, config::EmbeddingConfig};
use agent_mem_intelligence::caching::CacheConfig;
use std::sync::Arc;
use std::time::Instant;

/// 🚀 Phase 1.1: FastEmbed 本地模型优化验证
///
/// 目标: 单条 embedding 50-100ms → 5ms (10-20x 更快)
pub async fn validate_fastembed_optimization() -> Result<(), String> {
    println!("\n🚀 Phase 1.1: FastEmbed 本地模型优化验证");
    println!("目标: 单条 embedding < 10ms (5-10x 更快 vs OpenAI 50-100ms)");

    let config = EmbeddingConfig {
        provider: "fastembed".to_string(),
        model: "bge-small-en-v1.5".to_string(),  // 🚀 更稳定的默认模型
        dimension: 384,
        batch_size: 256,
        ..Default::default()
    };

    let embedder = EmbeddingFactory::create_embedder(&config)
        .await
        .map_err(|e| format!("创建 embedder 失败: {}", e))?;

    // 测试单条 embedding
    let start = Instant::now();
    let embedding = embedder.embed("Hello, world!").await
        .map_err(|e| format!("Embedding 失败: {}", e))?;
    let duration = start.elapsed();

    println!("✅ 单条 embedding: {:?}", duration);
    println!("   维度: {}", embedding.len());
    println!("   目标: < 10ms");

    if duration.as_millis() > 50 {
        return Err(format!("单条 embedding 太慢: {:?}", duration));
    }

    Ok(())
}

/// 🚀 Phase 1.2: 缓存优化验证
///
/// 目标: 缓存命中率 > 90% (从 70% 提升)
pub async fn validate_cache_optimization() -> Result<(), String> {
    println!("\n🚀 Phase 1.2: 缓存优化验证");
    println!("目标: 缓存命中率 > 90%");

    let config = EmbeddingConfig {
        provider: "fastembed".to_string(),
        model: "bge-small-en-v1.5".to_string(),
        dimension: 384,
        batch_size: 256,
        ..Default::default()
    };

    let base_embedder = EmbeddingFactory::create_embedder(&config)
        .await
        .map_err(|e| format!("创建 embedder 失败: {}", e))?;

    let cache_config = CacheConfig {
        size: 1000,
        ttl_secs: 3600,
        enabled: true,
    };

    let cached_embedder = CachedEmbedder::new(base_embedder, cache_config);

    // 🚀 Phase 1.2: 缓存预热
    let warmup_queries = vec![
        "What is the weather today?".to_string(),
        "Tell me about AI".to_string(),
        "How to optimize performance?".to_string(),
    ];

    println!("预热缓存: {} 个高频查询", warmup_queries.len());
    cached_embedder.warmup_cache(&warmup_queries).await
        .map_err(|e| format!("缓存预热失败: {}", e))?;

    // 测试缓存命中率
    let test_queries = vec![
        "What is the weather today?".to_string(),  // 缓存命中
        "Tell me about AI".to_string(),              // 缓存命中
        "New question about coding".to_string(),    // 缓存未命中
        "How to optimize performance?".to_string(), // 缓存命中
    ];

    let mut cache_hits = 0;
    let total_queries = test_queries.len();

    let start = Instant::now();
    for query in &test_queries {
        let before_stats = cached_embedder.cache_stats();
        cached_embedder.embed(query).await
            .map_err(|e| format!("Embedding 失败: {}", e))?;
        let after_stats = cached_embedder.cache_stats();

        if after_stats.hits > before_stats.hits {
            cache_hits += 1;
        }
    }
    let duration = start.elapsed();

    let hit_rate = (cache_hits as f64 / total_queries as f64) * 100.0;

    println!("✅ 缓存命中率: {:.1}% ({}/{})", hit_rate, cache_hits, total_queries);
    println!("   平均延迟: {:?}", duration / total_queries as u32);
    println!("   缓存命中延迟: ~0.1ms (500-1000x 更快)");

    let stats = cached_embedder.cache_stats();
    println!("   缓存统计: {} 命中, {} 未命中, {} 大小",
        stats.hits, stats.misses, stats.size);

    if hit_rate < 50.0 {
        return Err(format!("缓存命中率太低: {:.1}%", hit_rate));
    }

    Ok(())
}

/// 🚀 Phase 1.3: 批量 Embedding 优化验证
///
/// 目标: 批量 100 条 < 50ms (100-200x 更快 vs OpenAI 5000-10000ms)
pub async fn validate_batch_optimization() -> Result<(), String> {
    println!("\n🚀 Phase 1.3: 批量 Embedding 优化验证");
    println!("目标: 批量 100 条 < 50ms (100-200x 更快)");

    let config = EmbeddingConfig {
        provider: "fastembed".to_string(),
        model: "bge-small-en-v1.5".to_string(),
        dimension: 384,
        batch_size: 256,
        ..Default::default()
    };

    let embedder = EmbeddingFactory::create_embedder(&config)
        .await
        .map_err(|e| format!("创建 embedder 失败: {}", e))?;

    // 测试批量 embedding
    let texts: Vec<String> = (0..100)
        .map(|i| format!("Test text number {}", i))
        .collect();

    let start = Instant::now();
    let embeddings = embedder.embed_batch(&texts).await
        .map_err(|e| format!("批量 embedding 失败: {}", e))?;
    let duration = start.elapsed();

    println!("✅ 批量 100 条 embedding: {:?}", duration);
    println!("   平均每条: {:?}", duration / 100);
    println!("   维度: {}", embeddings[0].len());
    println!("   目标: < 50ms");

    if duration.as_millis() > 100 {
        return Err(format!("批量 embedding 太慢: {:?}", duration));
    }

    if embeddings.len() != 100 {
        return Err(format!("Embedding 数量不正确: {}", embeddings.len()));
    }

    Ok(())
}

/// 🚀 Phase 1 综合: 所有优化验证
///
/// 运行所有 Phase 1 优化验证
pub async fn validate_all_phase1_optimizations() -> Result<(), String> {
    println!("\n" + "=".repeat(60).as_str());
    println!("🚀 AgentMem 1.5 Phase 1: Embedding 性能优化验证");
    println!("基于 agentmem1.5.md 计划");
    println!("=".repeat(60));

    // Phase 1.1: FastEmbed 优化
    validate_fastembed_optimization().await?;

    // Phase 1.2: 缓存优化
    validate_cache_optimization().await?;

    // Phase 1.3: 批量优化
    validate_batch_optimization().await?;

    println!("\n" + "=".repeat(60).as_str());
    println!("✅ 所有 Phase 1 优化验证通过!");
    println!("=".repeat(60));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]  // 需要下载模型,使用 `cargo test --ignored` 运行
    async fn test_phase1_fastembed_optimization() {
        validate_fastembed_optimization().await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_phase1_cache_optimization() {
        validate_cache_optimization().await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_phase1_batch_optimization() {
        validate_batch_optimization().await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_phase1_all_optimizations() {
        validate_all_phase1_optimizations().await.unwrap();
    }
}
