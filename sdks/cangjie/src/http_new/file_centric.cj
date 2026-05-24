/*
 * Copyright (c) AgentMem Team 2024. All rights reserved.
 */

/**
 * @file file_centric.cj
 * File-centric类型定义 - AgentMem 仓颉 SDK HTTP方案
 */

package agentmem_http

/// 资源状态枚举
public enum ResourceStatus {
    | Pending    // 待挂载
    | Mounted    // 已挂载
    | Failed     // 挂载失败
    | Unmounted  // 已卸载

    /// 转换为字符串
    public func toString(): String {
        return match (this) {
            case Pending => "pending"
            case Mounted => "mounted"
            case Failed => "failed"
            case Unmounted => "unmounted"
        }
    }

    /// 从字符串解析
    public static func fromString(s: String): ResourceStatus {
        return match (s) {
            case "pending" => Pending
            case "mounted" => Mounted
            case "failed" => Failed
            case "unmounted" => Unmounted
            case _ => Pending
        }
    }
}

/// 类别状态枚举
public enum CategoryStatus {
    | Active     // 活跃
    | Archived   // 已归档
    | Deleted    // 已删除

    /// 转换为字符串
    public func toString(): String {
        return match (this) {
            case Active => "active"
            case Archived => "archived"
            case Deleted => "deleted"
        }
    }

    /// 从字符串解析
    public static func fromString(s: String): CategoryStatus {
        return match (s) {
            case "active" => Active
            case "archived" => Archived
            case "deleted" => Deleted
            case _ => Active
        }
    }
}

/// 操作状态枚举
public enum OperationStatus {
    | Pending    // 待执行
    | Running    // 执行中
    | Succeeded  // 成功
    | Failed     // 失败
    | Cancelled  // 已取消

    /// 转换为字符串
    public func toString(): String {
        return match (this) {
            case Pending => "pending"
            case Running => "running"
            case Succeeded => "succeeded"
            case Failed => "failed"
            case Cancelled => "cancelled"
        }
    }

    /// 从字符串解析
    public static func fromString(s: String): OperationStatus {
        return match (s) {
            case "pending" => Pending
            case "running" => Running
            case "succeeded" => Succeeded
            case "failed" => Failed
            case "cancelled" => Cancelled
            case _ => Pending
        }
    }
}

/// 平台错误码枚举
public enum PlatformErrorCode {
    | Validation                // 参数验证错误
    | CategoryNotFound          // 类别不存在
    | ResourceUriConflict       // 资源URI冲突
    | MigrationConflict         // 迁移冲突
    | TaskTimeout               // 任务超时
    | BackgroundTaskUnavailable // 后台任务不可用

    /// 转换为字符串
    public func toString(): String {
        return match (this) {
            case Validation => "validation"
            case CategoryNotFound => "category_not_found"
            case ResourceUriConflict => "resource_uri_conflict"
            case MigrationConflict => "migration_conflict"
            case TaskTimeout => "task_timeout"
            case BackgroundTaskUnavailable => "background_task_unavailable"
        }
    }

    /// 从字符串解析
    public static func fromString(s: String): PlatformErrorCode {
        return match (s) {
            case "validation" => Validation
            case "category_not_found" => CategoryNotFound
            case "resource_uri_conflict" => ResourceUriConflict
            case "migration_conflict" => MigrationConflict
            case "task_timeout" => TaskTimeout
            case "background_task_unavailable" => BackgroundTaskUnavailable
            case _ => Validation
        }
    }
}

/// 资源作用域
public class ResourceScope {
    public var userId: Option<String>
    public var agentId: String

    public init(userId: Option<String>, agentId: String) {
        this.userId = userId
        this.agentId = agentId
    }
}

/// 类别作用域
public class CategoryScope {
    public var userId: Option<String>
    public var agentId: String

    public init(userId: Option<String>, agentId: String) {
        this.userId = userId
        this.agentId = agentId
    }
}

/// 资源元数据描述符
public class ResourceMetadataDescriptor {
    public var author: Option<String>
    public var tags: Array<String>
    public var sizeBytes: Option<Int64>
    public var modifiedAt: Option<String>
    public var attributes: Map<String, String>

    public init() {
        this.author = None
        this.tags = Array<String>()
        this.sizeBytes = None
        this.modifiedAt = None
        this.attributes = Map<String, String>()
    }
}

/// 类别元数据描述符
public class CategoryMetadataDescriptor {
    public var tags: Array<String>
    public var attributes: Map<String, String>

    public init() {
        this.tags = Array<String>()
        this.attributes = Map<String, String>()
    }
}

/// 资源描述符
public class ResourceDescriptor {
    public var id: String
    public var uri: String
    public var mediaType: String
    public var status: ResourceStatus
    public var scope: ResourceScope
    public var metadata: ResourceMetadataDescriptor
    public var createdAt: String
    public var updatedAt: String

    public init(
        id: String,
        uri: String,
        mediaType: String,
        status: ResourceStatus,
        scope: ResourceScope,
        metadata: ResourceMetadataDescriptor,
        createdAt: String,
        updatedAt: String
    ) {
        this.id = id
        this.uri = uri
        this.mediaType = mediaType
        this.status = status
        this.scope = scope
        this.metadata = metadata
        this.createdAt = createdAt
        this.updatedAt = updatedAt
    }
}

