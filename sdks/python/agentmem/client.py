"""
AgentMem Python SDK - Main Client

Official Python client for AgentMem API.
"""

import asyncio
import json
import logging
import time
from typing import Dict, List, Optional, Any, Union
from urllib.parse import urljoin

import httpx

from .config import Config
from .types import (
    Memory,
    MemoryType,
    SearchQuery,
    SearchResult,
    MemoryStats,
    AgentMemError,
    AuthenticationError,
    ValidationError,
    NetworkError,
    NotFoundError,
    RateLimitError,
    ServerError,
    # File-centric types
    ResourceDescriptor,
    CategoryDescriptor,
    ExtractionRequest,
    ExtractionResult,
    MigrationPlan,
    MigrationReport,
    ProactiveTaskInfo,
    SchedulerStats,
    ErrorResponse,
    ResourceStatus,
    CategoryStatus,
    OperationStatus,
    PlatformErrorCode,
    ScopeDescriptor,
    CategoryNotFoundError,
    ResourceUriConflictError,
    MigrationConflictError,
    TaskTimeoutError,
    BackgroundTaskUnavailableError,
)


class AgentMemClient:
    """
    AgentMem Python client for interacting with AgentMem API.
    
    Example:
        ```python
        import asyncio
        from agentmem import AgentMemClient, Config
        
        async def main():
            config = Config.from_env()
            client = AgentMemClient(config)
            
            # Add a memory
            memory_id = await client.add_memory(
                content="Important project information",
                agent_id="agent_1",
                memory_type=MemoryType.SEMANTIC,
                importance=0.8
            )
            
            # Search memories
            results = await client.search_memories(
                SearchQuery(
                    agent_id="agent_1",
                    text_query="project information",
                    limit=5
                )
            )
            
            await client.close()
        
        asyncio.run(main())
        ```
    """
    
    def __init__(self, config: Config):
        """Initialize AgentMem client."""
        self.config = config
        self.config.validate()
        
        # Setup logging
        if config.enable_logging:
            logging.basicConfig(level=getattr(logging, config.log_level))
            self.logger = logging.getLogger(__name__)
        else:
            self.logger = logging.getLogger(__name__)
            self.logger.addHandler(logging.NullHandler())
        
        # HTTP client setup
        self._client: Optional[httpx.AsyncClient] = None
        self._session_created = False
        
        # Cache setup
        self._cache: Dict[str, Any] = {}
        self._cache_timestamps: Dict[str, float] = {}
    
    async def _get_client(self) -> httpx.AsyncClient:
        """Get or create HTTP client."""
        if self._client is None:
            limits = httpx.Limits(
                max_connections=self.config.max_connections,
                max_keepalive_connections=self.config.max_keepalive_connections,
                keepalive_expiry=self.config.keepalive_expiry,
            )
            
            timeout = httpx.Timeout(self.config.timeout)
            
            headers = {
                "Authorization": f"Bearer {self.config.api_key}",
                "Content-Type": "application/json",
                "User-Agent": f"agentmem-python/6.0.0",
            }
            
            if self.config.enable_compression:
                headers["Accept-Encoding"] = "gzip, deflate"
            
            self._client = httpx.AsyncClient(
                base_url=self.config.api_base_url,
                headers=headers,
                limits=limits,
                timeout=timeout,
            )
            self._session_created = True
        
        return self._client
    
    async def close(self) -> None:
        """Close the HTTP client."""
        if self._client is not None:
            await self._client.aclose()
            self._client = None
            self._session_created = False
    
    async def __aenter__(self):
        """Async context manager entry."""
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        """Async context manager exit."""
        await self.close()
    
    def _get_cache_key(self, method: str, url: str, params: Optional[Dict] = None) -> str:
        """Generate cache key."""
        key_parts = [method, url]
        if params:
            key_parts.append(json.dumps(params, sort_keys=True))
        return "|".join(key_parts)
    
    def _is_cache_valid(self, key: str) -> bool:
        """Check if cache entry is valid."""
        if key not in self._cache_timestamps:
            return False
        
        age = time.time() - self._cache_timestamps[key]
        return age < self.config.cache_ttl
    
    def _set_cache(self, key: str, value: Any) -> None:
        """Set cache entry."""
        if self.config.enable_caching:
            self._cache[key] = value
            self._cache_timestamps[key] = time.time()
    
    def _get_cache(self, key: str) -> Optional[Any]:
        """Get cache entry."""
        if not self.config.enable_caching:
            return None
        
        if self._is_cache_valid(key):
            return self._cache.get(key)
        
        # Clean up expired entry
        self._cache.pop(key, None)
        self._cache_timestamps.pop(key, None)
        return None
    
    async def _make_request(
        self,
        method: str,
        endpoint: str,
        data: Optional[Dict] = None,
        params: Optional[Dict] = None,
        use_cache: bool = False,
    ) -> Dict[str, Any]:
        """Make HTTP request with retry logic."""
        client = await self._get_client()
        url = endpoint
        
        # Check cache for GET requests
        if method == "GET" and use_cache:
            cache_key = self._get_cache_key(method, url, params)
            cached_result = self._get_cache(cache_key)
            if cached_result is not None:
                self.logger.debug(f"Cache hit for {method} {url}")
                return cached_result
        
        last_exception = None
        
        for attempt in range(self.config.max_retries + 1):
            try:
                self.logger.debug(f"Making request: {method} {url} (attempt {attempt + 1})")
                
                response = await client.request(
                    method=method,
                    url=url,
                    json=data,
                    params=params,
                )
                
                # Handle different status codes
                if response.status_code == 200:
                    result = response.json()
                    
                    # Cache successful GET requests
                    if method == "GET" and use_cache:
                        cache_key = self._get_cache_key(method, url, params)
                        self._set_cache(cache_key, result)
                    
                    return result
                
                elif response.status_code == 401:
                    raise AuthenticationError("Invalid API key or authentication failed")
                
                elif response.status_code == 400:
                    error_data = response.json() if response.content else {}
                    raise ValidationError(f"Request validation failed: {error_data.get('message', 'Unknown error')}")
                
                elif response.status_code == 404:
                    raise NotFoundError("Resource not found")
                
                elif response.status_code == 429:
                    raise RateLimitError("Rate limit exceeded")
                
                elif response.status_code >= 500:
                    error_data = response.json() if response.content else {}
                    raise ServerError(f"Server error: {error_data.get('message', 'Unknown error')}")
                
                else:
                    raise AgentMemError(f"Unexpected status code: {response.status_code}")
            
            except httpx.RequestError as e:
                last_exception = NetworkError(f"Network error: {str(e)}")
                
                if attempt < self.config.max_retries:
                    delay = self.config.retry_delay * (2 ** attempt)  # Exponential backoff
                    self.logger.warning(f"Request failed, retrying in {delay}s: {str(e)}")
                    await asyncio.sleep(delay)
                else:
                    break
            
            except (AuthenticationError, ValidationError, NotFoundError, RateLimitError) as e:
                # Don't retry these errors
                raise e
        
        # If we get here, all retries failed
        if last_exception:
            raise last_exception
        else:
            raise AgentMemError("Request failed after all retries")
    
    async def add_memory(
        self,
        content: str,
        agent_id: str,
        memory_type: MemoryType = MemoryType.UNTYPED,
        user_id: Optional[str] = None,
        session_id: Optional[str] = None,
        importance: float = 0.5,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> str:
        """
        Add a new memory.
        
        Args:
            content: Memory content
            agent_id: Agent identifier
            memory_type: Type of memory
            user_id: Optional user identifier
            session_id: Optional session identifier
            importance: Memory importance (0.0 to 1.0)
            metadata: Optional metadata dictionary
        
        Returns:
            Memory ID
        """
        data = {
            "content": content,
            "agent_id": agent_id,
            "memory_type": memory_type.value,
            "importance": importance,
        }
        
        if user_id:
            data["user_id"] = user_id
        if session_id:
            data["session_id"] = session_id
        if metadata:
            data["metadata"] = metadata
        
        # ✅ 修复：使用正确的API端点路径
        response = await self._make_request("POST", "/api/v1/memories", data=data)
        return response["id"]

    async def get_memory(self, memory_id: str) -> Memory:
        """
        Get a memory by ID.

        Args:
            memory_id: Memory identifier

        Returns:
            Memory object
        """
        response = await self._make_request("GET", f"/api/v1/memories/{memory_id}", use_cache=True)
        return Memory.from_dict(response)

    async def update_memory(
        self,
        memory_id: str,
        content: Optional[str] = None,
        importance: Optional[float] = None,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> Memory:
        """
        Update an existing memory.

        Args:
            memory_id: Memory identifier
            content: New content (optional)
            importance: New importance (optional)
            metadata: New metadata (optional)

        Returns:
            Updated memory object
        """
        data = {}
        if content is not None:
            data["content"] = content
        if importance is not None:
            data["importance"] = importance
        if metadata is not None:
            data["metadata"] = metadata

        response = await self._make_request("PUT", f"/api/v1/memories/{memory_id}", data=data)
        return Memory.from_dict(response)

    async def delete_memory(self, memory_id: str) -> bool:
        """
        Delete a memory.

        Args:
            memory_id: Memory identifier

        Returns:
            True if successful
        """
        await self._make_request("DELETE", f"/api/v1/memories/{memory_id}")
        return True

    async def search_memories(self, query: SearchQuery) -> List[SearchResult]:
        """
        Search memories.

        Args:
            query: Search query parameters

        Returns:
            List of search results
        """
        response = await self._make_request("POST", "/api/v1/memories/search", data=query.to_dict())
        return [SearchResult.from_dict(result) for result in response["results"]]

    async def batch_add_memories(self, memories: List[Dict[str, Any]]) -> List[str]:
        """
        Add multiple memories in batch.

        Args:
            memories: List of memory data dictionaries

        Returns:
            List of memory IDs
        """
        data = {"memories": memories}
        response = await self._make_request("POST", "/api/v1/memories/batch", data=data)
        return response["results"]  # ✅ 修复：使用正确的响应字段

    async def batch_delete_memories(self, memory_ids: List[str]) -> Dict[str, Any]:
        """
        Delete multiple memories in batch.

        Args:
            memory_ids: List of memory IDs to delete

        Returns:
            Batch operation result
        """
        response = await self._make_request("POST", "/api/v1/memories/batch/delete", data=memory_ids)
        return response

    async def get_memory_history(self, memory_id: str) -> Dict[str, Any]:
        """
        Get memory history.

        Args:
            memory_id: Memory identifier

        Returns:
            Memory history
        """
        response = await self._make_request("GET", f"/api/v1/memories/{memory_id}/history", use_cache=True)
        return response

    async def get_all_memories(
        self,
        agent_id: Optional[str] = None,
        user_id: Optional[str] = None,
        limit: int = 100,
    ) -> List[Memory]:
        """
        Get all memories (with optional filters).

        Args:
            agent_id: Optional agent filter
            user_id: Optional user filter
            limit: Maximum number of memories to return

        Returns:
            List of memory objects
        """
        params = {"limit": limit}
        if agent_id:
            params["agent_id"] = agent_id
        if user_id:
            params["user_id"] = user_id
        
        response = await self._make_request("GET", "/api/v1/memories", params=params, use_cache=True)
        return [Memory.from_dict(mem) for mem in response.get("memories", [])]

    async def get_memory_stats(self, agent_id: Optional[str] = None) -> MemoryStats:
        """
        Get memory statistics.

        Args:
            agent_id: Optional agent identifier

        Returns:
            Memory statistics
        """
        params = {}
        if agent_id:
            params["agent_id"] = agent_id
        response = await self._make_request("GET", "/metrics", params=params, use_cache=True)
        return MemoryStats.from_dict(response.get("metrics", {}))

    async def health_check(self) -> Dict[str, Any]:
        """
        Check API health status.

        Returns:
            Health status information
        """
        return await self._make_request("GET", "/health", use_cache=True)

    async def get_metrics(self) -> Dict[str, Any]:
        """
        Get system metrics.

        Returns:
            System metrics
        """
        return await self._make_request("GET", "/metrics", use_cache=True)

    # =========================================================================
    # File-Centric API Methods (Phase D1)
    # =========================================================================

    # --- Resource Operations ---

    async def mount_resource(
        self,
        uri: str,
        media_type: str,
        user_id: str,
        agent_id: str,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> ResourceDescriptor:
        """
        Mount a resource (file-like entity) into the memory system.

        Args:
            uri: Resource URI (e.g., file:///path/to/file.md)
            media_type: MIME type of the resource
            user_id: User identifier
            agent_id: Agent identifier
            metadata: Optional resource metadata

        Returns:
            Resource descriptor
        """
        data: Dict[str, Any] = {
            "uri": uri,
            "media_type": media_type,
            "scope": {"user_id": user_id, "agent_id": agent_id},
        }
        if metadata:
            data["metadata"] = metadata

        response = await self._make_request("POST", "/api/v1/file-centric/resources", data=data)
        return ResourceDescriptor.from_dict(response)

    async def get_resource(self, resource_id: str) -> ResourceDescriptor:
        """
        Get a resource by ID.

        Args:
            resource_id: Resource identifier

        Returns:
            Resource descriptor
        """
        response = await self._make_request(
            "GET", f"/api/v1/file-centric/resources/{resource_id}", use_cache=True
        )
        return ResourceDescriptor.from_dict(response)

    async def list_resources(
        self,
        user_id: str,
        agent_id: str,
        status: Optional[ResourceStatus] = None,
        limit: int = 100,
    ) -> List[ResourceDescriptor]:
        """
        List resources for a scope.

        Args:
            user_id: User identifier
            agent_id: Agent identifier
            status: Optional filter by status
            limit: Maximum number of resources to return

        Returns:
            List of resource descriptors
        """
        params: Dict[str, Any] = {
            "user_id": user_id,
            "agent_id": agent_id,
            "limit": limit,
        }
        if status:
            params["status"] = status.value

        response = await self._make_request(
            "GET", "/api/v1/file-centric/resources", params=params, use_cache=True
        )
        return [ResourceDescriptor.from_dict(r) for r in response.get("resources", [])]

    # --- Category Operations ---

    async def get_category(self, category_id: str) -> CategoryDescriptor:
        """
        Get a category by ID.

        Args:
            category_id: Category identifier

        Returns:
            Category descriptor
        """
        response = await self._make_request(
            "GET", f"/api/v1/file-centric/categories/{category_id}", use_cache=True
        )
        return CategoryDescriptor.from_dict(response)

    async def get_category_by_path(self, path: str, user_id: str, agent_id: str) -> CategoryDescriptor:
        """
        Get a category by path.

        Args:
            path: Category path (e.g., /preferences/communication)
            user_id: User identifier
            agent_id: Agent identifier

        Returns:
            Category descriptor
        """
        params = {"path": path, "user_id": user_id, "agent_id": agent_id}
        response = await self._make_request(
            "GET", "/api/v1/file-centric/categories/by-path", params=params, use_cache=True
        )
        return CategoryDescriptor.from_dict(response)

    async def list_categories(
        self,
        user_id: str,
        agent_id: str,
        parent_id: Optional[str] = None,
        limit: int = 100,
    ) -> List[CategoryDescriptor]:
        """
        List categories for a scope.

        Args:
            user_id: User identifier
            agent_id: Agent identifier
            parent_id: Optional filter by parent category
            limit: Maximum number of categories to return

        Returns:
            List of category descriptors
        """
        params: Dict[str, Any] = {
            "user_id": user_id,
            "agent_id": agent_id,
            "limit": limit,
        }
        if parent_id:
            params["parent_id"] = parent_id

        response = await self._make_request(
            "GET", "/api/v1/file-centric/categories", params=params, use_cache=True
        )
        return [CategoryDescriptor.from_dict(c) for c in response.get("categories", [])]

    async def search_categories(
        self,
        query: str,
        user_id: str,
        agent_id: str,
        limit: int = 10,
    ) -> List[CategoryDescriptor]:
        """
        Search categories by query.

        Args:
            query: Search query string
            user_id: User identifier
            agent_id: Agent identifier
            limit: Maximum number of results

        Returns:
            List of matching category descriptors
        """
        data = {
            "query": query,
            "scope": {"user_id": user_id, "agent_id": agent_id},
            "limit": limit,
        }
        response = await self._make_request("POST", "/api/v1/file-centric/categories/search", data=data)
        return [CategoryDescriptor.from_dict(c) for c in response.get("categories", [])]

    # --- Extraction Operations ---

    async def extract_resource(self, request: ExtractionRequest) -> ExtractionResult:
        """
        Extract structured data from a mounted resource.

        Args:
            request: Extraction request parameters

        Returns:
            Extraction result
        """
        response = await self._make_request(
            "POST", "/api/v1/file-centric/extraction", data=request.to_dict()
        )
        return ExtractionResult.from_dict(response)

    async def get_extraction_status(self, job_id: str) -> ExtractionResult:
        """
        Get the status of an extraction job.

        Args:
            job_id: Extraction job identifier

        Returns:
            Extraction result
        """
        response = await self._make_request(
            "GET", f"/api/v1/file-centric/extraction/{job_id}", use_cache=True
        )
        return ExtractionResult.from_dict(response)

    # --- Migration Operations ---

    async def plan_legacy_migration(
        self,
        user_id: str,
        agent_id: str,
        dry_run: bool = True,
    ) -> MigrationPlan:
        """
        Plan a migration from legacy memory to file-centric surface.

        Args:
            user_id: User identifier
            agent_id: Agent identifier
            dry_run: If True, only simulate the migration

        Returns:
            Migration plan
        """
        data = {
            "scope": {"user_id": user_id, "agent_id": agent_id},
            "dry_run": dry_run,
        }
        response = await self._make_request("POST", "/api/v1/file-centric/migration/plan", data=data)
        return MigrationPlan.from_dict(response)

    async def apply_legacy_migration(
        self,
        plan_id: str,
        dry_run: bool = False,
    ) -> MigrationReport:
        """
        Apply a migration plan.

        Args:
            plan_id: Migration plan identifier
            dry_run: If True, only simulate the migration

        Returns:
            Migration report
        """
        data = {"plan_id": plan_id, "dry_run": dry_run}
        response = await self._make_request("POST", "/api/v1/file-centric/migration/apply", data=data)
        return MigrationReport.from_dict(response)

    async def get_migration_status(self, migration_id: str) -> MigrationReport:
        """
        Get the status of a migration.

        Args:
            migration_id: Migration identifier

        Returns:
            Migration report
        """
        response = await self._make_request(
            "GET", f"/api/v1/file-centric/migration/{migration_id}", use_cache=True
        )
        return MigrationReport.from_dict(response)

    async def rollback_migration(self, migration_id: str) -> bool:
        """
        Roll back a migration.

        Args:
            migration_id: Migration identifier

        Returns:
            True if successful
        """
        await self._make_request("POST", f"/api/v1/file-centric/migration/{migration_id}/rollback")
        return True

    # --- Proactive Task Operations ---

    async def list_proactive_tasks(
        self,
        user_id: str,
        agent_id: str,
        task_type: Optional[str] = None,
    ) -> List[ProactiveTaskInfo]:
        """
        List proactive background tasks.

        Args:
            user_id: User identifier
            agent_id: Agent identifier
            task_type: Optional filter by task type

        Returns:
            List of proactive task info
        """
        params: Dict[str, Any] = {
            "user_id": user_id,
            "agent_id": agent_id,
        }
        if task_type:
            params["task_type"] = task_type

        response = await self._make_request(
            "GET", "/api/v1/file-centric/proactive/tasks", params=params, use_cache=True
        )
        return [ProactiveTaskInfo.from_dict(t) for t in response.get("tasks", [])]

    async def get_proactive_task(self, task_id: str) -> ProactiveTaskInfo:
        """
        Get a proactive task by ID.

        Args:
            task_id: Task identifier

        Returns:
            Proactive task info
        """
        response = await self._make_request(
            "GET", f"/api/v1/file-centric/proactive/tasks/{task_id}", use_cache=True
        )
        return ProactiveTaskInfo.from_dict(response)

    async def run_proactive_task(self, task_id: str) -> ProactiveTaskInfo:
        """
        Trigger a proactive task to run immediately.

        Args:
            task_id: Task identifier

        Returns:
            Updated proactive task info
        """
        response = await self._make_request("POST", f"/api/v1/file-centric/proactive/tasks/{task_id}/run")
        return ProactiveTaskInfo.from_dict(response)

    async def cancel_proactive_task(self, task_id: str) -> ProactiveTaskInfo:
        """
        Cancel a running proactive task.

        Args:
            task_id: Task identifier

        Returns:
            Updated proactive task info
        """
        response = await self._make_request("POST", f"/api/v1/file-centric/proactive/tasks/{task_id}/cancel")
        return ProactiveTaskInfo.from_dict(response)

    async def get_scheduler_stats(self) -> SchedulerStats:
        """
        Get scheduler statistics.

        Returns:
            Scheduler statistics
        """
        response = await self._make_request("GET", "/api/v1/file-centric/proactive/stats", use_cache=True)
        return SchedulerStats.from_dict(response)
