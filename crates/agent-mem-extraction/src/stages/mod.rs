//! Standard extraction stages

pub mod ingestor;
pub mod preprocessor;
pub mod extractor;
pub mod deduper;
pub mod categorizer;
pub mod indexer;
pub mod response;

// Re-export standard stages
pub use ingestor::ResourceIngestor;
pub use preprocessor::MultimodalPreprocessor;
pub use extractor::ItemExtractor;
pub use deduper::DedupeMerger;
pub use categorizer::AutoCategorizer;
pub use indexer::IndexPersistor;
pub use response::ResponseBuilder;
