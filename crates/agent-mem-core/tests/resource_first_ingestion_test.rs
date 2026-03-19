//! Integration test for resource-first ingestion path
//!
//! This test validates the Phase B goal: resource-centric routing
//! - RouteDecision with resource_id routes to MemoryType::Resource
//! - RetrievalRequest with resource_id creates correct routing
//! - MemoryType is no longer the only agent routing key

use agent_mem_core::retrieval::{
    ActiveRetrievalConfig, ActiveRetrievalSystem, RetrievalRequest, RetrievalResponse,
    RetrievalStrategy,
};
use agent_mem_core::retrieval::router::{RetrievalRouter, RetrievalRouterConfig};
use agent_mem_core::types::MemoryType;

/// Create a test request without resource/category context
fn create_legacy_request() -> RetrievalRequest {
    RetrievalRequest {
        query: "test query".to_string(),
        target_memory_types: Some(vec![MemoryType::Semantic]),
        max_results: 10,
        preferred_strategy: Some(RetrievalStrategy::Embedding),
        context: None,
        enable_topic_extraction: false,
        enable_context_synthesis: false,
        resource_id: None,
        category_path: None,
    }
}

/// Create a test request with resource_id for resource-first routing
fn create_resource_first_request() -> RetrievalRequest {
    RetrievalRequest {
        query: "test query for resource".to_string(),
        target_memory_types: None,
        max_results: 10,
        preferred_strategy: None,
        context: None,
        enable_topic_extraction: false,
        enable_context_synthesis: false,
        resource_id: Some("resource-123".to_string()),
        category_path: None,
    }
}

/// Create a test request with category_path for category-aware routing
fn create_category_aware_request() -> RetrievalRequest {
    RetrievalRequest {
        query: "test query for category".to_string(),
        target_memory_types: None,
        max_results: 10,
        preferred_strategy: None,
        context: None,
        enable_topic_extraction: false,
        enable_context_synthesis: false,
        resource_id: None,
        category_path: Some("/preferences/communication".to_string()),
    }
}

/// Create a test request with both resource_id and category_path
fn create_resource_category_request() -> RetrievalRequest {
    RetrievalRequest {
        query: "test query for resource in category".to_string(),
        target_memory_types: None,
        max_results: 10,
        preferred_strategy: None,
        context: None,
        enable_topic_extraction: false,
        enable_context_synthesis: false,
        resource_id: Some("resource-456".to_string()),
        category_path: Some("/skills/programming".to_string()),
    }
}

/// Test 1: RouteDecision with resource_id routes to MemoryType::Resource
/// This verifies that resource_id takes precedence over MemoryType inference
#[tokio::test]
async fn test_resource_id_routes_to_resource_memory_type() {
    let config = RetrievalRouterConfig::default();
    let router = RetrievalRouter::new(config).await.expect("Failed to create router");

    let request = create_resource_first_request();
    let result = router.route_retrieval(&request, &[]).await.expect("Routing failed");

    // Verify that resource_id triggers Resource memory type routing
    assert!(
        result.decision.target_memory_types.contains(&MemoryType::Resource),
        "Expected MemoryType::Resource when resource_id is provided, got: {:?}",
        result.decision.target_memory_types
    );

    // Verify file-centric routing flag is set
    assert!(
        result.decision.route_by_resource_or_category,
        "Expected route_by_resource_or_category to be true when resource_id is provided"
    );

    // Verify target_resource_id is captured
    assert_eq!(
        result.decision.target_resource_id,
        Some("resource-123".to_string()),
        "Expected target_resource_id to match request resource_id"
    );
}

/// Test 2: RetrievalRequest with category_path captures category context
#[tokio::test]
async fn test_category_path_captured_in_routing() {
    let config = RetrievalRouterConfig::default();
    let router = RetrievalRouter::new(config).await.expect("Failed to create router");

    let request = create_category_aware_request();
    let result = router.route_retrieval(&request, &[]).await.expect("Routing failed");

    // Verify file-centric routing flag is set
    assert!(
        result.decision.route_by_resource_or_category,
        "Expected route_by_resource_or_category to be true when category_path is provided"
    );

    // Verify target_category_path is captured
    assert_eq!(
        result.decision.target_category_path,
        Some("/preferences/communication".to_string()),
        "Expected target_category_path to match request category_path"
    );
}

/// Test 3: Both resource_id and category_path are captured
#[tokio::test]
async fn test_both_resource_and_category_captured() {
    let config = RetrievalRouterConfig::default();
    let router = RetrievalRouter::new(config).await.expect("Failed to create router");

    let request = create_resource_category_request();
    let result = router.route_retrieval(&request, &[]).await.expect("Routing failed");

    // Verify both are captured
    assert_eq!(
        result.decision.target_resource_id,
        Some("resource-456".to_string()),
        "Expected target_resource_id to match"
    );
    assert_eq!(
        result.decision.target_category_path,
        Some("/skills/programming".to_string()),
        "Expected target_category_path to match"
    );

    // Resource takes precedence for memory type
    assert!(
        result.decision.target_memory_types.contains(&MemoryType::Resource),
        "Expected MemoryType::Resource when resource_id is present"
    );
}

