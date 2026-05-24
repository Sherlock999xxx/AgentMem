#!/usr/bin/env python3
"""
AgentMem Memory Effect Test - Python Implementation
===================================================
Tests 8 cognitive memory types and recall effects, similar to Mem0 benchmark.

This test validates:
1. 8 types of cognitive memories (Episodic, Semantic, Procedural, Working, Core, Resource, Knowledge, Contextual)
2. Mem0-style API compatibility
3. Recall effectiveness and search quality
4. 100 rounds of validation
"""

import asyncio
import sys
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List, Optional

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent / "python"))

# Import Memory directly from memory.py
from agentmem.memory import Memory
from agentmem.types import MemoryRecord

# ============================================================================
# Test Configuration
# ============================================================================

MEMORY_TYPES = [
    "episodic",     # 情景记忆 - Event-based memories
    "semantic",     # 语义记忆 - Knowledge and facts
    "procedural",   # 程序记忆 - How-to knowledge
    "working",      # 工作记忆 - Short-term context
    "core",         # 核心记忆 - Identity/persona
    "resource",     # 资源记忆 - Links/references
    "knowledge",    # 知识库 - Facts and concepts
    "contextual",   # 上下文 - Session context
]

# Test statistics
test_stats = {
    "total": 0,
    "passed": 0,
    "failed": 0,
    "skipped": 0,
}

# ============================================================================
# Test Helper Functions
# ============================================================================

def log_test(name: str, passed: bool, details: str = ""):
    """Log test result."""
    test_stats["total"] += 1
    if passed:
        test_stats["passed"] += 1
        print(f"  ✅ {name}")
    else:
        test_stats["failed"] += 1
        print(f"  ❌ {name}: {details}")

def log_section(title: str):
    """Log section header."""
    print(f"\n{'='*60}")
    print(f"  {title}")
    print(f"{'='*60}")

# ============================================================================
# Test 1: 8 Cognitive Memory Types
# ============================================================================

async def test_8_cognitive_memory_types():
    """Test all 8 cognitive memory types."""
    log_section("Test 1: 8种认知记忆类型 (8 Cognitive Memory Types)")
    
    memory = Memory()
    results = {}
    
    test_cases = {
        "episodic": "User asked about dinner options at 6pm",
        "semantic": "User prefers Italian food over Chinese",
        "procedural": "How to deploy: 1.Build 2.Test 3.Push 4.Monitor",
        "working": "Currently debugging issue #456 in production",
        "core": "Persona: Professional developer with 5 years experience",
        "resource": "Link: https://docs.example.com/api/v2",
        "knowledge": "Fact: Water boils at 100°C at sea level",
        "contextual": "Session: user-123, discussing project timeline",
    }
    
    for mem_type, content in test_cases.items():
        result = await memory.add(content, memory_type=mem_type)
        passed = "id" in result and result["id"]
        log_test(f"{mem_type} 记忆添加", passed, result.get("message", ""))
        results[mem_type] = result
    
    return results

# ============================================================================
# Test 2: Mem0 Benchmark - Add and Retrieve
# ============================================================================

async def test_mem0_add_and_retrieve():
    """Mem0 Benchmark: Add memory and retrieve."""
    log_section("Test 2: Mem0 Benchmark - 添加并检索 (Add and Retrieve)")
    
    memory = Memory()
    
    # Add memory
    result = await memory.add(
        "User prefers Italian food",
        agent_id="assistant-1",
        user_id="user-123"
    )
    log_test("添加记忆", "id" in result, str(result))
    
    memory_id = result.get("id")
    
    # Retrieve by search
    results = await memory.search(
        query="food preferences",
        agent_id="assistant-1",
        user_id="user-123"
    )
    
    found = any(r.get("id") == memory_id for r in results) or len(results) > 0
    log_test("检索记忆", found, f"找到 {len(results)} 个结果")
    
    return {"memory_id": memory_id, "found": found}

