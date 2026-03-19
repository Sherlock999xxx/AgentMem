/*
 * Copyright (c) AgentMem Team 2024. All rights reserved.
 */

/**
 * @file api.cj
 * Memory API实现（基于HTTP）
 */

package agentmem_http

/// Memory API客户端
public class MemoryApi {
    private let client: AgentMemHttpClient
    
    /// 构造函数
    public init(client: AgentMemHttpClient) {
        this.client = client
    }
    
    /// 添加记忆
    public func addMemory(memory: Memory): Result<String, AgentMemError> {
        let body = JsonBuilder()
            .addString("agent_id", memory.agentId)
            .addString("content", memory.content)
            .addString("memory_type", memory.memoryType.toString())
            .addNumber("importance", memory.importance)
            .build()
        
        let response = this.client.post("/api/v1/memories", body)
        match (response) {
            case Ok(jsonStr) =>
                let memoryId = extractField(jsonStr, "id")
                return Ok(memoryId)
            case Err(e) => return Err(e)
        }
    }
    
    /// 获取记忆
    public func getMemory(memoryId: String): Result<Memory, AgentMemError> {
        let response = this.client.get("/api/v1/memories/${memoryId}")
        match (response) {
            case Ok(jsonStr) =>
                let memory = parseMemory(jsonStr)
                return Ok(memory)
            case Err(e) => return Err(e)
        }
    }
    
    /// 更新记忆
    public func updateMemory(memoryId: String, content: String): Result<Unit, AgentMemError> {
        let body = JsonBuilder()
            .addString("content", content)
            .build()
        
        let response = this.client.put("/api/v1/memories/${memoryId}", body)
        match (response) {
            case Ok(_) => return Ok(())
            case Err(e) => return Err(e)
        }
    }
    
    /// 删除记忆
    public func deleteMemory(memoryId: String): Result<Unit, AgentMemError> {
        let response = this.client.delete("/api/v1/memories/${memoryId}")
        match (response) {
            case Ok(_) => return Ok(())
            case Err(e) => return Err(e)
        }
    }
    
    /// 搜索记忆
    public func searchMemories(query: String, limit: Int32): Result<Array<SearchResult>, AgentMemError> {
        let body = JsonBuilder()
            .addString("query", query)
            .addInt("limit", Int64(limit))
            .build()
        
        let response = this.client.post("/api/v1/memories/search", body)
        match (response) {
            case Ok(jsonStr) =>
                // 简化实现：返回空数组
                // 实际需要解析JSON数组
                let results = Array<SearchResult>()
                return Ok(results)
            case Err(e) => return Err(e)
        }
    }
    
    /// 批量添加记忆
    public func addMemoriesBatch(memories: Array<Memory>): Result<Int32, AgentMemError> {
        // 构建批量请求体
        var memoriesJson = Array<String>(memories.size, {_ => ""})
        for (i in 0..memories.size) {
            memoriesJson[i] = memories[i].toJson()
        }
        
        var bodyStr = "{\"memories\":["
        for (i in 0..memoriesJson.size) {
            bodyStr += memoriesJson[i]
            if (i < memoriesJson.size - 1) {
                bodyStr += ","
            }
        }
        bodyStr += "]}"
        
        let response = this.client.post("/api/v1/memories/batch", bodyStr)
        match (response) {
            case Ok(jsonStr) =>
                let successCount = extractIntField(jsonStr, "success_count")
                return Ok(Int32(successCount))
            case Err(e) => return Err(e)
        }
    }
    
    /// 获取记忆统计
    public func getMemoryStats(agentId: String): Result<MemoryStats, AgentMemError> {
        let response = this.client.get("/api/v1/agents/${agentId}/stats")
        match (response) {
            case Ok(jsonStr) =>
                let stats = MemoryStats(
                    Int32(extractIntField(jsonStr, "total_memories")),
                    extractFloatField(jsonStr, "average_importance")
                )
                return Ok(stats)
            case Err(e) => return Err(e)
        }
    }
}

/// 记忆统计信息
public class MemoryStats {
    public var totalMemories: Int32
    public var averageImportance: Float64

    public init(totalMemories: Int32, averageImportance: Float64) {
        this.totalMemories = totalMemories
        this.averageImportance = averageImportance
    }
}

/// ====================================
/// File-Centric API
/// ====================================

/// 文件中心API客户端
public class FileCentricApi {
    private let client: AgentMemHttpClient

    /// 构造函数
    public init(client: AgentMemHttpClient) {
        this.client = client
    }

    // ====================
    /// 资源操作
    // ====================

