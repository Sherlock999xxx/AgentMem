//! Performance analysis routes
//!
//! 🆕 Phase 4.2: 性能分析功能
//! 提供性能分析、瓶颈识别和优化建议

use crate::error::ServerResult;
use crate::models;
use crate::routes::memory::{get_search_stats, MemoryManager};
use axum::{extract::Extension, response::Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// 性能分析响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysisResponse {
    /// 总体性能评分（0-100）
    pub overall_score: f64,
    /// 性能指标
    pub metrics: HashMap<String, f64>,
    /// 性能瓶颈
    pub bottlenecks: Vec<Bottleneck>,
    /// 优化建议
    pub recommendations: Vec<String>,
    /// 分析时间戳
    pub timestamp: chrono::DateTime<Utc>,
}

/// 性能瓶颈
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottleneck {
    /// 瓶颈类型
    pub category: String,
    /// 瓶颈描述
    pub description: String,
    /// 严重程度（LOW, MEDIUM, HIGH）
    pub severity: String,
    /// 影响评分
    pub impact_score: f64,
}

/// 🆕 Phase 4.2: 计算性能评分
///
/// 基于多个性能指标计算总体性能评分（0-100）
fn calculate_performance_score(metrics: &HashMap<String, f64>) -> f64 {
    let mut score: f64 = 100.0;

    // 1. 搜索延迟评分（权重：30%）
    if let Some(&search_latency) = metrics.get("avg_search_latency_ms") {
        if search_latency > 100.0 {
            score -= 30.0; // 延迟超过100ms，扣30分
        } else if search_latency > 50.0 {
            score -= 15.0; // 延迟50-100ms，扣15分
        } else if search_latency > 20.0 {
            score -= 5.0; // 延迟20-50ms，扣5分
        }
    }

    // 2. 缓存命中率评分（权重：25%）
    if let Some(&cache_hit_rate) = metrics.get("cache_hit_rate") {
        if cache_hit_rate < 0.5 {
            score -= 25.0; // 缓存命中率低于50%，扣25分
        } else if cache_hit_rate < 0.7 {
            score -= 12.0; // 缓存命中率50-70%，扣12分
        } else if cache_hit_rate < 0.8 {
            score -= 5.0; // 缓存命中率70-80%，扣5分
        }
    }

    // 3. 吞吐量评分（权重：25%）
    if let Some(&throughput) = metrics.get("avg_throughput_ops_per_sec") {
        if throughput < 10.0 {
            score -= 25.0; // 吞吐量低于10 ops/s，扣25分
        } else if throughput < 50.0 {
            score -= 12.0; // 吞吐量10-50 ops/s，扣12分
        } else if throughput < 100.0 {
            score -= 5.0; // 吞吐量50-100 ops/s，扣5分
        }
    }

    // 4. 错误率评分（权重：20%）
    if let Some(&error_rate) = metrics.get("error_rate") {
        if error_rate > 0.1 {
            score -= 20.0; // 错误率超过10%，扣20分
        } else if error_rate > 0.05 {
            score -= 10.0; // 错误率5-10%，扣10分
        } else if error_rate > 0.01 {
            score -= 5.0; // 错误率1-5%，扣5分
        }
    }

    score.max(0.0f64).min(100.0f64)
}

