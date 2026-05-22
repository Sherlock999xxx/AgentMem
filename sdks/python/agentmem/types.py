"""
AgentMem Python SDK - Type Definitions

Core data types and enums for the AgentMem Python SDK.
"""

from enum import Enum
from typing import Dict, List, Optional, Any, Union
from dataclasses import dataclass
from datetime import datetime


class MemoryType(str, Enum):
    """记忆类型枚举 (4+4 架构)

    基础认知记忆 (与 Rust 兼容):
    - EPISODIC: 情景记忆 - 具体事件和经验
    - SEMANTIC: 语义记忆 - 事实、概念和通用知识
    - PROCEDURAL: 程序记忆 - 技能、流程和操作步骤
    - WORKING: 工作记忆 - 临时信息和活跃上下文

    高级认知记忆 (AgentMem 扩展):
    - CORE: 核心记忆 - 持久身份、偏好和核心信念
    - RESOURCE: 资源记忆 - 多媒体内容和外部资源
    - KNOWLEDGE: 知识记忆 - 结构化知识图谱
    - CONTEXTUAL: 上下文记忆 - 环境感知和情境信息
    """
    # 基础认知记忆 (原有 4 种)
    EPISODIC = "episodic"
    SEMANTIC = "semantic"
    PROCEDURAL = "procedural"
    WORKING = "working"

    # 高级认知记忆 (新增 4 种)
    CORE = "core"
    RESOURCE = "resource"
    KNOWLEDGE = "knowledge"
    CONTEXTUAL = "contextual"

    # 别名 (兼容旧格式)
    UNTYPED = "working"  # UNTYPED 映射到 WORKING

    @classmethod
    def from_string(cls, s: str) -> "MemoryType":
        """从字符串解析，支持各种格式"""
        s = s.lower().strip()
        mapping = {
            "episodic": cls.EPISODIC,
            "semantic": cls.SEMANTIC,
            "procedural": cls.PROCEDURAL,
            "working": cls.WORKING,
            "untyped": cls.WORKING,
            "core": cls.CORE,
            "resource": cls.RESOURCE,
            "knowledge": cls.KNOWLEDGE,
            "contextual": cls.CONTEXTUAL,
        }
        return mapping.get(s, cls.EPISODIC)

    @classmethod
    def all_types(cls) -> List["MemoryType"]:
        """获取所有类型"""
        return [
            cls.EPISODIC, cls.SEMANTIC, cls.PROCEDURAL, cls.WORKING,
            cls.CORE, cls.RESOURCE, cls.KNOWLEDGE, cls.CONTEXTUAL
        ]

    @property
    def description(self) -> str:
        """获取类型描述"""
        descriptions = {
            self.EPISODIC: "Specific events and experiences",
            self.SEMANTIC: "Facts, concepts, and general knowledge",
            self.PROCEDURAL: "Skills, procedures, and how-to knowledge",
            self.WORKING: "Temporary information processing",
            self.CORE: "Persistent identity and preferences",
            self.RESOURCE: "Multimedia content and documents",
            self.KNOWLEDGE: "Structured knowledge graphs",
            self.CONTEXTUAL: "Environment-aware information",
        }
        return descriptions.get(self, "")


class ImportanceLevel(Enum):
    """Memory importance level."""
    LOW = 1
    MEDIUM = 2
    HIGH = 3
    CRITICAL = 4


class MatchType(Enum):
    """Search match type."""
    EXACT_TEXT = "exact_text"
    PARTIAL_TEXT = "partial_text"
    SEMANTIC = "semantic"
    METADATA = "metadata"


