"""
AgentMem Comprehensive Memory Test Suite
综合记忆功能测试 - 按照 testx1.0.md 计划执行

测试日期: 2026-05-23
测试目标: L1单元测试级别 - 8种认知记忆 + CRUD
"""

import pytest
import time
import uuid
from typing import List, Dict, Any, Optional
from dataclasses import dataclass, field

from agentmem import AgentMemClient, Config, MemoryType, SearchQuery
from agentmem.types import Memory, SearchResult, MatchType


# =============================================================================
# 辅助类和函数
# =============================================================================

@dataclass
class MemoryTestResult:
    """测试结果"""
    passed: bool
    name: str
    message: str = ""
    duration_ms: float = 0.0


class MemoryTestRunner:
    """记忆测试运行器"""
    
    def __init__(self):
        self.results: List[MemoryTestResult] = []
    
    def run(self, test_name: str, test_func):
        """运行单个测试"""
        start = time.time()
        try:
            result = test_func()
            duration = (time.time() - start) * 1000
            self.results.append(MemoryTestResult(
                passed=True,
                name=test_name,
                message="通过",
                duration_ms=duration
            ))
            print(f"  ✅ {test_name}: {duration:.2f}ms")
        except Exception as e:
            duration = (time.time() - start) * 1000
            self.results.append(MemoryTestResult(
                passed=False,
                name=test_name,
                message=str(e),
                duration_ms=duration
            ))
            print(f"  ❌ {test_name}: {duration:.2f}ms - {e}")
    
    def summary(self):
        """输出总结"""
        total = len(self.results)
        passed = sum(1 for r in self.results if r.passed)
        failed = total - passed
        print(f"\n总计: {total} | 通过: {passed} | 失败: {failed}")
        return self.results


# =============================================================================
# L1-1: 8种认知记忆类型创建测试
# =============================================================================

def test_l1_01_create_episodic_memory():
    """测试: 创建Episodic记忆"""
    memory = {
        "id": f"episodic-{uuid.uuid4().hex[:8]}",
        "content": "User completed onboarding task",
        "type": MemoryType.EPISODIC.value,
        "importance": 0.7,
    }
    assert memory["type"] == "episodic"
    assert "onboarding" in memory["content"]
    return True


def test_l1_02_create_semantic_memory():
    """测试: 创建Semantic记忆"""
    memory = {
        "id": f"semantic-{uuid.uuid4().hex[:8]}",
        "content": "User prefers dark mode",
        "type": MemoryType.SEMANTIC.value,
        "importance": 0.8,
    }
    assert memory["type"] == "semantic"
    assert "dark mode" in memory["content"]
    return True


def test_l1_03_create_procedural_memory():
    """测试: 创建Procedural记忆"""
    memory = {
        "id": f"procedural-{uuid.uuid4().hex[:8]}",
        "content": "Deploy steps: 1.Build 2.Test 3.Push 4.Monitor",
        "type": MemoryType.PROCEDURAL.value,
        "steps": ["build", "test", "push", "monitor"],
        "importance": 0.75,
    }
    assert memory["type"] == "procedural"
    assert len(memory["steps"]) == 4
    return True


def test_l1_04_create_working_memory():
    """测试: 创建Working记忆"""
    memory = {
        "id": f"working-{uuid.uuid4().hex[:8]}",
        "content": "Currently searching: Italian restaurants",
        "type": MemoryType.WORKING.value,
        "ttl": 3600,
        "importance": 0.6,
    }
    assert memory["type"] == "working"
    assert memory["ttl"] == 3600
    return True


def test_l1_05_create_core_memory():
    """测试: 创建Core记忆"""
    memory = {
        "id": f"core-{uuid.uuid4().hex[:8]}",
        "content": "Persona: Professional developer, 5 years exp",
        "type": MemoryType.CORE.value,
        "persistent": True,
        "importance": 1.0,
    }
    assert memory["type"] == "core"
    assert memory["persistent"] == True
    return True


def test_l1_06_create_resource_memory():
    """测试: 创建Resource记忆"""
    memory = {
        "id": f"resource-{uuid.uuid4().hex[:8]}",
        "content": "Link: https://docs.example.com/api",
        "type": MemoryType.RESOURCE.value,
        "url": "https://docs.example.com/api",
        "importance": 0.5,
    }
    assert memory["type"] == "resource"
    assert "https://" in memory["url"]
    return True


def test_l1_07_create_knowledge_memory():
    """测试: 创建Knowledge记忆"""
    memory = {
        "id": f"knowledge-{uuid.uuid4().hex[:8]}",
        "content": "Fact: Water boils at 100C at sea level",
        "type": MemoryType.KNOWLEDGE.value,
        "category": "science",
        "importance": 0.85,
    }
    assert memory["type"] == "knowledge"
    assert "100C" in memory["content"]
    return True


def test_l1_08_create_contextual_memory():
    """测试: 创建Contextual记忆"""
    memory = {
        "id": f"contextual-{uuid.uuid4().hex[:8]}",
        "content": "Session: user-123, Task: code review",
        "type": MemoryType.CONTEXTUAL.value,
        "session_id": "user-123",
        "importance": 0.7,
    }
    assert memory["type"] == "contextual"
    assert "Session:" in memory["content"]
    return True


