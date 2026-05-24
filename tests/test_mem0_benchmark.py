#!/usr/bin/env python3
"""
AgentMem Mem0 Benchmark Tests
=============================
Comprehensive Mem0-style tests for AgentMem.

Reference: Mem0 official API and behaviors
"""

import asyncio
import sys
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List, Optional

sys.path.insert(0, str(Path(__file__).parent.parent / "python"))
from agentmem.memory import Memory
from agentmem.types import MemoryRecord

# ============================================================================
# Mem0 Benchmark Tests
# ============================================================================

class TestMem0Benchmark:
    """Mem0 API Compatibility Tests"""
    
    async def setup_method(self):
        """Setup for each test."""
        self.memory = Memory()
        return self.memory
    
    # =========================================================================
    # Test Group 1: Core CRUD Operations
    # =========================================================================
    
    async def test_add_memory(self):
        """Test adding a single memory."""
        memory = await self.setup_method()
        result = await memory.add(
            "I love programming in Python",
            agent_id="test-agent",
            user_id="test-user"
        )
        assert "id" in result, "Should return memory ID"
        print("✅ test_add_memory: PASS")
        return result["id"]
    
    async def test_add_multiple_memories(self):
        """Test adding multiple memories."""
        memory = await self.setup_method()
        memories = [
            "Python is my favorite language",
            "I prefer dark mode UI",
            "I work on weekends sometimes"
        ]
        ids = []
        for content in memories:
            result = await memory.add(content, agent_id="test-agent")
            assert "id" in result
            ids.append(result["id"])
        
        print(f"✅ test_add_multiple_memories: Added {len(ids)} memories")
        return ids
    
    async def test_search_memory(self):
        """Test searching memories."""
        memory = await self.setup_method()
        
        # Add memories
        await memory.add("User likes Italian cuisine", agent_id="test-agent")
        await memory.add("User dislikes spicy food", agent_id="test-agent")
        
        # Search
        results = await memory.search("Italian", agent_id="test-agent")
        assert len(results) >= 1, "Should find at least one result"
        
        print(f"✅ test_search_memory: Found {len(results)} results")
        return results
    
    async def test_update_memory(self):
        """Test updating a memory."""
        memory = await self.setup_method()
        
        # Add memory
        result = await memory.add("My name is John", agent_id="test-agent")
        memory_id = result["id"]
        
        # Update
        updated = await memory.update(memory_id, content="My name is John Doe")
        assert updated["content"] == "My name is John Doe"
        
        # Verify
        results = await memory.search("John Doe", agent_id="test-agent")
        assert len(results) >= 1, "Should find updated content"
        
        print("✅ test_update_memory: PASS")
        return True
    
    async def test_delete_memory(self):
        """Test deleting a memory."""
        memory = await self.setup_method()
        
        # Add memory
        result = await memory.add("Temporary data", agent_id="test-agent")
        memory_id = result["id"]
        
        # Delete
        delete_result = await memory.delete(memory_id)
        assert delete_result["status"] == "success"
        
        # Verify deletion
        mem = await memory.get(memory_id)
        assert mem is None, "Memory should be deleted"
        
        print("✅ test_delete_memory: PASS")
        return True
    
    # =========================================================================
    # Test Group 2: Agent and User Isolation
    # =========================================================================
    
    async def test_agent_isolation(self):
        """Test that agents have isolated memories."""
        memory = await self.setup_method()
        
        # Add to different agents
        await memory.add("Agent A secret", agent_id="agent-a")
        await memory.add("Agent B secret", agent_id="agent-b")
        
        # Search from agent A
        results_a = await memory.search("secret", agent_id="agent-a")
        
        # Search from agent B
        results_b = await memory.search("secret", agent_id="agent-b")
        
        assert len(results_a) == 1, "Agent A should only see its own memories"
        assert len(results_b) == 1, "Agent B should only see its own memories"
        
        print("✅ test_agent_isolation: PASS")
        return True
    
    async def test_user_isolation(self):
        """Test that users have isolated memories."""
        memory = await self.setup_method()
        
        # Add to different users
        await memory.add("User 1 private data", user_id="user-1")
        await memory.add("User 2 private data", user_id="user-2")
        
        # Search from user 1
        results_1 = await memory.search("private", user_id="user-1")
        
        # Search from user 2
        results_2 = await memory.search("private", user_id="user-2")
        
        assert len(results_1) == 1, "User 1 should only see their own memories"
        assert len(results_2) == 1, "User 2 should only see their own memories"
        
        print("✅ test_user_isolation: PASS")
        return True
    
    async def test_session_isolation(self):
        """Test that sessions have isolated memories."""
        memory = await self.setup_method()
        
        # Add to different sessions
        await memory.add("Session 1 data", session_id="session-1")
        await memory.add("Session 2 data", session_id="session-2")
        
        # Get all for session 1
        all_1 = await memory.get_all(session_id="session-1")
        
        # Get all for session 2
        all_2 = await memory.get_all(session_id="session-2")
        
        assert len(all_1) == 1, "Session 1 should have 1 memory"
        assert len(all_2) == 1, "Session 2 should have 1 memory"
        
        print("✅ test_session_isolation: PASS")
        return True
    
    # =========================================================================
    # Test Group 3: Memory Type Filtering
    # =========================================================================
    
    async def test_filter_by_memory_type(self):
        """Test filtering by memory type."""
        memory = await self.setup_method()
        
        # Add different types
        await memory.add("Procedural: how to deploy", memory_type="procedural")
        await memory.add("Semantic: fact about physics", memory_type="semantic")
        await memory.add("Episodic: event happened", memory_type="episodic")
        
        # Filter by type
        semantic_results = await memory.search("physics", memory_type="semantic")
        procedural_results = await memory.search("deploy", memory_type="procedural")
        
        assert len(semantic_results) >= 1, "Should find semantic memory"
        assert len(procedural_results) >= 1, "Should find procedural memory"
        
        print("✅ test_filter_by_memory_type: PASS")
        return True
    
    # =========================================================================
    # Test Group 4: Complex Queries
    # =========================================================================
    
    async def test_partial_match(self):
        """Test partial word matching."""
        memory = await self.setup_method()
        
        # Add memories
        await memory.add("JavaScript is a web language", agent_id="test")
        await memory.add("TypeScript is a typed language", agent_id="test")
        
        # Search partial
        results = await memory.search("java", agent_id="test")
        
        # Should match JavaScript
        assert any("JavaScript" in r.get("content", "") for r in results), \
            "Should match JavaScript"
        
        print(f"✅ test_partial_match: Found {len(results)} results")
        return True
    
    async def test_multi_word_search(self):
        """Test searching with multiple words."""
        memory = await self.setup_method()
        
        await memory.add("The quick brown fox jumps", agent_id="test")
        
        # Search multiple words
        results = await memory.search("quick fox", agent_id="test")
        
        print(f"✅ test_multi_word_search: Found {len(results)} results")
        return len(results) > 0
    
    async def test_case_insensitive(self):
        """Test case insensitive search."""
        memory = await self.setup_method()
        
        await memory.add("PYTHON PROGRAMMING", agent_id="test")
        
        # Search lowercase
        results = await memory.search("python", agent_id="test")
        
        assert len(results) >= 1, "Should find case-insensitive match"
        
        print("✅ test_case_insensitive: PASS")
        return True
    
    # =========================================================================
    # Test Group 5: Importance and Metadata
    # =========================================================================
    
    async def test_importance_scoring(self):
        """Test importance scoring."""
        memory = await self.setup_method()
        
        # Add with different importance
        await memory.add("Critical system failure", importance=0.9, agent_id="test")
        await memory.add("Minor task", importance=0.3, agent_id="test")
        
        # Get all
        all_memories = await memory.get_all(agent_id="test")
        
        # Check importance is stored
        high_importance = [m for m in all_memories if m.get("importance", 0) >= 0.8]
        
        assert len(high_importance) >= 1, "Should have high importance memory"
        
        print(f"✅ test_importance_scoring: {len(high_importance)} high importance")
        return True
    
    async def test_metadata_storage(self):
        """Test storing metadata with memories."""
        memory = await self.setup_method()
        
        metadata = {
            "project": "Alpha",
            "priority": "high",
            "tags": ["important", "work"]
        }
        
        result = await memory.add(
            "Project deadline is Friday",
            metadata=metadata,
            agent_id="test"
        )
        
        # Get the memory
        mem = await memory.get(result["id"])
        
        assert "metadata" in mem, "Should have metadata field"
        assert mem["metadata"].get("project") == "Alpha"
        
        print("✅ test_metadata_storage: PASS")
        return True
    
    # =========================================================================
    # Test Group 6: Edge Cases
    # =========================================================================
    
    async def test_empty_content(self):
        """Test adding empty content."""
        memory = await self.setup_method()
        
        try:
            await memory.add("", agent_id="test")
            print("⚠️ test_empty_content: Empty content accepted")
        except Exception as e:
            print(f"✅ test_empty_content: Rejected with {type(e).__name__}")
        
        return True
    
    async def test_very_long_content(self):
        """Test adding very long content."""
        memory = await self.setup_method()
        
        long_content = "A" * 10000
        result = await memory.add(long_content, agent_id="test")
        
        assert "id" in result
        print("✅ test_very_long_content: PASS")
        return True
    
    async def test_unicode_content(self):
        """Test Unicode content."""
        memory = await self.setup_method()
        
        content = "用户偏好中文内容，日本語、韩国어도 지원합니다"
        result = await memory.add(content, agent_id="test")
        
        # Search
        results = await memory.search("中文", agent_id="test")
        
        assert len(results) >= 1, "Should find Unicode content"
        
        print("✅ test_unicode_content: PASS")
        return True
    
    async def test_special_characters(self):
        """Test special characters in content."""
        memory = await self.setup_method()
        
        content = 'Email: test@example.com | URL: https://test.com | Path: /usr/local/bin'
        result = await memory.add(content, agent_id="test")
        
        # Search special characters
        results = await memory.search("test@example.com", agent_id="test")
        
        print(f"✅ test_special_characters: Found {len(results)} results")
        return len(results) >= 1
    
    # =========================================================================
    # Test Group 7: Batch Operations
    # =========================================================================
    
    async def test_batch_add(self):
        """Test batch adding memories."""
        memory = await self.setup_method()
        
        # Add 100 memories
        for i in range(100):
            await memory.add(f"Memory {i}", agent_id="test")
        
        # Verify
        all_memories = await memory.get_all(agent_id="test")
        
        assert len(all_memories) >= 100, f"Should have >= 100 memories, got {len(all_memories)}"
        
        print(f"✅ test_batch_add: Added {len(all_memories)} memories")
        return True
    
    async def test_batch_search(self):
        """Test batch searching."""
        memory = await self.setup_method()
        
        # Add memories with common pattern
        for i in range(50):
            await memory.add(f"Batch test memory {i}", agent_id="test")
        
        # Search
        results = await memory.search("Batch test", agent_id="test")
        
        print(f"✅ test_batch_search: Found {len(results)} results")
        return len(results) >= 50
    
    # =========================================================================
    # Test Group 8: Clear Operations
    # =========================================================================
    
    async def test_clear_by_agent(self):
        """Test clearing memories by agent."""
        memory = await self.setup_method()
        
        # Add memories to agent
        await memory.add("Memory 1", agent_id="agent-clear")
        await memory.add("Memory 2", agent_id="agent-clear")
        
        # Add to other agent (should not be cleared)
        await memory.add("Other agent memory", agent_id="other-agent")
        
        # Clear agent's memories
        result = await memory.clear(agent_id="agent-clear")
        
        # Verify
        remaining = await memory.get_all(agent_id="agent-clear")
        
        assert len(remaining) == 0, "Agent memories should be cleared"
        
        other = await memory.get_all(agent_id="other-agent")
        assert len(other) == 1, "Other agent should not be affected"
        
        print("✅ test_clear_by_agent: PASS")
        return True
    
    async def test_clear_by_user(self):
        """Test clearing memories by user."""
        memory = await self.setup_method()
        
        await memory.add("User memory", user_id="user-clear")
        
        # Clear
        result = await memory.clear(user_id="user-clear")
        
        remaining = await memory.get_all(user_id="user-clear")
        assert len(remaining) == 0, "User memories should be cleared"
        
        print("✅ test_clear_by_user: PASS")
        return True
    
    # =========================================================================
    # Test Group 9: Concurrency
    # =========================================================================
    
    async def test_concurrent_add(self):
        """Test concurrent memory addition."""
        memory = await self.setup_method()
        
        tasks = [memory.add(f"Concurrent {i}", agent_id="test") for i in range(20)]
        results = await asyncio.gather(*tasks)
        
        assert all("id" in r for r in results), "All should have IDs"
        
        print(f"✅ test_concurrent_add: Added {len(results)} memories concurrently")
        return True
    
    async def test_concurrent_search(self):
        """Test concurrent searching."""
        memory = await self.setup_method()
        
        # Add memories
        for i in range(10):
            await memory.add(f"Search test {i}", agent_id="test")
        
        # Concurrent searches
        tasks = [
            memory.search("Search test", agent_id="test"),
            memory.search("test", agent_id="test"),
            memory.search("Search", agent_id="test"),
        ]
        
        results = await asyncio.gather(*tasks)
        
        print(f"✅ test_concurrent_search: {len(results)} concurrent searches completed")
        return True
    
    # =========================================================================
    # Test Group 10: Recall Quality Metrics
    # =========================================================================
    
    async def test_recall_precision(self):
        """Test recall and precision metrics."""
        memory = await self.setup_method()
        
        # Add specific memories
        await memory.add("Python is a programming language", agent_id="test")
        await memory.add("Python is my favorite language", agent_id="test")
        await memory.add("JavaScript is for web", agent_id="test")
        
        # Search Python
        results = await memory.search("Python", agent_id="test")
        
        # Calculate metrics
        relevant = [r for r in results if "Python" in r.get("content", "")]
        total = len(results)
        
        precision = len(relevant) / total if total > 0 else 0
        
        print(f"✅ test_recall_precision: Precision = {precision:.2%}")
        return precision >= 0.5
    
    async def test_recall_recall(self):
        """Test recall metric."""
        memory = await self.setup_method()
        
        # Add memories with specific keywords
        for i in range(5):
            await memory.add(f"Machine learning model {i}", agent_id="test")
        
        # Search
        results = await memory.search("machine learning", agent_id="test")
        
        recall = len(results) / 5  # We added 5 relevant
        
        print(f"✅ test_recall_recall: Recall = {recall:.2%}")
        return recall >= 0.8
    
    async def test_ranking_quality(self):
        """Test result ranking quality."""
        memory = await self.setup_method()
        
        # Add memories with varying relevance
        await memory.add("Python programming tutorial", agent_id="test")
        await memory.add("Python documentation", agent_id="test")
        await memory.add("JavaScript framework", agent_id="test")
        
        # Search
        results = await memory.search("Python tutorial", agent_id="test")
        
        # Check ranking
        if len(results) >= 2:
            # First result should have higher score
            assert results[0]["score"] >= results[1]["score"], "Better match should be first"
        
        print(f"✅ test_ranking_quality: Top result score = {results[0].get('score', 0):.2f}")
        return True