@dataclass
class Memory:
    """Memory data structure."""
    id: str
    content: str
    memory_type: MemoryType
    agent_id: str
    user_id: Optional[str] = None
    session_id: Optional[str] = None
    importance: float = 0.5
    metadata: Optional[Dict[str, Any]] = None
    created_at: Optional[datetime] = None
    updated_at: Optional[datetime] = None
    access_count: int = 0
    last_accessed: Optional[datetime] = None
    embedding: Optional[List[float]] = None
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert memory to dictionary."""
        return {
            "id": self.id,
            "content": self.content,
            "memory_type": self.memory_type.value,
            "agent_id": self.agent_id,
            "user_id": self.user_id,
            "session_id": self.session_id,
            "importance": self.importance,
            "metadata": self.metadata or {},
            "created_at": self.created_at.isoformat() if self.created_at else None,
            "updated_at": self.updated_at.isoformat() if self.updated_at else None,
            "access_count": self.access_count,
            "last_accessed": self.last_accessed.isoformat() if self.last_accessed else None,
            "embedding": self.embedding,
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Memory":
        """Create memory from dictionary."""
        return cls(
            id=data["id"],
            content=data["content"],
            memory_type=MemoryType(data["memory_type"]),
            agent_id=data["agent_id"],
            user_id=data.get("user_id"),
            session_id=data.get("session_id"),
            importance=data.get("importance", 0.5),
            metadata=data.get("metadata"),
            created_at=datetime.fromisoformat(data["created_at"]) if data.get("created_at") else None,
            updated_at=datetime.fromisoformat(data["updated_at"]) if data.get("updated_at") else None,
            access_count=data.get("access_count", 0),
            last_accessed=datetime.fromisoformat(data["last_accessed"]) if data.get("last_accessed") else None,
            embedding=data.get("embedding"),
        )


@dataclass
class SearchQuery:
    """Search query parameters."""
    agent_id: str
    text_query: Optional[str] = None
    vector_query: Optional[List[float]] = None
    memory_type: Optional[MemoryType] = None
    user_id: Optional[str] = None
    min_importance: Optional[float] = None
    max_age_seconds: Optional[int] = None
    limit: int = 10
    metadata_filters: Optional[Dict[str, Any]] = None
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert query to dictionary."""
        query = {
            "agent_id": self.agent_id,
            "limit": self.limit,
        }
        
        if self.text_query:
            query["text_query"] = self.text_query
        if self.vector_query:
            query["vector_query"] = self.vector_query
        if self.memory_type:
            query["memory_type"] = self.memory_type.value
        if self.user_id:
            query["user_id"] = self.user_id
        if self.min_importance is not None:
            query["min_importance"] = self.min_importance
        if self.max_age_seconds is not None:
            query["max_age_seconds"] = self.max_age_seconds
        if self.metadata_filters:
            query["metadata_filters"] = self.metadata_filters
            
        return query


@dataclass
class SearchResult:
    """Search result with score and match type."""
    memory: Memory
    score: float
    match_type: MatchType
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "SearchResult":
        """Create search result from dictionary."""
        return cls(
            memory=Memory.from_dict(data["memory"]),
            score=data["score"],
            match_type=MatchType(data["match_type"]),
        )


@dataclass
class MemoryStats:
    """Memory statistics."""
    total_memories: int
    memories_by_type: Dict[str, int]
    memories_by_agent: Dict[str, int]
    average_importance: float
    oldest_memory_age_days: float
    most_accessed_memory_id: Optional[str]
    total_access_count: int
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "MemoryStats":
        """Create stats from dictionary."""
        return cls(
            total_memories=data["total_memories"],
            memories_by_type=data["memories_by_type"],
            memories_by_agent=data["memories_by_agent"],
            average_importance=data["average_importance"],
            oldest_memory_age_days=data["oldest_memory_age_days"],
            most_accessed_memory_id=data.get("most_accessed_memory_id"),
            total_access_count=data["total_access_count"],
        )


# Exception classes
class AgentMemError(Exception):
    """Base exception for AgentMem SDK."""
    pass


class AuthenticationError(AgentMemError):
    """Authentication failed."""
    pass


class ValidationError(AgentMemError):
    """Request validation failed."""
    pass


class NetworkError(AgentMemError):
    """Network communication error."""
    pass