/// 🆕 Phase 4.2: 识别性能瓶颈
fn identify_bottlenecks(metrics: &HashMap<String, f64>) -> Vec<Bottleneck> {
    let mut bottlenecks = Vec::new();

    // 1. 搜索延迟瓶颈
    if let Some(&latency) = metrics.get("avg_search_latency_ms") {
        if latency > 100.0 {
            bottlenecks.push(Bottleneck {
                category: "搜索延迟".to_string(),
                description: format!(
                    "平均搜索延迟 {}ms 过高，建议优化向量搜索或增加缓存",
                    latency
                ),
                severity: "HIGH".to_string(),
                impact_score: 0.8,
            });
        } else if latency > 50.0 {
            bottlenecks.push(Bottleneck {
                category: "搜索延迟".to_string(),
                description: format!("平均搜索延迟 {}ms 较高，建议优化查询逻辑", latency),
                severity: "MEDIUM".to_string(),
                impact_score: 0.5,
            });
        }
    }

    // 2. 缓存命中率瓶颈
    if let Some(&hit_rate) = metrics.get("cache_hit_rate") {
        if hit_rate < 0.5 {
            bottlenecks.push(Bottleneck {
                category: "缓存效率".to_string(),
                description: format!(
                    "缓存命中率 {:.1}% 过低，建议增加缓存容量或优化缓存策略",
                    hit_rate * 100.0
                ),
                severity: "HIGH".to_string(),
                impact_score: 0.7,
            });
        } else if hit_rate < 0.7 {
            bottlenecks.push(Bottleneck {
                category: "缓存效率".to_string(),
                description: format!(
                    "缓存命中率 {:.1}% 较低，建议优化缓存预热策略",
                    hit_rate * 100.0
                ),
                severity: "MEDIUM".to_string(),
                impact_score: 0.4,
            });
        }
    }

    // 3. 吞吐量瓶颈
    if let Some(&throughput) = metrics.get("avg_throughput_ops_per_sec") {
        if throughput < 10.0 {
            bottlenecks.push(Bottleneck {
                category: "吞吐量".to_string(),
                description: format!(
                    "吞吐量 {:.1} ops/s 过低，建议优化批量操作或增加并发",
                    throughput
                ),
                severity: "HIGH".to_string(),
                impact_score: 0.9,
            });
        } else if throughput < 50.0 {
            bottlenecks.push(Bottleneck {
                category: "吞吐量".to_string(),
                description: format!(
                    "吞吐量 {:.1} ops/s 较低，建议优化数据库查询或索引",
                    throughput
                ),
                severity: "MEDIUM".to_string(),
                impact_score: 0.5,
            });
        }
    }

    bottlenecks
}

/// 🆕 Phase 4.2: 生成优化建议
fn generate_recommendations(
    _metrics: &HashMap<String, f64>,
    bottlenecks: &[Bottleneck],
) -> Vec<String> {
    let mut recommendations = Vec::new();

    // 基于瓶颈生成建议
    for bottleneck in bottlenecks {
        match bottleneck.category.as_str() {
            "搜索延迟" => {
                recommendations
                    .push("优化向量搜索：考虑使用更高效的向量索引（如HNSW）".to_string());
                recommendations.push("增加缓存：提高查询结果缓存命中率".to_string());
            }
            "缓存效率" => {
                recommendations.push("增加缓存容量：调整SEARCH_CACHE_CAPACITY环境变量".to_string());
                recommendations.push("优化缓存预热：定期执行缓存预热操作".to_string());
            }
            "吞吐量" => {
                recommendations.push("优化批量操作：使用批量API减少网络往返".to_string());
                recommendations.push("增加并发：调整连接池大小或使用异步处理".to_string());
            }
            _ => {}
        }
    }

    // 通用建议
    if bottlenecks.is_empty() {
        recommendations.push("性能表现良好，继续保持当前配置".to_string());
    } else {
        recommendations.push("定期监控性能指标，及时发现问题".to_string());
        recommendations.push("考虑使用性能基准测试API进行定期测试".to_string());
    }

    // 去重
    recommendations.sort();
    recommendations.dedup();

    recommendations
}