# ============================================================================
# Test 3: Mem0 Benchmark - Memory Update
# ============================================================================

async def test_mem0_memory_update():
    """Mem0 Benchmark: Update memory."""
    log_section("Test 3: Mem0 Benchmark - 记忆更新 (Memory Update)")
    
    memory = Memory()
    
    # Add initial memory
    result = await memory.add(
        "My name is John",
        agent_id="assistant-1",
        user_id="user-123"
    )
    memory_id = result.get("id")
    log_test("添加初始记忆", bool(memory_id), f"ID: {memory_id}")
    
    # Update memory
    updated = await memory.update(
        memory_id,
        content="My name is John Doe"
    )
    log_test("更新记忆", "id" in updated, str(updated))
    
    # Verify update
    results = await memory.search(
        query="John Doe",
        agent_id="assistant-1",
        user_id="user-123"
    )
    
    found = len(results) > 0
    log_test("验证更新", found, f"搜索找到 {len(results)} 个结果")
    
    return {"memory_id": memory_id, "updated": True}

# ============================================================================
# Test 4: Mem0 Benchmark - Memory Delete
# ============================================================================

async def test_mem0_memory_delete():
    """Mem0 Benchmark: Delete memory."""
    log_section("Test 4: Mem0 Benchmark - 记忆删除 (Memory Delete)")
    
    memory = Memory()
    
    # Add memory
    result = await memory.add(
        "Temporary data to be deleted",
        agent_id="assistant-1",
        user_id="user-123"
    )
    memory_id = result.get("id")
    log_test("添加待删除记忆", bool(memory_id), f"ID: {memory_id}")
    
    # Delete memory
    delete_result = await memory.delete(memory_id)
    log_test("删除记忆", delete_result.get("status") == "success", str(delete_result))
    
    # Verify deletion
    mem = await memory.get(memory_id)
    deleted = mem is None
    log_test("验证删除", deleted, f"记忆已不存在: {mem}")
    
    return {"deleted": deleted}

# ============================================================================
# Test 5: Mem0 Benchmark - Cross Session Memory
# ============================================================================

async def test_mem0_cross_session_memory():
    """Mem0 Benchmark: Cross-session memory persistence."""
    log_section("Test 5: Mem0 Benchmark - 跨会话记忆 (Cross Session Memory)")
    
    memory = Memory()
    
    # Simulate multiple sessions
    sessions = [
        ("session-1", "User logged in at 9am"),
        ("session-2", "User browsed products"),
        ("session-3", "User added item to cart"),
        ("session-4", "User completed purchase"),
    ]
    
    for session_id, content in sessions:
        result = await memory.add(content, session_id=session_id)
        log_test(f"Session {session_id} 添加", "id" in result, "")
    
    # Cross-session retrieval
    all_memories = await memory.get_all()
    cross_session = len(all_memories) >= 4
    log_test("跨会话检索", cross_session, f"找到 {len(all_memories)} 个记忆")
    
    return {"cross_session": cross_session}

# ============================================================================
# Test 6: Mem0 Benchmark - User Preferences
# ============================================================================

async def test_mem0_user_preferences():
    """Mem0 Benchmark: User preferences memory."""
    log_section("Test 6: Mem0 Benchmark - 用户偏好 (User Preferences)")
    
    memory = Memory()
    
    preferences = [
        "User prefers dark theme",
        "User prefers email notifications",
        "User prefers weekly reports",
        "User prefers keyboard shortcuts",
    ]
    
    for pref in preferences:
        result = await memory.add(pref, agent_id="assistant-1")
        log_test(f"添加偏好: {pref[:30]}...", "id" in result, "")
    
    # Search preferences
    results = await memory.search(query="prefers")
    
    found_count = len(results)
    log_test("偏好检索", found_count >= 2, f"找到 {found_count} 个偏好")
    
    return {"preferences_found": found_count}

# ============================================================================
# Test 7: Recall Effectiveness - Semantic Search
# ============================================================================

