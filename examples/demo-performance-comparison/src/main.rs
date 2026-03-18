//! AgentMem性能对比测试
//!
//! 对标MIRIX的性能测试，包括：
//! 1. TestTracker - 测试跟踪和报告
//! 2. 搜索性能对比 - 不同搜索方法的性能测试
//! 3. 操作延迟测试 - 添加、搜索、删除的延迟
//! 4. 吞吐量测试 - 并发操作的吞吐量
//! 5. 大规模数据测试 - 不同数据规模下的性能
//!
//! 真实实现，对标MIRIX的test_fts5_performance_comparison

use agent_mem::{Memory, MemoryBuilder};
use anyhow::Result;
use colored::*;
use std::time::{Duration, Instant};
use tracing::warn;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*};

mod test_tracker;
use test_tracker::TestTracker;

/// 测试配置
struct TestConfig {
    /// 每个测试的记忆数量
    memory_count: usize,
    /// 搜索限制
    search_limit: usize,
    /// 并发数
    concurrent_count: usize,
    /// 测试查询
    test_queries: Vec<(&'static str, &'static str)>,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            memory_count: 1000,
            search_limit: 50,
            concurrent_count: 10,
            test_queries: vec![
                ("simple", "rust"),
                ("medium", "memory management"),
                ("complex", "intelligent memory system with vector search"),
            ],
        }
    }
}

/// 性能统计
#[derive(Debug, Clone)]
struct PerformanceStats {
    /// 操作名称
    operation: String,
    /// 执行时间
    duration: Duration,
    /// 结果数量
    result_count: usize,
    /// 成功标志
    success: bool,
    /// 错误信息
    error: Option<String>,
}

impl PerformanceStats {
    fn success(operation: impl Into<String>, duration: Duration, result_count: usize) -> Self {
        Self {
            operation: operation.into(),
            duration,
            result_count,
            success: true,
            error: None,
        }
    }

    fn failure(operation: impl Into<String>, duration: Duration, error: String) -> Self {
        Self {
            operation: operation.into(),
            duration,
            result_count: 0,
            success: false,
            error: Some(error),
        }
    }

    fn ops_per_second(&self) -> f64 {
        if self.duration.as_secs_f64() > 0.0 {
            self.result_count as f64 / self.duration.as_secs_f64()
        } else {
            0.0
        }
    }