/// 获取性能分析报告
///
/// 🆕 Phase 4.2: 性能分析 - 提供性能分析、瓶颈识别和优化建议
#[utoipa::path(
    get,
    path = "/api/v1/performance/analysis",
    tag = "performance",
    responses(
        (status = 200, description = "Performance analysis completed successfully", body = PerformanceAnalysisResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_performance_analysis(
    Extension(_memory_manager): Extension<Arc<MemoryManager>>,
) -> ServerResult<Json<models::ApiResponse<PerformanceAnalysisResponse>>> {
    info!("📊 生成性能分析报告");

    // 1. 收集性能指标
    let search_stats = get_search_stats();
    let stats_read = search_stats.read().await;

    let mut metrics = HashMap::new();

    // 搜索相关指标
    metrics.insert(
        "avg_search_latency_ms".to_string(),
        stats_read.avg_latency_ms(),
    );
    metrics.insert("cache_hit_rate".to_string(), stats_read.cache_hit_rate());
    metrics.insert(
        "total_searches".to_string(),
        stats_read.get_total_searches() as f64,
    );
    metrics.insert("cache_hits".to_string(), stats_read.get_cache_hits() as f64);
    metrics.insert(
        "cache_misses".to_string(),
        stats_read.get_cache_misses() as f64,
    );

    // 计算吞吐量（基于总搜索次数和平均延迟）
    let total_searches = stats_read.get_total_searches() as f64;
    let avg_latency_ms = stats_read.avg_latency_ms();
    let throughput = if avg_latency_ms > 0.0 {
        1000.0 / avg_latency_ms // ops per second
    } else {
        0.0
    };
    metrics.insert("avg_throughput_ops_per_sec".to_string(), throughput);

    // 计算错误率（基于缓存未命中率作为代理）
    let error_rate = if total_searches > 0.0 {
        let failed_searches = stats_read.get_cache_misses() as f64;
        failed_searches / total_searches * 0.1 // 假设10%的未命中会导致错误
    } else {
        0.0
    };
    metrics.insert("error_rate".to_string(), error_rate);

    // 2. 计算性能评分
    let overall_score = calculate_performance_score(&metrics);

    // 3. 识别性能瓶颈
    let bottlenecks = identify_bottlenecks(&metrics);

    // 4. 生成优化建议
    let recommendations = generate_recommendations(&metrics, &bottlenecks);

    let response = PerformanceAnalysisResponse {
        overall_score,
        metrics,
        bottlenecks,
        recommendations,
        timestamp: Utc::now(),
    };

    info!(
        "📊 性能分析完成: 总体评分={:.1}, 瓶颈数={}, 建议数={}",
        response.overall_score,
        response.bottlenecks.len(),
        response.recommendations.len()
    );

    Ok(Json(models::ApiResponse::success(response)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// 🆕 Phase 4.2: 测试性能评分计算
    #[test]
    fn test_performance_score_calculation() {
        // 测试1: 优秀性能
        let mut metrics1 = HashMap::new();
        metrics1.insert("avg_search_latency_ms".to_string(), 10.0);
        metrics1.insert("cache_hit_rate".to_string(), 0.9);
        metrics1.insert("avg_throughput_ops_per_sec".to_string(), 100.0);
        metrics1.insert("error_rate".to_string(), 0.0);
        let score1 = calculate_performance_score(&metrics1);
        assert!(score1 >= 90.0, "优秀性能应该得到高分");

        // 测试2: 较差性能
        let mut metrics2 = HashMap::new();
        metrics2.insert("avg_search_latency_ms".to_string(), 200.0);
        metrics2.insert("cache_hit_rate".to_string(), 0.3);
        metrics2.insert("avg_throughput_ops_per_sec".to_string(), 5.0);
        metrics2.insert("error_rate".to_string(), 0.2);
        let score2 = calculate_performance_score(&metrics2);
        assert!(score2 < 50.0, "较差性能应该得到低分");
        assert!(score1 > score2, "优秀性能应该比较差性能得分高");
    }

    /// 🆕 Phase 4.2: 测试瓶颈识别
    #[test]
    fn test_bottleneck_identification() {
        let mut metrics = HashMap::new();
        metrics.insert("avg_search_latency_ms".to_string(), 150.0); // 高延迟
        metrics.insert("cache_hit_rate".to_string(), 0.3); // 低命中率
        metrics.insert("avg_throughput_ops_per_sec".to_string(), 5.0); // 低吞吐量

        let bottlenecks = identify_bottlenecks(&metrics);

        assert!(bottlenecks.len() >= 2, "应该识别出多个瓶颈");
        assert!(
            bottlenecks.iter().any(|b| b.category == "搜索延迟"),
            "应该识别搜索延迟瓶颈"
        );
        assert!(
            bottlenecks.iter().any(|b| b.category == "缓存效率"),
            "应该识别缓存效率瓶颈"
        );
    }

    /// 🆕 Phase 4.2: 测试优化建议生成
    #[test]
    fn test_recommendations_generation() {
        let mut metrics = HashMap::new();
        metrics.insert("avg_search_latency_ms".to_string(), 150.0);
        metrics.insert("cache_hit_rate".to_string(), 0.3);

        let bottlenecks = identify_bottlenecks(&metrics);
        let recommendations = generate_recommendations(&metrics, &bottlenecks);

        assert!(!recommendations.is_empty(), "应该生成优化建议");
        assert!(
            recommendations.iter().any(|r| r.contains("缓存")),
            "应该包含缓存相关建议"
        );
    }
}
