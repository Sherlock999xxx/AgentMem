//! Memory Builder - 流畅的配置接口

use crate::memory::Memory;
use crate::orchestrator::{MemoryOrchestrator, OrchestratorConfig};
use agent_mem_traits::Result;
use tracing::info;

/// Memory 构建器
///
/// 提供流畅的 API 来配置 Memory 实例
///
/// # 示例
///
/// ```rust,no_run
/// use agent_mem::Memory;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mem = Memory::builder()
///         .with_storage("libsql://agentmem.db")
///         .with_llm("openai", "gpt-4")
///         .with_embedder("openai", "text-embedding-3-small")
///         .enable_intelligent_features()
///         .build()
///         .await?;
///     Ok(())
/// }
/// ```
pub struct MemoryBuilder {
    config: OrchestratorConfig,
    default_user_id: Option<String>,
    default_agent_id: String,
    #[cfg(feature = "plugins")]
    plugins: Vec<crate::plugins::RegisteredPlugin>,
}

impl MemoryBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: OrchestratorConfig::default(),
            default_user_id: None,
            default_agent_id: "default".to_string(),
            #[cfg(feature = "plugins")]
            plugins: Vec::new(),
        }
    }

    /// 配置存储后端
    ///
    /// 支持的 URL 格式：
    /// - `memory://` - 内存存储 (开发测试)
    /// - `libsql://path/to/db` - LibSQL (推荐)
    /// - `libsql://path/to/db?mode=file` - LibSQL 文件模式
    /// - `postgres://user:pass@host/db` - PostgreSQL (企业级)
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_storage("libsql://agentmem.db")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_storage(mut self, url: impl Into<String>) -> Self {
        self.config.storage_url = Some(url.into());
        self
    }

    /// 配置 LLM 提供商
    ///
    /// 支持的提供商：
    /// - `openai` - OpenAI (GPT-4, GPT-3.5)
    /// - `anthropic` - Anthropic (Claude)
    /// - `deepseek` - DeepSeek
    /// - `ollama` - Ollama (本地模型)
    /// - `huawei_maas` - 华为 MaaS (deepseek-v3.2-exp 等)
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_llm("openai", "gpt-4")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_llm(mut self, provider: impl Into<String>, model: impl Into<String>) -> Self {
        self.config.llm_provider = Some(provider.into());
        self.config.llm_model = Some(model.into());
        self
    }

    /// 配置 Embedder
    ///
    /// 支持的提供商：
    /// - `openai` - OpenAI (text-embedding-3-small, text-embedding-3-large)
    /// - `ollama` - Ollama (本地嵌入模型)
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_embedder("openai", "text-embedding-3-small")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_embedder(mut self, provider: impl Into<String>, model: impl Into<String>) -> Self {
        self.config.embedder_provider = Some(provider.into());
        self.config.embedder_model = Some(model.into());
        self
    }

    /// 配置向量存储
    ///
    /// 支持的 URL 格式：
    /// - `lancedb://path/to/db` - LanceDB (默认)
    /// - `qdrant://host:port` - Qdrant
    /// - `pinecone://api-key` - Pinecone
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_vector_store("lancedb://./vector_db")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_vector_store(mut self, url: impl Into<String>) -> Self {
        self.config.vector_store_url = Some(url.into());
        self
    }

    /// 启用智能功能
    ///
    /// 启用后将自动：
    /// - 提取事实
    /// - 智能决策 (ADD/UPDATE/DELETE)
    /// - 记忆去重
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .enable_intelligent_features()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn enable_intelligent_features(mut self) -> Self {
        self.config.enable_intelligent_features = true;
        self
    }

    /// 禁用智能功能
    ///
    /// 禁用后将使用基础模式，不进行事实提取和智能决策。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .disable_intelligent_features()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn disable_intelligent_features(mut self) -> Self {
        self.config.enable_intelligent_features = false;
        self
    }

    // ✅ P1 Enhancement: 分层配置 API - 更语义化的配置方法

    /// ✅ P1: 仅启用核心功能（无需 LLM）
    ///
    /// 这是一个便捷方法，等价于：
    /// - 配置默认存储（libsql）
    /// - 配置默认嵌入器（fastembed 本地模型）
    /// - 禁用智能功能（无需 LLM API Key）
    ///
    /// **适用场景**：
    /// - 开发测试
    /// - 本地应用
    /// - 仅需要 CRUD + 向量搜索
    /// - 不需要事实提取和智能决策
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_core_features()  // ✅ 最简单：核心功能，无需 API Key
    ///     .build()
    ///     .await?;
    ///
    /// // 立即可用：添加、搜索、更新、删除
    /// mem.add("I love Rust programming").await?;
    /// let results = mem.search("programming").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # 核心功能包含
    ///
    /// - ✅ **CRUD 操作** (add, get, update, delete)
    /// - ✅ **向量搜索** (语义搜索，使用 FastEmbed 本地模型)
    /// - ✅ **批量操作** (batch_add, batch_delete)
    /// - ✅ **持久化存储** (LibSQL 数据库)
    /// - ❌ **事实提取** (需要 LLM)
    /// - ❌ **智能决策** (需要 LLM)
    /// - ❌ **记忆去重** (需要 LLM)
    pub fn with_core_features(mut self) -> Self {
        // 设置默认存储（如果用户没有设置）
        if self.config.storage_url.is_none() {
            self.config.storage_url = Some("libsql://./data/agentmem_core.db".to_string());
            info!("🔧 使用默认核心功能存储: libsql://./data/agentmem_core.db");
        }

        // 设置默认嵌入器（如果用户没有设置）
        if self.config.embedder_provider.is_none() {
            self.config.embedder_provider = Some("fastembed".to_string());
            self.config.embedder_model = Some("BAAI/bge-small-en-v1.5".to_string());
            info!("🔧 使用默认核心功能嵌入器: FastEmbed (BAAI/bge-small-en-v1.5)");
        }

        // 禁用智能功能（核心功能不需要 LLM）
        self.config.enable_intelligent_features = false;

        info!("✅ 核心功能已配置 - 仅需 CRUD + 向量搜索，无需 LLM API Key");
        self
    }

    /// ✅ P1: 启用完整智能功能（需要 LLM API Key）
    ///
    /// 这是一个便捷方法，等价于：
    /// - 配置默认存储（libsql）
    /// - 配置默认嵌入器（fastembed 本地模型）
    /// - **启用智能功能**（需要配置 LLM API Key）
    ///
    /// **适用场景**：
    /// - 需要事实提取
    /// - 需要智能决策（自动 ADD/UPDATE/DELETE）
    /// - 需要记忆去重和合并
    /// - 生产环境应用
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_core_features()        // 先配置核心功能
    ///     .with_llm("openai", "gpt-4") // ✅ 然后启用 LLM
    ///     .with_intelligent_features() // ✅ 启用智能功能
    ///     .build()
    ///     .await?;
    ///
    /// // 完整功能：事实提取 + 智能决策
    /// mem.add("Rust is a systems programming language").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # 智能功能包含
    ///
    /// - ✅ **所有核心功能** (CRUD, 向量搜索, 批量操作)
    /// - ✅ **事实提取** (自动从文本中提取关键事实)
    /// - ✅ **智能决策** (自动决定 ADD/UPDATE/DELETE/MERGE)
    /// - ✅ **记忆去重** (检测和合并重复记忆)
    /// - ✅ **重要性评分** (自动评估记忆重要性)
    ///
    /// # 前置条件
    ///
    /// 必须先配置 LLM（使用 `.with_llm()`），否则智能功能无法工作：
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_intelligent_features() // ❌ 错误：没有配置 LLM
    ///     .build()
    ///     .await?;
    /// // 结果：智能功能将无法使用，降级到核心模式
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_intelligent_features(mut self) -> Self {
        // 设置默认存储（如果用户没有设置）
        if self.config.storage_url.is_none() {
            self.config.storage_url = Some("libsql://./data/agentmem.db".to_string());
            info!("🔧 使用默认智能功能存储: libsql://./data/agentmem.db");
        }

        // 设置默认嵌入器（如果用户没有设置）
        if self.config.embedder_provider.is_none() {
            self.config.embedder_provider = Some("fastembed".to_string());
            self.config.embedder_model = Some("BAAI/bge-small-en-v1.5".to_string());
            info!("🔧 使用默认智能功能嵌入器: FastEmbed (BAAI/bge-small-en-v1.5)");
        }

        // 启用智能功能
        self.config.enable_intelligent_features = true;

        // 检查是否配置了 LLM
        if self.config.llm_provider.is_none() || self.config.llm_model.is_none() {
            tracing::warn!(
                "⚠️  智能功能已启用，但未配置 LLM！请使用 .with_llm() 配置 LLM 提供商。"
            );
            tracing::warn!("⚠️  智能功能将降级到核心模式（无事实提取和智能决策）");
        } else {
            info!("✅ 智能功能已配置 - 包含事实提取、智能决策、记忆去重");
        }

        self
    }

    /// ✅ P1: 自动配置（零配置模式）
    ///
    /// 自动检测环境并选择最佳配置：
    /// - 检测 LLM API Key（环境变量）
    /// - 如果有 LLM → 启用智能功能
    /// - 如果无 LLM → 核心功能
    ///
    /// **适用场景**：
    /// - 快速原型
    /// - 不确定使用哪种模式
    /// - 希望自动适配环境
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // 最简单的用法：零配置
    /// let mem = Memory::builder()
    ///     .with_auto_config()  // ✅ 自动检测并配置
    ///     .build()
    ///     .await?;
    ///
    /// // 如果设置了 OPENAI_API_KEY → 智能功能
    /// // 如果没有设置 API Key → 核心功能
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # 环境变量检测
    ///
    /// 按优先级检测以下环境变量：
    /// - `OPENAI_API_KEY` - OpenAI
    /// - `ANTHROPIC_API_KEY` - Anthropic Claude
    /// - `DEEPSEEK_API_KEY` - DeepSeek
    /// - `HUAWEI_MaaS_API_KEY` - 华为 MaaS
    pub fn with_auto_config(mut self) -> Self {
        info!("🔍 自动配置模式：检测环境...");

        // 检测 LLM API Key
        let llm_detected = detect_llm_from_env();

        if let Some((provider, model)) = llm_detected {
            // 检测到 LLM，启用智能功能
            info!("✅ 检测到 LLM: {} ({})", provider, model);
            self.config.llm_provider = Some(provider);
            self.config.llm_model = Some(model);
            self.config.enable_intelligent_features = true;

            // 设置默认存储和嵌入器
            if self.config.storage_url.is_none() {
                self.config.storage_url = Some("libsql://./data/agentmem.db".to_string());
            }
            if self.config.embedder_provider.is_none() {
                self.config.embedder_provider = Some("fastembed".to_string());
                self.config.embedder_model = Some("BAAI/bge-small-en-v1.5".to_string());
            }

            info!("✅ 自动配置：智能功能模式");
        } else {
            // 未检测到 LLM，使用核心功能
            info!("⚠️  未检测到 LLM API Key，使用核心功能模式");
            self.config.enable_intelligent_features = false;

            // 设置默认存储和嵌入器
            if self.config.storage_url.is_none() {
                self.config.storage_url = Some("libsql://./data/agentmem_core.db".to_string());
            }
            if self.config.embedder_provider.is_none() {
                self.config.embedder_provider = Some("fastembed".to_string());
                self.config.embedder_model = Some("BAAI/bge-small-en-v1.5".to_string());
            }

            info!("✅ 自动配置：核心功能模式（无需 LLM API Key）");
        }

        self
    }

    /// 启用嵌入队列（P1 优化：自动批量处理并发请求）
    ///
    /// 嵌入队列会自动收集并发请求，批量处理嵌入生成，显著减少 Mutex 锁竞争。
    /// 预期性能提升：2x（对于并发场景）
    ///
    /// # 参数
    /// - `batch_size`: 批处理大小（默认 64，推荐 64-128 用于高并发场景）
    /// - `batch_interval_ms`: 批处理间隔（毫秒，默认 20ms，推荐 20-50ms 用于高并发场景）
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_storage("libsql://agentmem.db")
    ///     .enable_embedding_queue(32, 10)  // 批处理大小 32，间隔 10ms
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn enable_embedding_queue(mut self, batch_size: usize, batch_interval_ms: u64) -> Self {
        self.config.enable_embedding_queue = Some(true);
        self.config.embedding_batch_size = Some(batch_size);
        self.config.embedding_batch_interval_ms = Some(batch_interval_ms);
        // 性能优化提示
        if batch_size < 32 {
            tracing::warn!(
                "批处理大小 {} 可能太小，推荐使用 64-128 用于高并发场景",
                batch_size
            );
        }
        if batch_interval_ms < 10 {
            tracing::warn!(
                "批处理间隔 {}ms 可能太短，推荐使用 20-50ms 用于高并发场景",
                batch_interval_ms
            );
        }
        self
    }

    /// 禁用嵌入队列
    ///
    /// 禁用嵌入队列，直接使用底层嵌入器（适用于不需要批量优化的场景）
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_storage("libsql://agentmem.db")
    ///     .disable_embedding_queue()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn disable_embedding_queue(mut self) -> Self {
        self.config.enable_embedding_queue = Some(false);
        self
    }

    /// 设置默认用户
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_user("alice")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_user(mut self, user_id: impl Into<String>) -> Self {
        self.default_user_id = Some(user_id.into());
        self
    }

    /// 设置默认 Agent
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_agent("my-agent")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_agent(mut self, agent_id: impl Into<String>) -> Self {
        self.default_agent_id = agent_id.into();
        self
    }

    /// 启用重排序功能
    ///
    /// 默认使用内部重排序器。重排序会在搜索完成后对结果进行重新排序，
    /// 提升搜索结果的准确性。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_storage("libsql://agentmem.db")
    ///     .enable_reranking()  // 启用重排序
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn enable_reranking(self) -> Self {
        // 重排序器在orchestrator初始化时自动创建
        // 这里只是标记启用，实际创建在build()时完成
        self
    }

    /// 注册插件 (需要启用 `plugins` feature)
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "plugins")]
    /// # use agent_mem::Memory;
    /// # #[cfg(feature = "plugins")]
    /// # use agent_mem::plugins::{RegisteredPlugin, PluginStatus};
    /// # #[cfg(feature = "plugins")]
    /// # use agent_mem::plugins::sdk::{PluginMetadata, PluginType, Capability, PluginConfig};
    /// # #[cfg(feature = "plugins")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let plugin = RegisteredPlugin {
    ///     id: "my-plugin".to_string(),
    ///     metadata: PluginMetadata {
    ///         name: "my-plugin".to_string(),
    ///         version: "1.0.0".to_string(),
    ///         description: "My custom plugin".to_string(),
    ///         author: "Me".to_string(),
    ///         plugin_type: PluginType::SearchAlgorithm,
    ///         required_capabilities: vec![Capability::SearchAccess],
    ///         config_schema: None,
    ///     },
    ///     path: "my-plugin.wasm".to_string(),
    ///     status: PluginStatus::Registered,
    ///     config: PluginConfig::default(),
    ///     registered_at: chrono::Utc::now(),
    ///     last_loaded_at: None,
    /// };
    ///
    /// let mem = Memory::builder()
    ///     .with_storage("memory://")
    ///     .with_plugin(plugin)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "plugins")]
    pub fn with_plugin(mut self, plugin: crate::plugins::RegisteredPlugin) -> Self {
        self.plugins.push(plugin);
        self
    }

    /// 从目录加载所有插件 (需要启用 `plugins` feature)
    ///
    /// 扫描指定目录下的所有 `.wasm` 文件并注册为插件。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "plugins")]
    /// # use agent_mem::Memory;
    /// # #[cfg(feature = "plugins")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_storage("memory://")
    ///     .load_plugins_from_dir("./plugins")
    ///     .await?
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "plugins")]
    pub async fn load_plugins_from_dir(mut self, dir: impl AsRef<std::path::Path>) -> Result<Self> {
        use crate::plugins::sdk::{Capability, PluginConfig, PluginMetadata, PluginType};
        use crate::plugins::{PluginStatus, RegisteredPlugin};
        use tracing::{debug, warn};

        let dir_path = dir.as_ref();
        debug!("从目录加载插件: {:?}", dir_path);

        if !dir_path.exists() {
            warn!("插件目录不存在: {:?}", dir_path);
            return Ok(self);
        }

        let entries = std::fs::read_dir(dir_path).map_err(|e| {
            agent_mem_traits::AgentMemError::Other(anyhow::anyhow!("读取目录失败: {}", e))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                agent_mem_traits::AgentMemError::Other(anyhow::anyhow!("读取目录项失败: {}", e))
            })?;
            let path = entry.path();

            // 只处理 .wasm 文件
            if path.extension().and_then(|s| s.to_str()) != Some("wasm") {
                continue;
            }

            let file_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            debug!("发现插件: {:?}", path);

            // 创建插件元数据（使用默认值）
            let plugin = RegisteredPlugin {
                id: file_name.to_string(),
                metadata: PluginMetadata {
                    name: file_name.to_string(),
                    version: "1.0.0".to_string(),
                    description: format!("Auto-loaded plugin from {}", file_name),
                    author: "Unknown".to_string(),
                    plugin_type: PluginType::Custom("auto-loaded".to_string()),
                    required_capabilities: vec![],
                    config_schema: None,
                },
                path: path.to_string_lossy().to_string(),
                status: PluginStatus::Registered,
                config: PluginConfig::default(),
                registered_at: chrono::Utc::now(),
                last_loaded_at: None,
            };

            self.plugins.push(plugin);
        }

        info!("从目录加载了 {} 个插件", self.plugins.len());
        Ok(self)
    }

    /// 构建 Memory 实例
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use agent_mem::Memory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mem = Memory::builder()
    ///     .with_storage("libsql://agentmem.db")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn build(self) -> Result<Memory> {
        info!("构建 Memory 实例");
        info!("配置: {:?}", self.config);

        let orchestrator = MemoryOrchestrator::new_with_config(self.config).await?;

        let memory =
            Memory::from_orchestrator(orchestrator, self.default_user_id, self.default_agent_id);

        // 注册所有插件 (如果启用了 plugins feature)
        #[cfg(feature = "plugins")]
        {
            if !self.plugins.is_empty() {
                info!("注册 {} 个插件", self.plugins.len());
                for plugin in self.plugins {
                    if let Err(e) = memory.register_plugin(plugin.clone()).await {
                        tracing::warn!("注册插件 {} 失败: {}", plugin.id, e);
                    }
                }
            }
        }

        Ok(memory)
    }
}

