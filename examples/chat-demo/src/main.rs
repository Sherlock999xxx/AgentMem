//! Chat Demo - 演示 AgentMem 的对话功能
//!
//! 这个示例展示如何使用 AgentMem 的 chat() 方法进行智能对话：
//! 1. 配置 LLM（使用 Ollama 本地模型）
//! 2. 创建用户并添加背景信息
//! 3. 进行多轮对话
//! 4. 展示记忆如何影响对话
//! 5. 验证对话历史的保存
//!
//! 运行前提：
//! - 设置环境变量 DEEPSEEK_API_KEY（获取 API key: https://platform.deepseek.com/）
//! - 或者使用其他 LLM 提供商（OpenAI, Claude, etc.）
//!
//! 运行方式：
//! ```bash
//! export DEEPSEEK_API_KEY="your-api-key"
//! cargo run --package chat-demo
//! ```

use agent_mem_core::client::{AgentMemClient, AgentMemClientConfig, MemoryType, Messages};
use agent_mem_traits::{LLMConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== AgentMem Chat Demo ===\n");

    // 1. 配置 LLM（使用 DeepSeek）
    println!("📝 Step 1: 配置 LLM");

    // 从环境变量读取 API key
    let api_key = std::env::var("DEEPSEEK_API_KEY").ok();

    if api_key.is_none() {
        println!("⚠️  警告: 未设置 DEEPSEEK_API_KEY 环境变量");
        println!("   请运行: export DEEPSEEK_API_KEY=\"your-api-key\"");
        println!("   获取 API key: https://platform.deepseek.com/");
        println!("\n   或者使用其他 LLM 提供商（修改代码中的 provider 和 model）");
    }

    let mut config = AgentMemClientConfig::default();
    config.llm = Some(LLMConfig {
        provider: "deepseek".to_string(),
        model: "deepseek-chat".to_string(),
        api_key,
        base_url: Some("https://api.deepseek.com".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(200),
        top_p: None,
        frequency_penalty: None,
        presence_penalty: None,
        response_format: None,
    });

    let client = AgentMemClient::new(config);
    println!("✅ LLM 配置完成: DeepSeek (deepseek-chat)\n");

    // 2. 创建用户
    println!("📝 Step 2: 创建用户");
    let user = client.create_user("Alice".to_string()).await?;
    println!("✅ 用户创建成功: {} (ID: {})\n", user.name, user.id);

    // 3. 添加背景信息
    println!("📝 Step 3: 添加背景信息");
    let facts = vec![
        ("I am a software engineer at Google", MemoryType::Semantic),
        ("I live in San Francisco", MemoryType::Semantic),
        ("I enjoy hiking and photography", MemoryType::Semantic),
        (
            "My favorite programming language is Rust",
            MemoryType::Semantic,
        ),
        ("I have a cat named Whiskers", MemoryType::Semantic),
    ];

    for (fact, memory_type) in &facts {
        client
            .add(
                Messages::Single(fact.to_string()),
                Some(user.id.clone()),
                None,
                None,
                None,
                false,
                Some(*memory_type),
                None,
            )
            .await?;
        println!("  ✓ 添加记忆: {fact}");
    }
    println!("✅ 背景信息添加完成\n");

    // 4. 进行多轮对话
    println!("📝 Step 4: 进行多轮对话\n");
    println!("{}", "=".repeat(60));

    let conversations = [
        ("What is my profession?", true),
        ("Where do I live?", true),
        ("What are my hobbies?", true),
        ("What programming language do I prefer?", true),
        ("Tell me about my pet", true),
    ];

    for (i, (question, save_to_memory)) in conversations.iter().enumerate() {
        println!("\n🗣️  Round {}: {}", i + 1, question);
        println!("{}", "-".repeat(60));

        match client
            .chat(question.to_string(), Some(user.id.clone()), *save_to_memory)
            .await
        {
            Ok(response) => {
                println!("🤖 Assistant: {response}");
            }
            Err(e) => {
                println!("❌ Error: {e}");
                println!("\n⚠️  提示：请检查 LLM 配置");
                println!("   1. 确保设置了 DEEPSEEK_API_KEY 环境变量");
                println!("   2. 检查 API key 是否有效");
                println!("   3. 检查网络连接");
                println!("\n   或者使用其他 LLM 提供商（修改代码中的 provider 和 model）");
                return Err(e);
            }
        }
    }

    println!("\n{}", "=".repeat(60));

    // 5. 验证对话历史
    println!("\n📝 Step 5: 验证对话历史");
    let all_memories = client
        .get_all(Some(user.id.clone()), None, None, None)
        .await?;

    let episodic_memories: Vec<_> = all_memories
        .iter()
        .filter(|m| m.memory_type == MemoryType::Episodic)
        .collect();

    println!("✅ 总记忆数: {}", all_memories.len());
    println!("✅ 对话记录数: {}", episodic_memories.len());
    println!("   (每轮对话保存 2 条记忆: 用户消息 + 助手回复)");

    // 6. 展示记忆可视化
    println!("\n📝 Step 6: 记忆可视化");
    let visualization = client.visualize_memories(Some(user.id.clone())).await?;

    println!("\n📊 记忆统计:");
    println!("  - 语义记忆: {}", visualization.summary.semantic_count);
    println!("  - 情景记忆: {}", visualization.summary.episodic_count);
    println!("  - 总计: {}", visualization.summary.total_count);

    // 7. 测试清空对话历史
    println!("\n📝 Step 7: 清空对话历史");
    let deleted_count = client.clear_conversation_history(user.id.clone()).await?;
    println!("✅ 删除了 {deleted_count} 条对话记录");

    let remaining_memories = client
        .get_all(Some(user.id.clone()), None, None, None)
        .await?;
    println!(
        "✅ 剩余记忆数: {} (语义记忆被保留)",
        remaining_memories.len()
    );

    // 8. 再次对话，验证语义记忆仍然有效
    println!("\n📝 Step 8: 验证语义记忆保留");
    println!("{}", "=".repeat(60));
    println!("\n🗣️  Question: What do you know about me?");
    println!("{}", "-".repeat(60));

    match client
        .chat(
            "What do you know about me?".to_string(),
            Some(user.id.clone()),
            false,
        )
        .await
    {
        Ok(response) => {
            println!("🤖 Assistant: {response}");
            println!("\n✅ 语义记忆仍然有效！");
        }
        Err(e) => {
            println!("❌ Error: {e}");
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("\n🎉 Chat Demo 完成！");
    println!("\n总结:");
    println!("  ✓ LLM 配置和初始化");
    println!("  ✓ 用户创建和背景信息添加");
    println!("  ✓ 多轮智能对话");
    println!("  ✓ 对话历史自动保存");
    println!("  ✓ 记忆可视化");
    println!("  ✓ 对话历史清空");
    println!("  ✓ 语义记忆保留验证");

    Ok(())
}
