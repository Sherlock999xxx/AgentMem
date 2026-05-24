//! Cognitive Memory Module
//!
//! Provides unified cognitive memory management with:
//! - CognitiveMemoryManager: Main memory management interface
//! - CognitiveMemoryConfig: Configuration options
//! - CognitiveStats: Statistics collection
//! - MemoryMetrics: Performance metrics
//! - MemoryExport/Import: Data serialization

pub mod manager;
pub mod metrics;
pub mod export;

pub use manager::{CognitiveMemoryManager, CognitiveMemoryConfig, CognitiveOperation, CognitiveResult, CognitiveStats};
pub use metrics::{MemoryMetrics, MemoryStatsByType, OperationTimer, CacheStats};
pub use export::{MemoryExport, MemoryExportItem, MemoryImportResult};