    /// 挂载资源
    public func mountResource(req: MountResourceRequest): Result<ResourceDescriptor, AgentMemError> {
        let builder = JsonBuilder()
            .addString("uri", req.uri)
            .addString("media_type", req.mediaType)
            .addString("agent_id", req.agentId)

        match (req.userId) {
            case Some(userId) => builder.addString("user_id", userId)
            case None => builder
        }

        let body = builder.build()

        let response = this.client.post("/api/v1/resources", body)
        match (response) {
            case Ok(jsonStr) =>
                let resource = parseResourceDescriptor(jsonStr)
                return Ok(resource)
            case Err(e) => return Err(e)
        }
    }

    /// 获取资源
    public func getResource(resourceId: String): Result<ResourceDescriptor, AgentMemError> {
        let response = this.client.get("/api/v1/resources/${resourceId}")
        match (response) {
            case Ok(jsonStr) =>
                let resource = parseResourceDescriptor(jsonStr)
                return Ok(resource)
            case Err(e) => return Err(e)
        }
    }

    /// 列出资源
    public func listResources(
        userId: Option<String>,
        agentId: String,
        limit: Int32
    ): Result<Array<ResourceDescriptor>, AgentMemError> {
        var path = "/api/v1/resources?agent_id=${agentId}&limit=${limit}"
        match (userId) {
            case Some(uid) => path = "${path}&user_id=${uid}"
            case None => path
        }

        let response = this.client.get(path)
        match (response) {
            case Ok(jsonStr) =>
                let resources = parseResourceDescriptors(jsonStr)
                return Ok(resources)
            case Err(e) => return Err(e)
        }
    }

    // ====================
    /// 类别操作
    // ====================

    /// 获取类别
    public func getCategory(categoryId: String): Result<CategoryDescriptor, AgentMemError> {
        let response = this.client.get("/api/v1/categories/${categoryId}")
        match (response) {
            case Ok(jsonStr) =>
                let category = parseCategoryDescriptor(jsonStr)
                return Ok(category)
            case Err(e) => return Err(e)
        }
    }

    /// 通过路径获取类别
    public func getCategoryByPath(path: String, agentId: String): Result<CategoryDescriptor, AgentMemError> {
        let response = this.client.get("/api/v1/categories/by-path?path=${path}&agent_id=${agentId}")
        match (response) {
            case Ok(jsonStr) =>
                let category = parseCategoryDescriptor(jsonStr)
                return Ok(category)
            case Err(e) => return Err(e)
        }
    }

    /// 列出类别
    public func listCategories(
        userId: Option<String>,
        agentId: String,
        parentId: Option<String>,
        limit: Int32
    ): Result<Array<CategoryDescriptor>, AgentMemError> {
        var path = "/api/v1/categories?agent_id=${agentId}&limit=${limit}"
        match (userId) {
            case Some(uid) => path = "${path}&user_id=${uid}"
            case None => path
        }
        match (parentId) {
            case Some(pid) => path = "${path}&parent_id=${pid}"
            case None => path
        }

        let response = this.client.get(path)
        match (response) {
            case Ok(jsonStr) =>
                let categories = parseCategoryDescriptors(jsonStr)
                return Ok(categories)
            case Err(e) => return Err(e)
        }
    }

    /// 搜索类别
    public func searchCategories(
        query: String,
        agentId: String,
        limit: Int32
    ): Result<Array<CategoryDescriptor>, AgentMemError> {
        let body = JsonBuilder()
            .addString("query", query)
            .addString("agent_id", agentId)
            .addInt("limit", Int64(limit))
            .build()

        let response = this.client.post("/api/v1/categories/search", body)
        match (response) {
            case Ok(jsonStr) =>
                let categories = parseCategoryDescriptors(jsonStr)
                return Ok(categories)
            case Err(e) => return Err(e)
        }
    }

    // ====================
    /// 提取操作
    // ====================

    /// 提取资源
    public func extractResource(req: ExtractionRequest): Result<ExtractionResult, AgentMemError> {
        let builder = JsonBuilder()
            .addString("resource_id", req.resourceId)
            .addString("agent_id", req.scope.agentId)

        match (req.scope.userId) {
            case Some(userId) => builder.addString("user_id", userId)
            case None => builder
        }

        let body = builder.build()

        if (req.extractors.size > 0) {
            // Keep the baseline request shape minimal until array serialization is implemented.
        }

        let response = this.client.post("/api/v1/extractions", body)
        match (response) {
            case Ok(jsonStr) =>
                let result = parseExtractionResult(jsonStr)
                return Ok(result)
            case Err(e) => return Err(e)
        }
    }

    /// 获取提取状态
    public func getExtractionStatus(taskId: String): Result<ExtractionResult, AgentMemError> {
        let response = this.client.get("/api/v1/extractions/${taskId}")
        match (response) {
            case Ok(jsonStr) =>
                let result = parseExtractionResult(jsonStr)
                return Ok(result)
            case Err(e) => return Err(e)
        }
    }

    // ====================
    /// 迁移操作
    // ====================

