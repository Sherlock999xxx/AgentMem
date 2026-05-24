"""
AgentMem - AI Agent Memory Platform

Simple Memory API (Mem0-compatible) for AgentMem.
"""

# Import directly from memory module to avoid langchain dependency issues
from .memory import Memory
from .types import MemoryRecord, SearchResult

__all__ = ["Memory", "MemoryRecord", "SearchResult"]
__version__ = "1.0.0"