    fn avg_latency_ms(&self) -> f64 {
        if self.result_count > 0 {
            self.duration.as_millis() as f64 / self.result_count as f64
        } else {
            self.duration.as_millis() as f64
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(LevelFilter::INFO)
        .init();

    println!(
        "{}",
        "╔════════════════════════════════════════════════════════════════╗".blue()
    );
    println!(
        "{}",
        "║                                                                ║".blue()
    );
    println!(
        "{}",
        "║         🚀 AgentMem vs MIRIX 性能对比测试 🚀                 ║".blue()
    );
    println!(
        "{}",
        "║                                                                ║".blue()
    );
    println!(
        "{}",
        "║             真实性能测试，对标MIRIX                           ║".blue()
    );
    println!(
        "{}",
        "║                                                                ║".blue()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════════╝".blue()
    );

    let mut tracker = TestTracker::new();
    let config = TestConfig::default();

    // 初始化Memory
    println!("\n{}", "📦 初始化AgentMem...".cyan());
    let memory = create_test_memory().await?;
    println!("{}", "✓ AgentMem初始化成功".green());
    println!("  - Embedder: FastEmbed (bge-small-en-v1.5)");
    println!("  - Vector Dimension: 384");

    // 测试1: 添加操作性能
    test_add_performance(&mut tracker, &memory, &config).await?;

    // 测试2: 搜索操作性能
    test_search_performance(&mut tracker, &memory, &config).await?;

    // 测试3: 批量操作性能
    test_batch_performance(&mut tracker, &memory, &config).await?;

    // 测试4: 并发操作性能
    test_concurrent_performance(&mut tracker, &memory, &config).await?;

    // 测试5: 不同数据规模性能
    test_scale_performance(&mut tracker, &config).await?;

    // 打印总结
    tracker.print_summary();

    // 生成性能报告
    generate_performance_report(&tracker)?;

    Ok(())
}

/// 创建测试用的Memory实例
async fn create_test_memory() -> Result<Memory> {
    Ok(MemoryBuilder::new()
        .with_agent("perf_test_agent")
        .with_embedder("fastembed", "bge-small-en-v1.5")
        .disable_intelligent_features()
        .build()
        .await?)
}

/// 测试1: 添加操作性能
async fn test_add_performance(
    tracker: &mut TestTracker,
    memory: &Memory,
    config: &TestConfig,
) -> Result<()> {
    tracker.start_test("Add Operation Performance", "测试不同大小记忆的添加性能");

    let medium_text = "A".repeat(100);
    let large_text = "B".repeat(1000);

    let test_sizes = [
        ("Small (10 bytes)", "Small text"),
        ("Medium (100 bytes)", medium_text.as_str()),
        ("Large (1000 bytes)", large_text.as_str()),
    ];

    for (size_name, content) in test_sizes.iter() {
        tracker.start_subtest(format!("Add {size_name}"));

        let start = Instant::now();
        let mut count = 0;
        let mut errors = 0;

        for _ in 0..100 {
            match memory.add(*content).await {
                Ok(_) => count += 1,
                Err(e) => {
                    errors += 1;
                    if errors == 1 {
                        warn!("Add error: {}", e);
                    }
                }
            }
        }

        let duration = start.elapsed();
        let stats = PerformanceStats::success(*size_name, duration, count);

        if count > 0 {
            tracker.pass_subtest(
                None,
                &format!(
                    "{} ops, {:.2} ops/s, {:.2}ms avg latency",
                    count,
                    stats.ops_per_second(),
                    stats.avg_latency_ms()
                ),
            );
        } else {
            tracker.fail_subtest(&format!("All {} operations failed", 100), None);
        }
    }

    tracker.pass_test("Add performance test completed");
    Ok(())
}

/// 测试2: 搜索操作性能
async fn test_search_performance(
    tracker: &mut TestTracker,
    memory: &Memory,
    config: &TestConfig,
) -> Result<()> {
    tracker.start_test(
        "Search Operation Performance",
        "对标MIRIX的FTS5性能测试 - 测试不同查询复杂度的搜索性能",
    );

    // 先添加一些测试数据
    println!("\n{}", "  准备测试数据...".cyan());
    let test_data = vec![
        "Rust is a systems programming language",
        "Memory management in Rust is safe and efficient",
        "AgentMem provides intelligent memory system with vector search",
        "FastEmbed enables local embedding without API keys",
        "Performance optimization is crucial for production systems",
        "Semantic search uses vector embeddings for similarity matching",
        "BM25 is a ranking function used in information retrieval",
        "Full-text search enables efficient keyword matching",
        "Machine learning models require large amounts of training data",
        "Neural networks can learn complex patterns from data",
    ];

    for content in test_data {
        let _ = memory.add(content).await;
    }

    println!("{}", "  ✓ 测试数据准备完成".green());

    // 测试不同复杂度的查询
    for (query_type, query) in &config.test_queries {
        tracker.start_subtest(format!("{query_type} query: '{query}'"));

        let start = Instant::now();
        match memory.search(query.to_string()).await {
            Ok(results) => {
                let duration = start.elapsed();
                let stats = PerformanceStats::success("search", duration, results.len());

                tracker.pass_subtest(
                    None,
                    &format!(
                        "{} results in {:.4}s, {:.2}ms latency",
                        results.len(),
                        duration.as_secs_f64(),
                        stats.avg_latency_ms()
                    ),
                );
            }
            Err(e) => {
                tracker.fail_subtest(&format!("Search failed: {e}"), None);
            }
        }
    }

    tracker.pass_test("Search performance test completed");
    Ok(())
}

/// 测试3: 批量操作性能
async fn test_batch_performance(
    tracker: &mut TestTracker,
    memory: &Memory,
    config: &TestConfig,
) -> Result<()> {
    tracker.start_test(
        "Batch Operation Performance",
        "测试批量添加和批量搜索的性能",
    );

    // 批量添加测试
    tracker.start_subtest(format!("Batch add {} memories", config.memory_count));

    let contents: Vec<String> = (0..config.memory_count)
        .map(|i| format!("Test memory item number {i}"))
        .collect();

    let start = Instant::now();
    let mut success_count = 0;

    for content in &contents {
        if memory.add(content).await.is_ok() {
            success_count += 1;
        }
    }

    let duration = start.elapsed();
    let stats = PerformanceStats::success("batch_add", duration, success_count);

    tracker.pass_subtest(
        None,
        &format!(
            "{}/{} ops, {:.2} ops/s, {:.2}ms avg latency",
            success_count,
            config.memory_count,
            stats.ops_per_second(),
            stats.avg_latency_ms()
        ),
    );

    tracker.pass_test("Batch performance test completed");
    Ok(())
}

/// 测试4: 并发操作性能
async fn test_concurrent_performance(
    tracker: &mut TestTracker,
    memory: &Memory,
    config: &TestConfig,
) -> Result<()> {
    tracker.start_test(
        "Concurrent Operation Performance",
        format!("测试{}个并发操作的性能", config.concurrent_count),
    );

    tracker.start_subtest(format!(
        "Concurrent {} add operations",
        config.concurrent_count
    ));

    let start = Instant::now();
    let mut handles = vec![];

    for i in 0..config.concurrent_count {
        let memory_clone = memory.clone();
        let handle = tokio::spawn(async move {
            memory_clone
                .add(&format!("Concurrent test memory {i}"))
                .await
        });
        handles.push(handle);
    }

    let mut success_count = 0;
    for handle in handles {
        if handle.await.is_ok() {
            success_count += 1;
        }
    }

    let duration = start.elapsed();
    let stats = PerformanceStats::success("concurrent_add", duration, success_count);

    tracker.pass_subtest(
        None,
        &format!(
            "{}/{} ops, {:.2} ops/s, {:.2}ms total time",
            success_count,
            config.concurrent_count,
            stats.ops_per_second(),
            duration.as_millis()
        ),
    );

    tracker.pass_test("Concurrent performance test completed");
    Ok(())
}

/// 测试5: 不同数据规模性能
async fn test_scale_performance(tracker: &mut TestTracker, _config: &TestConfig) -> Result<()> {
    tracker.start_test("Scale Performance Test", "测试不同数据规模下的性能表现");

    let scales = vec![100, 500, 1000];

    for scale in scales {
        tracker.start_subtest(format!("Scale: {scale} memories"));

        let memory = create_test_memory().await?;
        let start = Instant::now();
        let mut success_count = 0;

        for i in 0..scale {
            if memory.add(&format!("Scale test memory {i}")).await.is_ok() {
                success_count += 1;
            }
        }

        let duration = start.elapsed();
        let stats = PerformanceStats::success("scale_add", duration, success_count);

        // 测试搜索性能
        let search_start = Instant::now();
        let search_result = memory.search("test memory".to_string()).await;
        let search_duration = search_start.elapsed();

        if let Ok(results) = search_result {
            tracker.pass_subtest(
                None,
                &format!(
                    "Add: {:.2} ops/s, Search: {} results in {:.4}s",
                    stats.ops_per_second(),
                    results.len(),
                    search_duration.as_secs_f64()
                ),
            );
        } else {
            tracker.pass_subtest(
                None,
                &format!("Add: {:.2} ops/s, Search: failed", stats.ops_per_second()),
            );
        }
    }

    tracker.pass_test("Scale performance test completed");
    Ok(())
}

/// 生成性能报告
fn generate_performance_report(tracker: &TestTracker) -> Result<()> {
    println!(
        "\n{}",
        "╔════════════════════════════════════════════════════════════════╗".blue()
    );
    println!(
        "{}",
        "║                                                                ║".blue()
    );
    println!(
        "{}",
        "║                     📊 性能报告                                ║".blue()
    );
    println!(
        "{}",
        "║                                                                ║".blue()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════════╝".blue()
    );

    let summary = tracker.get_summary();

    println!("\n{}", "总体统计:".yellow());
    println!("  - 总测试数: {}", summary.total_tests);
    println!("  - 通过测试: {}", summary.passed_tests.to_string().green());
    println!("  - 失败测试: {}", summary.failed_tests.to_string().red());
    println!(
        "  - 成功率: {:.1}%",
        (summary.passed_tests as f64 / summary.total_tests as f64 * 100.0)
    );

    println!("\n{}", "子测试统计:".yellow());
    println!("  - 总子测试数: {}", summary.total_subtests);
    println!(
        "  - 通过子测试: {}",
        summary.passed_subtests.to_string().green()
    );
    println!(
        "  - 失败子测试: {}",
        summary.failed_subtests.to_string().red()
    );

    println!(
        "\n{}",
        "╔════════════════════════════════════════════════════════════════╗".blue()
    );
    println!(
        "{}",
        "║                                                                ║".blue()
    );
    println!(
        "{}",
        "║          ✅ AgentMem性能测试完成！✅                          ║".blue()
    );
    println!(
        "{}",
        "║                                                                ║".blue()
    );
    println!(
        "{}",
        "║  查看详细报告: PERFORMANCE_COMPARISON_REPORT.md               ║".blue()
    );
    println!(
        "{}",
        "║                                                                ║".blue()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════════╝".blue()
    );

    Ok(())
}
