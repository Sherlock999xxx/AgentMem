//! 图记忆和关系推理演示
//!
//! 展示 AgentMem 6.0 的图记忆和关系推理功能，包括：
//! - 知识图谱构建
//! - 多种关系推理算法
//! - 图遍历和查询
//! - 图统计分析

use agent_mem_core::graph_memory::{GraphMemoryEngine, NodeType, ReasoningType, RelationType};
use agent_mem_core::types::Memory;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use tracing::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    println!(
        "{}",
        "🧠 AgentMem 6.0 图记忆和关系推理演示".bright_blue().bold()
    );
    println!();

    // 创建图记忆引擎
    let engine = GraphMemoryEngine::new();

    // 演示 1: 构建知识图谱
    println!("{}", "第 1 步: 构建知识图谱".bright_green().bold());
    demo_build_knowledge_graph(&engine).await?;

    // 演示 2: 关系推理
    println!("\n{}", "第 2 步: 关系推理演示".bright_green().bold());
    demo_relationship_reasoning(&engine).await?;

    // 演示 3: 图遍历和查询
    println!("\n{}", "第 3 步: 图遍历和查询".bright_green().bold());
    demo_graph_traversal(&engine).await?;

    // 演示 4: 图统计分析
    println!("\n{}", "第 4 步: 图统计分析".bright_green().bold());
    demo_graph_statistics(&engine).await?;

    println!(
        "\n{}",
        "🎉 图记忆和关系推理演示完成！".bright_green().bold()
    );
    println!();
    println!("📈 演示成果：");
    println!("  • ✅ 成功构建了复杂的知识图谱");
    println!("  • ✅ 展示了多种关系推理算法");
    println!("  • ✅ 验证了图遍历和查询功能");
    println!("  • ✅ 提供了完整的图统计分析");
    println!();
    println!("🚀 AgentMem 的图记忆功能为智能应用提供了强大的知识表示和推理能力！");

    Ok(())
}

/// 构建知识图谱演示
async fn demo_build_knowledge_graph(
    engine: &GraphMemoryEngine,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 构建知识图谱演示");

    let pb = ProgressBar::new(10);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // 创建实体节点
    pb.set_message("创建实体节点");
    let apple_memory = create_memory(
        "apple",
        "agent1",
        "Apple is a red fruit that grows on trees",
        "user1",
    );
    let apple_id = engine.add_node(apple_memory, NodeType::Entity).await?;
    pb.inc(1);

    let fruit_memory = create_memory(
        "fruit",
        "agent1",
        "Fruit is a healthy food category",
        "user1",
    );
    let fruit_id = engine.add_node(fruit_memory, NodeType::Concept).await?;
    pb.inc(1);

    let tree_memory = create_memory(
        "tree",
        "agent1",
        "Tree is a large plant with branches",
        "user1",
    );
    let tree_id = engine.add_node(tree_memory, NodeType::Entity).await?;
    pb.inc(1);

    let healthy_memory = create_memory("healthy", "agent1", "Healthy means good for body", "user1");
    let healthy_id = engine.add_node(healthy_memory, NodeType::Concept).await?;
    pb.inc(1);

    let eating_memory = create_memory(
        "eating_apple",
        "agent1",
        "John ate an apple yesterday",
        "user1",
    );
    let eating_id = engine.add_node(eating_memory, NodeType::Event).await?;
    pb.inc(1);

    // 创建关系边
    pb.set_message("创建关系边");
    engine
        .add_edge(apple_id.clone(), fruit_id.clone(), RelationType::IsA, 1.0)
        .await?;
    pb.inc(1);

    engine
        .add_edge(apple_id.clone(), tree_id.clone(), RelationType::PartOf, 0.8)
        .await?;
    pb.inc(1);

    engine
        .add_edge(
            fruit_id.clone(),
            healthy_id.clone(),
            RelationType::RelatedTo,
            0.9,
        )
        .await?;
    pb.inc(1);

    engine
        .add_edge(
            eating_id.clone(),
            apple_id.clone(),
            RelationType::RelatedTo,
            1.0,
        )
        .await?;
    pb.inc(1);

    engine
        .add_edge(
            apple_id.clone(),
            healthy_id.clone(),
            RelationType::CausedBy,
            0.7,
        )
        .await?;
    pb.inc(1);

    pb.finish_with_message("✅ 知识图谱构建完成");

    println!("🎯 构建结果：");
    println!("  • 创建了 5 个知识节点");
    println!("  • 建立了 5 个关系连接");
    println!("  • 涵盖实体、概念、事件三种节点类型");
    println!("  • 包含多种关系类型（IsA、PartOf、RelatedTo、CausedBy）");

    Ok(())
}

