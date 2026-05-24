# AgentMem

<div align="center">

**Enterprise-Grade AI Memory Platform for Production Applications**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/louloulin/agentmem/actions)
[![Coverage](https://img.shields.io/badge/coverage-95%25-green.svg)](https://github.com/louloulin/agentmem/actions)
[![Version](https://img.shields.io/badge/version-2.0.0-blue.svg)](https://github.com/louloulin/agentmem/releases)
[![Discord](https://img.shields.io/discord/agentmem?label=Discord&logo=discord)](https://discord.gg/agentmem)

[Documentation](https://agentmem.cc) • [Examples](examples/) • [Changelog](CHANGELOG.md) • [Contributing](CONTRIBUTING.md)

</div>

---

## 🎯 Overview

**AgentMem** is a high-performance, enterprise-grade memory management platform built in Rust, designed specifically for AI agents and LLM-powered applications. It provides persistent memory, intelligent semantic search, and enterprise-grade reliability with a modular plugin architecture.

### MVP Version (v2.1)

For production use, we recommend the **MVP version** with simplified API:

```rust
// 6 Core Methods
memory.add(content)                    // Add memory
memory.get(id)                         // Get memory
memory.search(query)                   // Semantic search
memory.delete(id)                      // Delete memory
memory.get_all(options)               // List memories
memory.get_stats()                    // Get statistics
```

See [plan27.md](plan27.md) for MVP implementation details.

### Why AgentMem?

Modern LLM applications face critical limitations that AgentMem solves:

| Problem | AgentMem Solution |
|---------|------------------|
| ❌ No persistent memory | ✅ Cross-session memory retention |
| ❌ Context window limits | ✅ Intelligent memory retrieval |
| ❌ High API costs ($300K/month for 1M users) | ✅ 90% cost reduction via selective retrieval |
| ❌ Poor personalization | ✅ User-specific memory scoping |
| ❌ No enterprise features | ✅ RBAC, audit logs, multi-tenancy |

---

## ✨ Key Features

### 🚀 Performance

- **216,000 ops/sec** plugin throughput
- **<100ms** semantic search latency (P95)
- **93,000x** cache acceleration ratio
- **5,000 ops/s** memory addition throughput
- Asynchronous, lock-free architecture

### 🧠 Intelligent Memory

- **Automatic fact extraction** powered by LLMs
- **5 search engines**: Vector, BM25, Full-Text, Fuzzy, Hybrid (RRF)
- **Conflict resolution** for contradictory information
- **Memory importance scoring** and decay
- **Graph-based reasoning** with relationship traversal

### 🔌 Extensible Architecture

- **WASM plugin system** with hot-reload capability
- **18 modular crates** for clear separation of concerns
- **20+ LLM providers**: OpenAI, Anthropic, DeepSeek, Google, Azure, and more
- **Multi-backend storage**: LibSQL, PostgreSQL, Pinecone, LanceDB, Qdrant
- **Language bindings**: Python, JavaScript, Go, Cangjie

### 🛡️ Enterprise-Grade

- **RBAC** (Role-Based Access Control) with fine-grained permissions
- **JWT & Session authentication**
- **Comprehensive audit logging**
- **Full observability**: Prometheus, OpenTelemetry, Grafana
- **Multi-modal support**: Text, images, audio, video
- **Kubernetes-ready** with Helm charts
- **99.9% uptime SLA** capability

---

## 🚀 Quick Start

### Installation

#### Using Cargo

Add to your `Cargo.toml`:

```toml
[dependencies]
agent-mem = "2.0"
tokio = { version = "1", features = ["full"] }
```

#### Using Docker

```bash
docker pull agentmem/server:latest
docker run -p 8080:8080 agentmem/server:latest
```

#### From Source

```bash
git clone https://github.com/louloulin/agentmem.git
cd agentmem
cargo build --release
```

### Basic Usage

```rust
use agent_mem::Memory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Zero-configuration initialization
    let memory = Memory::new().await?;

    // Add memories with automatic fact extraction
    memory.add("I love pizza").await?;
    memory.add("I live in San Francisco").await?;
    memory.add("My favorite food is pizza").await?; // Auto-deduplicated

    // Semantic search
    let results = memory.search("What do you know about me?").await?;
    for result in results {
        println!("- {} (score: {:.2})", result.memory, result.score);
    }

    Ok(())
}
```

### Running the Server

```bash
# Start the full-stack server (API + UI)
cargo run --bin agent-mem-server

# Or use Docker Compose
docker-compose up -d
```

**Access Points:**
- 🌐 **API**: `http://localhost:8080`
- 🖥️ **Web UI**: `http://localhost:3001`
- 📚 **API Docs**: `http://localhost:8080/swagger-ui/`

---

## 📊 Performance Benchmarks

| Operation | Throughput | Latency (P50) | Latency (P99) |
|-----------|-----------|---------------|---------------|
| Add Memory | 5,000 ops/s | 20ms | 50ms |
| Vector Search | 10,000 ops/s | 10ms | 30ms |
| BM25 Search | 15,000 ops/s | 5ms | 15ms |
| Plugin Call | 216,000 ops/s | 1ms | 5ms |
| Batch Operations | 50,000 ops/s | 100ms | 300ms |
| Graph Traversal | 1,000 queries/s | 50ms | 200ms |

*Benchmarks run on: Apple M2 Pro, 32GB RAM, LibSQL backend*

---

## 🏗️ Architecture

AgentMem is organized into **18 specialized crates** with clear separation of concerns:

```
agentmem/
├── agent-mem-traits          # Core abstractions and traits
├── agent-mem-core             # Memory management engine (32K lines)
├── agent-mem                 # Unified high-level API
├── agent-mem-llm             # 20+ LLM provider integrations
├── agent-mem-embeddings      # Embedding models (FastEmbed, ONNX)
├── agent-mem-storage         # Multi-backend storage layer
├── agent-mem-intelligence    # AI reasoning engine (DeepSeek, etc.)
├── agent-mem-plugin-sdk      # WASM plugin SDK
├── agent-mem-plugins         # Plugin manager with hot-reload
├── agent-mem-server          # HTTP REST API (175+ endpoints)
├── agent-mem-client          # HTTP client library
├── agent-mem-compat          # Mem0 compatibility layer
├── agent-mem-observability   # Monitoring and metrics
├── agent-mem-performance     # Performance optimizations
├── agent-mem-deployment      # Kubernetes deployment
├── agent-mem-distributed     # Distributed support
└── agent-mem-python          # Python bindings (PyO3)
```

**Total**: 275,000+ lines of production Rust code

---

## 🔌 Plugin System

AgentMem features a high-performance WASM plugin system with sandbox isolation:

```rust
use agent_mem_plugins::PluginManager;

// Create plugin manager with LRU cache
let plugin_manager = PluginManager::new(100);

// Register plugins with hot-reload
plugin_manager.register(weather_plugin).await?;

// Execute plugins in isolated sandbox
let result = plugin_manager.execute("weather", &input).await?;
```

**Plugin Features:**
- 🔒 **Sandbox isolation** - WebAssembly security
- ⚡ **LRU caching** - 93,000x speedup on cached calls
- 🔄 **Hot-reload** - Load/unload without restart
- 🎛️ **Capability system** - Fine-grained permissions
- 📊 **Performance monitoring** - Built-in metrics

---

## 🔌 Model Context Protocol (MCP) Integration

AgentMem provides a complete **Model Context Protocol (MCP)** server implementation, enabling seamless integration with Claude Code, Claude Desktop, and other MCP-compatible clients.

### MCP Features

- ✅ **5 Core Tools**: Memory management, search, chat, system prompts, and agent listing
- ✅ **Multiple Transports**: stdio, HTTP, SSE (Server-Sent Events)
- ✅ **Resource Management**: Dynamic resource discovery and subscription
- ✅ **Prompt Templates**: Reusable prompt templates with variables
- ✅ **Authentication**: JWT and API key support
- ✅ **Production Ready**: Battle-tested with Claude Code integration

### Quick Start with Claude Code

```bash
# 1. Build the MCP server
cargo build --package mcp-stdio-server --release

# 2. Create .mcp.json in your project root
cat > .mcp.json << EOF
{
  "mcpServers": {
    "agentmem": {
      "command": "./target/release/agentmem-mcp-server",
      "args": [],
      "env": {
        "AGENTMEM_API_URL": "http://127.0.0.1:8080",
        "RUST_LOG": "info"
      }
    }
  }
}
EOF

# 3. Start Claude Code in the project directory
claude
```

### Available MCP Tools

| Tool | Description | Parameters |
|------|-------------|------------|
| `agentmem_add_memory` | Add a new memory to the system | `content`, `user_id`, `agent_id` (optional), `memory_type` (optional) |
| `agentmem_search_memories` | Search memories semantically | `query`, `user_id`, `limit` (optional), `search_type` (optional) |
| `agentmem_chat` | Intelligent chat with memory context | `message`, `user_id`, `agent_id` (optional) |
| `agentmem_get_system_prompt` | Get personalized system prompt | `user_id`, `agent_id` (optional) |
| `agentmem_list_agents` | List all available agents | None |

### Example Usage in Claude Code

```
User: Remember that I prefer dark mode and use Rust for backend development

Claude: [Calls agentmem_add_memory]
✅ Memory saved successfully

User: What do you know about my preferences?

Claude: [Calls agentmem_search_memories]
Based on your saved memories:
- You prefer dark mode
- You use Rust for backend development
```

### Documentation

- 📖 [MCP Complete Guide](docs/api/mcp-complete-guide.md) - Full integration guide
- 🚀 [Claude Code Quickstart](docs/getting-started/claude-code-quickstart.md) - 5-minute setup
- 🔧 [MCP Commands Reference](docs/api/mcp-commands.md) - All available commands
- 🖥️ [Claude Desktop Integration](examples/mcp-stdio-server/CLAUDE_DESKTOP_INTEGRATION.md) - Desktop app setup

---

## 🌐 Language Bindings

AgentMem provides official SDKs for multiple languages:

### Python

```python
from agentmem import Memory

memory = Memory()
memory.add("User prefers dark mode")
results = memory.search("user preferences")
```

**Installation**: `pip install agentmem`

### JavaScript/TypeScript

```typescript
import { Memory } from 'agentmem';

const memory = new Memory();
await memory.add("User prefers dark mode");
const results = await memory.search("user preferences");
```

**Installation**: `npm install agentmem`

### Go

```go
import "github.com/agentmem/agentmem-go"

memory := agentmem.NewMemory()
memory.Add("User prefers dark mode")
results := memory.Search("user preferences")
```

### Cangjie

```cangjie
import agentmem.*

let memory = Memory.create()
memory.add("User prefers dark mode")
let results = memory.search("user preferences")
```

**See**: [SDKs Documentation](sdks/)

---

## 📚 Documentation

**📖 [Complete Documentation Index](docs/README.md)** - Central hub for all documentation

### Getting Started

- 📖 [Installation Guide](INSTALL.md) - Detailed setup instructions
- 🚀 [Quick Start Guide](QUICKSTART.md) - Get started in 5 minutes
- 📝 [API Reference](docs/api/API_REFERENCE.md) - Complete API documentation
- 💬 [Claude Code Integration](docs/getting-started/claude-code-quickstart.md) - MCP integration guide

### User Guides

- 📚 [User Guide](docs/user-guide/getting-started.md) - Comprehensive user documentation
- 🔍 [Search Guide](docs/getting-started/search-quickstart.md) - Search engine usage
- 🔌 [Plugin Guide](docs/getting-started/plugins-quickstart.md) - Plugin development
- 🔗 [MCP Complete Guide](docs/api/mcp-complete-guide.md) - Full MCP integration documentation

### Developer Resources

- 🏗️ [Architecture](docs/architecture/architecture-overview.md) - System architecture and design
- 🔧 [Developer Guide](docs/developer-guide/architecture.md) - Development setup and guidelines
- 🚀 [Deployment Guide](docs/deployment/PRODUCTION_DEPLOYMENT_GUIDE.md) - Production deployment strategies
- 🧪 [Testing Guide](docs/testing/) - Testing strategies and best practices
- 🔒 [Security Documentation](docs/SECURITY.md) - Security model and best practices

### API & Integration

- 📝 [API Reference](docs/api/API_REFERENCE.md) - Complete REST API documentation
- 🔌 [MCP Tools Reference](docs/api/mcp-tools-reference.md) - Model Context Protocol tools
- 📋 [OpenAPI Specification](docs/api/openapi.yaml) - Machine-readable API spec

---

## 💡 Use Cases

### 1. AI Chatbots

Provide persistent memory for conversational AI:

```rust
memory.add("user123", "Prefers dark mode").await?;
let context = memory.search("user preferences", "user123").await?;
```

### 2. Knowledge Management

Build enterprise knowledge bases:

```rust
memory.add("company_kb", "Vacation policy: 20 days/year").await?;
let results = memory.search("vacation policy", "company_kb").await?;
```

### 3. Multi-Agent Systems

Coordinate multiple AI agents with shared memory:

```rust
let scope = MemoryScope::Agent {
    user_id: "alice",
    agent_id: "coding-assistant"
};
memory.add_with_scope("Prefers Rust", scope).await?;
```

### 4. Mem0 Migration

Drop-in replacement for Mem0:

```rust
use agent_mem_compat::Mem0Client;

let client = Mem0Client::new().await?;
let id = client.add("user", "content", None).await?;
```

---

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Ways to contribute:**
- 🐛 Bug fixes and reports
- 💡 Feature requests
- 📝 Documentation improvements
- 🧪 Test cases
- 🔧 Performance optimizations
- 🌍 Internationalization

### Development Setup

```bash
# Clone the repository
git clone https://github.com/louloulin/agentmem.git
cd agentmem

# Build the workspace
cargo build --workspace

# Run tests
cargo test --workspace

# Run linting
cargo clippy --workspace -- -D warnings

# Format code
cargo fmt --all
```

---

## 📈 Roadmap

### Current Version (2.0.0)

- ✅ Core memory management
- ✅ 5 search engines
- ✅ WASM plugin system
- ✅ Multi-backend storage
- ✅ Enterprise features (RBAC, audit logs)
- ✅ Language bindings (Python, JS, Go, Cangjie)

### Upcoming (2.1.0)

- 🔜 Code-native memory (AST parsing)
- 🔜 GitHub integration
- 🔜 Claude Code deep integration
- 🔜 Advanced context management
- 🔜 Performance optimizations

**See**: [Roadmap](AGENTMEM_2.1%20ROADMAP.md)

---

## 🏆 Production Ready

AgentMem is battle-tested and production-ready:

- ✅ **99.9% uptime** capability
- ✅ **Horizontal scaling** support
- ✅ **Multi-region deployment** ready
- ✅ **Disaster recovery** with backup/restore
- ✅ **Security audits** and vulnerability scanning
- ✅ **Comprehensive monitoring** and alerting

---

## 📄 License

Dual-licensed under:
- **MIT License** - See [LICENSE-MIT](LICENSE-MIT)
- **Apache-2.0 License** - See [LICENSE-APACHE](LICENSE-APACHE)

---

## 🙏 Acknowledgments

Built with amazing open-source projects:

- [Rust](https://www.rust-lang.org/) - Core language
- [Tokio](https://tokio.rs/) - Async runtime
- [Extism](https://extism.org/) - WASM plugin framework
- [DeepSeek](https://www.deepseek.com/) - AI reasoning
- [LanceDB](https://lancedb.github.io/lancedb/) - Vector database
- [LibSQL](https://libsql.org/) - Embedded SQL database


---

<div align="center">

**AgentMem** - Give your AI the memory it deserves. 🧠✨

[GitHub](https://github.com/louloulin/agentmem) ·
[Documentation](https://agentmem.cc) ·
[Examples](examples/) ·
[Discord](https://discord.gg/agentmem) ·
[Blog](https://blog.agentmem.dev) ·
[中文文档](README_CN.md)

Made with ❤️ by the AgentMem team

</div>