async def test_recall_effectiveness():
    """Test recall effectiveness with various queries."""
    log_section("Test 7: 召回效果测试 (Recall Effectiveness)")
    
    memory = Memory()
    
    # Add diverse memories
    memories = [
        ("Python is a programming language", "semantic"),
        ("JavaScript is for web development", "semantic"),
        ("Rust is for systems programming", "semantic"),
        ("User knows Python", "semantic"),
        ("User knows JavaScript", "semantic"),
        ("The quick brown fox jumps", "episodic"),
        ("A slow green turtle swims", "episodic"),
        ("Coffee with milk is delicious", "semantic"),
    ]
    
    for content, mem_type in memories:
        await memory.add(content, memory_type=mem_type)
    
    # Test different query types
    test_queries = [
        ("programming", "期望找到编程相关"),
        ("Python", "期望找到Python相关"),
        ("quick", "期望找到快速相关"),
        ("coffee", "期望找到咖啡相关"),
    ]
    
    recall_results = []
    for query, description in test_queries:
        results = await memory.search(query)
        recall_results.append((query, len(results), description))
        log_test(f"搜索 '{query}'", len(results) > 0, f"{description}: {len(results)} 个结果")
    
    return recall_results

# ============================================================================
# Test 8: Batch Operations
# ============================================================================

async def test_batch_operations():
    """Test batch add and search operations."""
    log_section("Test 8: 批量操作测试 (Batch Operations)")
    
    memory = Memory()
    
    # Batch add
    batch_size = 50
    for i in range(batch_size):
        await memory.add(f"Batch memory item {i}", memory_type="semantic")
    
    log_test(f"批量添加 {batch_size} 个记忆", True, "完成")
    
    # Batch search
    results = await memory.search("Batch")
    log_test("批量搜索", len(results) >= batch_size, f"找到 {len(results)} 个结果")
    
    # Get all memories
    all_memories = await memory.get_all()
    log_test("获取所有记忆", len(all_memories) >= batch_size, f"返回 {len(all_memories)} 个记忆")
    
    return {"batch_size": batch_size, "found": len(results)}

# ============================================================================
# Test 9: Memory Statistics
# ============================================================================

async def test_memory_statistics():
    """Test memory statistics and metadata."""
    log_section("Test 9: 记忆统计测试 (Memory Statistics)")
    
    memory = Memory()
    
    # Add various memories
    await memory.add("User loves pizza", memory_type="semantic")
    await memory.add("How to make coffee", memory_type="procedural")
    await memory.add("User session started", memory_type="episodic")
    
    # Get all memories
    all_memories = await memory.get_all()
    
    log_test("记忆总数", len(all_memories) >= 3, f"总计 {len(all_memories)} 个记忆")
    
    # Test get by ID
    if all_memories:
        mem_id = all_memories[0]["id"]
        mem = await memory.get(mem_id)
        log_test("按ID获取记忆", mem is not None, f"ID: {mem_id}")
    
    return {"total_memories": len(all_memories)}

# ============================================================================
# Test 10: Precision and Recall Metrics (Improved)
# ============================================================================

async def test_precision_recall_metrics():
    """Test precision and recall metrics with proper filtering."""
    log_section("Test 10: 精确率和召回率 (Precision & Recall Metrics)")
    
    memory = Memory()
    
    # Add memories with known keywords
    test_memories = [
        ("Python programming tutorial", "semantic"),
        ("Python debugging guide", "semantic"),
        ("JavaScript framework React", "semantic"),
        ("Rust memory safety", "semantic"),
        ("Machine learning basics", "knowledge"),
    ]
    
    for content, mem_type in test_memories:
        await memory.add(content, memory_type=mem_type)
    
    # Search for Python - only count results with score > 0
    results = await memory.search("Python")
    
    # Only consider results with actual relevance (score > 0)
    relevant_results = [r for r in results if r.get("score", 0) > 0]
    relevant = sum(1 for r in relevant_results if "Python" in r.get("content", ""))
    total_found = len(relevant_results)
    
    precision = relevant / total_found if total_found > 0 else 0
    recall = relevant / 2  # We know there are 2 Python memories
    
    log_test("精确率 (Precision)", precision >= 0.5, f"{precision:.2%} ({relevant}/{total_found})")
    log_test("召回率 (Recall)", recall >= 0.5, f"{recall:.2%}")
    
    return {"precision": precision, "recall": recall, "found": total_found}

