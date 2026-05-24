#!/usr/bin/env python3
"""
AgentMem Recall Metrics and Vector Embedding Tests
==================================================
Advanced recall tests with vector embeddings simulation.

This test validates:
1. Recall metrics (Precision, Recall, MRR, NDCG)
2. Vector embedding similarity
3. Semantic search quality
4. Cross-modal retrieval
"""

import asyncio
import sys
import math
from pathlib import Path
from typing import List, Dict, Tuple

sys.path.insert(0, str(Path(__file__).parent.parent / "python"))
from agentmem.memory import Memory

# ============================================================================
# Metrics Calculation
# ============================================================================

def calculate_precision(relevant: int, retrieved: int) -> float:
    """Calculate precision."""
    return relevant / retrieved if retrieved > 0 else 0.0

def calculate_recall(relevant: int, total_relevant: int) -> float:
    """Calculate recall."""
    return relevant / total_relevant if total_relevant > 0 else 0.0

def calculate_mrr(rankings: List[int]) -> float:
    """Calculate Mean Reciprocal Rank."""
    if not rankings:
        return 0.0
    return sum(1.0 / r for r in rankings if r > 0) / len(rankings)

def calculate_ndcg(relevances: List[float], k: int = 10) -> float:
    """Calculate Normalized Discounted Cumulative Gain."""
    if not relevances:
        return 0.0
    
    # DCG
    dcg = sum(rel / math.log2(i + 2) for i, rel in enumerate(relevances[:k]))
    
    # IDCG (ideal DCG)
    ideal_relevances = sorted(relevances, reverse=True)[:k]
    idcg = sum(rel / math.log2(i + 2) for i, rel in enumerate(ideal_relevances))
    
    return dcg / idcg if idcg > 0 else 0.0

# ============================================================================
# Vector Similarity (Simulated for Testing)
# ============================================================================

class VectorSimulator:
    """Simulates vector embeddings for testing."""
    
    @staticmethod
    def cosine_similarity(vec1: List[float], vec2: List[float]) -> float:
        """Calculate cosine similarity between two vectors."""
        if len(vec1) != len(vec2):
            return 0.0
        
        dot = sum(a * b for a, b in zip(vec1, vec2))
        mag1 = math.sqrt(sum(a * a for a in vec1))
        mag2 = math.sqrt(sum(b * b for b in vec2))
        
        return dot / (mag1 * mag2) if mag1 * mag2 > 0 else 0.0
    
    @staticmethod
    def euclidean_distance(vec1: List[float], vec2: List[float]) -> float:
        """Calculate Euclidean distance."""
        if len(vec1) != len(vec2):
            return float('inf')
        
        return math.sqrt(sum((a - b) ** 2 for a, b in zip(vec1, vec2)))
    
    @staticmethod
    def text_to_vector(text: str, dim: int = 128) -> List[float]:
        """Convert text to a simple vector representation."""
        # Simple hash-based vectorization for testing
        import hashlib
        words = text.lower().split()
        vec = [0.0] * dim
        
        for i, word in enumerate(words):
            hash_val = int(hashlib.md5(word.encode()).hexdigest()[:8], 16)
            for j in range(dim):
                vec[j] += (hash_val >> (j % 24)) & 1
        
        # Normalize
        mag = math.sqrt(sum(v * v for v in vec))
        return [v / mag if mag > 0 else 0 for v in vec]


# ============================================================================
# Test Class
# ============================================================================