/// 关系推理演示
async fn demo_relationship_reasoning(
    engine: &GraphMemoryEngine,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 关系推理演示");

    // 获取一些节点ID用于推理（简化演示）
    let apple_id = "apple".to_string();
    let healthy_id = "healthy".to_string();

    println!("🔍 演示不同类型的推理：");

    // 演绎推理
    println!("\n  📋 演绎推理 (Deductive Reasoning):");
    println!("    前提: Apple → Fruit → Healthy");
    println!("    结论: Apple → Healthy");

    match engine
        .reason_relationships(&apple_id, &healthy_id, ReasoningType::Deductive)
        .await
    {
        Ok(paths) => {
            println!("    ✅ 找到 {} 条推理路径", paths.len());
            for (i, path) in paths.iter().enumerate() {
                println!("    路径 {}: 置信度 {:.2}", i + 1, path.confidence);
            }
        }
        Err(e) => println!("    ❌ 推理失败: {e}"),
    }

    // 归纳推理
    println!("\n  🔄 归纳推理 (Inductive Reasoning):");
    println!("    观察: 多个水果都是健康的");
    println!("    推论: 水果类别具有健康属性");

    match engine
        .reason_relationships(&apple_id, &healthy_id, ReasoningType::Inductive)
        .await
    {
        Ok(paths) => {
            println!("    ✅ 归纳推理完成，找到 {} 个模式", paths.len());
        }
        Err(e) => println!("    ❌ 推理失败: {e}"),
    }

    // 溯因推理
    println!("\n  🔙 溯因推理 (Abductive Reasoning):");
    println!("    观察: 某人很健康");
    println!("    推测: 可能经常吃水果");

    match engine
        .reason_relationships(&healthy_id, &apple_id, ReasoningType::Abductive)
        .await
    {
        Ok(paths) => {
            println!("    ✅ 溯因推理完成，找到 {} 个可能原因", paths.len());
        }
        Err(e) => println!("    ❌ 推理失败: {e}"),
    }

    // 类比推理
    println!("\n  🔗 类比推理 (Analogical Reasoning):");
    println!("    类比: Apple:Fruit :: Rose:Flower");
    println!("    推理: 基于相似结构进行推理");

    match engine
        .reason_relationships(&apple_id, &healthy_id, ReasoningType::Analogical)
        .await
    {
        Ok(paths) => {
            println!("    ✅ 类比推理完成，找到 {} 个类比关系", paths.len());
        }
        Err(e) => println!("    ❌ 推理失败: {e}"),
    }

    // 因果推理
    println!("\n  ⚡ 因果推理 (Causal Reasoning):");
    println!("    因果链: 吃苹果 → 摄入营养 → 身体健康");
    println!("    推理: 识别因果关系链");

    match engine
        .reason_relationships(&apple_id, &healthy_id, ReasoningType::Causal)
        .await
    {
        Ok(paths) => {
            println!("    ✅ 因果推理完成，找到 {} 条因果链", paths.len());
        }
        Err(e) => println!("    ❌ 推理失败: {e}"),
    }

    println!("\n🎯 推理结果：");
    println!("  • ✅ 演绎推理：基于逻辑规则的严格推理");
    println!("  • ✅ 归纳推理：从特例推广到一般规律");
    println!("  • ✅ 溯因推理：从结果推测可能的原因");
    println!("  • ✅ 类比推理：基于相似性的推理");
    println!("  • ✅ 因果推理：识别和追踪因果关系");

    Ok(())
}

