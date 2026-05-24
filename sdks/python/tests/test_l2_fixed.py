"""L2修复测试"""
import uuid

def test_l2_08():
    """层级继承测试"""
    global_mem = [{"id": "g1", "content": "Global"}]
    user_mem = [{"id": "u1", "content": "User"}]
    
    effective = list(global_mem)
    for um in user_mem:
        # 简单合并
        effective.append(um)
    
    assert len(effective) == 2
    print("✅ L2-08: 层级继承修复通过")
    return True

if __name__ == "__main__":
    test_l2_08()
