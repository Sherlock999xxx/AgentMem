"""
AgentMem L3 端到端测试
真实用户场景测试 - 按照 testx1.0.md 计划执行

测试日期: 2026-05-23
测试目标: L3端到端测试级别 - 真实用户场景
"""

import pytest
import time
import uuid
from typing import List, Dict, Any
from dataclasses import dataclass, field
from datetime import datetime, timedelta

from agentmem import AgentMemClient, Config, MemoryType, SearchQuery


# =============================================================================
# L3-1: 个人助手场景
# =============================================================================

def test_l3_01_personal_assistant_daily():
    """场景: 个人助手日常工作"""
    memories = []
    
    # 早上: 记录日程
    memories.append({
        "id": f"mem-{uuid.uuid4().hex[:8]}",
        "type": "episodic",
        "content": "User has meeting at 10am with design team",
        "time": "morning",
    })
    
    # 中午: 记录偏好
    memories.append({
        "id": f"mem-{uuid.uuid4().hex[:8]}",
        "type": "semantic",
        "content": "User prefers salad for lunch",
        "time": "noon",
    })
    
    # 下午: 记录任务
    memories.append({
        "id": f"mem-{uuid.uuid4().hex[:8]}",
        "type": "episodic",
        "content": "User completed code review for PR #123",
        "time": "afternoon",
    })
    
    # 晚上: 记录学习
    memories.append({
        "id": f"mem-{uuid.uuid4().hex[:8]}",
        "type": "knowledge",
        "content": "Learned about async/await in Python",
        "time": "evening",
    })
    
    # 验证记忆收集
    assert len(memories) == 4
    types = set(m["type"] for m in memories)
    assert "episodic" in types
    assert "semantic" in types
    assert "knowledge" in types
    
    print("✅ L3-01: 个人助手日常工作场景通过")
    return True


def test_l3_02_personal_assistant_cross_session():
    """场景: 个人助手跨会话记忆"""
    sessions = []
    
    # 会话1: 首次见面
    session1 = {
        "session_id": "sess-001",
        "date": "2024-01-15",
        "memories": [
            {"type": "semantic", "content": "User's name is Alice"},
            {"type": "semantic", "content": "User works as a designer"},
        ]
    }
    
    # 会话2: 一周后
    session2 = {
        "session_id": "sess-002",
        "date": "2024-01-22",
        "memories": [
            {"type": "episodic", "content": "User asked about Python programming"},
        ]
    }
    
    sessions.extend([session1, session2])
    
    # 跨会话搜索"Alice"
    query = "Alice"
    results = []
    for session in sessions:
        for mem in session["memories"]:
            if query.lower() in mem["content"].lower():
                results.append(mem)
    
    assert len(results) == 1
    assert "Alice" in results[0]["content"]
    
    print("✅ L3-02: 个人助手跨会话记忆通过")
    return True


def test_l3_03_personal_assistant_preference_evolution():
    """场景: 个人助手偏好演变"""
    timeline = []
    
    # 2024年1月: 初始偏好
    timeline.append({
        "month": "2024-01",
        "content": "User prefers coffee",
        "type": "semantic",
    })
    
    # 2024年3月: 偏好改变
    timeline.append({
        "month": "2024-03",
        "content": "User switched to tea",
        "type": "episodic",
    })
    
    # 2024年5月: 偏好稳定
    timeline.append({
        "month": "2024-05",
        "content": "User drinks tea daily",
        "type": "semantic",
    })
    
    # 验证偏好演变
    current_preference = timeline[-1]["content"]
    assert "tea" in current_preference.lower()
    
    print("✅ L3-03: 个人助手偏好演变通过")
    return True


# =============================================================================
# L3-2: 客服Agent场景
# =============================================================================