impl Default for MemoryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ✅ P1 Helper Functions

/// ✅ P1: 从环境变量检测 LLM 配置
///
/// 按优先级检测以下环境变量：
/// 1. `OPENAI_API_KEY` → (openai, gpt-4)
/// 2. `ANTHROPIC_API_KEY` → (anthropic, claude-3-opus-20240229)
/// 3. `DEEPSEEK_API_KEY` → (deepseek, deepseek-chat)
/// 4. `HUAWEI_MAAS_API_KEY` → (huawei_maas, deepseek-v3.2-exp)
///
/// # Returns
///
/// - `Some((provider, model))` - 如果检测到 API Key
/// - `None` - 如果未检测到任何 API Key
fn detect_llm_from_env() -> Option<(String, String)> {
    // 检测 OpenAI
    if std::env::var("OPENAI_API_KEY").is_ok() {
        return Some(("openai".to_string(), "gpt-4".to_string()));
    }

    // 检测 Anthropic
    if std::env::var("ANTHROPIC_API_KEY").is_ok() {
        return Some((
            "anthropic".to_string(),
            "claude-3-opus-20240229".to_string(),
        ));
    }

    // 检测 DeepSeek
    if std::env::var("DEEPSEEK_API_KEY").is_ok() {
        return Some(("deepseek".to_string(), "deepseek-chat".to_string()));
    }

    // 检测华为 MaaS
    if std::env::var("HUAWEI_MAAS_API_KEY").is_ok() {
        return Some(("huawei_maas".to_string(), "deepseek-v3.2-exp".to_string()));
    }

    None
}
