//! Resource Abstraction Layer for AgentMem
//!
//! This crate implements the resource abstraction layer that treats all memory sources
//! as file-like entities with URIs, media types, and metadata.
//!
//! # Architecture
//!
//! ```text
//! ResourceManager
//!     ├── mount_resource(uri, scope) -> ResourceId
//!     ├── resolve_resource(resource_id) -> ResourceContent
//!     └── list_resources(scope) -> Vec<Resource>
//!
//! Components:
//!     ├── MediaTypeDetector (magic bytes + extension)
//!     ├── URIResolver (file://, http://, conv://, doc://)
//!     └── ResourceStorage (persistence layer)
//! ```
//!
//! # Example
//!
//! ```rust
//! use agent_mem_resource::{ResourceManager, Resource, URI};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let manager = ResourceManager::new();
//!
//!     // Mount a resource
//!     let resource_id = manager.mount_resource(
//!         "file:///path/to/document.md",
//!         "user-123",
//!         Some("agent-456")
//!     ).await?;
//!
//!     // Resolve resource content
//!     let content = manager.resolve_resource(&resource_id).await?;
//!     println!("Content: {:?}", content);
//!
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod models;
pub mod detector;
pub mod resolver;
pub mod manager;

// Re-exports for convenience
pub use error::{ResourceError, Result};
pub use models::{Resource, ResourceId, ResourceMetadata, ResourceStatus, MediaType};
pub use detector::MediaTypeDetector;
pub use resolver::{URIResolver, URI};
pub use manager::ResourceManager;