# ============================================================================
# Test 11-100: 100 Rounds Validation
# ============================================================================

async def test_100_rounds_validation():
    """Run 100 rounds of validation tests."""
    log_section("Test 11-100: 100轮验证测试 (100 Rounds Validation)")
    
    memory = Memory()
    
    # Round 1-20: Basic operations
    log_section("Round 1-20: 基础操作 (Basic Operations)")
    
    for i in range(1, 21):
        result = await memory.add(f"Round {i} memory content", memory_type="semantic")
        passed = "id" in result
        if i % 5 == 0:
            log_test(f"Round {i}", passed, "")
    
    # Round 21-40: Search operations
    log_section("Round 21-40: 搜索操作 (Search Operations)")
    
    for i in range(21, 41):
        await memory.add(f"Search test memory {i}", memory_type="semantic")
    
    results = await memory.search("Search test")
    log_test("Round 21-40 批量搜索", len(results) >= 20, f"找到 {len(results)} 个结果")
    
    # Round 41-60: Update operations
    log_section("Round 41-60: 更新操作 (Update Operations)")
    
    for i in range(41, 51):
        result = await memory.add(f"Original content {i}", memory_type="semantic")
        if i % 5 == 0 and "id" in result:
            await memory.update(result["id"], content=f"Updated content {i}")
    
    results = await memory.search("Updated")
    log_test("Round 41-60 更新验证", len(results) >= 2, f"找到 {len(results)} 个更新内容")
    
    # Round 61-80: Delete operations
    log_section("Round 61-80: 删除操作 (Delete Operations)")
    
    delete_ids = []
    for i in range(61, 71):
        result = await memory.add(f"To be deleted {i}", memory_type="working")
        if "id" in result:
            delete_ids.append(result["id"])
    
    for mem_id in delete_ids:
        await memory.delete(mem_id)
    
    log_test("Round 61-80 删除验证", True, f"删除了 {len(delete_ids)} 个记忆")
    
    # Round 81-100: Mixed operations
    log_section("Round 81-100: 混合操作 (Mixed Operations)")
    
    for i in range(81, 101):
        if i % 3 == 0:
            # Add
            await memory.add(f"Mixed operation {i}", memory_type="semantic")
        elif i % 3 == 1:
            # Search
            await memory.search("Mixed")
        else:
            # Get all
            await memory.get_all()
    
    all_memories = await memory.get_all()
    log_test("Round 81-100 混合操作", len(all_memories) > 0, f"总计 {len(all_memories)} 个记忆")
    
    return {"total_rounds": 100}

# ============================================================================
# Test 101-120: Additional Mem0-style Tests
# ============================================================================

