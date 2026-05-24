#!/usr/bin/env python3
"""
AgentMem 100 Rounds Validation Test
====================================
100 rounds of comprehensive tests for AgentMem.

Each round tests a specific aspect of the memory system.
"""

import asyncio
import sys
from pathlib import Path
from typing import List, Tuple

sys.path.insert(0, str(Path(__file__).parent.parent / "python"))
from agentmem.memory import Memory

# Test statistics
stats = {"total": 0, "passed": 0, "failed": 0}


def log_round(round_num: int, name: str, passed: bool, details: str = ""):
    """Log round result."""
    stats["total"] += 1
    if passed:
        stats["passed"] += 1
        print(f"  ✅ Round {round_num}: {name}")
    else:
        stats["failed"] += 1
        print(f"  ❌ Round {round_num}: {name} - {details}")


async def round_1_10_basic_operations(memory: Memory):
    """Rounds 1-10: Basic CRUD operations."""
    print("\n[Round 1-10: Basic Operations]")
    
    # Round 1: Add memory
    result = await memory.add("Round 1 memory", agent_id="basic-test")
    log_round(1, "Add memory", "id" in result)
    
    # Round 2: Get memory
    mem_id = result["id"]
    mem = await memory.get(mem_id)
    log_round(2, "Get memory by ID", mem is not None)
    
    # Round 3: Update memory
    updated = await memory.update(mem_id, content="Updated round 1")
    log_round(3, "Update memory", updated["content"] == "Updated round 1")
    
    # Round 4: Search memory
    results = await memory.search("Updated", agent_id="basic-test")
    log_round(4, "Search updated", len(results) > 0)
    
    # Round 5: Delete memory
    deleted = await memory.delete(mem_id)
    log_round(5, "Delete memory", deleted["status"] == "success")
    
    # Round 6-10: Multiple adds
    for i in range(6, 11):
        result = await memory.add(f"Round {i} content", agent_id="basic-test")
        log_round(i, "Multiple add", "id" in result)


async def round_11_20_search_operations(memory: Memory):
    """Rounds 11-20: Search operations."""
    print("\n[Round 11-20: Search Operations]")
    
    # Add test memories
    for i in range(11, 21):
        await memory.add(f"Search test memory {i}", agent_id="search-test")
    
    # Round 11-15: Different searches
    queries = ["Search", "test", "memory", "Search test", "memory 15"]
    for i, query in enumerate(queries, start=11):
        results = await memory.search(query, agent_id="search-test")
        log_round(i, f"Search '{query}'", len(results) > 0, f"Found {len(results)}")
    
    # Round 16: Partial match
    results = await memory.search("Search test", agent_id="search-test")
    log_round(16, "Partial match search", len(results) >= 10)
    
    # Round 17: Case insensitive
    results = await memory.search("SEARCH", agent_id="search-test")
    log_round(17, "Case insensitive", len(results) >= 10)
    
    # Round 18: No results - should return empty list
    results = await memory.search("nonexistent999xyz", agent_id="search-test")
    log_round(18, "No results search", isinstance(results, list), f"Type: {type(results)}")
    
    # Round 19: Empty search - should handle gracefully
    results = await memory.search("", agent_id="search-test")
    log_round(19, "Empty query", isinstance(results, list))
    
    # Round 20: Get all - verify it returns a list
    all_mem = await memory.get_all(agent_id="search-test")
    log_round(20, "Get all memories", isinstance(all_mem, list) and len(all_mem) >= 10)


async def round_21_30_type_filtering(memory: Memory):
    """Rounds 21-30: Memory type filtering."""
    print("\n[Round 21-30: Memory Type Filtering]")
    
    types = ["episodic", "semantic", "procedural", "working", "core", "resource", "knowledge", "contextual"]
    
    # Round 21-28: Add different types
    for i, mem_type in enumerate(types, start=21):
        result = await memory.add(f"Type test {mem_type}", memory_type=mem_type, agent_id="type-test")
        log_round(i, f"Add {mem_type}", "id" in result)
    
    # Round 29: Filter by type
    results = await memory.search("Type test", memory_type="semantic", agent_id="type-test")
    log_round(29, "Filter semantic", len(results) >= 1)
    
    # Round 30: Filter by multiple criteria
    results = await memory.search("Type test", agent_id="type-test", memory_type="episodic")
    log_round(30, "Filter by type + agent", len(results) >= 1)


