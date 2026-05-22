"""
AgentMem Python SDK - Simplified MVP Client

Core 6 Methods:
- add_memory, get_memory, update_memory, delete_memory
- search_memories, get_all_memories

Batch Operations:
- batch_add_memories, batch_delete_memories

Stats & Health:
- get_health (merged health_check + get_metrics)

File-Centric Operations:
- mount_resource, list_resources
- list_categories, search_categories
- extract_resource
"""

import asyncio
import json
import logging
import time
from typing import Dict, List, Optional, Any
from urllib.parse import urljoin

import httpx

from .config import Config
from .types import (
    Memory,
    MemoryType,
    SearchQuery,
    SearchResult,
    ResourceDescriptor,
    CategoryDescriptor,
    ExtractionRequest,
    ExtractionResult,
    WebhookSubscription,
    WebhookStats,
)


class AgentMemClient:
    """
    AgentMem Python client - MVP simplified API.

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

        if config.enable_logging:
            logging.basicConfig(level=getattr(logging, config.log_level))
            self.logger = logging.getLogger(__name__)
        else:
            self.logger = logging.getLogger(__name__)
            self.logger.addHandler(logging.NullHandler())

        self._client: Optional[httpx.AsyncClient] = None
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
                "User-Agent": "agentmem-python/7.0.0",
            }
            self._client = httpx.AsyncClient(
                base_url=self.config.api_base_url,
                headers=headers,
                limits=limits,
                timeout=timeout,
            )
        return self._client

    async def close(self) -> None:
        """Close the HTTP client."""
        if self._client is not None:
            await self._client.aclose()
            self._client = None

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

        if method == "GET" and use_cache:
            cache_key = self._get_cache_key(method, url, params)
            cached_result = self._get_cache(cache_key)
            if cached_result is not None:
                return cached_result

        last_exception = None
        for attempt in range(self.config.max_retries + 1):
            try:
                response = await client.request(
                    method=method, url=url, json=data, params=params,
                )

                if response.status_code == 200:
                    result = response.json()
                    if method == "GET" and use_cache:
                        cache_key = self._get_cache_key(method, url, params)
                        self._set_cache(cache_key, result)
                    return result
                elif response.status_code == 401:
                    raise Exception("Authentication failed")
                elif response.status_code >= 500:
                    raise Exception(f"Server error: {response.status_code}")
                else:
                    raise Exception(f"Request failed: {response.status_code}")
            except httpx.RequestError as e:
                last_exception = e
                if attempt < self.config.max_retries:
                    delay = self.config.retry_delay * (2 ** attempt)
                    await asyncio.sleep(delay)
        raise last_exception or Exception("Request failed")

    # =========================================================================
    # Core Memory Operations (6 Methods)
    # =========================================================================

    async def add_memory(
        self,
        content: str,
        agent_id: str,
        memory_type: MemoryType = MemoryType.UNTYPED,
        user_id: Optional[str] = None,
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
        if metadata:
            data["metadata"] = metadata
        response = await self._make_request("POST", "/api/v1/memories", data=data)
        return response["id"]

    async def get_memory(self, memory_id: str) -> Memory:
        """Get a memory by ID."""
        response = await self._make_request("GET", f"/api/v1/memories/{memory_id}", use_cache=True)
        return Memory.from_dict(response)

    async def update_memory(
        self,
        memory_id: str,
        content: Optional[str] = None,
        importance: Optional[float] = None,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> Memory:
        """Update an existing memory."""
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
        """Delete a memory."""
        await self._make_request("DELETE", f"/api/v1/memories/{memory_id}")
        return True

    async def search_memories(self, query: SearchQuery) -> List[SearchResult]:
        """Search memories semantically."""
        response = await self._make_request("POST", "/api/v1/memories/search", data=query.to_dict())
        return [SearchResult.from_dict(result) for result in response.get("results", [])]

    async def get_all_memories(
        self,
        agent_id: Optional[str] = None,
        user_id: Optional[str] = None,
        limit: int = 100,
    ) -> List[Memory]:
        """Get all memories with optional filters."""
        params = {"limit": limit}
        if agent_id:
            params["agent_id"] = agent_id
        if user_id:
            params["user_id"] = user_id
        response = await self._make_request("GET", "/api/v1/memories", params=params, use_cache=True)
        return [Memory.from_dict(mem) for mem in response.get("memories", [])]

    # =========================================================================
    # Batch Operations (2 Methods)
    # =========================================================================

    async def batch_add_memories(self, memories: List[Dict[str, Any]]) -> List[str]:
        """Add multiple memories in batch."""
        response = await self._make_request("POST", "/api/v1/memories/batch", data={"memories": memories})
        return response.get("results", [])

    async def batch_delete_memories(self, memory_ids: List[str]) -> Dict[str, Any]:
        """Delete multiple memories in batch."""
        return await self._make_request("POST", "/api/v1/memories/batch/delete", data=memory_ids)

    # =========================================================================
    # Health & Stats (1 Method - Merged)
    # =========================================================================

    async def get_health(self) -> Dict[str, Any]:
        """
        Get health status and metrics (merged health_check + get_metrics).

        Returns:
            Combined health and metrics information
        """
        health = await self._make_request("GET", "/health", use_cache=True)
        metrics = await self._make_request("GET", "/metrics", use_cache=True)
        return {
            "health": health,
            "metrics": metrics.get("metrics", {}),
        }

    # =========================================================================
    # File-Centric Operations (5 Methods)
    # =========================================================================

    async def mount_resource(
        self,
        uri: str,
        media_type: str,
        user_id: str,
        agent_id: str,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> ResourceDescriptor:
        """Mount a resource (file) into the memory system."""
        data: Dict[str, Any] = {
            "uri": uri,
            "media_type": media_type,
            "scope": {"user_id": user_id, "agent_id": agent_id},
        }
        if metadata:
            data["metadata"] = metadata
        response = await self._make_request("POST", "/api/v1/file-centric/resources", data=data)
        return ResourceDescriptor.from_dict(response)

    async def list_resources(
        self,
        user_id: str,
        agent_id: str,
        limit: int = 100,
    ) -> List[ResourceDescriptor]:
        """List resources for a scope."""
        params = {"user_id": user_id, "agent_id": agent_id, "limit": limit}
        response = await self._make_request("GET", "/api/v1/file-centric/resources", params=params, use_cache=True)
        return [ResourceDescriptor.from_dict(r) for r in response.get("resources", [])]

    async def list_categories(
        self,
        user_id: str,
        agent_id: str,
        limit: int = 100,
    ) -> List[CategoryDescriptor]:
        """List categories for a scope."""
        params = {"user_id": user_id, "agent_id": agent_id, "limit": limit}
        response = await self._make_request("GET", "/api/v1/file-centric/categories", params=params, use_cache=True)
        return [CategoryDescriptor.from_dict(c) for c in response.get("categories", [])]

    async def search_categories(
        self,
        query: str,
        user_id: str,
        agent_id: str,
        limit: int = 10,
    ) -> List[CategoryDescriptor]:
        """Search categories by query."""
        data = {
            "query": query,
            "scope": {"user_id": user_id, "agent_id": agent_id},
            "limit": limit,
        }
        response = await self._make_request("POST", "/api/v1/file-centric/categories/search", data=data)
        return [CategoryDescriptor.from_dict(c) for c in response.get("categories", [])]

    async def extract_resource(self, resource_id: str, user_id: str, agent_id: str) -> ExtractionResult:
        """Extract structured data from a mounted resource."""
        data = {
            "resource_id": resource_id,
            "scope": {"user_id": user_id, "agent_id": agent_id},
        }
        response = await self._make_request("POST", f"/api/v1/file-centric/resources/{resource_id}/extract", data=data)
        return ExtractionResult.from_dict(response)

    # =========================================================================
    # Webhook Operations (6 Methods) 🆕 Gap vs Mem0/Letta
    # =========================================================================

    async def create_webhook(
        self,
        name: str,
        url: str,
        event_types: List[str],
        is_active: bool = True,
    ) -> WebhookSubscription:
        """
        Create a new webhook subscription.

        Args:
            name: Webhook name
            url: Target URL to receive events
            event_types: List of event types to subscribe to
            is_active: Whether the webhook is active (default: True)

        Returns:
            WebhookSubscription object
        """
        data = {
            "name": name,
            "url": url,
            "event_types": event_types,
            "is_active": is_active,
        }
        response = await self._make_request("POST", "/api/v1/webhooks", data=data)
        return WebhookSubscription.from_dict(response)

    async def list_webhooks(self) -> List[WebhookSubscription]:
        """List all webhooks for the current user."""
        response = await self._make_request("GET", "/api/v1/webhooks", use_cache=True)
        return [WebhookSubscription.from_dict(w) for w in response.get("webhooks", [])]

    async def get_webhook(self, webhook_id: str) -> WebhookSubscription:
        """Get a webhook by ID."""
        response = await self._make_request("GET", f"/api/v1/webhooks/{webhook_id}")
        return WebhookSubscription.from_dict(response)

    async def update_webhook(
        self,
        webhook_id: str,
        name: Optional[str] = None,
        url: Optional[str] = None,
        event_types: Optional[List[str]] = None,
        is_active: Optional[bool] = None,
    ) -> WebhookSubscription:
        """Update a webhook."""
        data = {}
        if name is not None:
            data["name"] = name
        if url is not None:
            data["url"] = url
        if event_types is not None:
            data["event_types"] = event_types
        if is_active is not None:
            data["is_active"] = is_active
        response = await self._make_request("PUT", f"/api/v1/webhooks/{webhook_id}", data=data)
        return WebhookSubscription.from_dict(response)

    async def delete_webhook(self, webhook_id: str) -> bool:
        """Delete a webhook."""
        await self._make_request("DELETE", f"/api/v1/webhooks/{webhook_id}")
        return True

    async def get_webhook_stats(self) -> WebhookStats:
        """Get webhook statistics."""
        response = await self._make_request("GET", "/api/v1/webhooks/stats")
        return WebhookStats.from_dict(response)

    async def test_webhook(self, webhook_id: str) -> Dict[str, Any]:
        """Send a test event to a webhook."""
        return await self._make_request("POST", f"/api/v1/webhooks/{webhook_id}/test")


# Alias for backward compatibility
Memory = AgentMemClient
