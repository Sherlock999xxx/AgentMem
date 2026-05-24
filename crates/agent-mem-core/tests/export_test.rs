//! Export/Import Module Tests

use agent_mem_core::{
    cognitive_memory::{MemoryExport, MemoryImportResult},
    types::{Memory, MemoryType},
};

#[tokio::test]
async fn test_memory_export_basic() {
    let memories = vec![
        Memory::new(
            "agent-1".to_string(),
            Some("user-1".to_string()),
            MemoryType::Semantic,
            "Test semantic memory".to_string(),
            0.9,
        ),
        Memory::new(
            "agent-1".to_string(),
            None,
            MemoryType::Core,
            "Important core memory".to_string(),
            1.0,
        ),
    ];

    let export = MemoryExport::new(memories);
    assert_eq!(export.memories.len(), 2);
    assert_eq!(export.version, "1.0");
}

#[tokio::test]
async fn test_export_to_json() {
    let memory = Memory::new(
        "test-agent".to_string(),
        None,
        MemoryType::Episodic,
        "Test episodic".to_string(),
        0.7,
    );

    let export = MemoryExport::new(vec![memory]);
    let json = export.to_json().expect("Should serialize to JSON");
    
    // Verify JSON structure - check for expected keys
    assert!(json.contains("\"memories\""), "JSON should contain memories array");
    assert!(json.contains("\"version\""), "JSON should contain version field");
    assert!(json.contains("\"1.0\""), "JSON should contain version 1.0");
    assert!(!json.is_empty(), "JSON should not be empty");
}

#[tokio::test]
async fn test_export_from_json() {
    let json = r#"{
        "version": "1.0",
        "timestamp": "2024-01-01T00:00:00Z",
        "memories": [
            {
                "id": "mem-1",
                "agent_id": "agent-1",
                "user_id": null,
                "memory_type": "semantic",
                "content": "Test content",
                "importance": 0.8,
                "created_at": "2024-01-01T00:00:00Z"
            }
        ],
        "metadata": {}
    }"#;

    let export = MemoryExport::from_json(json).expect("Should deserialize from JSON");
    assert_eq!(export.version, "1.0");
    assert_eq!(export.memories.len(), 1);
}

#[tokio::test]
async fn test_import_result_success() {
    let result = MemoryImportResult::success(100);

    assert_eq!(result.total, 100);
    assert_eq!(result.imported, 100);
    assert_eq!(result.failed, 0);
    assert!(result.errors.is_empty());
}

#[tokio::test]
async fn test_import_result_with_errors() {
    let errors = vec![
        "Failed to parse memory 1".to_string(),
        "Invalid type for memory 2".to_string(),
    ];

    let result = MemoryImportResult::with_errors(50, errors);

    assert_eq!(result.total, 50);
    assert_eq!(result.imported, 48);
    assert_eq!(result.failed, 2);
    assert_eq!(result.errors.len(), 2);
}

#[tokio::test]
async fn test_export_empty() {
    let export = MemoryExport::new(vec![]);
    assert_eq!(export.memories.len(), 0);

    let json = export.to_json().expect("Should serialize empty export");
    assert!(json.contains("\"memories\": []"));
}