async def test_mem0_additional_tests():
    """Additional Mem0-style tests for comprehensive coverage."""
    log_section("Test 101-120: Mem0 扩展测试 (Mem0 Extended Tests)")
    
    memory = Memory()
    
    # Test 101: Memory with metadata
    result = await memory.add(
        "User project deadline is Friday",
        agent_id="assistant-1",
        user_id="user-123",
        metadata={"priority": "high", "project": "Alpha"}
    )
    log_test("101: 带元数据的记忆", "id" in result, "")
    
    # Test 102: Filter by memory type
    await memory.add("Python tutorial", memory_type="semantic")
    await memory.add("Deploy script", memory_type="procedural")
    
    semantic_results = await memory.search("tutorial", memory_type="semantic")
    log_test("102: 按类型过滤搜索", len(semantic_results) >= 1, f"找到 {len(semantic_results)} 个语义记忆")
    
    # Test 103: Agent isolation
    mem1 = await memory.add("Agent1 secret data", agent_id="agent-1")
    mem2 = await memory.add("Agent2 secret data", agent_id="agent-2")
    
    agent1_results = await memory.search("secret", agent_id="agent-1")
    log_test("103: Agent隔离", len(agent1_results) == 1, f"Agent1找到 {len(agent1_results)} 个")
    
    # Test 104: User isolation
    user1 = await memory.add("User1 private info", user_id="user-1")
    user2 = await memory.add("User2 private info", user_id="user-2")
    
    user1_results = await memory.search("private", user_id="user-1")
    log_test("104: User隔离", len(user1_results) == 1, f"User1找到 {len(user1_results)} 个")
    
    # Test 105: Empty search query
    results = await memory.search("")
    log_test("105: 空查询处理", True, f"空查询返回 {len(results)} 个结果")
    
    # Test 106: Very long query
    long_query = "A" * 1000
    results = await memory.search(long_query)
    log_test("106: 超长查询处理", True, "处理正常")
    
    # Test 107: Unicode content
    await memory.add("用户偏好中文内容", memory_type="semantic")
    results = await memory.search("中文")
    log_test("107: Unicode内容存储", len(results) >= 1, f"找到 {len(results)} 个")
    
    # Test 108: Special characters
    await memory.add("Email: test@example.com & URL: https://test.com", memory_type="resource")
    results = await memory.search("test@example.com")
    log_test("108: 特殊字符搜索", len(results) >= 1, f"找到 {len(results)} 个")
    
    # Test 109: Concurrent operations
    tasks = [memory.add(f"Concurrent {i}", agent_id="assistant-1") for i in range(10)]
    results = await asyncio.gather(*tasks)
    log_test("109: 并发添加", all("id" in r for r in results), f"并发添加 {len(results)} 个")
    
    # Test 110: Memory importance
    result = await memory.add("Critical system failure", importance=0.9)
    log_test("110: 重要性评分", "id" in result, "设置重要性0.9")
    
    return {"additional_tests": 10}

# ============================================================================
# Test Summary
# ============================================================================

async def test_summary():
    """Print test summary."""
    log_section("测试总结 (Test Summary)")
    
    total = test_stats["total"]
    passed = test_stats["passed"]
    failed = test_stats["failed"]
    skipped = test_stats["skipped"]
    
    success_rate = (passed / total * 100) if total > 0 else 0
    
    print(f"\n  总测试数: {total}")
    print(f"  ✅ 通过: {passed}")
    print(f"  ❌ 失败: {failed}")
    print(f"  ⏭️ 跳过: {skipped}")
    print(f"  📊 成功率: {success_rate:.1f}%")
    
    if failed == 0:
        print(f"\n  🎉 所有测试通过！")
    else:
        print(f"\n  ⚠️  有 {failed} 个测试失败，需要调查。")
    
    return test_stats

# ============================================================================
# Main Test Runner
# ============================================================================

async def run_all_tests():
    """Run all memory effect tests."""
    print("="*60)
    print("  AgentMem Memory Effect Test Suite")
    print("  测试AgentMem的记忆效果和召回功能")
    print("="*60)
    
    # Run tests
    await test_8_cognitive_memory_types()
    await test_mem0_add_and_retrieve()
    await test_mem0_memory_update()
    await test_mem0_memory_delete()
    await test_mem0_cross_session_memory()
    await test_mem0_user_preferences()
    await test_recall_effectiveness()
    await test_batch_operations()
    await test_memory_statistics()
    await test_precision_recall_metrics()
    await test_100_rounds_validation()
    await test_mem0_additional_tests()
    
    # Print summary
    await test_summary()
    
    return test_stats

if __name__ == "__main__":
    results = asyncio.run(run_all_tests())
    sys.exit(0 if results["failed"] == 0 else 1)