class NotFoundError(AgentMemError):
    """Resource not found."""
    pass


class RateLimitError(AgentMemError):
    """Rate limit exceeded."""
    pass


class ServerError(AgentMemError):
    """Server internal error."""
    pass


# ============================================================================
# File-Centric Types (Phase D1)
# ============================================================================

class ResourceStatus(Enum):
    """Lifecycle state for mounted resources."""
    PENDING = "pending"
    MOUNTED = "mounted"
    FAILED = "failed"
    UNMOUNTED = "unmounted"


class CategoryStatus(Enum):
    """Lifecycle state for categories."""
    ACTIVE = "active"
    ARCHIVED = "archived"
    DELETED = "deleted"


class OperationStatus(Enum):
    """Cross-language status model for async and long-running operations."""
    PENDING = "pending"
    RUNNING = "running"
    SUCCEEDED = "succeeded"
    FAILED = "failed"
    CANCELLED = "cancelled"


class PlatformErrorCode(Enum):
    """File-centric error code baseline for SDK alignment."""
    VALIDATION_ERROR = "validation_error"
    CATEGORY_NOT_FOUND = "category_not_found"
    RESOURCE_URI_CONFLICT = "resource_uri_conflict"
    MIGRATION_CONFLICT = "migration_conflict"
    TASK_TIMEOUT = "task_timeout"
    BACKGROUND_TASK_UNAVAILABLE = "background_task_unavailable"


@dataclass
class ScopeDescriptor:
    """Multi-tenant ownership scope."""
    user_id: str
    agent_id: str

    def to_dict(self) -> Dict[str, Any]:
        return {"user_id": self.user_id, "agent_id": self.agent_id}

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "ScopeDescriptor":
        return cls(user_id=data["user_id"], agent_id=data["agent_id"])


@dataclass
class ResourceMetadataDescriptor:
    """Open metadata surface for resources."""
    author: Optional[str] = None
    tags: Optional[List[str]] = None
    size_bytes: Optional[int] = None
    modified_at: Optional[datetime] = None
    attributes: Optional[Dict[str, str]] = None

    def to_dict(self) -> Dict[str, Any]:
        result: Dict[str, Any] = {}
        if self.author is not None:
            result["author"] = self.author
        if self.tags is not None:
            result["tags"] = self.tags
        if self.size_bytes is not None:
            result["size_bytes"] = self.size_bytes
        if self.modified_at is not None:
            result["modified_at"] = self.modified_at.isoformat()
        if self.attributes is not None:
            result["attributes"] = self.attributes
        return result

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "ResourceMetadataDescriptor":
        return cls(
            author=data.get("author"),
            tags=data.get("tags"),
            size_bytes=data.get("size_bytes"),
            modified_at=datetime.fromisoformat(data["modified_at"]) if data.get("modified_at") else None,
            attributes=data.get("attributes"),
        )


@dataclass
class CategoryMetadataDescriptor:
    """Open metadata surface for categories."""
    tags: Optional[List[str]] = None
    attributes: Optional[Dict[str, str]] = None

    def to_dict(self) -> Dict[str, Any]:
        result: Dict[str, Any] = {}
        if self.tags is not None:
            result["tags"] = self.tags
        if self.attributes is not None:
            result["attributes"] = self.attributes
        return result

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "CategoryMetadataDescriptor":
        return cls(tags=data.get("tags"), attributes=data.get("attributes"))


@dataclass
class ResourceDescriptor:
    """Stable resource DTO for the file-centric public contract."""
    id: str
    uri: str
    media_type: str
    status: ResourceStatus
    scope: ScopeDescriptor
    metadata: ResourceMetadataDescriptor
    created_at: datetime
    updated_at: datetime

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "uri": self.uri,
            "media_type": self.media_type,
            "status": self.status.value,
            "scope": self.scope.to_dict(),
            "metadata": self.metadata.to_dict(),
            "created_at": self.created_at.isoformat(),
            "updated_at": self.updated_at.isoformat(),
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "ResourceDescriptor":
        return cls(
            id=data["id"],
            uri=data["uri"],
            media_type=data["media_type"],
            status=ResourceStatus(data["status"]),
            scope=ScopeDescriptor.from_dict(data["scope"]),
            metadata=ResourceMetadataDescriptor.from_dict(data.get("metadata", {})),
            created_at=datetime.fromisoformat(data["created_at"]),
            updated_at=datetime.fromisoformat(data["updated_at"]),
        )


