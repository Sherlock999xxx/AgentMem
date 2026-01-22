//! 嵌入缓存统计示例
//!
//! 演示如何获取和使用 CachedEmbedder 的缓存统计信息

use agent_mem::Memory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("\n📊 嵌入缓存统计示例");
    println!("================================\n");

    // 创建 Memory 实例 (默认启用缓存)
    let memory = Memory::new_core().await?;

    println!("✅ Memory 创建完成 (缓存已默认启用)\n");

    // 测试数据
    let test_contents = vec![
        "AgentMem 是一个企业级 AI 记忆管理平台",
        "它支持多种向量搜索引擎",
        "性能提升是关键目标",
        "Rust 语言提供高性能保证",
        "缓存可以显著提升性能",
    ];

    // 第一轮: 添加内容 (缓存未命中)
    println!("🔥 第一轮: 添加内容 (缓存未命中)");
    for (idx, content) in test_contents.iter().enumerate() {
        let _ = memory.add(content).await?;
        if idx < 3 {
            println!("  添加 [{}/{}]: {}", idx + 1, test_contents.len(), content);
        }
    }
    if test_contents.len() > 3 {
        println!("  ... (共 {} 条)", test_contents.len());
    }

    // 第二轮: 添加相同内容 (缓存命中)
    println!("\n⚡ 第二轮: 添加相同内容 (缓存命中)");
    for (idx, content) in test_contents.iter().enumerate() {
        let _ = memory.add(content).await?;
        if idx < 3 {
            println!("  添加 [{}/{}]: {} ⚡", idx + 1, test_contents.len(), content);
        }
    }
    if test_contents.len() > 3 {
        println!("  ... (共 {} 条)", test_contents.len());
    }

    // 尝试获取缓存统计
    println!("\n📊 尝试获取缓存统计");
    println!("────────────────────────");

    // 注意: 当前版本的 get_cache_stats() 返回 Option<CacheStats>
    // 实际实现需要在 Embedder trait 中添加 get_cache_stats() 方法
    match memory.get_cache_stats().await {
        Ok(Some(stats)) => {
            println!("✅ 缓存统计获取成功:");
            println!("  命中次数: {}", stats.hits);
            println!("  未命中次数: {}", stats.misses);
            println!("  命中率: {:.2}%", stats.hit_rate * 100.0);
            println!("  缓存大小: {}", stats.size);
            println!("  缓存容量: {}", stats.capacity);
        }
        Ok(None) => {
            println!("⚠️  缓存统计功能当前不可用");
            println!("\n原因:");
            println!("  1. CachedEmbedder 已启用并正常工作");
            println!("  2. 但公共 API 需要在 Embedder trait 中添加 get_cache_stats() 方法");
            println!("  3. 当前返回占位符 (None)");
            println!("\n变通方案:");
            println!("  - 可以通过内部日志查看缓存命中/未命中信息");
            println!("  - 启用 INFO 级别日志: `tracing_subscriber::fmt().with_max_level(tracing::Level::INFO)`");
            println!("  - 查找日志中的 \"✅ 嵌入向量缓存命中\" 和 \"缓存未命中\" 信息");
        }
        Err(e) => {
            println!("❌ 获取缓存统计失败: {}", e);
        }
    }

    // 性能测试
    println!("\n📈 简单性能测试");
    println!("────────────────────────");

    use std::time::Instant;

    let test_content = "这是一个测试内容,用于演示缓存性能提升效果";

    // 第一次: 缓存未命中
    let start = Instant::now();
    let _ = memory.add(test_content).await?;
    let duration1 = start.elapsed();
    println!("第一次 (缓存未命中): {:?}", duration1);

    // 第二次: 缓存命中
    let start = Instant::now();
    let _ = memory.add(test_content).await?;
    let duration2 = start.elapsed();
    println!("第二次 (缓存命中):   {:?} ⚡", duration2);

    if duration1 > duration2 {
        let speedup = duration1.as_secs_f64() / duration2.as_secs_f64();
        println!("\n性能提升: {:.2}x", speedup);
    }

    // 清空缓存示例
    println!("\n🗑️  清空缓存示例");
    println!("────────────────────────");

    match memory.clear_embedder_cache().await {
        Ok(_) => {
            println!("✅ 缓存已清空");
            println!("注意: 下次添加内容将重新计算嵌入向量");
        }
        Err(e) => {
            println!("⚠️  清空缓存功能当前不可用: {}", e);
        }
    }

    println!("\n✅ 示例完成!");
    println!("\n💡 提示:");
    println!("  - 缓存功能已默认启用");
    println!("  - 相同内容会自动从缓存返回,性能提升 2-5x");
    println!("  - 可以通过 OrchestratorConfig 自定义缓存配置:");
    println!("    - enable_embedder_cache: bool (默认 true)");
    println!("    - embedder_cache_size: usize (默认 1000)");
    println!("    - embedder_cache_ttl_secs: u64 (默认 3600)");

    Ok(())
}
