# File-Centric Contract Baseline

This document freezes the first cross-language DTO baseline for the file-centric AgentMem surface.

Scope of this freeze:

- `ResourceDescriptor`
- `CategoryDescriptor`
- `ExtractionRequest`
- `ExtractionResult`
- `MigrationPlan`
- `MigrationReport`
- `ProactiveTaskInfo`
- `SchedulerStats`

Rules:

- Field names are `snake_case`.
- Timestamps use RFC 3339 UTC strings.
- Long-running operations share the status set `pending | running | succeeded | failed | cancelled`.
- File-centric error codes are frozen as:
  - `validation_error`
  - `category_not_found`
  - `resource_uri_conflict`
  - `migration_conflict`
  - `task_timeout`
  - `background_task_unavailable`

This iteration intentionally keeps the contract DTOs independent from the internal `resource`, `category`, `extraction`, and `proactive` crate structs. The server and Rust client are the first adopters of the frozen wire shapes; top-level `agent-mem`, routes, and non-Rust SDKs will layer on top of this baseline in later stages.

Fixtures live in [docs/specs/file-centric-fixtures](/Users/louloulin/Documents/linchong/cjproject/contextengine/agentmen/docs/specs/file-centric-fixtures) and are treated as the canonical wire examples for serialization parity tests.