@dataclass
class CategoryDescriptor:
    """Stable category DTO for the file-centric public contract."""
    id: str
    path: str
    name: str
    parent_id: Optional[str]
    children_ids: List[str]
    summary: Optional[str]
    item_count: int
    status: CategoryStatus
    scope: ScopeDescriptor
    metadata: CategoryMetadataDescriptor
    created_at: datetime
    updated_at: datetime

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "path": self.path,
            "name": self.name,
            "parent_id": self.parent_id,
            "children_ids": self.children_ids,
            "summary": self.summary,
            "item_count": self.item_count,
            "status": self.status.value,
            "scope": self.scope.to_dict(),
            "metadata": self.metadata.to_dict(),
            "created_at": self.created_at.isoformat(),
            "updated_at": self.updated_at.isoformat(),
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "CategoryDescriptor":
        return cls(
            id=data["id"],
            path=data["path"],
            name=data["name"],
            parent_id=data.get("parent_id"),
            children_ids=data.get("children_ids", []),
            summary=data.get("summary"),
            item_count=data.get("item_count", 0),
            status=CategoryStatus(data["status"]),
            scope=ScopeDescriptor.from_dict(data["scope"]),
            metadata=CategoryMetadataDescriptor.from_dict(data.get("metadata", {})),
            created_at=datetime.fromisoformat(data["created_at"]),
            updated_at=datetime.fromisoformat(data["updated_at"]),
        )


@dataclass
class ExtractedEntity:
    """Entity extracted from a resource."""
    id: str
    name: str
    entity_type: str
    confidence: float
    attributes: Optional[Dict[str, str]] = None
    span_start: Optional[int] = None
    span_end: Optional[int] = None

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "ExtractedEntity":
        return cls(
            id=data["id"],
            name=data["name"],
            entity_type=data["entity_type"],
            confidence=data["confidence"],
            attributes=data.get("attributes"),
            span_start=data.get("span_start"),
            span_end=data.get("span_end"),
        )


@dataclass
class ExtractedRelation:
    """Relation extracted from a resource."""
    id: str
    subject_id: str
    subject: str
    predicate: str
    object_id: str
    object: str
    relation_type: str
    confidence: float
    attributes: Optional[Dict[str, str]] = None

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "ExtractedRelation":
        return cls(
            id=data["id"],
            subject_id=data["subject_id"],
            subject=data["subject"],
            predicate=data["predicate"],
            object_id=data["object_id"],
            object=data["object"],
            relation_type=data["relation_type"],
            confidence=data["confidence"],
            attributes=data.get("attributes"),
        )


@dataclass
class ExtractionRequest:
    """Request to extract structured data from a mounted resource."""
    resource_id: str
    scope: ScopeDescriptor
    category_hint_paths: Optional[List[str]] = None
    persist_output: bool = True
    include_entities: bool = True
    include_relations: bool = True

    def to_dict(self) -> Dict[str, Any]:
        result: Dict[str, Any] = {
            "resource_id": self.resource_id,
            "scope": self.scope.to_dict(),
            "persist_output": self.persist_output,
            "include_entities": self.include_entities,
            "include_relations": self.include_relations,
        }
        if self.category_hint_paths is not None:
            result["category_hint_paths"] = self.category_hint_paths
        return result


