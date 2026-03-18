//! AgentMem Server Binary
//!
//! Standalone server for AgentMem memory management platform.

use agent_mem_server::{MemoryServer, ServerConfig};
use clap::Parser;
use once_cell::sync::Lazy;
use std::process;
use tracing::{error, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Parser)]
#[command(name = "agent-mem-server")]
#[command(about = "AgentMem REST API Server")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    /// Server port
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// Server host
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Enable CORS
    #[arg(long, default_value = "true")]
    cors: bool,

    /// Enable authentication
    #[arg(long, default_value = "false")]
    auth: bool,

    /// JWT secret (required if auth is enabled)
    #[arg(long)]
    jwt_secret: Option<String>,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,

    /// Configuration file
    #[arg(short, long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // 初始化日志系统（在配置加载之前）
    init_logging(&cli.log_level);

    info!("🚀 AgentMem Server 启动中...");
    info!("版本: {}", env!("CARGO_PKG_VERSION"));

    // Create server configuration
    // Phase 10 MVP P0-1: 配置文件加载系统 ✅
    info!("📝 加载配置文件...");
    let mut config = match ServerConfig::load(cli.config.as_deref()) {
        Ok(cfg) => {
            info!("✅ 配置文件加载成功");
            cfg
        }
        Err(e) => {
            error!("❌ 配置文件加载失败: {}", e);
            eprintln!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    };

    // Override with CLI arguments
    info!("🔧 应用命令行参数覆盖...");
    config.port = cli.port;
    config.host = cli.host.clone();
    config.enable_cors = cli.cors;
    config.enable_auth = cli.auth;
    config.log_level = cli.log_level.clone();

    info!("  - 主机: {}", cli.host);
    info!("  - 端口: {}", cli.port);
    info!("  - CORS: {}", cli.cors);
    info!("  - 认证: {}", cli.auth);
    info!("  - 日志级别: {}", cli.log_level);

    if cli.auth {
        if let Some(secret) = cli.jwt_secret {
            config.jwt_secret = secret;
            info!("  - JWT Secret: 已配置");
        } else {
            error!("❌ 认证已启用但未提供 JWT Secret");
            eprintln!("Error: JWT secret is required when authentication is enabled");
            eprintln!("Use --jwt-secret <SECRET> or set AGENT_MEM_JWT_SECRET environment variable");
            process::exit(1);
        }
    }

    // Validate configuration
    info!("✅ 验证配置...");
    if let Err(e) = config.validate() {
        error!("❌ 配置验证失败: {}", e);
        eprintln!("Configuration error: {}", e);
        process::exit(1);
    }
    info!("✅ 配置验证通过");

    // Create necessary directories
    info!("📁 创建必要的目录...");
    if let Err(e) = create_directories() {
        error!("❌ 创建目录失败: {}", e);
        eprintln!("Failed to create directories: {}", e);
        process::exit(1);
    }
    info!("✅ 目录创建完成");

    // Create and start server
    info!("🔨 创建服务器实例...");
    info!("⏳ 正在初始化 Memory 组件（可能需要下载模型文件）...");
    match MemoryServer::new(config).await {
        Ok(server) => {
            info!("✅ 服务器实例创建成功");
            info!("🚀 启动 HTTP 服务器...");

            // Start server with graceful shutdown (handled inside server.start())
            if let Err(e) = server.start().await {
                error!("❌ 服务器运行错误: {}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            error!("❌ 服务器创建失败: {}", e);
            eprintln!("Failed to create server: {}", e);
            process::exit(1);
        }
    }
}

// 全局 guard 保持文件日志 writer 存活
static FILE_APPENDER_GUARD: Lazy<
    std::sync::Mutex<Option<tracing_appender::non_blocking::WorkerGuard>>,
> = Lazy::new(|| std::sync::Mutex::new(None));

/// 创建必要的目录
fn create_directories() -> std::io::Result<()> {
    use std::fs;
    use std::path::Path;

    // 需要创建的目录列表
    let directories = vec![
        "data",               // 数据库文件目录
        "data/vectors.lance", // 向量存储目录（LanceDB 会自动创建，但我们先创建父目录）
        "logs",               // 日志文件目录
    ];

    for dir in directories {
        let path = Path::new(dir);
        if !path.exists() {
            eprintln!("   创建目录: {}", dir);
            fs::create_dir_all(path)?;
        }
    }

    Ok(())
}

/// 初始化日志系统（控制台 + 文件）
fn init_logging(log_level: &str) {
    use std::fs;
    use std::path::Path;

    eprintln!("📝 初始化日志系统...");

    // 创建日志目录
    let log_dir = Path::new("logs");
    if !log_dir.exists() {
        eprintln!("   创建日志目录: {}", log_dir.display());
        fs::create_dir_all(log_dir)
            .map_err(|e| {
                eprintln!("Failed to create logs directory: {}", e);
                e
            })
            .unwrap_or_else(|e| {
                eprintln!("⚠️  Warning: Failed to create logs directory: {}. Logging will continue but may fail.", e);
            });
    }

    // 获取当前日期，用于生成日志文件名
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let dated_log_file = format!("agentmem-server.log.{}", today);
    let symlink_path = log_dir.join("agentmem-server.log");

    eprintln!("   日志文件: {}", dated_log_file);
    eprintln!(
        "   软链接: {} -> {}",
        symlink_path.display(),
        dated_log_file
    );

    // 文件日志层（每日轮转）
    let file_appender = tracing_appender::rolling::daily(log_dir, "agentmem-server.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true);

    // 控制台日志层
    let console_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_line_number(false);

    // 环境过滤器
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    eprintln!("   配置日志层...");

    // 组合所有层
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    // 保存 guard 到全局变量，防止被丢弃
    if let Ok(mut guard_lock) = FILE_APPENDER_GUARD.lock() {
        *guard_lock = Some(guard);
    } else {
        eprintln!("⚠️  Warning: Failed to acquire file appender guard lock. Logging may not work correctly.");
    }

    // 创建软链接指向最新的日志文件
    create_log_symlink(&symlink_path, &dated_log_file);

    eprintln!("✅ 日志系统已初始化");
    eprintln!("   - 控制台日志级别: {}", log_level);
    eprintln!("   - 文件日志: logs/{}", dated_log_file);
    eprintln!("   - 快捷访问: logs/agentmem-server.log (软链接)");
}

/// 创建软链接指向最新的日志文件
fn create_log_symlink(symlink_path: &std::path::Path, target_filename: &str) {
    use std::fs;

    // 如果软链接已存在，先删除
    if symlink_path.exists() || symlink_path.is_symlink() {
        if let Err(e) = fs::remove_file(symlink_path) {
            eprintln!("   ⚠️  删除旧软链接失败: {}", e);
            return;
        }
    }

    // 创建新的软链接
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        if let Err(e) = symlink(target_filename, symlink_path) {
            eprintln!("   ⚠️  创建软链接失败: {}", e);
        } else {
            eprintln!("   ✅ 软链接创建成功");
        }
    }

    #[cfg(windows)]
    {
        // Windows 需要管理员权限创建符号链接，这里使用硬链接或复制
        eprintln!("   ⚠️  Windows 平台不支持软链接，请直接访问带日期的日志文件");
    }
}