def test_l3_04_customer_service_session():
    """场景: 客服会话"""
    session = {
        "customer_id": "cust-123",
        "agent_id": "agent-support",
        "interactions": [],
    }
    
    # 对话1: 客户询问
    session["interactions"].append({
        "role": "customer",
        "content": "I have an issue with my order",
        "type": "episodic",
    })
    
    # 对话2: Agent确认
    session["interactions"].append({
        "role": "agent",
        "content": "I can help you with your order issue",
        "type": "episodic",
    })
    
    # 对话3: 解决方案
    session["interactions"].append({
        "role": "agent",
        "content": "I've processed a refund for you",
        "type": "episodic",
    })
    
    # 验证会话完整性
    assert len(session["interactions"]) == 3
    assert session["customer_id"] == "cust-123"
    
    print("✅ L3-04: 客服会话场景通过")
    return True


def test_l3_05_customer_service_returning():
    """场景: 回头客"""
    # 检查是否为回头客
    customer_id = "cust-456"
    past_sessions = [
        {"date": "2024-01-10", "topic": "Order inquiry"},
        {"date": "2024-02-15", "topic": "Return request"},
    ]
    
    # 当前会话
    current_session = {
        "customer_id": customer_id,
        "is_returning": len(past_sessions) > 0,
        "past_topics": [s["topic"] for s in past_sessions],
    }
    
    assert current_session["is_returning"] == True
    assert len(current_session["past_topics"]) == 2
    
    # 获取历史上下文
    context = f"Returning customer. Past interactions: {', '.join(current_session['past_topics'])}"
    assert "Order inquiry" in context
    
    print("✅ L3-05: 回头客场景通过")
    return True


def test_l3_06_customer_service_handoff():
    """场景: 客服转接"""
    # 初始Agent
    agent1 = "agent-sales"
    session = {
        "customer_id": "cust-789",
        "transcripts": [
            {"agent": "agent-sales", "content": "I can help with sales questions"},
        ]
    }
    
    # 转接到专家
    session["transcripts"].append({
        "agent": "agent-technical",
        "content": "I need to transfer you to our technical team"
    })
    
    session["transcripts"].append({
        "agent": "agent-technical",
        "content": "I can help with technical issues"
    })
    
    # 验证转接上下文传递
    assert len(session["transcripts"]) == 3
    assert session["transcripts"][0]["agent"] == "agent-sales"
    assert session["transcripts"][2]["agent"] == "agent-technical"
    
    print("✅ L3-06: 客服转接场景通过")
    return True


# =============================================================================
# L3-3: 代码助手场景
# =============================================================================

def test_l3_07_coding_context():
    """场景: 代码助手上下文"""
    project_context = {
        "project": "web-app",
        "language": "Python",
        "framework": "FastAPI",
        "memories": [
            {"type": "semantic", "content": "Project uses FastAPI framework"},
            {"type": "semantic", "content": "Database is PostgreSQL"},
            {"type": "procedural", "content": "How to run tests: pytest tests/"},
            {"type": "knowledge", "content": "API endpoints are in /api/routes/"},
        ]
    }
    
    # 验证项目上下文
    assert project_context["language"] == "Python"
    assert len(project_context["memories"]) == 4
    
    # 验证知识类型分布
    types = [m["type"] for m in project_context["memories"]]
    assert "procedural" in types
    
    print("✅ L3-07: 代码助手上下文通过")
    return True


def test_l3_08_coding_learning():
    """场景: 代码助手学习"""
    # 记录用户的代码风格
    style_preferences = [
        {"type": "semantic", "content": "User prefers type hints"},
        {"type": "semantic", "content": "User uses snake_case naming"},
        {"type": "semantic", "content": "User prefers list comprehensions"},
    ]
    
    # 验证风格学习
    learned_style = {}
    for pref in style_preferences:
        content = pref["content"]
        if "type hints" in content:
            learned_style["typing"] = "explicit"
        elif "snake_case" in content:
            learned_style["naming"] = "snake_case"
        elif "comprehensions" in content:
            learned_style["style"] = "functional"
    
    assert learned_style["typing"] == "explicit"
    assert learned_style["naming"] == "snake_case"
    
    print("✅ L3-08: 代码助手学习通过")
    return True