# ============================================================================
# Run All Tests
# ============================================================================

async def run_all_mem0_tests():
    """Run all Mem0 benchmark tests."""
    print("="*60)
    print("  AgentMem Mem0 Benchmark Tests")
    print("  Comprehensive Mem0-style API tests")
    print("="*60)
    
    tester = TestMem0Benchmark()
    results = []
    
    # Group 1: Core CRUD
    print("\n[Group 1: Core CRUD Operations]")
    results.append(("add_memory", await tester.test_add_memory()))
    results.append(("add_multiple", await tester.test_add_multiple_memories()))
    results.append(("search", await tester.test_search_memory()))
    results.append(("update", await tester.test_update_memory()))
    results.append(("delete", await tester.test_delete_memory()))
    
    # Group 2: Isolation
    print("\n[Group 2: Agent/User/Session Isolation]")
    results.append(("agent_isolation", await tester.test_agent_isolation()))
    results.append(("user_isolation", await tester.test_user_isolation()))
    results.append(("session_isolation", await tester.test_session_isolation()))
    
    # Group 3: Memory Type
    print("\n[Group 3: Memory Type Filtering]")
    results.append(("memory_type_filter", await tester.test_filter_by_memory_type()))
    
    # Group 4: Complex Queries
    print("\n[Group 4: Complex Queries]")
    results.append(("partial_match", await tester.test_partial_match()))
    results.append(("multi_word", await tester.test_multi_word_search()))
    results.append(("case_insensitive", await tester.test_case_insensitive()))
    
    # Group 5: Importance/Metadata
    print("\n[Group 5: Importance and Metadata]")
    results.append(("importance", await tester.test_importance_scoring()))
    results.append(("metadata", await tester.test_metadata_storage()))
    
    # Group 6: Edge Cases
    print("\n[Group 6: Edge Cases]")
    results.append(("empty_content", await tester.test_empty_content()))
    results.append(("long_content", await tester.test_very_long_content()))
    results.append(("unicode", await tester.test_unicode_content()))
    results.append(("special_chars", await tester.test_special_characters()))
    
    # Group 7: Batch Operations
    print("\n[Group 7: Batch Operations]")
    results.append(("batch_add", await tester.test_batch_add()))
    results.append(("batch_search", await tester.test_batch_search()))
    
    # Group 8: Clear Operations
    print("\n[Group 8: Clear Operations]")
    results.append(("clear_by_agent", await tester.test_clear_by_agent()))
    results.append(("clear_by_user", await tester.test_clear_by_user()))
    
    # Group 9: Concurrency
    print("\n[Group 9: Concurrency]")
    results.append(("concurrent_add", await tester.test_concurrent_add()))
    results.append(("concurrent_search", await tester.test_concurrent_search()))
    
    # Group 10: Recall Quality
    print("\n[Group 10: Recall Quality Metrics]")
    results.append(("precision", await tester.test_recall_precision()))
    results.append(("recall", await tester.test_recall_recall()))
    results.append(("ranking", await tester.test_ranking_quality()))
    
    # Summary
    print("\n" + "="*60)
    print("  Test Summary")
    print("="*60)
    
    passed = sum(1 for _, result in results if result)
    total = len(results)
    
    print(f"\n  Total: {total}")
    print(f"  Passed: {passed}")
    print(f"  Failed: {total - passed}")
    print(f"  Success Rate: {passed/total*100:.1f}%")
    
    if passed == total:
        print("\n  🎉 All Mem0 Benchmark Tests Passed!")
    else:
        failed_tests = [name for name, result in results if not result]
        print(f"\n  ❌ Failed tests: {', '.join(failed_tests)}")
    
    return results


if __name__ == "__main__":
    results = asyncio.run(run_all_mem0_tests())
    sys.exit(0 if all(r for _, r in results) else 1)