@dataclass
class ExtractionResult:
    """Result of an extraction job."""
    job_id: str
    resource_id: str
    status: OperationStatus
    category_paths: List[str]
    memory_ids: List[str]
    entities: List[ExtractedEntity]
    relations: List[ExtractedRelation]
    warnings: List[str]
    error_code: Optional[PlatformErrorCode]
    error_message: Optional[str]
    duration_ms: int
    started_at: datetime
    completed_at: Optional[datetime]

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "ExtractionResult":
        return cls(
            job_id=data["job_id"],
            resource_id=data["resource_id"],
            status=OperationStatus(data["status"]),
            category_paths=data.get("category_paths", []),
            memory_ids=data.get("memory_ids", []),
            entities=[ExtractedEntity.from_dict(e) for e in data.get("entities", [])],
            relations=[ExtractedRelation.from_dict(r) for r in data.get("relations", [])],
            warnings=data.get("warnings", []),
            error_code=PlatformErrorCode(data["error_code"]) if data.get("error_code") else None,
            error_message=data.get("error_message"),
            duration_ms=data.get("duration_ms", 0),
            started_at=datetime.fromisoformat(data["started_at"]),
            completed_at=datetime.fromisoformat(data["completed_at"]) if data.get("completed_at") else None,
        )


@dataclass
class MigrationPlan:
    """Plan for migrating legacy memories to file-centric surface."""
    plan_id: str
    scope: ScopeDescriptor
    dry_run: bool
    source_surface: str
    target_surface: str
    legacy_memory_count: int
    projected_resource_count: int
    projected_category_count: int
    warnings: List[str]
    created_at: datetime

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "MigrationPlan":
        return cls(
            plan_id=data["plan_id"],
            scope=ScopeDescriptor.from_dict(data["scope"]),
            dry_run=data["dry_run"],
            source_surface=data["source_surface"],
            target_surface=data["target_surface"],
            legacy_memory_count=data["legacy_memory_count"],
            projected_resource_count=data["projected_resource_count"],
            projected_category_count=data["projected_category_count"],
            warnings=data.get("warnings", []),
            created_at=datetime.fromisoformat(data["created_at"]),
        )


@dataclass
class MigrationReport:
    """Report of a migration execution."""
    migration_id: str
    plan_id: str
    dry_run: bool
    status: OperationStatus
    migrated_memories: int
    mounted_resources: int
    created_categories: int
    conflicts: List[str]
    warnings: List[str]
    errors: List[str]
    error_code: Optional[PlatformErrorCode]
    rollback_available: bool
    started_at: datetime
    completed_at: Optional[datetime]

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "MigrationReport":
        return cls(
            migration_id=data["migration_id"],
            plan_id=data["plan_id"],
            dry_run=data["dry_run"],
            status=OperationStatus(data["status"]),
            migrated_memories=data["migrated_memories"],
            mounted_resources=data["mounted_resources"],
            created_categories=data["created_categories"],
            conflicts=data.get("conflicts", []),
            warnings=data.get("warnings", []),
            errors=data.get("errors", []),
            error_code=PlatformErrorCode(data["error_code"]) if data.get("error_code") else None,
            rollback_available=data.get("rollback_available", False),
            started_at=datetime.fromisoformat(data["started_at"]),
            completed_at=datetime.fromisoformat(data["completed_at"]) if data.get("completed_at") else None,
        )


@dataclass
class ProactiveTaskInfo:
    """Information about a proactive background task."""
    id: str
    task_type: str
    status: OperationStatus
    scope: ScopeDescriptor
    schedule: str
    pending_runs: int
    running_count: int
    last_started_at: Optional[datetime]
    last_completed_at: Optional[datetime]
    last_error_code: Optional[PlatformErrorCode]
    last_error: Optional[str]

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "ProactiveTaskInfo":
        return cls(
            id=data["id"],
            task_type=data["task_type"],
            status=OperationStatus(data["status"]),
            scope=ScopeDescriptor.from_dict(data["scope"]),
            schedule=data["schedule"],
            pending_runs=data.get("pending_runs", 0),
            running_count=data.get("running_count", 0),
            last_started_at=datetime.fromisoformat(data["last_started_at"]) if data.get("last_started_at") else None,
            last_completed_at=datetime.fromisoformat(data["last_completed_at"]) if data.get("last_completed_at") else None,
            last_error_code=PlatformErrorCode(data["last_error_code"]) if data.get("last_error_code") else None,
            last_error=data.get("last_error"),
        )