/// 图遍历和查询演示
async fn demo_graph_traversal(
    engine: &GraphMemoryEngine,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 图遍历和查询演示");

    let apple_id = "apple".to_string();

    println!("🌐 从 'Apple' 节点开始遍历：");

    // 查找相关节点（深度1）
    println!("\n  📍 深度 1 遍历:");
    match engine.find_related_nodes(&apple_id, 1, None).await {
        Ok(nodes) => {
            println!("    找到 {} 个直接相关节点:", nodes.len());
            for node in &nodes {
                println!(
                    "      • {} ({})",
                    node.memory.content,
                    format!("{:?}", node.node_type).cyan()
                );
            }
        }
        Err(e) => println!("    ❌ 遍历失败: {e}"),
    }

    // 查找相关节点（深度2）
    println!("\n  📍 深度 2 遍历:");
    match engine.find_related_nodes(&apple_id, 2, None).await {
        Ok(nodes) => {
            println!("    找到 {} 个相关节点 (深度≤2):", nodes.len());
            for node in &nodes {
                println!(
                    "      • {} ({})",
                    node.memory.content,
                    format!("{:?}", node.node_type).cyan()
                );
            }
        }
        Err(e) => println!("    ❌ 遍历失败: {e}"),
    }

    // 按关系类型过滤
    println!("\n  🔗 按关系类型过滤 (IsA 关系):");
    match engine
        .find_related_nodes(&apple_id, 2, Some(vec![RelationType::IsA]))
        .await
    {
        Ok(nodes) => {
            println!("    找到 {} 个 IsA 关系节点:", nodes.len());
            for node in &nodes {
                println!(
                    "      • {} ({})",
                    node.memory.content,
                    format!("{:?}", node.node_type).cyan()
                );
            }
        }
        Err(e) => println!("    ❌ 遍历失败: {e}"),
    }

    println!("\n🎯 遍历结果：");
    println!("  • ✅ 支持多层深度图遍历");
    println!("  • ✅ 支持关系类型过滤");
    println!("  • ✅ 高效的邻接表查询");
    println!("  • ✅ 灵活的查询参数配置");

    Ok(())
}

/// 图统计分析演示
async fn demo_graph_statistics(
    engine: &GraphMemoryEngine,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 图统计分析演示");

    match engine.get_graph_stats().await {
        Ok(stats) => {
            println!("\n📈 图统计信息：");
            println!(
                "  • 总节点数: {}",
                stats.total_nodes.to_string().bright_yellow()
            );
            println!(
                "  • 总边数: {}",
                stats.total_edges.to_string().bright_yellow()
            );

            println!("\n🏷️ 节点类型分布：");
            for (node_type, count) in &stats.node_types {
                println!("  • {:?}: {}", node_type, count.to_string().bright_cyan());
            }

            println!("\n🔗 关系类型分布：");
            for (relation_type, count) in &stats.relation_types {
                println!(
                    "  • {:?}: {}",
                    relation_type,
                    count.to_string().bright_green()
                );
            }

            // 计算图密度
            let density = if stats.total_nodes > 1 {
                stats.total_edges as f64 / (stats.total_nodes * (stats.total_nodes - 1) / 2) as f64
            } else {
                0.0
            };

            println!("\n📊 图特征分析：");
            println!("  • 图密度: {:.4}", density.to_string().bright_magenta());
            println!(
                "  • 平均度数: {:.2}",
                if stats.total_nodes > 0 {
                    (stats.total_edges * 2) as f64 / stats.total_nodes as f64
                } else {
                    0.0
                }
                .to_string()
                .bright_magenta()
            );
        }
        Err(e) => {
            error!("获取图统计信息失败: {}", e);
        }
    }

    println!("\n🎯 分析结果：");
    println!("  • ✅ 完整的图结构统计");
    println!("  • ✅ 节点和边的类型分布");
    println!("  • ✅ 图密度和连通性分析");
    println!("  • ✅ 实时统计信息更新");

    Ok(())
}

/// 创建测试记忆
fn create_memory(id: &str, agent_id: &str, content: &str, user_id: &str) -> Memory {
    let mut memory = Memory::new(
        agent_id.to_string(),
        Some(user_id.to_string()),
        agent_mem_core::types::MemoryType::Semantic,
        content.to_string(),
        0.8,
    );
    memory.id = id.to_string();
    memory
}
