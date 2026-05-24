//! # 高级搜索和批量操作演示
//!
//! 这个演示展示了 AgentMem Mem0 兼容层的高级功能：
//! - 语义搜索和相似度计算
//! - 复杂过滤和排序
//! - 批量添加、更新、删除操作
//! - 历史记录追踪

use agent_mem_compat::client::{
    BatchAddRequest, EnhancedAddRequest, EnhancedSearchRequest, Messages,
};
use agent_mem_compat::{
    BatchDeleteItem, BatchDeleteRequest, BatchUpdateItem, BatchUpdateRequest, Mem0Client,
    MemoryFilter,
};
use anyhow::Result;
use serde_json::json;
use std::collections::HashMap;
use tracing::{info, warn};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 启动高级搜索和批量操作演示");

    // 创建 Mem0 客户端
    let client = Mem0Client::new().await?;
    let user_id = "demo_user";

    // 演示 1: 批量添加记忆
    info!("\n📝 演示 1: 批量添加记忆");
    demo_batch_add(&client, user_id).await?;

    // 演示 2: 高级搜索功能
    info!("\n🔍 演示 2: 高级搜索功能");
    demo_advanced_search(&client, user_id).await?;

    // 演示 3: 复杂过滤和排序
    info!("\n🎯 演示 3: 复杂过滤和排序");
    demo_complex_filtering(&client, user_id).await?;

    // 演示 4: 批量更新操作
    info!("\n✏️ 演示 4: 批量更新操作");
    demo_batch_update(&client, user_id).await?;

    // 演示 5: 历史记录追踪
    info!("\n📚 演示 5: 历史记录追踪");
    demo_history_tracking(&client, user_id).await?;

    // 演示 6: 批量删除操作
    info!("\n🗑️ 演示 6: 批量删除操作");
    demo_batch_delete(&client, user_id).await?;

    info!("✅ 所有演示完成！");
    Ok(())
}

/// 演示批量添加记忆
async fn demo_batch_add(client: &Mem0Client, user_id: &str) -> Result<()> {
    let memories = vec![
        (
            "我喜欢吃意大利面",
            json!({"category": "food", "preference": "like", "cuisine": "italian"}),
        ),
        (
            "我不喜欢吃辣的食物",
            json!({"category": "food", "preference": "dislike", "spice_level": "hot"}),
        ),
        (
            "我的生日是3月15日",
            json!({"category": "personal", "type": "birthday", "month": 3, "day": 15}),
        ),
        (
            "我住在北京",
            json!({"category": "personal", "type": "location", "city": "Beijing", "country": "China"}),
        ),
        (
            "我是一名软件工程师",
            json!({"category": "work", "profession": "software_engineer", "industry": "tech"}),
        ),
        (
            "我喜欢看科幻电影",
            json!({"category": "entertainment", "type": "movies", "genre": "sci-fi"}),
        ),
        (
            "我每天早上7点起床",
            json!({"category": "routine", "time": "07:00", "activity": "wake_up"}),
        ),
        (
            "我的宠物是一只猫",
            json!({"category": "personal", "type": "pet", "animal": "cat"}),
        ),
    ];

    // 使用新的批量添加 API
    let batch_request = BatchAddRequest {
        requests: memories
            .into_iter()
            .map(|(content, metadata)| EnhancedAddRequest {
                messages: Messages::Single(content.to_string()),
                user_id: Some(user_id.to_string()),
                agent_id: Some("demo_agent".to_string()),
                run_id: Some(Uuid::new_v4().to_string()),
                metadata: Some(
                    metadata
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                ),
                infer: true,
                memory_type: Some("episodic".to_string()),
                prompt: None,
            })
            .collect(),
    };

    let result = client.add_batch(batch_request).await?;
    info!(
        "批量添加结果: 成功 {}, 失败 {}",
        result.successful, result.failed
    );

    if !result.errors.is_empty() {
        warn!("错误信息: {:?}", result.errors);
    }

    Ok(())
}

/// 演示高级搜索功能
async fn demo_advanced_search(client: &Mem0Client, user_id: &str) -> Result<()> {
    // 语义搜索测试
    let search_queries = vec!["食物偏好", "个人信息", "工作相关", "娱乐活动", "日常习惯"];

    for query in search_queries {
        let enhanced_request = EnhancedSearchRequest {
            query: query.to_string(),
            user_id: Some(user_id.to_string()),
            agent_id: Some("demo_agent".to_string()),
            run_id: None,
            limit: 3,
            filters: None,
            threshold: Some(0.1), // 设置相似度阈值
        };

        let results = client.search_enhanced(enhanced_request).await?;
        info!("搜索 '{}' 找到 {} 条结果:", query, results.memories.len());

        for (i, memory) in results.memories.iter().enumerate() {
            info!(
                "  {}. {} (相似度: {:.3})",
                i + 1,
                memory.memory,
                memory.score.unwrap_or(0.0)
            );
        }
    }

    Ok(())
}

