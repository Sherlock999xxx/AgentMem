# Scratchpad - AgentMem file-centric Penetration Plan 1.1.1

## Current Session: Phase B - Agent Collaboration Chain Refactoring

### Context
- Plan: plan1.1.1.md - convert existing resource/category/extraction/proactive capabilities into default platform experience
- Phase A (public models) and Phase C (dual-surface entrypoints) are complete per memory mem-1773883158-1fd4
- Current task: task-1773891236-2473 - Phase B: Agent collaboration chain refactoring

### Phase B Goals (from plan1.1.1)
1. ResourceAgent upgrade - from parallel agent to resource mount and preprocessing entrypoint
2. SemanticAgent/ProceduralAgent - consume extraction output and category context
3. KnowledgeAgent/ContextualAgent - category-aware retrieval
4. Retrieval router - from MemoryType mapping to resource/category-aware scheduling

### Verification Standards
- At least one resource ingestion path defaults to `mount -> extract -> categorize -> store`
- Search entry can explicitly consume category/resource context
- MemoryType is no longer the only agent routing key

## Analysis Notes

## Architecture Analysis (2026-03-19)

### Current State: MemoryType-first Routing

1. **Agent Registry** (`agent_registry.rs:33`): Maps `MemoryType → AgentType`
   - Uses `HashMap<MemoryType, AgentType>` for routing
   - `execute_task()` takes `memory_type: &MemoryType` as primary routing key

2. **Retrieval Router** (`router.rs`):
   - `RouteDecision` includes `target_memory_types: Vec<MemoryType>`
   - `determine_target_memory_types()` infers MemoryTypes from topics
   - Already has `memory_type_strategy_mapping` configuration

3. **TaskRequest** (`meta_manager.rs:116-140`):
   - ✅ **Already has file-centric fields**: `resource_id: Option<String>`, `category_path: Option<String>`
   - Still requires `memory_type: MemoryType` as primary field

4. **AgentOrchestrator** (`orchestrator/mod.rs`):
   - Uses `MemoryIntegrator.retrieve_episodic_first()` for retrieval
   - Not directly using resource/category routing yet

### Phase B Goals vs Current Gap

| Goal | Current Status | Gap |
|------|----------------|-----|
| ResourceAgent as entrypoint | Exists but operates as peer agent | Needs to be entrypoint for resource ingestion |
| SemanticAgent/ProceduralAgent consume extraction | No extraction output consumption | Need to wire extraction pipeline |
| KnowledgeAgent/ContextualAgent category-aware | No category context | Need category-path routing |
| Retrieval router MemoryType → resource/category | MemoryType-only | Add resource/category routing paths |

### Minimal Changes Required

**File 1: `agent_registry.rs`**
- Add `RouteBy` enum: `MemoryType(MemoryType) | Resource(String) | Category(String)`
- Add `execute_task_by_route()` method for file-centric routing
- Keep `execute_task()` for backward compatibility

**File 2: `router.rs`**
- Add `route_by_resource_category` flag to `RouteDecision`
- Extend `determine_target_memory_types()` to consider `resource_id`/`category_path`

**File 3: `resource_agent.rs`**
- Add `mount_resource`, `extract`, `preprocess` operations
- Wire to extraction pipeline

**File 4: Orchestrator integration**
- Add resource-first ingestion path option

### Verification Strategy

1. Unit test: `TaskRequest.with_resource_id()` creates correct routing
2. Unit test: `TaskRequest.with_category_path()` creates correct routing
3. Integration test: Resource mount → extract → categorize → store flow
4. Ensure backward compatibility: legacy MemoryType routing still works

### Phase B Atomic Task Breakdown

The umbrella task `task-1773891236-2473` should be split into these atomic tasks:

**Task B.1: Add RouteBy enum and dual routing to AgentRegistry** (~150 LOC)
- File: `crates/agent-mem-core/src/retrieval/agent_registry.rs`
- Add `RouteBy` enum with `MemoryType`, `Resource`, `Category` variants
- Add `execute_task_by_route()` method
- Keep `execute_task()` for backward compatibility
- Verification: Unit test for RouteBy::Resource and RouteBy::Category

**Task B.2: Extend RouteDecision with file-centric routing** (~100 LOC)
- File: `crates/agent-mem-core/src/retrieval/router.rs`
- Add `route_by_resource_or_category` flag to `RouteDecision`
- Add resource/category consideration in routing
- Verification: Unit test for routing with resource_id/category_path

**Task B.3: Extend ResourceAgent with mount/extract operations** (~200 LOC)
- File: `crates/agent-mem-core/src/agents/resource_agent.rs`
- Add `mount_resource` operation
- Add `preprocess` operation
- Add `extract` operation (wire to extraction pipeline)
- Verification: Unit test for mount → preprocess → extract flow

**Task B.4: Integration test for resource-first ingestion path** (~100 LOC)
- File: `crates/agent-mem-core/src/orchestrator/tests/`
- Test: mount resource → extract → categorize → store
- Verify category/resource context in retrieval
- Verification: `cargo test` passes

### Recommended Execution Order
1. B.1 (AgentRegistry) - foundation for routing
2. B.3 (ResourceAgent) - enable resource ingestion
3. B.2 (Router) - connect routing to resource/category
4. B.4 (Integration test) - verify end-to-end

### Decision
- Close umbrella task `task-1773891236-2473`
- Create atomic tasks B.1, B.2, B.3, B.4
- Execute in order B.1 → B.3 → B.2 → B.4