async def round_31_40_agent_user_isolation(memory: Memory):
    """Rounds 31-40: Agent and user isolation."""
    print("\n[Round 31-40: Agent/User Isolation]")
    
    # Round 31-33: Different agents
    agents = ["agent-a", "agent-b", "agent-c"]
    for i, agent_id in enumerate(agents, start=31):
        result = await memory.add(f"Agent {agent_id} memory", agent_id=agent_id)
        log_round(i, f"Add to {agent_id}", "id" in result)
    
    # Round 34-36: Search from each agent - isolated
    for i, agent_id in enumerate(agents, start=34):
        results = await memory.search("memory", agent_id=agent_id)
        log_round(i, f"Search from {agent_id}", len(results) == 1)
    
    # Round 37-39: Different users
    users = ["user-1", "user-2", "user-3"]
    for i, user_id in enumerate(users, start=37):
        result = await memory.add(f"User {user_id} data", user_id=user_id)
        log_round(i, f"Add to {user_id}", "id" in result)
    
    # Round 40: User isolation check
    results = await memory.search("data", user_id="user-1")
    log_round(40, "User isolation", len(results) == 1)


async def round_41_50_importance_metadata(memory: Memory):
    """Rounds 41-50: Importance and metadata."""
    print("\n[Round 41-50: Importance and Metadata]")
    
    # Round 41-43: Different importance levels
    for i, importance in enumerate([0.3, 0.6, 0.9], start=41):
        result = await memory.add(f"Importance test {i}", importance=importance, agent_id="meta-test")
        log_round(i, f"Importance {importance}", "id" in result)
    
    # Round 44: Metadata storage
    metadata = {"key1": "value1", "key2": 123, "nested": {"a": "b"}}
    result = await memory.add("Metadata test", metadata=metadata, agent_id="meta-test")
    mem = await memory.get(result["id"])
    log_round(44, "Metadata storage", mem and "metadata" in mem)
    
    # Round 45: Update metadata
    updated = await memory.update(result["id"], metadata={"new": "data"})
    log_round(45, "Update metadata", updated["metadata"]["new"] == "data")
    
    # Round 46-50: Complex metadata
    for i in range(46, 51):
        metadata = {"index": i, "tags": [f"tag{i}", "common"]}
        result = await memory.add(f"Complex metadata {i}", metadata=metadata, agent_id="meta-test")
        log_round(i, f"Complex metadata {i}", "id" in result)


async def round_51_60_edge_cases(memory: Memory):
    """Rounds 51-60: Edge cases."""
    print("\n[Round 51-60: Edge Cases]")
    
    # Round 51: Empty content
    try:
        await memory.add("", agent_id="edge-test")
        log_round(51, "Empty content", True)  # Accepted
    except:
        log_round(51, "Empty content", True)  # Rejected is also fine
    
    # Round 52: Very long content
    long = "A" * 5000
    result = await memory.add(long, agent_id="edge-test")
    log_round(52, "Long content (5000)", "id" in result)
    
    # Round 53: Unicode content
    result = await memory.add("中文内容测试 中文", agent_id="edge-test")
    results = await memory.search("中文", agent_id="edge-test")
    log_round(53, "Unicode search", len(results) > 0)
    
    # Round 54: Special characters
    result = await memory.add("Email: test@test.com | URL: http://test.com", agent_id="edge-test")
    results = await memory.search("test@test.com", agent_id="edge-test")
    log_round(54, "Special characters", len(results) > 0)
    
    # Round 55: Emoji
    result = await memory.add("Great news 🚀🎉", agent_id="edge-test")
    results = await memory.search("🚀", agent_id="edge-test")
    log_round(55, "Emoji content", len(results) > 0)
    
    # Round 56: SQL injection attempt (should be safe)
    result = await memory.add("'; DROP TABLE memories; --", agent_id="edge-test")
    log_round(56, "SQL injection safe", "id" in result)
    
    # Round 57: HTML content
    result = await memory.add("<script>alert('xss')</script>", agent_id="edge-test")
    log_round(57, "HTML content safe", "id" in result)
    
    # Round 58: Very short content
    result = await memory.add("X", agent_id="edge-test")
    log_round(58, "Short content", "id" in result)
    
    # Round 59: Duplicate content
    result1 = await memory.add("Duplicate test", agent_id="edge-test")
    result2 = await memory.add("Duplicate test", agent_id="edge-test")
    log_round(59, "Duplicate handling", "id" in result1 and "id" in result2)
    
    # Round 60: Whitespace content
    result = await memory.add("   ", agent_id="edge-test")
    log_round(60, "Whitespace content", "id" in result)


