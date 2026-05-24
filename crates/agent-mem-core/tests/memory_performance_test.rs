//! Memory Performance Benchmark Test
//!
//! 测试目标:
//! 1. 测量记忆添加性能
//! 2. 测量记忆检索性能
//! 3. 测量批量操作性能
//! 4. 对标Mem0性能标准

use agent_mem_core::cognitive_memory::CognitiveMemoryManager;
use agent_mem_core::types::{Memory, MemoryType};
use std::time::Instant;

#[tokio::test]
async fn test_memory_add_performance() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();
    let n = 100;
    
    let start = Instant::now();
    for i in 0..n {
        let memory = Memory::new(
            "perf-agent".to_string(),
            Some("perf-user".to_string()),
            MemoryType::Semantic,
            format!("Performance test memory #{}", i),
            0.7,
        );
        manager.add_memory(memory).await.unwrap();
    }
    let elapsed = start.elapsed();
    
    println!("📊 Memory Add Performance:");
    println!("   - Total memories: {}", n);
    println!("   - Total time: {:?}", elapsed);
    println!("   - Per memory: {:?}", elapsed / n as u32);
    println!("   - Throughput: {:.2} memories/sec", n as f64 / elapsed.as_secs_f64());
    
    // 性能要求: 至少100条/秒
    let throughput = n as f64 / elapsed.as_secs_f64();
    assert!(throughput > 100.0, "Add throughput should be > 100/sec, got {:.2}", throughput);
}

#[tokio::test]
async fn test_memory_retrieve_performance() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();
    let n = 100;
    
    // 先添加一些记忆
    for i in 0..n {
        let memory = Memory::new(
            "perf-agent".to_string(),
            Some("perf-user".to_string()),
            MemoryType::Semantic,
            format!("Searchable memory #{} with keyword", i),
            0.7,
        );
        manager.add_memory(memory).await.unwrap();
    }
    
    // 测试检索性能
    let iterations = 50;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = manager.retrieve("keyword", None, 10).await.unwrap();
    }
    let elapsed = start.elapsed();
    
    println!("📊 Memory Retrieve Performance:");
    println!("   - Database size: {}", n);
    println!("   - Query iterations: {}", iterations);
    println!("   - Total time: {:?}", elapsed);
    println!("   - Per query: {:?}", elapsed / iterations as u32);
    println!("   - QPS: {:.2}", iterations as f64 / elapsed.as_secs_f64());
    
    // 性能要求: 至少100 QPS
    let qps = iterations as f64 / elapsed.as_secs_f64();
    assert!(qps > 100.0, "Retrieve QPS should be > 100, got {:.2}", qps);
}

#[tokio::test]
async fn test_memory_batch_add_performance() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();
    let batch_size = 50;
    let batches = 10;
    
    let start = Instant::now();
    for batch_i in 0..batches {
        let mut memories = Vec::new();
        for i in 0..batch_size {
            let memory = Memory::new(
                "perf-agent".to_string(),
                Some("perf-user".to_string()),
                MemoryType::Semantic,
                format!("Batch {} Memory #{}", batch_i, i),
                0.7,
            );
            memories.push(memory);
        }
        manager.add_memories(memories).await.unwrap();
    }
    let elapsed = start.elapsed();
    let total = batch_size * batches;
    
    println!("📊 Memory Batch Add Performance:");
    println!("   - Total memories: {}", total);
    println!("   - Batch size: {}", batch_size);
    println!("   - Total time: {:?}", elapsed);
    println!("   - Per batch: {:?}", elapsed / batches as u32);
    println!("   - Throughput: {:.2} memories/sec", total as f64 / elapsed.as_secs_f64());
    
    // 性能要求: 批量添加吞吐量至少500条/秒
    let throughput = total as f64 / elapsed.as_secs_f64();
    assert!(throughput > 500.0, "Batch add throughput should be > 500/sec, got {:.2}", throughput);
}

