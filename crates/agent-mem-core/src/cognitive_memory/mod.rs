//! Cognitive Memory Module
//!
//! Provides unified cognitive memory management with:
//! - CognitiveMemoryManager: Main memory management interface
//! - CognitiveMemoryConfig: Configuration options
//! - CognitiveStats: Statistics collection
//! - MemoryMetrics: Performance metrics

pub mod manager;
pub mod metrics;

pub use manager::{CognitiveMemoryManager, CognitiveMemoryConfig, CognitiveOperation, CognitiveResult, CognitiveStats};
pub use metrics::{MemoryMetrics, MemoryStatsByType, OperationTimer};