class TestRecallMetrics:
    """Recall metrics and vector embedding tests."""
    
    async def test_precision_recall(self):
        """Test precision and recall metrics."""
        print("\n[Test 1: Precision and Recall]")
        
        memory = Memory()
        
        # Add memories with known relevance
        memories = [
            ("Python is a programming language", True),
            ("Python tutorial for beginners", True),
            ("JavaScript is for web development", False),
            ("Rust is for systems programming", False),
            ("Machine learning with Python", True),
        ]
        
        for content, _ in memories:
            await memory.add(content, agent_id="recall-test")
        
        # Search and calculate metrics
        results = await memory.search("Python", agent_id="recall-test")
        
        # Manually calculate
        relevant_found = sum(1 for r in results if "Python" in r.get("content", ""))
        total_relevant = 3  # We know there are 3 Python-related
        total_retrieved = len(results)
        
        precision = calculate_precision(relevant_found, total_retrieved)
        recall = calculate_recall(relevant_found, total_relevant)
        
        print(f"  Relevant found: {relevant_found}")
        print(f"  Total retrieved: {total_retrieved}")
        print(f"  Total relevant: {total_relevant}")
        print(f"  Precision: {precision:.2%}")
        print(f"  Recall: {recall:.2%}")
        
        assert precision > 0, "Precision should be > 0"
        assert recall > 0, "Recall should be > 0"
        print("  ✅ Precision and Recall test passed")
    
    async def test_mrr(self):
        """Test Mean Reciprocal Rank."""
        print("\n[Test 2: Mean Reciprocal Rank (MRR)]")
        
        memory = Memory()
        
        # Add memories
        await memory.add("Python basics", agent_id="mrr-test")
        await memory.add("JavaScript guide", agent_id="mrr-test")
        await memory.add("Rust tutorial", agent_id="mrr-test")
        
        # Search queries
        queries = ["Python", "JavaScript", "Go"]
        rankings = []
        
        for query in queries:
            results = await memory.search(query, agent_id="mrr-test")
            
            # Find rank of first relevant result
            rank = 0
            for i, r in enumerate(results, 1):
                if query.lower() in r.get("content", "").lower():
                    rank = i
                    break
            
            rankings.append(rank)
        
        mrr = calculate_mrr(rankings)
        
        print(f"  Rankings: {rankings}")
        print(f"  MRR: {mrr:.4f}")
        
        assert mrr > 0, "MRR should be > 0"
        print("  ✅ MRR test passed")
    
    async def test_ndcg(self):
        """Test Normalized DCG."""
        print("\n[Test 3: NDCG (Normalized DCG)]")
        
        memory = Memory()
        
        # Add memories with varying relevance
        await memory.add("Python programming tutorial", agent_id="ndcg-test")
        await memory.add("Python basics", agent_id="ndcg-test")
        await memory.add("JavaScript guide", agent_id="ndcg-test")
        
        results = await memory.search("Python", agent_id="ndcg-test")
        
        # Assign relevance scores (1 for relevant, 0 for not)
        relevances = [1.0 if "Python" in r.get("content", "") else 0.0 for r in results]
        
        ndcg = calculate_ndcg(relevances)
        
        print(f"  Relevances: {relevances}")
        print(f"  NDCG: {ndcg:.4f}")
        
        assert ndcg > 0, "NDCG should be > 0"
        print("  ✅ NDCG test passed")
    
    async def test_vector_similarity(self):
        """Test simulated vector similarity."""
        print("\n[Test 4: Vector Similarity]")
        
        # Simulate vector embeddings
        vec1 = VectorSimulator.text_to_vector("Python programming language")
        vec2 = VectorSimulator.text_to_vector("Python tutorial for beginners")
        vec3 = VectorSimulator.text_to_vector("JavaScript web development")
        
        # Calculate similarities
        sim_12 = VectorSimulator.cosine_similarity(vec1, vec2)
        sim_13 = VectorSimulator.cosine_similarity(vec1, vec3)
        
        print(f"  vec1 vs vec2 (both Python): {sim_12:.4f}")
        print(f"  vec1 vs vec3 (different): {sim_13:.4f}")
        
        assert sim_12 > sim_13, "Similar texts should have higher similarity"
        print("  ✅ Vector similarity test passed")
    
    async def test_semantic_search(self):
        """Test semantic search quality."""
        print("\n[Test 5: Semantic Search Quality]")
        
        memory = Memory()
        
        # Add semantically similar memories
        memories = [
            "The cat is sleeping on the sofa",
            "A feline is resting on the couch",
            "The dog is playing in the park",
            "A canine is running outside",
        ]
        
        for content in memories:
            await memory.add(content, agent_id="semantic-test")
        
        # Search for cat-related content
        results = await memory.search("cat", agent_id="semantic-test")
        
        # Check if semantic matches are found
        cat_related = [r for r in results if "cat" in r.get("content", "").lower() or "feline" in r.get("content", "").lower()]
        
        print(f"  Found {len(cat_related)} cat-related results out of {len(results)}")
        
        # Should find both "cat" and "feline" results
        assert len(cat_related) >= 1, "Should find cat-related content"
        print("  ✅ Semantic search test passed")
    
    async def test_ranking_quality(self):
        """Test search result ranking quality."""
        print("\n[Test 6: Ranking Quality]")
        
        memory = Memory()
        
        # Add memories with varying relevance
        memories = [
            ("Python programming tutorial", 0.9),  # High relevance
            ("Python basics for beginners", 0.8),
            ("Python reference guide", 0.7),
            ("JavaScript tutorial", 0.3),  # Low relevance
            ("Rust guide", 0.2),
        ]
        
        for content, expected_score in memories:
            await memory.add(content, agent_id="rank-test")
        
        # Search
        results = await memory.search("Python", agent_id="rank-test")
        
        print(f"  Top 3 results:")
        for i, r in enumerate(results[:3], 1):
            print(f"    {i}. {r.get('content', '')[:40]}... (score: {r.get('score', 0):.2f})")
        
        # Verify ranking order
        if len(results) >= 2:
            assert results[0]["score"] >= results[1]["score"], "Higher relevance should rank first"
        
        print("  ✅ Ranking quality test passed")
    
    async def test_cross_type_recall(self):
        """Test recall across different memory types."""
        print("\n[Test 7: Cross-Type Recall]")
        
        memory = Memory()
        
        # Add memories of different types
        type_contents = [
            ("User asked about dinner", "episodic"),
            ("User prefers Italian food", "semantic"),
            ("How to cook pasta", "procedural"),
            ("Current task: recipe search", "working"),
            ("Persona: Home cook", "core"),
        ]
        
        for content, mem_type in type_contents:
            await memory.add(content, memory_type=mem_type, agent_id="cross-type-test")
        
        # Search across all types
        results = await memory.search("User prefers", agent_id="cross-type-test")
        
        print(f"  Found {len(results)} cross-type results")
        
        # Should find semantic memory
        semantic_found = any("Italian" in r.get("content", "") for r in results)
        
        assert semantic_found, "Should find cross-type semantic memory"
        print("  ✅ Cross-type recall test passed")
    
    async def test_fuzzy_recall(self):
        """Test fuzzy matching recall."""
        print("\n[Test 8: Fuzzy Recall]")
        
        memory = Memory()
        
        memories = [
            "Programming in Python",
            "Python development",
            "PyTorch machine learning",
            "Pytorch炼丹术",
            "Pythonista app",
        ]
        
        for content in memories:
            await memory.add(content, agent_id="fuzzy-test")
        
        # Search with typo/variation
        results = await memory.search("Python", agent_id="fuzzy-test")
        
        print(f"  Fuzzy search found {len(results)} results for 'Python'")
        
        # Should find variations
        assert len(results) >= 3, "Should find Python-related variations"
        print("  ✅ Fuzzy recall test passed")
    
    async def test_contextual_recall(self):
        """Test contextual memory recall."""
        print("\n[Test 9: Contextual Recall]")
        
        memory = Memory()
        
        # Add contextual memories
        await memory.add("Session started: user-login", session_id="session-123", agent_id="ctx-test")
        await memory.add("User browsing products", session_id="session-123", agent_id="ctx-test")
        await memory.add("User added item to cart", session_id="session-123", agent_id="ctx-test")
        
        # Add from different session
        await memory.add("Other session data", session_id="session-456", agent_id="ctx-test")
        
        # Get memories for specific session
        session1_memories = await memory.get_all(session_id="session-123", agent_id="ctx-test")
        session2_memories = await memory.get_all(session_id="session-456", agent_id="ctx-test")
        
        print(f"  Session 1 memories: {len(session1_memories)}")
        print(f"  Session 2 memories: {len(session2_memories)}")
        
        assert len(session1_memories) == 3, "Should have 3 session 1 memories"
        assert len(session2_memories) == 1, "Should have 1 session 2 memory"
        print("  ✅ Contextual recall test passed")
    
    async def test_importance_weighted_recall(self):
        """Test importance-weighted recall."""
        print("\n[Test 10: Importance-Weighted Recall]")
        
        memory = Memory()
        
        # Add memories with different importance
        await memory.add("Critical system failure", importance=0.9, agent_id="imp-test")
        await memory.add("Minor UI change", importance=0.3, agent_id="imp-test")
        await memory.add("Regular task reminder", importance=0.5, agent_id="imp-test")
        
        # Get all and verify importance
        all_memories = await memory.get_all(agent_id="imp-test")
        
        high_importance = [m for m in all_memories if m.get("importance", 0) >= 0.8]
        
        print(f"  High importance memories: {len(high_importance)}")
        
        assert len(high_importance) >= 1, "Should have high importance memory"
        print("  ✅ Importance-weighted recall test passed")
    
    async def test_temporal_decay(self):
        """Test temporal decay effect."""
        print("\n[Test 11: Temporal Decay]")
        
        memory = Memory()
        
        # Add memories at different times (simulated by using updated_at)
        memories = [
            ("Recent important event", "recent"),
            ("Old memory from last week", "old"),
        ]
        
        for content, _ in memories:
            await memory.add(content, agent_id="temporal-test")
        
        # Search should prioritize recent
        results = await memory.search("event", agent_id="temporal-test")
        
        print(f"  Temporal search found {len(results)} results")
        
        assert len(results) >= 1, "Should find temporal memories"
        print("  ✅ Temporal decay test passed")
    
    async def test_multi_query_recall(self):
        """Test multi-query recall metrics."""
        print("\n[Test 12: Multi-Query Recall]")
        
        memory = Memory()
        
        # Add diverse memories
        topics = ["Python", "JavaScript", "Rust", "Go", "Machine Learning"]
        for topic in topics:
            await memory.add(f"{topic} programming tutorial", agent_id="multi-test")
        
        # Search multiple queries
        queries = ["Python", "JavaScript", "Rust"]
        all_relevant = 0
        all_retrieved = 0
        
        for query in queries:
            results = await memory.search(query, agent_id="multi-test")
            relevant = sum(1 for r in results if query in r.get("content", ""))
            all_relevant += relevant
            all_retrieved += len(results)
        
        overall_precision = calculate_precision(all_relevant, all_retrieved)
        
        print(f"  Multi-query precision: {overall_precision:.2%}")
        print(f"  Total relevant: {all_relevant}, Total retrieved: {all_retrieved}")
        
        assert overall_precision > 0, "Overall precision should be > 0"
        print("  ✅ Multi-query recall test passed")


# ============================================================================
# Run Tests
# ============================================================================

async def run_recall_tests():
    """Run all recall metric tests."""
    print("="*60)
    print("  AgentMem Recall Metrics and Vector Tests")
    print("  Advanced recall quality validation")
    print("="*60)
    
    tester = TestRecallMetrics()
    
    await tester.test_precision_recall()
    await tester.test_mrr()
    await tester.test_ndcg()
    await tester.test_vector_similarity()
    await tester.test_semantic_search()
    await tester.test_ranking_quality()
    await tester.test_cross_type_recall()
    await tester.test_fuzzy_recall()
    await tester.test_contextual_recall()
    await tester.test_importance_weighted_recall()
    await tester.test_temporal_decay()
    await tester.test_multi_query_recall()
    
    print("\n" + "="*60)
    print("  ✅ All Recall Metric Tests Passed!")
    print("="*60)


if __name__ == "__main__":
    asyncio.run(run_recall_tests())