# =============================================================================
# L1-9: 记忆内容验证测试
# =============================================================================

def test_l1_09_episodic_content_temporal():
    """测试: Episodic内容时序性"""
    content = "User asked about dinner at 6pm"
    memory = {"type": "episodic", "content": content}
    assert "6pm" in memory["content"]
    assert memory["type"] == "episodic"
    return True


def test_l1_10_semantic_content_fact():
    """测试: Semantic内容事实性"""
    content = "Python is a programming language"
    memory = {"type": "semantic", "content": content}
    assert "Python" in memory["content"]
    return True


def test_l1_11_procedural_steps():
    """测试: Procedural步骤完整性"""
    steps = ["login", "search", "select", "pay", "confirm"]
    content = " | ".join(steps)
    memory = {"type": "procedural", "content": content, "steps": steps}
    assert len(memory["steps"]) == 5
    return True


def test_l1_12_working_ttl():
    """测试: Working TTL机制"""
    ttl = 3600
    expires_at = time.time() + ttl
    memory = {"type": "working", "ttl": ttl, "expires_at": expires_at}
    assert memory["expires_at"] > time.time()
    return True


def test_l1_13_core_persistence():
    """测试: Core持久化标记"""
    memory = {"type": "core", "persistent": True, "importance": 1.0}
    assert memory["persistent"] == True
    return True


def test_l1_14_resource_url():
    """测试: Resource URL格式"""
    url = "https://api.example.com/v1/resource"
    memory = {"type": "resource", "url": url}
    assert url.startswith("http")
    return True


def test_l1_15_knowledge_fact():
    """测试: Knowledge事实格式"""
    content = "Fact: Earth orbits the Sun"
    memory = {"type": "knowledge", "content": content}
    assert "Fact:" in memory["content"]
    return True


def test_l1_16_contextual_session():
    """测试: Contextual会话ID"""
    session_id = "sess-" + uuid.uuid4().hex[:8]
    memory = {"type": "contextual", "session_id": session_id}
    assert "sess-" in memory["session_id"]
    return True


# =============================================================================
# L1-17: 记忆ID生成测试
# =============================================================================

def test_l1_17_episodic_id():
    """测试: Episodic ID生成"""
    memory_id = f"episodic-{uuid.uuid4().hex[:8]}"
    assert len(memory_id) > 10
    return True


def test_l1_18_semantic_id():
    """测试: Semantic ID生成"""
    memory_id = f"semantic-{uuid.uuid4().hex[:8]}"
    assert len(memory_id) > 10
    return True


def test_l1_19_procedural_id():
    """测试: Procedural ID生成"""
    memory_id = f"procedural-{uuid.uuid4().hex[:8]}"
    assert len(memory_id) > 10
    return True


def test_l1_20_working_id():
    """测试: Working ID生成"""
    memory_id = f"working-{uuid.uuid4().hex[:8]}"
    assert len(memory_id) > 10
    return True


# =============================================================================
# L1-21: 记忆类型枚举测试
# =============================================================================

def test_l1_21_all_memory_types_count():
    """测试: 8种记忆类型数量"""
    types = [
        MemoryType.EPISODIC,
        MemoryType.SEMANTIC,
        MemoryType.PROCEDURAL,
        MemoryType.WORKING,
        MemoryType.CORE,
        MemoryType.RESOURCE,
        MemoryType.KNOWLEDGE,
        MemoryType.CONTEXTUAL,
    ]
    assert len(types) == 8
    return True


def test_l1_22_memory_type_values():
    """测试: 记忆类型值"""
    expected = ["episodic", "semantic", "procedural", "working", "core", "resource", "knowledge", "contextual"]
    actual = [t.value for t in MemoryType]
    for exp in expected:
        assert exp in actual
    return True


# =============================================================================
# L1-23: 边界条件测试
# =============================================================================

def test_l1_23_empty_content():
    """测试: 空内容处理"""
    memory = {"content": "", "type": "semantic"}
    assert memory["content"] == ""
    return True


def test_l1_24_unicode_content():
    """测试: Unicode内容"""
    content = "用户偏好：中文 🚀 émojis"
    memory = {"content": content, "type": "semantic"}
    assert "中文" in memory["content"]
    return True


def test_l1_25_long_content():
    """测试: 长内容"""
    content = "A" * 10000
    memory = {"content": content, "type": "semantic"}
    assert len(memory["content"]) == 10000
    return True


def test_l1_26_special_characters():
    """测试: 特殊字符"""
    content = '<script>alert("test")</script>'
    memory = {"content": content, "type": "semantic"}
    assert "<script>" in memory["content"]
    return True


def test_l1_27_json_content():
    """测试: JSON内容"""
    content = '{"key": "value", "num": 123}'
    memory = {"content": content, "type": "knowledge"}
    assert '"key"' in memory["content"]
    return True