/// Test 4: Legacy request without resource/category still works (backward compatibility)
#[tokio::test]
async fn test_legacy_routing_backward_compatible() {
    let config = RetrievalRouterConfig::default();
    let router = RetrievalRouter::new(config).await.expect("Failed to create router");

    let request = create_legacy_request();
    let result = router.route_retrieval(&request, &[]).await.expect("Routing failed");

    // Verify file-centric routing flag is NOT set
    assert!(
        !result.decision.route_by_resource_or_category,
        "Expected route_by_resource_or_category to be false for legacy requests"
    );

    // Verify no resource/category targets
    assert!(
        result.decision.target_resource_id.is_none(),
        "Expected no target_resource_id for legacy request"
    );
    assert!(
        result.decision.target_category_path.is_none(),
        "Expected no target_category_path for legacy request"
    );

    // Verify MemoryType routing still works
    assert!(
        result.decision.target_memory_types.contains(&MemoryType::Semantic),
        "Expected MemoryType::Semantic from request target_memory_types"
    );
}

/// Test 5: MemoryType is no longer the ONLY routing key
/// This test verifies the Phase B goal that resource_id provides
/// an alternative routing mechanism
#[tokio::test]
async fn test_memory_type_not_only_routing_key() {
    let config = RetrievalRouterConfig::default();
    let router = RetrievalRouter::new(config).await.expect("Failed to create router");

    // Test that resource_id can override MemoryType
    let request_with_both = RetrievalRequest {
        query: "test".to_string(),
        target_memory_types: Some(vec![MemoryType::Episodic]), // Explicit Episodic
        max_results: 10,
        preferred_strategy: None,
        context: None,
        enable_topic_extraction: false,
        enable_context_synthesis: false,
        resource_id: Some("resource-override".to_string()), // But also has resource_id
        category_path: None,
    };

    let result = router
        .route_retrieval(&request_with_both, &[])
        .await
        .expect("Routing failed");

    // Resource takes precedence, proving MemoryType is not the only routing key
    assert!(
        result.decision.target_memory_types.contains(&MemoryType::Resource),
        "Expected MemoryType::Resource to take precedence over explicit Episodic"
    );

    // File-centric routing is enabled
    assert!(
        result.decision.route_by_resource_or_category,
        "Expected file-centric routing when resource_id present"
    );
}

/// Test 6: ActiveRetrievalSystem integrates resource-first routing
#[tokio::test]
async fn test_active_retrieval_with_resource_context() {
    let config = ActiveRetrievalConfig::default();
    let system = ActiveRetrievalSystem::new(config)
        .await
        .expect("Failed to create ActiveRetrievalSystem");

    let request = create_resource_first_request();
    let response = system.retrieve(request).await.expect("Retrieval failed");

    // Verify routing decision has resource context
    assert!(
        response.routing_info.route_by_resource_or_category,
        "Expected file-centric routing in response"
    );
    assert_eq!(
        response.routing_info.target_resource_id,
        Some("resource-123".to_string()),
        "Expected resource_id in routing info"
    );

    // Verify MemoryType::Resource was used
    assert!(
        response.routing_info.target_memory_types.contains(&MemoryType::Resource),
        "Expected MemoryType::Resource in target_memory_types"
    );
}

/// Test 7: ActiveRetrievalSystem with category context
#[tokio::test]
async fn test_active_retrieval_with_category_context() {
    let config = ActiveRetrievalConfig::default();
    let system = ActiveRetrievalSystem::new(config)
        .await
        .expect("Failed to create ActiveRetrievalSystem");

    let request = create_category_aware_request();
    let response = system.retrieve(request).await.expect("Retrieval failed");

    // Verify routing decision has category context
    assert!(
        response.routing_info.route_by_resource_or_category,
        "Expected file-centric routing in response"
    );
    assert_eq!(
        response.routing_info.target_category_path,
        Some("/preferences/communication".to_string()),
        "Expected category_path in routing info"
    );
}

/// Test 8: Serialization of file-centric fields in RetrievalRequest
#[test]
fn test_retrieval_request_serialization_with_file_centric_fields() {
    let request = create_resource_category_request();

    // Serialize to JSON
    let json = serde_json::to_string(&request).expect("Failed to serialize");

    // Verify fields are present
    assert!(
        json.contains("resource_id"),
        "Expected resource_id in JSON"
    );
    assert!(
        json.contains("category_path"),
        "Expected category_path in JSON"
    );

    // Deserialize back
    let deserialized: RetrievalRequest =
        serde_json::from_str(&json).expect("Failed to deserialize");

    // Verify fields preserved
    assert_eq!(
        deserialized.resource_id,
        Some("resource-456".to_string())
    );
    assert_eq!(
        deserialized.category_path,
        Some("/skills/programming".to_string())
    );
}

/// Test 9: Empty resource_id and category_path (skip_serializing_if)
#[test]
fn test_retrieval_request_skips_none_fields() {
    let request = create_legacy_request();

    // Serialize to JSON
    let json = serde_json::to_string(&request).expect("Failed to serialize");

    // Verify None fields are skipped (not present in JSON)
    // Note: serde's skip_serializing_if should remove these
    let parsed: serde_json::Value =
        serde_json::from_str(&json).expect("Failed to parse JSON");

    // When fields are None and skip_serializing_if is used, they should be absent
    // or explicitly null - let's verify the value
    if let Some(obj) = parsed.as_object() {
        // If resource_id is present, it must be null or the value
        if let Some(resource_val) = obj.get("resource_id") {
            assert!(
                resource_val.is_null(),
                "Expected resource_id to be null or absent, got: {:?}",
                resource_val
            );
        }
        if let Some(category_val) = obj.get("category_path") {
            assert!(
                category_val.is_null(),
                "Expected category_path to be null or absent, got: {:?}",
                category_val
            );
        }
    }
}