/// 类别描述符
public class CategoryDescriptor {
    public var id: String
    public var path: String
    public var name: String
    public var parentId: Option<String>
    public var childrenIds: Array<String>
    public var summary: String
    public var itemCount: Int32
    public var status: CategoryStatus
    public var scope: CategoryScope
    public var metadata: CategoryMetadataDescriptor
    public var createdAt: String
    public var updatedAt: String

    public init(
        id: String,
        path: String,
        name: String,
        parentId: Option<String>,
        childrenIds: Array<String>,
        summary: String,
        itemCount: Int32,
        status: CategoryStatus,
        scope: CategoryScope,
        metadata: CategoryMetadataDescriptor,
        createdAt: String,
        updatedAt: String
    ) {
        this.id = id
        this.path = path
        this.name = name
        this.parentId = parentId
        this.childrenIds = childrenIds
        this.summary = summary
        this.itemCount = itemCount
        this.status = status
        this.scope = scope
        this.metadata = metadata
        this.createdAt = createdAt
        this.updatedAt = updatedAt
    }
}

/// 提取请求
public class ExtractionRequest {
    public var resourceId: String
    public var extractors: Array<String>
    public var scope: ResourceScope

    public init(resourceId: String, extractors: Array<String>, scope: ResourceScope) {
        this.resourceId = resourceId
        this.extractors = extractors
        this.scope = scope
    }
}

/// 提取结果
public class ExtractionResult {
    public var taskId: String
    public var resourceId: String
    public var status: OperationStatus
    public var itemCounts: Map<String, Int32>
    public var errorMessage: Option<String>
    public var createdAt: String
    public var updatedAt: String

    public init(
        taskId: String,
        resourceId: String,
        status: OperationStatus,
        itemCounts: Map<String, Int32>,
        errorMessage: Option<String>,
        createdAt: String,
        updatedAt: String
    ) {
        this.taskId = taskId
        this.resourceId = resourceId
        this.status = status
        this.itemCounts = itemCounts
        this.errorMessage = errorMessage
        this.createdAt = createdAt
        this.updatedAt = updatedAt
    }
}

/// 迁移计划
public class MigrationPlan {
    public var dryRun: Bool
    public var sourceType: String
    public var targetCategories: Array<String>
    public var estimatedItems: Int32
    public var warnings: Array<String>

    public init(
        dryRun: Bool,
        sourceType: String,
        targetCategories: Array<String>,
        estimatedItems: Int32,
        warnings: Array<String>
    ) {
        this.dryRun = dryRun
        this.sourceType = sourceType
        this.targetCategories = targetCategories
        this.estimatedItems = estimatedItems
        this.warnings = warnings
    }
}

/// 迁移报告
public class MigrationReport {
    public var taskId: String
    public var status: OperationStatus
    public var migratedItems: Int32
    public var failedItems: Int32
    public var categoriesCreated: Int32
    public var errorMessage: Option<String>
    public var startedAt: String
    public var completedAt: Option<String>

    public init(
        taskId: String,
        status: OperationStatus,
        migratedItems: Int32,
        failedItems: Int32,
        categoriesCreated: Int32,
        errorMessage: Option<String>,
        startedAt: String,
        completedAt: Option<String>
    ) {
        this.taskId = taskId
        this.status = status
        this.migratedItems = migratedItems
        this.failedItems = failedItems
        this.categoriesCreated = categoriesCreated
        this.errorMessage = errorMessage
        this.startedAt = startedAt
        this.completedAt = completedAt
    }
}

/// 主动任务信息
public class ProactiveTaskInfo {
    public var taskId: String
    public var taskType: String
    public var status: OperationStatus
    public var schedule: String
    public var lastRun: Option<String>
    public var nextRun: Option<String>
    public var metadata: Map<String, String>

    public init(
        taskId: String,
        taskType: String,
        status: OperationStatus,
        schedule: String,
        lastRun: Option<String>,
        nextRun: Option<String>,
        metadata: Map<String, String>
    ) {
        this.taskId = taskId
        this.taskType = taskType
        this.status = status
        this.schedule = schedule
        this.lastRun = lastRun
        this.nextRun = nextRun
        this.metadata = metadata
    }
}

/// 调度器统计
public class SchedulerStats {
    public var totalTasks: Int32
    public var runningTasks: Int32
    public var pendingTasks: Int32
    public var completedTasks: Int32
    public var failedTasks: Int32

    public init(
        totalTasks: Int32,
        runningTasks: Int32,
        pendingTasks: Int32,
        completedTasks: Int32,
        failedTasks: Int32
    ) {
        this.totalTasks = totalTasks
        this.runningTasks = runningTasks
        this.pendingTasks = pendingTasks
        this.completedTasks = completedTasks
        this.failedTasks = failedTasks
    }
}

/// 错误响应
public class ErrorResponse {
    public var errorCode: PlatformErrorCode
    public var message: String
    public var details: Map<String, String>

    public init(
        errorCode: PlatformErrorCode,
        message: String,
        details: Map<String, String>
    ) {
        this.errorCode = errorCode
        this.message = message
        this.details = details
    }
}