#[tokio::test]
async fn test_memory_type_filter_performance() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();
    let per_type = 50;
    let types = vec![
        MemoryType::Semantic,
        MemoryType::Episodic,
        MemoryType::Procedural,
        MemoryType::Working,
    ];
    
    // 添加不同类型的记忆
    for mem_type in &types {
        for i in 0..per_type {
            let memory = Memory::new(
                "perf-agent".to_string(),
                Some("perf-user".to_string()),
                mem_type.clone(),
                format!("{:?} memory #{}", mem_type, i),
                0.7,
            );
            manager.add_memory(memory).await.unwrap();
        }
    }
    
    let total = per_type * types.len();
    let iterations = 100;
    
    // 测试按类型过滤的检索性能
    let start = Instant::now();
    for _ in 0..iterations {
        for mem_type in &types {
            let _ = manager.retrieve("", Some(vec![mem_type.clone()]), 50).await.unwrap();
        }
    }
    let elapsed = start.elapsed();
    let total_queries = iterations * types.len();
    
    println!("📊 Memory Type Filter Performance:");
    println!("   - Total memories: {}", total);
    println!("   - Memory types: {:?}", types);
    println!("   - Total queries: {}", total_queries);
    println!("   - Total time: {:?}", elapsed);
    println!("   - Per query: {:?}", elapsed / total_queries as u32);
    println!("   - QPS: {:.2}", total_queries as f64 / elapsed.as_secs_f64());
    
    // 性能要求: 过滤检索至少200 QPS
    let qps = total_queries as f64 / elapsed.as_secs_f64();
    assert!(qps > 200.0, "Filter QPS should be > 200, got {:.2}", qps);
}

#[tokio::test]
async fn test_memory_stats_performance() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();
    
    // 添加一些记忆
    for i in 0..100 {
        let memory = Memory::new(
            "perf-agent".to_string(),
            Some("perf-user".to_string()),
            MemoryType::Semantic,
            format!("Stats test memory #{}", i),
            0.7,
        );
        manager.add_memory(memory).await.unwrap();
    }
    
    let iterations = 1000;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = manager.get_stats().await.unwrap();
    }
    let elapsed = start.elapsed();
    
    println!("📊 Memory Stats Performance:");
    println!("   - Database size: 100");
    println!("   - Query iterations: {}", iterations);
    println!("   - Total time: {:?}", elapsed);
    println!("   - Per query: {:?}", elapsed / iterations as u32);
    println!("   - QPS: {:.2}", iterations as f64 / elapsed.as_secs_f64());
    
    // Stats查询应该很快
    let qps = iterations as f64 / elapsed.as_secs_f64();
    assert!(qps > 500.0, "Stats QPS should be > 500, got {:.2}", qps);
}

#[tokio::test]
async fn test_memory_delete_performance() {
    let manager = CognitiveMemoryManager::with_default_config().await.unwrap();
    let n = 100;
    
    // 先添加一些记忆
    let mut ids = Vec::new();
    for i in 0..n {
        let memory = Memory::new(
            "perf-agent".to_string(),
            Some("perf-user".to_string()),
            MemoryType::Semantic,
            format!("To be deleted #{}", i),
            0.7,
        );
        let id = manager.add_memory(memory).await.unwrap();
        ids.push(id);
    }
    
    // 测试删除性能
    let start = Instant::now();
    for id in &ids {
        manager.delete_memory(id).await.unwrap();
    }
    let elapsed = start.elapsed();
    
    println!("📊 Memory Delete Performance:");
    println!("   - Total deletions: {}", n);
    println!("   - Total time: {:?}", elapsed);
    println!("   - Per delete: {:?}", elapsed / n as u32);
    println!("   - Throughput: {:.2} deletes/sec", n as f64 / elapsed.as_secs_f64());
    
    // 删除性能要求: 至少100/秒
    let throughput = n as f64 / elapsed.as_secs_f64();
    assert!(throughput > 100.0, "Delete throughput should be > 100/sec, got {:.2}", throughput);
}