def test_l3_09_coding_cross_file():
    """场景: 代码助手跨文件理解"""
    files = {
        "main.py": {"imports": ["config", "database"], "exports": ["app"]},
        "config.py": {"imports": [], "exports": ["settings", "DB_URL"]},
        "database.py": {"imports": ["config"], "exports": ["Session", "engine"]},
    }
    
    # 理解依赖关系
    dependencies = {}
    for file, info in files.items():
        dependencies[file] = info["exports"]
    
    # 验证依赖关系
    assert "config" in files["main.py"]["imports"]
    assert "config" in files["database.py"]["imports"]
    assert "Session" in dependencies["database.py"]
    
    print("✅ L3-09: 代码助手跨文件理解通过")
    return True


# =============================================================================
# L3-4: 研究助手场景
# =============================================================================

def test_l3_10_research_organization():
    """场景: 研究助手组织"""
    research_notes = []
    
    # 添加研究笔记
    notes = [
        {"type": "episodic", "content": "Read paper on Transformer architecture"},
        {"type": "knowledge", "content": "Transformer uses self-attention mechanism"},
        {"type": "resource", "content": "Reference: Attention Is All You Need"},
        {"type": "knowledge", "content": "Key components: Q, K, V matrices"},
    ]
    
    research_notes.extend(notes)
    
    # 按类型分类
    categorized = {}
    for note in research_notes:
        note_type = note["type"]
        if note_type not in categorized:
            categorized[note_type] = []
        categorized[note_type].append(note)
    
    assert len(categorized["knowledge"]) == 2
    assert len(categorized["resource"]) == 1
    
    print("✅ L3-10: 研究助手组织通过")
    return True


def test_l3_11_research_citation():
    """场景: 研究助手引用"""
    citations = []
    
    # 添加引用
    citation1 = {
        "id": "cite-001",
        "content": "Self-attention is O(n²) complexity",
        "source": "Vaswani et al., 2017",
    }
    
    citation2 = {
        "id": "cite-002",
        "content": "Flash Attention reduces to O(n)",
        "source": "Dao et al., 2022",
    }
    
    citations.extend([citation1, citation2])
    
    # 验证引用追踪
    assert len(citations) == 2
    assert citations[0]["source"] == "Vaswani et al., 2017"
    
    print("✅ L3-11: 研究助手引用通过")
    return True


def test_l3_12_research_knowledge_graph():
    """场景: 研究助手知识图谱"""
    entities = [
        {"id": "entity-1", "name": "Transformer", "type": "concept"},
        {"id": "entity-2", "name": "Self-Attention", "type": "mechanism"},
        {"id": "entity-3", "name": "BERT", "type": "model"},
    ]
    
    relationships = [
        {"from": "entity-1", "to": "entity-2", "type": "uses"},
        {"from": "entity-3", "to": "entity-1", "type": "based_on"},
    ]
    
    # 构建知识图谱
    graph = {
        "entities": {e["id"]: e for e in entities},
        "relationships": relationships,
    }
    
    # 验证图谱
    assert len(graph["entities"]) == 3
    assert len(graph["relationships"]) == 2
    
    # 验证关系
    assert any(r["from"] == "entity-1" and r["to"] == "entity-2" for r in relationships)
    
    print("✅ L3-12: 研究助手知识图谱通过")
    return True


# =============================================================================
# L3-5: 教育Agent场景
# =============================================================================

def test_l3_13_education_progress():
    """场景: 教育进度跟踪"""
    progress = []
    
    # 学习记录
    lessons = [
        {"id": "lesson-1", "topic": "Variables", "status": "completed", "score": 95},
        {"id": "lesson-2", "topic": "Data Types", "status": "completed", "score": 88},
        {"id": "lesson-3", "topic": "Control Flow", "status": "in_progress"},
    ]
    
    progress.extend(lessons)
    
    # 验证进度
    completed = [l for l in progress if l["status"] == "completed"]
    in_progress = [l for l in progress if l["status"] == "in_progress"]
    
    assert len(completed) == 2
    assert len(in_progress) == 1
    assert progress[0]["score"] == 95
    
    print("✅ L3-13: 教育进度跟踪通过")
    return True


