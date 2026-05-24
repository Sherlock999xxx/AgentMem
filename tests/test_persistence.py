#!/usr/bin/env python3
"""
AgentMem Persistence Tests
=========================
Test memory persistence and storage capabilities.

This test validates:
1. In-memory persistence within session
2. JSON file storage
3. Import/Export functionality
4. Data integrity
"""

import asyncio
import sys
import os
import json
import tempfile
from pathlib import Path
from typing import List, Dict

sys.path.insert(0, str(Path(__file__).parent.parent / "python"))
from agentmem.memory import Memory

# ============================================================================
# Test Class
# ============================================================================

class TestPersistence:
    """Persistence and storage tests."""
    
    async def test_in_memory_persistence(self):
        """Test in-memory persistence within session."""
        print("\n[Test 1: In-Memory Persistence]")
        
        memory = Memory()
        
        # Add memories
        await memory.add("Memory 1", agent_id="persist-test")
        await memory.add("Memory 2", agent_id="persist-test")
        
        # Verify they persist
        all_memories = await memory.get_all(agent_id="persist-test")
        
        print(f"  Added 2 memories, retrieved: {len(all_memories)}")
        assert len(all_memories) >= 2, "Memories should persist in memory"
        
        # Add more
        await memory.add("Memory 3", agent_id="persist-test")
        all_memories = await memory.get_all(agent_id="persist-test")
        
        print(f"  Added 3rd memory, total: {len(all_memories)}")
        assert len(all_memories) >= 3, "New memories should persist"
        
        print("  ✅ In-memory persistence test passed")
    
    async def test_json_export_import(self):
        """Test JSON export and import."""
        print("\n[Test 2: JSON Export/Import]")
        
        memory = Memory()
        
        # Add test memories
        memories = [
            {"content": "Export test 1", "agent_id": "export-test"},
            {"content": "Export test 2", "agent_id": "export-test"},
        ]
        
        for mem in memories:
            await memory.add(mem["content"], agent_id=mem["agent_id"])
        
        # Export to JSON
        all_memories = await memory.get_all(agent_id="export-test")
        
        export_data = {
            "version": "1.0",
            "exported_at": str(asyncio.get_event_loop().time()),
            "memories": [mem.to_dict() if hasattr(mem, 'to_dict') else mem for mem in all_memories]
        }
        
        # Create temp file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
            json.dump(export_data, f)
            temp_path = f.name
        
        try:
            # Verify file exists
            assert os.path.exists(temp_path), "Export file should exist"
            
            # Read and verify
            with open(temp_path, 'r') as f:
                loaded_data = json.load(f)
            
            print(f"  Exported {len(loaded_data['memories'])} memories to JSON")
            assert len(loaded_data['memories']) >= 2, "Should export memories"
            
            print("  ✅ JSON export test passed")
        finally:
            os.unlink(temp_path)
    
    async def test_data_integrity(self):
        """Test data integrity after operations."""
        print("\n[Test 3: Data Integrity]")
        
        memory = Memory()
        
        # Add memory with all fields
        result = await memory.add(
            "Data integrity test",
            agent_id="integrity-test",
            user_id="user-123",
            session_id="session-456",
            memory_type="semantic",
            importance=0.8,
            metadata={"key": "value", "nested": {"a": 1}}
        )
        
        # Retrieve and verify
        mem_id = result["id"]
        retrieved = await memory.get(mem_id)
        
        print(f"  Stored metadata: {retrieved.get('metadata', {})}")
        assert retrieved["metadata"]["key"] == "value", "Metadata should be preserved"
        assert retrieved["metadata"]["nested"]["a"] == 1, "Nested metadata should be preserved"
        
        # Update and verify
        updated = await memory.update(mem_id, metadata={"new": "data"})
        print(f"  Updated metadata: {updated.get('metadata', {})}")
        assert updated["metadata"]["new"] == "data", "Update should be applied"
        
        print("  ✅ Data integrity test passed")
    
    async def test_bulk_export(self):
        """Test bulk memory export."""
        print("\n[Test 4: Bulk Export]")
        
        memory = Memory()
        
        # Add many memories
        for i in range(20):
            await memory.add(f"Bulk memory {i}", agent_id="bulk-test")
        
        # Export all
        all_memories = await memory.get_all(agent_id="bulk-test")
        
        export_data = {
            "count": len(all_memories),
            "memories": all_memories
        }
        
        print(f"  Bulk exported {len(all_memories)} memories")
        assert len(all_memories) >= 20, "Should export all memories"
        
        print("  ✅ Bulk export test passed")
    
    async def test_concurrent_persistence(self):
        """Test persistence under concurrent operations."""
        print("\n[Test 5: Concurrent Persistence]")
        
        memory = Memory()
        
        # Concurrent adds
        tasks = [memory.add(f"Concurrent {i}", agent_id="concurrent-test") for i in range(10)]
        results = await asyncio.gather(*tasks)
        
        # Verify all persisted
        all_memories = await memory.get_all(agent_id="concurrent-test")
        
        print(f"  Concurrent added {len(results)} memories, persisted: {len(all_memories)}")
        assert len(all_memories) >= 10, "All concurrent memories should persist"
        
        # Concurrent updates
        mem_ids = [m["id"] for m in all_memories[:5]]
        update_tasks = [memory.update(mem_id, content=f"Updated {i}") for i, mem_id in enumerate(mem_ids)]
        await asyncio.gather(*update_tasks)
        
        # Verify updates persisted
        updated_memories = await memory.get_all(agent_id="concurrent-test")
        updated_count = sum(1 for m in updated_memories if "Updated" in m.get("content", ""))
        
        print(f"  Updated {updated_count} memories persistently")
        assert updated_count >= 5, "Updates should persist"
        
        print("  ✅ Concurrent persistence test passed")
    
    async def test_delete_persistence(self):
        """Test that deletion is persistent."""
        print("\n[Test 6: Delete Persistence]")
        
        memory = Memory()
        
        # Add and delete
        result = await memory.add("To be deleted", agent_id="delete-test")
        mem_id = result["id"]
        
        # Delete
        await memory.delete(mem_id)
        
        # Verify deleted
        all_memories = await memory.get_all(agent_id="delete-test")
        deleted_mem = await memory.get(mem_id)
        
        print(f"  After delete: {len(all_memories)} memories, retrieved: {deleted_mem}")
        assert len(all_memories) == 0, "Memory should be gone"
        assert deleted_mem is None, "Deleted memory should not be retrievable"
        
        print("  ✅ Delete persistence test passed")
    
    async def test_clear_persistence(self):
        """Test that clear is persistent."""
        print("\n[Test 7: Clear Persistence]")
        
        memory = Memory()
        
        # Add to agent
        for i in range(5):
            await memory.add(f"Clear test {i}", agent_id="clear-test")
        
        # Add to other agent (should not be cleared)
        await memory.add("Other agent", agent_id="other-agent")
        
        # Clear
        result = await memory.clear(agent_id="clear-test")
        
        # Verify
        cleared_memories = await memory.get_all(agent_id="clear-test")
        other_memories = await memory.get_all(agent_id="other-agent")
        
        print(f"  Cleared {result['count']} memories, remaining: {len(cleared_memories)}")
        print(f"  Other agent still has: {len(other_memories)}")
        
        assert len(cleared_memories) == 0, "Agent memories should be cleared"
        assert len(other_memories) >= 1, "Other agents should not be affected"
        
        print("  ✅ Clear persistence test passed")
    
    async def test_large_data_persistence(self):
        """Test persistence with large data."""
        print("\n[Test 8: Large Data Persistence]")
        
        memory = Memory()
        
        # Add large memory
        large_content = "X" * 10000
        result = await memory.add(large_content, agent_id="large-test")
        
        # Verify
        retrieved = await memory.get(result["id"])
        
        print(f"  Stored {len(large_content)} chars, retrieved: {len(retrieved['content'])} chars")
        assert len(retrieved["content"]) == len(large_content), "Large content should be preserved"
        
        print("  ✅ Large data persistence test passed")
    
    async def test_special_chars_persistence(self):
        """Test persistence of special characters."""
        print("\n[Test 9: Special Characters Persistence]")
        
        memory = Memory()
        
        # Add with special characters
        special_content = 'Email: test@example.com | URL: https://test.com | Path: /usr/local/bin | Unicode: 中文'
        result = await memory.add(special_content, agent_id="special-test")
        
        # Verify
        retrieved = await memory.get(result["id"])
        
        print(f"  Special content length: {len(special_content)}")
        assert retrieved["content"] == special_content, "Special chars should be preserved"
        
        # Search with special chars
        results = await memory.search("test@example.com", agent_id="special-test")
        
        print(f"  Found {len(results)} results for special char search")
        assert len(results) >= 1, "Should find special char content"
        
        print("  ✅ Special characters persistence test passed")
    
    async def test_idempotent_operations(self):
        """Test idempotent operations don't corrupt data."""
        print("\n[Test 10: Idempotent Operations]")
        
        memory = Memory()
        
        # Add memory
        result = await memory.add("Idempotent test", agent_id="idempotent-test")
        mem_id = result["id"]
        
        # Update same memory multiple times
        for i in range(5):
            await memory.update(mem_id, content=f"Update {i}")
        
        # Try to delete non-existent (should handle gracefully)
        try:
            await memory.delete("non-existent-id")
            print("  ⚠️ Delete non-existent did not raise error")
        except ValueError:
            print("  ✅ Delete non-existent raises ValueError as expected")
        
        # Verify final state
        all_memories = await memory.get_all(agent_id="idempotent-test")
        
        print(f"  Final memory count: {len(all_memories)}")
        assert len(all_memories) == 1, "Memory should still exist"
        
        print("  ✅ Idempotent operations test passed")


# ============================================================================
# Run Tests
# ============================================================================

async def run_persistence_tests():
    """Run all persistence tests."""
    print("="*60)
    print("  AgentMem Persistence Tests")
    print("  Memory storage and data integrity validation")
    print("="*60)
    
    tester = TestPersistence()
    
    await tester.test_in_memory_persistence()
    await tester.test_json_export_import()
    await tester.test_data_integrity()
    await tester.test_bulk_export()
    await tester.test_concurrent_persistence()
    await tester.test_delete_persistence()
    await tester.test_clear_persistence()
    await tester.test_large_data_persistence()
    await tester.test_special_chars_persistence()
    await tester.test_idempotent_operations()
    
    print("\n" + "="*60)
    print("  ✅ All Persistence Tests Passed!")
    print("="*60)


if __name__ == "__main__":
    asyncio.run(run_persistence_tests())