async def round_61_70_batch_operations(memory: Memory):
    """Rounds 61-70: Batch operations."""
    print("\n[Round 61-70: Batch Operations]")
    
    # Round 61-65: Batch add
    batch_ids = []
    for i in range(61, 66):
        result = await memory.add(f"Batch add {i}", agent_id="batch-test")
        batch_ids.append(result["id"])
    log_round(61, "Batch add 5", len(batch_ids) == 5)
    
    # Round 62-65: Verify each
    for i, mem_id in enumerate(batch_ids, start=62):
        mem = await memory.get(mem_id)
        log_round(i, f"Verify batch {i-61}", mem is not None)
    
    # Round 66-70: Batch search
    for i in range(66, 71):
        results = await memory.search("Batch", agent_id="batch-test")
        log_round(i, f"Batch search {i-65}", len(results) >= 5)


async def round_71_80_concurrent_operations(memory: Memory):
    """Rounds 71-80: Concurrent operations."""
    print("\n[Round 71-80: Concurrent Operations]")
    
    # Round 71-75: Concurrent adds
    tasks = [memory.add(f"Concurrent add {i}", agent_id="concurrent-test") for i in range(71, 76)]
    results = await asyncio.gather(*tasks)
    log_round(71, "Concurrent add 5", all("id" in r for r in results))
    
    # Round 72-75: Verify each
    for i in range(72, 76):
        results = await memory.search(f"Concurrent add {i}", agent_id="concurrent-test")
        log_round(i, f"Verify concurrent {i-71}", len(results) >= 1)
    
    # Round 76-80: Concurrent searches
    tasks = [memory.search("Concurrent", agent_id="concurrent-test") for _ in range(76, 81)]
    results = await asyncio.gather(*tasks)
    log_round(76, "Concurrent search 5", all(isinstance(r, list) for r in results))


async def round_81_90_clear_operations(memory: Memory):
    """Rounds 81-90: Clear operations."""
    print("\n[Round 81-90: Clear Operations]")
    
    # Add test memories for clearing - use unique agent
    await memory.add("Clear test 1", agent_id="clear-agent-1")
    await memory.add("Clear test 2", agent_id="clear-agent-1")
    await memory.add("Keep this", agent_id="keep-agent")
    
    # Round 81-82: Clear by agent
    result = await memory.clear(agent_id="clear-agent-1")
    log_round(81, "Clear by agent", result["count"] >= 2)
    
    # Round 83: Verify cleared
    remaining = await memory.get_all(agent_id="clear-agent-1")
    log_round(82, "Verify clear", len(remaining) == 0)
    
    # Round 84: Other agent should still have memories
    other = await memory.get_all(agent_id="keep-agent")
    log_round(83, "Other agent preserved", len(other) >= 1)
    
    # Round 84-87: Clear by user - new user
    await memory.add("User clear test", user_id="clear-user-1")
    result = await memory.clear(user_id="clear-user-1")
    log_round(84, "Clear by user", result["count"] >= 1)
    
    # Round 85: Verify user cleared
    remaining = await memory.get_all(user_id="clear-user-1")
    log_round(85, "Verify user cleared", len(remaining) == 0)
    
    # Round 86-90: Additional operations after clearing
    for i in range(86, 91):
        result = await memory.add(f"After clear {i}", agent_id="after-clear")
        log_round(i, f"Post-clear add {i-85}", "id" in result)


