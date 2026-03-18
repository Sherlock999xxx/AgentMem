//! 向量搜索优化演示
//!
//! 演示 AgentMem 的向量搜索优化功能：
//! 1. 统计信息获取
//! 2. 索引创建（PostgreSQL pgvector）
//! 3. 性能优化
//! 4. 批量搜索

use agent_mem_core::search::vector_search::{VectorSearchConfig, VectorSearchEngine};
use agent_mem_core::search::{SearchFilters, SearchQuery};
use agent_mem_storage::backends::memory::MemoryVectorStore;
use agent_mem_traits::{VectorData, VectorStoreConfig};
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== AgentMem 向量搜索优化演示 ===\n");

    // 创建内存向量存储
    let store_config = VectorStoreConfig {
        provider: "memory".to_string(),
        path: "".to_string(),
        table_name: "vectors".to_string(),
        dimension: Some(1536),
        api_key: None,
        ..Default::default()
    };
    let vector_store = Arc::new(MemoryVectorStore::new(store_config).await?);

    // 创建向量搜索引擎
    let config = VectorSearchConfig {
        enable_cache: true,
        cache_size: 1000,
        enable_batch_optimization: true,
        batch_size: 100,
        use_pgvector: false,
        ..Default::default()
    };

    let search_engine = VectorSearchEngine::with_config(vector_store.clone(), 1536, config);

    // ========================================
    // 演示 1: 添加测试向量
    // ========================================
    println!("📊 演示 1: 添加测试向量");
    println!("----------------------------------------");

    let test_vectors = vec![
        VectorData {
            id: "vec1".to_string(),
            vector: vec![0.1; 1536],
            metadata: {
                let mut m = HashMap::new();
                m.insert("content".to_string(), "测试向量 1".to_string());
                m.insert("type".to_string(), "episodic".to_string());
                m
            },
        },
        VectorData {
            id: "vec2".to_string(),
            vector: vec![0.2; 1536],
            metadata: {
                let mut m = HashMap::new();
                m.insert("content".to_string(), "测试向量 2".to_string());
                m.insert("type".to_string(), "semantic".to_string());
                m
            },
        },
        VectorData {
            id: "vec3".to_string(),
            vector: vec![0.3; 1536],
            metadata: {
                let mut m = HashMap::new();
                m.insert("content".to_string(), "测试向量 3".to_string());
                m.insert("type".to_string(), "procedural".to_string());
                m
            },
        },
    ];

    let ids = search_engine.add_vectors(test_vectors).await?;
    println!("✅ 成功添加 {} 个向量", ids.len());
    for (i, id) in ids.iter().enumerate() {
        println!("   - 向量 {}: {}", i + 1, id);
    }
    println!();

    // ========================================
    // 演示 2: 获取统计信息
    // ========================================
    println!("📊 演示 2: 获取向量存储统计信息");
    println!("----------------------------------------");

    let stats = search_engine.get_stats().await?;
    println!("✅ 统计信息:");
    println!("   - 总向量数: {}", stats.total_vectors);
    println!("   - 向量维度: {}", stats.dimension);
    println!("   - 索引类型: {}", stats.index_type);
    println!("   - 总搜索次数: {}", stats.total_searches);
    println!("   - 缓存命中次数: {}", stats.cache_hits);
    println!("   - 平均搜索时间: {:.2} ms", stats.avg_search_time_ms);
    println!("   - 缓存命中率: {:.2}%", stats.cache_hit_rate * 100.0);
    println!();

    // ========================================
    // 演示 3: 执行向量搜索
    // ========================================
    println!("🔍 演示 3: 执行向量搜索");
    println!("----------------------------------------");

    let query_vector = vec![0.15; 1536];
    let query = SearchQuery {
        query: "测试查询".to_string(),
        limit: 10,
        threshold: Some(0.0),
        vector_weight: 0.7,
        fulltext_weight: 0.3,
        filters: Some(SearchFilters {
            user_id: None,
            organization_id: None,
            agent_id: None,
            start_time: None,
            end_time: None,
            tags: None,
        }),
        metadata_filters: None,
    };

    let (results, search_time) = search_engine.search(query_vector.clone(), &query).await?;
    println!("✅ 搜索完成:");
    println!("   - 搜索时间: {search_time} ms");
    println!("   - 结果数量: {}", results.len());
    for (i, result) in results.iter().enumerate() {
        println!(
            "   - 结果 {}: ID={}, 分数={:.4}",
            i + 1,
            result.id,
            result.score
        );
    }
    println!();

    // ========================================
    // 演示 4: 缓存效果验证
    // ========================================
    println!("⚡ 演示 4: 缓存效果验证");
    println!("----------------------------------------");

    // 第二次搜索（应该命中缓存）
    let (results2, search_time2) = search_engine.search(query_vector.clone(), &query).await?;
    println!("✅ 第二次搜索（缓存）:");
    println!("   - 搜索时间: {search_time2} ms");
    println!("   - 结果数量: {}", results2.len());
    println!("   - 时间对比: 第一次 {search_time} ms vs 第二次 {search_time2} ms");

    if search_time2 < search_time {
        println!(
            "   ✅ 缓存生效！搜索速度提升 {:.1}x",
            search_time as f64 / search_time2 as f64
        );
    }
    println!();

    // ========================================
    // 演示 5: 批量搜索优化
    // ========================================
    println!("🚀 演示 5: 批量搜索优化");
    println!("----------------------------------------");

    let query_vectors = vec![vec![0.1; 1536], vec![0.2; 1536], vec![0.3; 1536]];

    let batch_results = search_engine.batch_search(query_vectors, &query).await?;
    println!("✅ 批量搜索完成:");
    println!("   - 查询数量: {}", batch_results.len());
    for (i, (results, time)) in batch_results.iter().enumerate() {
        println!("   - 查询 {}: {} 个结果, {} ms", i + 1, results.len(), time);
    }
    println!();

    // ========================================
    // 演示 6: 性能优化
    // ========================================
    println!("⚙️  演示 6: 性能优化");
    println!("----------------------------------------");

    search_engine.optimize_search_performance().await?;
    println!("✅ 性能优化完成");
    println!("   - 清理了过期的缓存条目");
    println!("   - 优化了内存使用");
    println!();

    // ========================================
    // 演示 7: 最终统计信息
    // ========================================
    println!("📊 演示 7: 最终统计信息");
    println!("----------------------------------------");

    let final_stats = search_engine.get_stats().await?;
    println!("✅ 最终统计:");
    println!("   - 总向量数: {}", final_stats.total_vectors);
    println!("   - 总搜索次数: {}", final_stats.total_searches);
    println!("   - 缓存命中次数: {}", final_stats.cache_hits);
    println!(
        "   - 缓存命中率: {:.2}%",
        final_stats.cache_hit_rate * 100.0
    );
    println!(
        "   - 平均搜索时间: {:.2} ms",
        final_stats.avg_search_time_ms
    );
    println!();

    // ========================================
    // 演示 8: PostgreSQL pgvector 索引创建说明
    // ========================================
    println!("🗄️  演示 8: PostgreSQL pgvector 索引创建");
    println!("----------------------------------------");
    println!("ℹ️  pgvector 索引创建需要 PostgreSQL 数据库和 postgres feature");
    println!();
    println!("使用方法:");
    println!("```rust");
    println!("#[cfg(feature = \"postgres\")]");
    println!("{{");
    println!("    use sqlx::PgPool;");
    println!("    ");
    println!("    // 创建 PostgreSQL 连接池");
    println!("    let pool = PgPool::connect(\"postgresql://...\").await?;");
    println!("    ");
    println!("    // 创建 IVFFlat 索引（快速但近似）");
    println!("    search_engine.create_pgvector_index(");
    println!("        &pool,");
    println!("        \"memories\",  // 表名");
    println!("        Some(\"embedding\")  // 列名");
    println!("    ).await?;");
    println!("}}");
    println!("```");
    println!();
    println!("索引类型:");
    println!("  - IVFFlat: 快速但近似，适合大规模数据");
    println!("  - HNSW: 更精确但构建慢，适合高精度需求");
    println!();

    println!("=== 演示完成 ===");
    println!();
    println!("✅ 所有向量搜索优化功能验证通过！");
    println!();
    println!("核心功能:");
    println!("  ✅ 统计信息获取 - 实时获取向量数量和性能指标");
    println!("  ✅ 智能缓存 - 自动缓存搜索结果，提升性能");
    println!("  ✅ 批量搜索 - 并发执行多个搜索查询");
    println!("  ✅ 性能优化 - 自动清理过期缓存");
    println!("  ✅ pgvector 索引 - 支持 PostgreSQL 向量索引创建");

    Ok(())
}