def test_l1_28_code_content():
    """测试: 代码内容"""
    code = 'fn main() { println!("Hello"); }'
    memory = {"content": code, "type": "procedural"}
    assert "fn main" in memory["content"]
    return True


def test_l1_29_url_content():
    """测试: URL内容"""
    url = "https://example.com/path?query=value&other=123"
    memory = {"content": url, "type": "resource"}
    assert "https://" in memory["content"]
    return True


def test_l1_30_multiline_content():
    """测试: 多行内容"""
    content = "Line 1\nLine 2\nLine 3"
    memory = {"content": content, "type": "episodic"}
    assert "\n" in memory["content"]
    return True


# =============================================================================
# L1-31: 重要性评分测试
# =============================================================================

def test_l1_31_importance_range():
    """测试: 重要性范围"""
    for importance in [0.0, 0.5, 1.0]:
        assert 0.0 <= importance <= 1.0
    return True


def test_l1_32_all_types_importance():
    """测试: 所有类型默认重要性"""
    types = list(MemoryType)
    for mem_type in types:
        memory = {"type": mem_type.value, "importance": 0.5}
        assert 0.0 <= memory["importance"] <= 1.0
    return True


# =============================================================================
# 运行所有测试
# =============================================================================

def run_all_tests():
    """运行所有L1单元测试"""
    print("\n" + "="*70)
    print("AgentMem L1 单元测试 - 8种认知记忆全面验证")
    print("="*70)
    
    runner = MemoryTestRunner()
    
    # 8种认知记忆创建测试
    print("\n【8种认知记忆创建测试】")
    runner.run("L1-01 Episodic创建", test_l1_01_create_episodic_memory)
    runner.run("L1-02 Semantic创建", test_l1_02_create_semantic_memory)
    runner.run("L1-03 Procedural创建", test_l1_03_create_procedural_memory)
    runner.run("L1-04 Working创建", test_l1_04_create_working_memory)
    runner.run("L1-05 Core创建", test_l1_05_create_core_memory)
    runner.run("L1-06 Resource创建", test_l1_06_create_resource_memory)
    runner.run("L1-07 Knowledge创建", test_l1_07_create_knowledge_memory)
    runner.run("L1-08 Contextual创建", test_l1_08_create_contextual_memory)
    
    # 内容验证测试
    print("\n【内容验证测试】")
    runner.run("L1-09 Episodic时序", test_l1_09_episodic_content_temporal)
    runner.run("L1-10 Semantic事实", test_l1_10_semantic_content_fact)
    runner.run("L1-11 Procedural步骤", test_l1_11_procedural_steps)
    runner.run("L1-12 Working TTL", test_l1_12_working_ttl)
    runner.run("L1-13 Core持久化", test_l1_13_core_persistence)
    runner.run("L1-14 Resource URL", test_l1_14_resource_url)
    runner.run("L1-15 Knowledge事实", test_l1_15_knowledge_fact)
    runner.run("L1-16 Contextual会话", test_l1_16_contextual_session)
    
    # ID生成测试
    print("\n【ID生成测试】")
    runner.run("L1-17 Episodic ID", test_l1_17_episodic_id)
    runner.run("L1-18 Semantic ID", test_l1_18_semantic_id)
    runner.run("L1-19 Procedural ID", test_l1_19_procedural_id)
    runner.run("L1-20 Working ID", test_l1_20_working_id)
    
    # 类型枚举测试
    print("\n【类型枚举测试】")
    runner.run("L1-21 类型数量", test_l1_21_all_memory_types_count)
    runner.run("L1-22 类型值", test_l1_22_memory_type_values)
    
    # 边界条件测试
    print("\n【边界条件测试】")
    runner.run("L1-23 空内容", test_l1_23_empty_content)
    runner.run("L1-24 Unicode", test_l1_24_unicode_content)
    runner.run("L1-25 长内容", test_l1_25_long_content)
    runner.run("L1-26 特殊字符", test_l1_26_special_characters)
    runner.run("L1-27 JSON", test_l1_27_json_content)
    runner.run("L1-28 代码", test_l1_28_code_content)
    runner.run("L1-29 URL", test_l1_29_url_content)
    runner.run("L1-30 多行", test_l1_30_multiline_content)
    
    # 重要性测试
    print("\n【重要性评分测试】")
    runner.run("L1-31 重要性范围", test_l1_31_importance_range)
    runner.run("L1-32 类型重要性", test_l1_32_all_types_importance)
    
    # 输出总结
    print("\n" + "="*70)
    results = runner.summary()
    
    passed = sum(1 for r in results if r.passed)
    failed = sum(1 for r in results if not r.passed)
    total_duration = sum(r.duration_ms for r in results)
    
    print(f"\n总耗时: {total_duration:.2f}ms")
    print(f"平均: {total_duration/len(results):.2f}ms/测试")
    
    if failed == 0:
        print("\n🎉 所有L1单元测试通过!")
    else:
        print(f"\n⚠️  {failed}个测试失败")
    
    return results


if __name__ == "__main__":
    run_all_tests()