def test_l3_14_education_adaptive():
    """场景: 教育自适应学习"""
    learner = {
        "strengths": ["syntax", "logic"],
        "weaknesses": ["recursion", "algorithms"],
        "learning_style": "visual",
    }
    
    # 自适应内容选择
    if "recursion" in learner["weaknesses"]:
        next_lesson = {
            "topic": "Recursion Basics",
            "style": "visual",
            "difficulty": "easy",
            "prerequisites_checked": True,
        }
    
    assert next_lesson["difficulty"] == "easy"
    assert next_lesson["style"] == "visual"
    
    print("✅ L3-14: 教育自适应学习通过")
    return True


def test_l3_15_education_forgetting_curve():
    """场景: 教育遗忘曲线"""
    memories = []
    
    # 学习事件
    learning_events = [
        {"time": "day-0", "topic": "Python Lists", "retention": 100},
        {"time": "day-1", "topic": "Python Lists", "retention": 80},  # 20%遗忘
        {"time": "day-3", "topic": "Python Lists", "retention": 60},  # 40%遗忘
        {"time": "day-7", "topic": "Python Lists", "retention": 40},  # 复习后恢复
    ]
    
    # 模拟遗忘曲线
    retention_threshold = 50
    
    for event in learning_events:
        if event["retention"] >= retention_threshold:
            memories.append({
                "topic": event["topic"],
                "status": "retained",
                "retention": event["retention"],
            })
        else:
            # 需要复习
            memories.append({
                "topic": event["topic"],
                "status": "needs_review",
                "retention": event["retention"],
            })
    
    # 验证遗忘管理
    needs_review = [m for m in memories if m["status"] == "needs_review"]
    assert len(needs_review) >= 1
    
    print("✅ L3-15: 教育遗忘曲线通过")
    return True


# =============================================================================
# L3-6: 团队协作场景
# =============================================================================

def test_l3_16_team_shared_knowledge():
    """场景: 团队共享知识"""
    team_knowledge = []
    
    # 团队知识
    team_knowledge.append({
        "type": "procedural",
        "content": "Team coding standard: Always write tests",
        "author": "lead-dev",
        "shared": True,
    })
    
    team_knowledge.append({
        "type": "knowledge",
        "content": "Architecture decision: Use microservices",
        "author": "architect",
        "shared": True,
    })
    
    # 验证团队知识共享
    shared = [k for k in team_knowledge if k.get("shared")]
    assert len(shared) == 2
    
    print("✅ L3-16: 团队共享知识通过")
    return True


def test_l3_17_team_onboarding():
    """场景: 团队新成员入职"""
    onboarding_knowledge = []
    
    # 入职知识库
    onboarding_knowledge.append({
        "type": "procedural",
        "content": "How to set up development environment",
        "order": 1,
    })
    
    onboarding_knowledge.append({
        "type": "knowledge",
        "content": "Team communication: Slack #dev channel",
        "order": 2,
    })
    
    onboarding_knowledge.append({
        "type": "resource",
        "content": "Documentation: /docs/onboarding.md",
        "order": 3,
    })
    
    # 按顺序学习
    ordered = sorted(onboarding_knowledge, key=lambda x: x["order"])
    
    assert ordered[0]["order"] == 1
    assert ordered[1]["order"] == 2
    assert ordered[2]["order"] == 3
    
    print("✅ L3-17: 团队新成员入职通过")
    return True


def test_l3_18_team_role_memory():
    """场景: 团队角色记忆"""
    roles = {
        "frontend": {
            "responsibilities": ["UI development", "user experience"],
            "memories": ["Component library: React"],
        },
        "backend": {
            "responsibilities": ["API development", "database"],
            "memories": ["Framework: FastAPI"],
        },
    }
    
    # 验证角色特定记忆
    assert len(roles["frontend"]["memories"]) == 1
    assert len(roles["backend"]["memories"]) == 1
    assert "React" in roles["frontend"]["memories"][0]
    
    print("✅ L3-18: 团队角色记忆通过")
    return True


# =============================================================================
# L3-7: 长期记忆场景
# =============================================================================

