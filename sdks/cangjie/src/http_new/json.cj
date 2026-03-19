/*
 * Copyright (c) AgentMem Team 2024. All rights reserved.
 */

/**
 * @file json.cj
 * JSON工具 - 为 http_new 包保留最小可编译的序列化/反序列化表面
 */

package agentmem_http

/// JSON构建器 - 用于序列化
public class JsonBuilder {
    private var body: String
    private var hasFields: Bool

    public init() {
        this.body = "{"
        this.hasFields = false
    }

    private func addRaw(key: String, value: String): JsonBuilder {
        if (this.hasFields) {
            this.body = "${this.body},"
        }
        this.body = "${this.body}\"${key}\":${value}"
        this.hasFields = true
        return this
    }

    public func addString(key: String, value: String): JsonBuilder {
        let escapedValue = escapeJson(value)
        return this.addRaw(key, "\"${escapedValue}\"")
    }

    public func addNumber(key: String, value: Float64): JsonBuilder {
        return this.addRaw(key, "${value}")
    }

    public func addInt(key: String, value: Int64): JsonBuilder {
        return this.addRaw(key, "${value}")
    }

    public func addBool(key: String, value: Bool): JsonBuilder {
        let boolStr = if (value) { "true" } else { "false" }
        return this.addRaw(key, boolStr)
    }

    public func addOptionalString(key: String, value: Option<String>): JsonBuilder {
        return match (value) {
            case Some(v) => this.addString(key, v)
            case None => this
        }
    }

    public func build(): String {
        return "${this.body}}"
    }
}

/// JSON转义函数
public func escapeJson(s: String): String {
    return s
}

/// 从JSON提取字段（当前 baseline 仅保留占位解析）
public func extractField(json: String, fieldName: String): String {
    return ""
}

/// 从JSON提取整数字段
public func extractIntField(json: String, fieldName: String): Int64 {
    return 0
}

/// 从JSON提取浮点数字段
public func extractFloatField(json: String, fieldName: String): Float64 {
    return 0.0
}

/// 从JSON提取布尔字段
public func extractBoolField(json: String, fieldName: String): Bool {
    return false
}

/// ====================
/// File-Centric JSON解析函数
/// ====================

public func parseResourceDescriptor(json: String): ResourceDescriptor {
    let id = extractField(json, "id")
    let uri = extractField(json, "uri")
    let mediaType = extractField(json, "media_type")
    let status = ResourceStatus.fromString(extractField(json, "status"))
    let metadata = ResourceMetadataDescriptor()
    let scope = ResourceScope(None, "")
    let createdAt = extractField(json, "created_at")
    let updatedAt = extractField(json, "updated_at")

    return ResourceDescriptor(
        id,
        uri,
        mediaType,
        status,
        scope,
        metadata,
        createdAt,
        updatedAt
    )
}

public func parseResourceDescriptors(json: String): Array<ResourceDescriptor> {
    return Array<ResourceDescriptor>()
}

public func parseCategoryDescriptor(json: String): CategoryDescriptor {
    let parentIdStr = extractField(json, "parent_id")
    let parentId = if (parentIdStr.isEmpty()) { None<String> } else { Some(parentIdStr) }

    return CategoryDescriptor(
        extractField(json, "id"),
        extractField(json, "path"),
        extractField(json, "name"),
        parentId,
        Array<String>(),
        extractField(json, "summary"),
        Int32(extractIntField(json, "item_count")),
        CategoryStatus.fromString(extractField(json, "status")),
        CategoryScope(None, ""),
        CategoryMetadataDescriptor(),
        extractField(json, "created_at"),
        extractField(json, "updated_at")
    )
}

public func parseCategoryDescriptors(json: String): Array<CategoryDescriptor> {
    return Array<CategoryDescriptor>()
}

public func parseExtractionResult(json: String): ExtractionResult {
    return ExtractionResult(
        extractField(json, "task_id"),
        extractField(json, "resource_id"),
        OperationStatus.fromString(extractField(json, "status")),
        Map<String, Int32>(),
        None,
        extractField(json, "created_at"),
        extractField(json, "updated_at")
    )
}

public func parseMigrationPlan(json: String): MigrationPlan {
    return MigrationPlan(
        extractBoolField(json, "dry_run"),
        extractField(json, "source_type"),
        Array<String>(),
        Int32(extractIntField(json, "estimated_items")),
        Array<String>()
    )
}

public func parseMigrationReport(json: String): MigrationReport {
    let completedAtStr = extractField(json, "completed_at")
    let completedAt = if (completedAtStr.isEmpty()) { None<String> } else { Some(completedAtStr) }

    return MigrationReport(
        extractField(json, "task_id"),
        OperationStatus.fromString(extractField(json, "status")),
        Int32(extractIntField(json, "migrated_items")),
        Int32(extractIntField(json, "failed_items")),
        Int32(extractIntField(json, "categories_created")),
        None,
        extractField(json, "started_at"),
        completedAt
    )
}

public func parseProactiveTaskInfo(json: String): ProactiveTaskInfo {
    return ProactiveTaskInfo(
        extractField(json, "task_id"),
        extractField(json, "task_type"),
        OperationStatus.fromString(extractField(json, "status")),
        extractField(json, "schedule"),
        None,
        None,
        Map<String, String>()
    )
}

public func parseProactiveTaskInfos(json: String): Array<ProactiveTaskInfo> {
    return Array<ProactiveTaskInfo>()
}

public func parseSchedulerStats(json: String): SchedulerStats {
    return SchedulerStats(
        Int32(extractIntField(json, "total_tasks")),
        Int32(extractIntField(json, "running_tasks")),
        Int32(extractIntField(json, "pending_tasks")),
        Int32(extractIntField(json, "completed_tasks")),
        Int32(extractIntField(json, "failed_tasks"))
    )
}