/// 演示复杂过滤和排序
async fn demo_complex_filtering(client: &Mem0Client, user_id: &str) -> Result<()> {
    // 创建复杂过滤器
    let mut metadata_filters = HashMap::new();
    metadata_filters.insert(
        "category".to_string(),
        agent_mem_compat::FilterOperation::Eq(json!("food")),
    );

    let filter = MemoryFilter {
        agent_id: Some("demo_agent".to_string()),
        run_id: None,
        memory_type: None,
        created_after: None,
        created_before: None,
        updated_after: None,
        updated_before: None,
        min_score: None,
        max_score: None,
        min_content_length: Some(5),
        max_content_length: None,
        metadata_filters,
        metadata: HashMap::new(),
        content_contains: Some("喜欢".to_string()),
        content_regex: None,
        tags: Vec::new(),
        exclude_tags: Vec::new(),
        sort_field: agent_mem_compat::SortField::CreatedAt,
        sort_order: agent_mem_compat::SortOrder::Desc,
        limit: Some(5),
        offset: None,
    };

    let results = client.search("食物", user_id, Some(filter)).await?;
    info!("复杂过滤搜索结果 ({} 条):", results.memories.len());

    for (i, memory) in results.memories.iter().enumerate() {
        info!("  {}. {}", i + 1, memory.memory);
        info!("     元数据: {:?}", memory.metadata);
    }

    Ok(())
}

/// 演示批量更新操作
async fn demo_batch_update(client: &Mem0Client, user_id: &str) -> Result<()> {
    // 首先获取一些记忆 ID
    let all_memories = client.get_all(user_id, None).await?;
    if all_memories.len() < 2 {
        warn!("没有足够的记忆进行批量更新演示");
        return Ok(());
    }

    let update_requests = vec![
        BatchUpdateItem {
            memory_id: all_memories[0].id.clone(),
            user_id: user_id.to_string(),
            memory: Some("我非常喜欢吃意大利面 (已更新)".to_string()),
            metadata: Some({
                let mut meta = HashMap::new();
                meta.insert("category".to_string(), json!("food"));
                meta.insert("preference".to_string(), json!("love"));
                meta.insert("updated".to_string(), json!(true));
                meta
            }),
        },
        BatchUpdateItem {
            memory_id: all_memories[1].id.clone(),
            user_id: user_id.to_string(),
            memory: None, // 只更新元数据
            metadata: Some({
                let mut meta = HashMap::new();
                meta.insert("updated".to_string(), json!(true));
                meta.insert("batch_update".to_string(), json!(true));
                meta
            }),
        },
    ];

    let batch_request = BatchUpdateRequest {
        requests: update_requests,
    };

    let result = client.update_batch(batch_request).await?;
    info!(
        "批量更新结果: 成功 {}, 失败 {}",
        result.successful, result.failed
    );

    if !result.errors.is_empty() {
        warn!("更新错误: {:?}", result.errors);
    }

    Ok(())
}

/// 演示历史记录追踪
async fn demo_history_tracking(client: &Mem0Client, user_id: &str) -> Result<()> {
    // 获取所有记忆
    let all_memories = client.get_all(user_id, None).await?;
    if all_memories.is_empty() {
        warn!("没有记忆可以查看历史");
        return Ok(());
    }

    // 查看第一个记忆的历史
    let memory_id = &all_memories[0].id;
    let history = client.history(memory_id, user_id).await?;

    info!(
        "记忆 '{}' 的历史记录 ({} 条):",
        all_memories[0].memory,
        history.len()
    );

    for (i, entry) in history.iter().enumerate() {
        info!(
            "  {}. 版本 {} - {} ({})",
            i + 1,
            entry.version,
            entry.change_type.to_string(),
            entry.timestamp.format("%Y-%m-%d %H:%M:%S")
        );

        if let Some(ref new_memory) = entry.new_memory {
            info!("     内容: {}", new_memory);
        }
    }

    Ok(())
}

/// 演示批量删除操作
async fn demo_batch_delete(client: &Mem0Client, user_id: &str) -> Result<()> {
    // 获取要删除的记忆
    let all_memories = client.get_all(user_id, None).await?;
    if all_memories.len() < 2 {
        warn!("没有足够的记忆进行批量删除演示");
        return Ok(());
    }

    // 删除最后两个记忆
    let delete_requests = vec![
        BatchDeleteItem {
            memory_id: all_memories[all_memories.len() - 1].id.clone(),
            user_id: user_id.to_string(),
        },
        BatchDeleteItem {
            memory_id: all_memories[all_memories.len() - 2].id.clone(),
            user_id: user_id.to_string(),
        },
    ];

    let batch_request = BatchDeleteRequest {
        requests: delete_requests,
    };

    let result = client.delete_batch(batch_request).await?;
    info!(
        "批量删除结果: 成功 {}, 失败 {}",
        result.successful, result.failed
    );

    if !result.errors.is_empty() {
        warn!("删除错误: {:?}", result.errors);
    }

    // 验证删除结果
    let remaining_memories = client.get_all(user_id, None).await?;
    info!("剩余记忆数量: {}", remaining_memories.len());

    Ok(())
}
