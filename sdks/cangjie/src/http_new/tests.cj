/*
 * Copyright (c) AgentMem Team 2024. All rights reserved.
 */

/**
 * @file tests.cj
 * HTTP客户端和API测试
 */

package agentmem_http

/// 测试HTTP客户端基本功能
public func testHttpClient(): Bool {
    println("\n=== 测试1: HTTP客户端基本功能 ===")
    
    let config = ClientConfig("http://localhost:8080")
    let client = AgentMemHttpClient(config)
    
    // 测试健康检查
    let result = client.get("/health")
    match (result) {
        case Ok(response) =>
            println("✅ 健康检查成功: ${response}")
            return true
        case Err(e) =>
            println("❌ 健康检查失败: ${e.getMessage()}")
            return false
    }
}

/// 测试Memory对象创建和序列化
public func testMemoryCreation(): Bool {
    println("\n=== 测试2: Memory对象创建 ===")
    
    let memory = Memory.create("agent-1", "这是测试记忆内容")
    println("✅ Memory创建成功")
    println("  - ID: ${memory.id}")
    println("  - Agent ID: ${memory.agentId}")
    println("  - Content: ${memory.content}")
    println("  - Type: ${memory.memoryType.toString()}")
    
    let json = memory.toJson()
    println("✅ JSON序列化成功: ${json}")
    
    return true
}

/// 测试添加记忆
public func testAddMemory(): Bool {
    println("\n=== 测试3: 添加记忆 ===")
    
    let config = ClientConfig("http://localhost:8080")
    let client = AgentMemHttpClient(config)
    let api = MemoryApi(client)
    
    let memory = Memory.create("agent-1", "重要的测试记忆")
    let result = api.addMemory(memory)
    
    match (result) {
        case Ok(memoryId) =>
            println("✅ 添加记忆成功, ID: ${memoryId}")
            return true
        case Err(e) =>
            println("❌ 添加记忆失败: ${e.getMessage()}")
            return false
    }
}

/// 测试获取记忆
public func testGetMemory(): Bool {
    println("\n=== 测试4: 获取记忆 ===")
    
    let config = ClientConfig("http://localhost:8080")
    let client = AgentMemHttpClient(config)
    let api = MemoryApi(client)
    
    let result = api.getMemory("mem-001")
    
    match (result) {
        case Ok(memory) =>
            println("✅ 获取记忆成功")
            println("  - ID: ${memory.id}")
            println("  - Content: ${memory.content}")
            return true
        case Err(e) =>
            println("❌ 获取记忆失败: ${e.getMessage()}")
            return false
    }
}

/// 测试搜索记忆
public func testSearchMemories(): Bool {
    println("\n=== 测试5: 搜索记忆 ===")
    
    let config = ClientConfig("http://localhost:8080")
    let client = AgentMemHttpClient(config)
    let api = MemoryApi(client)
    
    let result = api.searchMemories("测试", 10)
    
    match (result) {
        case Ok(results) =>
            println("✅ 搜索成功, 找到 ${results.size} 条记忆")
            return true
        case Err(e) =>
            println("❌ 搜索失败: ${e.getMessage()}")
            return false
    }
}

/// 测试完整流程
public func testFullWorkflow(): Bool {
    println("\n=== 测试6: 完整工作流程 ===")
    
    let config = ClientConfig("http://localhost:8080")
    let client = AgentMemHttpClient(config)
    let api = MemoryApi(client)
    
    // 1. 添加记忆
    println("步骤1: 添加记忆...")
    let memory = Memory.create("agent-1", "工作流程测试记忆")
    let addResult = api.addMemory(memory)
    
    var memoryId = ""
    match (addResult) {
        case Ok(id) =>
            memoryId = id
            println("✅ 添加成功: ${id}")
        case Err(e) =>
            println("❌ 添加失败: ${e.getMessage()}")
            return false
    }
    
    // 2. 获取记忆
    println("步骤2: 获取记忆...")
    let getResult = api.getMemory(memoryId)
    match (getResult) {
        case Ok(mem) =>
            println("✅ 获取成功: ${mem.content}")
        case Err(e) =>
            println("❌ 获取失败: ${e.getMessage()}")
            return false
    }
    
    // 3. 更新记忆
    println("步骤3: 更新记忆...")
    let updateResult = api.updateMemory(memoryId, "更新后的内容")
    match (updateResult) {
        case Ok(_) =>
            println("✅ 更新成功")
        case Err(e) =>
            println("❌ 更新失败: ${e.getMessage()}")
            return false
    }
    
    // 4. 搜索记忆
    println("步骤4: 搜索记忆...")
    let searchResult = api.searchMemories("测试", 10)
    match (searchResult) {
        case Ok(results) =>
            println("✅ 搜索成功: 找到 ${results.size} 条")
        case Err(e) =>
            println("❌ 搜索失败: ${e.getMessage()}")
            return false
    }
    
    // 5. 删除记忆
    println("步骤5: 删除记忆...")
    let deleteResult = api.deleteMemory(memoryId)
    match (deleteResult) {
        case Ok(_) =>
            println("✅ 删除成功")
        case Err(e) =>
            println("❌ 删除失败: ${e.getMessage()}")
            return false
    }
    
    println("\n✅ 完整工作流程测试通过！")
    return true
}

/// 运行所有测试
public func runAllTests() {
    println("\n╔══════════════════════════════════════╗")
    println("║   AgentMem HTTP SDK 测试套件        ║")
    println("╚══════════════════════════════════════╝")
    
    var passedTests = 0
    var totalTests = 6
    
    if (testHttpClient()) { passedTests += 1 }
    if (testMemoryCreation()) { passedTests += 1 }
    if (testAddMemory()) { passedTests += 1 }
    if (testGetMemory()) { passedTests += 1 }
    if (testSearchMemories()) { passedTests += 1 }
    if (testFullWorkflow()) { passedTests += 1 }
    
    println("\n╔══════════════════════════════════════╗")
    println("║         测试结果统计                  ║")
    println("╠══════════════════════════════════════╣")
    println("║  通过: ${passedTests}/${totalTests}                       ║")
    println("║  通过率: ${(passedTests * 100) / totalTests}%                     ║")
    println("╚══════════════════════════════════════╝")
    
    if (passedTests == totalTests) {
        println("\n🎉 所有测试通过！HTTP方案验证成功！")
    } else {
        println("\n⚠️  部分测试失败，需要进一步调试")
    }
}

main() {
    runAllTests()
}