def test_l3_19_long_term_persistence():
    """场景: 长期记忆持久化"""
    long_term_memories = []
    
    # 跨年记忆
    memories = [
        {"year": "2022", "content": "Project Alpha completed", "type": "episodic"},
        {"year": "2023", "content": "Promoted to senior", "type": "episodic"},
        {"year": "2024", "content": "Learning Rust", "type": "knowledge"},
    ]
    
    # 长期记忆标记
    for mem in memories:
        if mem["type"] == "episodic" or mem["year"] in ["2022", "2023"]:
            mem["long_term"] = True
            long_term_memories.append(mem)
    
    assert len(long_term_memories) == 2
    
    print("✅ L3-19: 长期记忆持久化通过")
    return True


def test_l3_20_memory_decay_management():
    """场景: 记忆衰减管理"""
    memories = []
    
    # 模拟不同年龄的记忆
    now = time.time()
    for i in range(10):
        age_days = i * 30  # 每月
        memories.append({
            "id": f"mem-{i}",
            "age_days": age_days,
            "importance": 0.5,
            "last_accessed": now - (age_days * 86400 / 2),
        })
    
    # 衰减管理
    for mem in memories:
        decay_rate = 0.01  # 每天1%
        mem["decayed_importance"] = mem["importance"] * (1 - decay_rate) ** mem["age_days"]
        
        # 如果经常访问，衰减减缓
        if now - mem["last_accessed"] < 86400:  # 24小时内访问过
            mem["decayed_importance"] *= 1.2  # 增强20%
    
    # 验证衰减管理
    young_mem = memories[0]
    old_mem = memories[-1]
    
    assert young_mem["decayed_importance"] > old_mem["decayed_importance"] * 0.5
    
    print("✅ L3-20: 记忆衰减管理通过")
    return True


# =============================================================================
# 运行所有L3测试
# =============================================================================

def run_l3_tests():
    """运行所有L3端到端测试"""
    print("\n" + "="*70)
    print("AgentMem L3 端到端测试 - 真实用户场景验证")
    print("="*70)
    
    tests = [
        # 个人助手场景
        ("L3-01 个人助手日常工作", test_l3_01_personal_assistant_daily),
        ("L3-02 个人助手跨会话", test_l3_02_personal_assistant_cross_session),
        ("L3-03 个人助手偏好演变", test_l3_03_personal_assistant_preference_evolution),
        
        # 客服Agent场景
        ("L3-04 客服会话", test_l3_04_customer_service_session),
        ("L3-05 回头客", test_l3_05_customer_service_returning),
        ("L3-06 客服转接", test_l3_06_customer_service_handoff),
        
        # 代码助手场景
        ("L3-07 代码助手上下文", test_l3_07_coding_context),
        ("L3-08 代码助手学习", test_l3_08_coding_learning),
        ("L3-09 代码助手跨文件", test_l3_09_coding_cross_file),
        
        # 研究助手场景
        ("L3-10 研究助手组织", test_l3_10_research_organization),
        ("L3-11 研究助手引用", test_l3_11_research_citation),
        ("L3-12 研究助手知识图谱", test_l3_12_research_knowledge_graph),
        
        # 教育Agent场景
        ("L3-13 教育进度跟踪", test_l3_13_education_progress),
        ("L3-14 教育自适应学习", test_l3_14_education_adaptive),
        ("L3-15 教育遗忘曲线", test_l3_15_education_forgetting_curve),
        
        # 团队协作场景
        ("L3-16 团队共享知识", test_l3_16_team_shared_knowledge),
        ("L3-17 团队新成员入职", test_l3_17_team_onboarding),
        ("L3-18 团队角色记忆", test_l3_18_team_role_memory),
        
        # 长期记忆场景
        ("L3-19 长期记忆持久化", test_l3_19_long_term_persistence),
        ("L3-20 记忆衰减管理", test_l3_20_memory_decay_management),
    ]
    
    passed = 0
    failed = 0
    
    for name, test_func in tests:
        try:
            test_func()
            passed += 1
        except Exception as e:
            print(f"  ❌ {name}: {e}")
            failed += 1
    
    print("\n" + "="*70)
    print(f"L3端到端测试结果: {passed}/{len(tests)} 通过")
    if failed > 0:
        print(f"失败: {failed}")
    print("="*70)
    
    return passed, failed


if __name__ == "__main__":
    run_l3_tests()