    /// 规划遗留迁移
    public func planLegacyMigration(req: MigrationPlanRequest): Result<MigrationPlan, AgentMemError> {
        let body = JsonBuilder()
            .addString("agent_id", req.agentId)
            .addBool("dry_run", req.dryRun)
            .build()

        let response = this.client.post("/api/v1/migrations/plan", body)
        match (response) {
            case Ok(jsonStr) =>
                let plan = parseMigrationPlan(jsonStr)
                return Ok(plan)
            case Err(e) => return Err(e)
        }
    }

    /// 应用遗留迁移
    public func applyLegacyMigration(req: ApplyMigrationRequest): Result<MigrationReport, AgentMemError> {
        let body = JsonBuilder()
            .addString("plan_id", req.planId)
            .addString("agent_id", req.agentId)
            .addBool("dry_run", req.dryRun)
            .build()

        let response = this.client.post("/api/v1/migrations/apply", body)
        match (response) {
            case Ok(jsonStr) =>
                let report = parseMigrationReport(jsonStr)
                return Ok(report)
            case Err(e) => return Err(e)
        }
    }

    /// 获取迁移状态
    public func getMigrationStatus(migrationId: String): Result<MigrationReport, AgentMemError> {
        let response = this.client.get("/api/v1/migrations/${migrationId}")
        match (response) {
            case Ok(jsonStr) =>
                let report = parseMigrationReport(jsonStr)
                return Ok(report)
            case Err(e) => return Err(e)
        }
    }

    /// 回滚迁移
    public func rollbackMigration(migrationId: String): Result<MigrationReport, AgentMemError> {
        let response = this.client.post("/api/v1/migrations/${migrationId}/rollback", "{}")
        match (response) {
            case Ok(jsonStr) =>
                let report = parseMigrationReport(jsonStr)
                return Ok(report)
            case Err(e) => return Err(e)
        }
    }

    // ====================
    /// 主动任务操作
    // ====================

    /// 列出主动任务
    public func listProactiveTasks(agentId: String): Result<Array<ProactiveTaskInfo>, AgentMemError> {
        let response = this.client.get("/api/v1/proactive/tasks?agent_id=${agentId}")
        match (response) {
            case Ok(jsonStr) =>
                let tasks = parseProactiveTaskInfos(jsonStr)
                return Ok(tasks)
            case Err(e) => return Err(e)
        }
    }

    /// 获取主动任务
    public func getProactiveTask(taskId: String): Result<ProactiveTaskInfo, AgentMemError> {
        let response = this.client.get("/api/v1/proactive/tasks/${taskId}")
        match (response) {
            case Ok(jsonStr) =>
                let task = parseProactiveTaskInfo(jsonStr)
                return Ok(task)
            case Err(e) => return Err(e)
        }
    }

    /// 运行主动任务
    public func runProactiveTask(taskId: String): Result<ProactiveTaskInfo, AgentMemError> {
        let response = this.client.post("/api/v1/proactive/tasks/${taskId}/run", "{}")
        match (response) {
            case Ok(jsonStr) =>
                let task = parseProactiveTaskInfo(jsonStr)
                return Ok(task)
            case Err(e) => return Err(e)
        }
    }

    /// 取消主动任务
    public func cancelProactiveTask(taskId: String): Result<Unit, AgentMemError> {
        let response = this.client.post("/api/v1/proactive/tasks/${taskId}/cancel", "{}")
        match (response) {
            case Ok(_) => return Ok(())
            case Err(e) => return Err(e)
        }
    }

    /// 获取调度器统计
    public func getSchedulerStats(): Result<SchedulerStats, AgentMemError> {
        let response = this.client.get("/api/v1/proactive/stats")
        match (response) {
            case Ok(jsonStr) =>
                let stats = parseSchedulerStats(jsonStr)
                return Ok(stats)
            case Err(e) => return Err(e)
        }
    }
}

/// ====================
/// 辅助请求类型
/// ====================

/// 挂载资源请求
public class MountResourceRequest {
    public var uri: String
    public var mediaType: String
    public var userId: Option<String>
    public var agentId: String

    public init(uri: String, mediaType: String, userId: Option<String>, agentId: String) {
        this.uri = uri
        this.mediaType = mediaType
        this.userId = userId
        this.agentId = agentId
    }
}

/// 迁移计划请求
public class MigrationPlanRequest {
    public var agentId: String
    public var dryRun: Bool

    public init(agentId: String, dryRun: Bool) {
        this.agentId = agentId
        this.dryRun = dryRun
    }
}

/// 应用迁移请求
public class ApplyMigrationRequest {
    public var planId: String
    public var agentId: String
    public var dryRun: Bool

    public init(planId: String, agentId: String, dryRun: Bool) {
        this.planId = planId
        this.agentId = agentId
        this.dryRun = dryRun
    }
}