@dataclass
class SchedulerStats:
    """Statistics about the proactive task scheduler."""
    state: str
    total_tasks: int
    running_tasks: int
    completed_tasks: int
    failed_tasks: int
    cancelled_tasks: int
    total_execution_time_ms: int
    last_error: Optional[str]
    updated_at: datetime

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "SchedulerStats":
        return cls(
            state=data["state"],
            total_tasks=data["total_tasks"],
            running_tasks=data["running_tasks"],
            completed_tasks=data["completed_tasks"],
            failed_tasks=data["failed_tasks"],
            cancelled_tasks=data["cancelled_tasks"],
            total_execution_time_ms=data["total_execution_time_ms"],
            last_error=data.get("last_error"),
            updated_at=datetime.fromisoformat(data["updated_at"]),
        )


@dataclass
class ErrorResponse:
    """Standard error response structure."""
    code: PlatformErrorCode
    message: str
    details: Dict[str, Any]
    timestamp: datetime

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "ErrorResponse":
        return cls(
            code=PlatformErrorCode(data["code"]),
            message=data["message"],
            details=data.get("details", {}),
            timestamp=datetime.fromisoformat(data["timestamp"]),
        )


# File-centric exception classes

class CategoryNotFoundError(AgentMemError):
    """Category not found."""
    def __init__(self, path: str, response: Optional[ErrorResponse] = None):
        self.path = path
        self.response = response
        super().__init__(f"Category not found: {path}")


class ResourceUriConflictError(AgentMemError):
    """Resource URI conflict."""
    def __init__(self, uri: str, response: Optional[ErrorResponse] = None):
        self.uri = uri
        self.response = response
        super().__init__(f"Resource URI conflict: {uri}")


class MigrationConflictError(AgentMemError):
    """Migration conflict."""
    def __init__(self, message: str, response: Optional[ErrorResponse] = None):
        self.response = response
        super().__init__(message)


class TaskTimeoutError(AgentMemError):
    """Task timeout."""
    def __init__(self, task_id: str, response: Optional[ErrorResponse] = None):
        self.task_id = task_id
        self.response = response
        super().__init__(f"Task timeout: {task_id}")


class BackgroundTaskUnavailableError(AgentMemError):
    """Background task unavailable."""
    def __init__(self, response: Optional[ErrorResponse] = None):
        self.response = response
        super().__init__("Background task unavailable")


# ============================================================================
# Webhook Types 🆕 Gap vs Mem0/Letta
# ============================================================================

@dataclass
class WebhookSubscription:
    """Webhook subscription details."""
    id: str
    user_id: str
    name: str
    url: str
    event_types: List[str]
    is_active: bool
    created_at: int
    updated_at: int

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "WebhookSubscription":
        return cls(
            id=data["id"],
            user_id=data["user_id"],
            name=data["name"],
            url=data["url"],
            event_types=data.get("event_types", []),
            is_active=data.get("is_active", True),
            created_at=data.get("created_at", 0),
            updated_at=data.get("updated_at", 0),
        )


@dataclass
class WebhookStats:
    """Webhook delivery statistics."""
    total: int
    active: int
    total_deliveries: int
    successful_deliveries: int
    failed_deliveries: int
    success_rate: float

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "WebhookStats":
        return cls(
            total=data.get("total", 0),
            active=data.get("active", 0),
            total_deliveries=data.get("total_deliveries", 0),
            successful_deliveries=data.get("successful_deliveries", 0),
            failed_deliveries=data.get("failed_deliveries", 0),
            success_rate=data.get("success_rate", 0.0),
        )
