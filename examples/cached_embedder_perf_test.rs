//! CachedEmbedder 性能测试
//!
//! 验证 CachedEmbedder 的实际性能提升效果

use agent_mem::Memory;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("\n🚀 CachedEmbedder 性能测试");
    println!("================================\n");

    // 测试 1: 重复内容的嵌入缓存效果
    println!("📊 测试 1: 重复内容嵌入缓存效果");
    println!("─────────────────────────────────────");

    let test_contents = vec![
        "AgentMem 是一个企业级 AI 记忆管理平台".to_string(),
        "它支持多种向量搜索引擎".to_string(),
        "性能提升是关键目标".to_string(),
        "Rust 语言提供高性能保证".to_string(),
        "缓存可以显著提升性能".to_string(),
    ];

    // 创建 Memory 实例 (缓存已默认启用)
    let memory = Memory::new_core().await?;

    // 预热: 第一次生成嵌入 (缓存未命中)
    println!("\n🔥 预热阶段: 第一次生成嵌入 (缓存未命中)");
    let mut warmup_durations = Vec::new();
    for content in &test_contents {
        let start = Instant::now();
        let _ = memory.add(content).await;
        let duration = start.elapsed();
        warmup_durations.push(duration);
        println!("   内容: {:<40} | 耗时: {:?}", content, duration);
    }

    let avg_warmup: Duration = warmup_durations.iter().sum::<Duration>() / test_contents.len() as u32;
    println!("\n   平均延迟 (预热): {:?}", avg_warmup);

    // 测试: 重复相同内容 (缓存命中)
    println!("\n✅ 测试阶段: 重复内容 (缓存命中)");
    let mut cached_durations = Vec::new();

    // 重复 10 次
    for round in 0..10 {
        for (idx, content) in test_contents.iter().enumerate() {
            let start = Instant::now();
            let _ = memory.add(content).await;
            let duration = start.elapsed();
            cached_durations.push(duration);

            if round == 0 && idx < 3 {
                println!("   第 1 轮 | 内容: {:<40} | 耗时: {:?}", content, duration);
            }
        }
    }

    let avg_cached: Duration = cached_durations.iter().sum::<Duration>() / cached_durations.len() as u32;
    println!("   ... (共 10 轮)");
    println!("\n   平均延迟 (缓存命中): {:?}", avg_cached);

    // 计算性能提升
    let speedup = avg_warmup.as_secs_f64() / avg_cached.as_secs_f64();
    println!("\n   📈 性能提升: {:.2}x", speedup);

    // 测试 2: 批量操作的缓存效果
    println!("\n📊 测试 2: 批量操作缓存效果");
    println!("─────────────────────────────────────");

    let batch_size = 100;
    let batch_contents: Vec<String> = (0..batch_size)
        .map(|i| format!("测试记忆内容 {} - 这是一个关于编程和技术的描述", i % 10)) // 只有 10 个唯一内容
        .collect();

    // 第一次批量添加 (缓存未命中)
    println!("\n🔥 第一次批量添加 (缓存未命中)");
    let start = Instant::now();
    let _results1 = memory.add_batch(batch_contents.clone(), Default::default()).await?;
    let duration1 = start.elapsed();
    println!("   总耗时: {:?}", duration1);
    println!("   平均延迟: {:?}", duration1 / batch_size as u32);

    // 第二次批量添加 (缓存命中)
    println!("\n✅ 第二次批量添加 (缓存命中)");
    let start = Instant::now();
    let _results2 = memory.add_batch(batch_contents.clone(), Default::default()).await?;
    let duration2 = start.elapsed();
    println!("   总耗时: {:?}", duration2);
    println!("   平均延迟: {:?}", duration2 / batch_size as u32);

    let batch_speedup = duration1.as_secs_f64() / duration2.as_secs_f64();
    println!("\n   📈 批量操作性能提升: {:.2}x", batch_speedup);

    // 测试 3: 缓存命中率统计
    println!("\n📊 测试 3: 缓存命中率统计");
    println!("─────────────────────────────────────");

    // 获取缓存统计 (如果可用)
    // 注意: 需要通过内部 API 或添加 public 方法来获取缓存统计
    println!("\n   ⚠️  缓存统计功能需要通过内部 API 访问");
    println!("   建议: 在 CachedEmbedder 中添加 get_stats() 方法");

    // 总结
    println!("\n📊 性能测试总结");
    println!("═══════════════════");
    println!("✅ 单条嵌入性能提升: {:.2}x", speedup);
    println!("✅ 批量操作性能提升: {:.2}x", batch_speedup);

    // 计算理论 QPS 提升
    let baseline_qps = 404.5;
    let expected_qps = baseline_qps * speedup;
    println!("\n📈 理论 QPS 提升:");
    println!("   基准 QPS: {:.1} ops/s", baseline_qps);
    println!("   预期 QPS: {:.1} ops/s (提升 {:.2}x)", expected_qps, speedup);

    // 距离目标
    let target_qps = 10000.0;
    let gap = target_qps / expected_qps;
    println!("   目标 QPS: {:.1} ops/s", target_qps);
    println!("   距离目标: {:.1}x 差距", gap);

    println!("\n✅ 测试完成!");

    Ok(())
}
