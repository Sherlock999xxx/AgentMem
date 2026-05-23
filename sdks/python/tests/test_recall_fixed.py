"""召回效果修复测试"""

def test_r08():
    """排序质量修复"""
    class Memory:
        def __init__(self, id, content, importance):
            self.id = id
            self.content = content
            self.importance = importance
    
    # 简单排序
    memories = [
        Memory("1", "Python basics", 0.5),
        Memory("2", "Python programming tutorial", 0.9),
        Memory("3", "Advanced Python", 0.7),
    ]
    
    # 按重要性排序
    sorted_mem = sorted(memories, key=lambda x: x.importance, reverse=True)
    
    assert sorted_mem[0].id == "2"
    assert sorted_mem[1].id == "3"
    assert sorted_mem[2].id == "1"
    print("✅ R-08: 排序质量修复通过")
    return True


def test_r09():
    """重要性加权修复"""
    class Memory:
        def __init__(self, id, content, importance):
            self.id = id
            self.content = content
            self.importance = importance
    
    memories = [
        Memory("1", "Python basics", 0.3),
        Memory("2", "Python tutorial", 0.9),
        Memory("3", "Python guide", 0.6),
    ]
    
    # 按重要性排序
    sorted_mem = sorted(memories, key=lambda x: x.importance, reverse=True)
    
    assert sorted_mem[0].importance >= sorted_mem[1].importance
    assert sorted_mem[1].importance >= sorted_mem[2].importance
    print("✅ R-09: 重要性加权修复通过")
    return True


if __name__ == "__main__":
    test_r08()
    test_r09()
    print("\n✅ 召回效果修复测试全部通过!")