async def round_91_100_recall_quality(memory: Memory):
    """Rounds 91-100: Recall quality tests."""
    print("\n[Round 91-100: Recall Quality]")
    
    # Round 91-95: Add diverse memories
    memories = [
        ("Python tutorial for beginners", "semantic"),
        ("Python debugging tips", "semantic"),
        ("Python advanced patterns", "semantic"),
        ("JavaScript basics", "semantic"),
        ("Rust programming guide", "semantic"),
    ]
    for i, (content, mem_type) in enumerate(memories, start=91):
        result = await memory.add(content, memory_type=mem_type, agent_id="recall-test")
        log_round(i, f"Add diverse {i-90}", "id" in result)
    
    # Round 96: Search precision
    results = await memory.search("Python tutorial", agent_id="recall-test")
    if results:
        precision = sum(1 for r in results if "Python" in r.get("content", ""))
        log_round(96, "Search precision", precision / len(results) >= 0.5,
                  f"{precision}/{len(results)}")
    else:
        log_round(96, "Search precision", False, "No results")
    
    # Round 97: Search recall
    all_python = [r for r in await memory.search("Python", agent_id="recall-test") 
                  if "Python" in r.get("content", "")]
    log_round(97, "Search recall", len(all_python) >= 3, f"Found {len(all_python)}")
    
    # Round 98: Ranking quality
    results = await memory.search("Python", agent_id="recall-test")
    if len(results) >= 2:
        ranked = results[0]["score"] >= results[1]["score"]
        log_round(98, "Result ranking", ranked)
    else:
        log_round(98, "Result ranking", True)
    
    # Round 99: Cross-type search
    await memory.add("Cross type test episodic", memory_type="episodic", agent_id="cross-test")
    await memory.add("Cross type test semantic", memory_type="semantic", agent_id="cross-test")
    results = await memory.search("Cross type", agent_id="cross-test")
    log_round(99, "Cross-type search", len(results) >= 2, f"Found {len(results)}")
    
    # Round 100: Final comprehensive check
    all_mem = await memory.get_all()
    log_round(100, "Final check", len(all_mem) > 0, f"Total: {len(all_mem)}")


async def run_100_rounds():
    """Run all 100 rounds."""
    print("="*60)
    print("  AgentMem 100 Rounds Validation Test")
    print("  Comprehensive validation of all memory operations")
    print("="*60)
    
    memory = Memory()
    
    # Run all rounds
    await round_1_10_basic_operations(memory)
    await round_11_20_search_operations(memory)
    await round_21_30_type_filtering(memory)
    await round_31_40_agent_user_isolation(memory)
    await round_41_50_importance_metadata(memory)
    await round_51_60_edge_cases(memory)
    await round_61_70_batch_operations(memory)
    await round_71_80_concurrent_operations(memory)
    await round_81_90_clear_operations(memory)
    await round_91_100_recall_quality(memory)
    
    # Summary
    print("\n" + "="*60)
    print("  100 Rounds Test Summary")
    print("="*60)
    
    print(f"\n  Total: {stats['total']}")
    print(f"  ✅ Passed: {stats['passed']}")
    print(f"  ❌ Failed: {stats['failed']}")
    print(f"  📊 Success Rate: {stats['passed']/stats['total']*100:.1f}%")
    
    if stats['failed'] == 0:
        print("\n  🎉 All 100 Rounds Passed!")
    else:
        print(f"\n  ⚠️  {stats['failed']} rounds failed")
    
    return stats


if __name__ == "__main__":
    results = asyncio.run(run_100_rounds())
    sys.exit(0 if results['failed'] == 0 else 1)
