/*
 * Copyright (c) AgentMem Team 2024. All rights reserved.
 */

/**
 * @file types.cj
 * HTTP类型定义 - AgentMem 仓颉 SDK HTTP方案
 */

package agentmem_http

/// 最小Map实现，仅覆盖 http_new 包当前使用的构造与 put 语义
public class Map<K, V> {
    private var pairs: Array<(K, V)>

    public init() {
        this.pairs = Array<(K, V)>()
    }

    public func put(key: K, value: V): Unit {
        let newPairs = Array<(K, V)>(this.pairs.size + 1, {_ => (key, value)})
        for (i in 0..this.pairs.size) {
            newPairs[i] = this.pairs[i]
        }
        newPairs[this.pairs.size] = (key, value)
        this.pairs = newPairs
    }
}

/// HTTP错误类型
public enum AgentMemError {
    /// 网络错误
    | NetworkError(String)
    /// HTTP状态码错误
    | HttpError(Int32, String)
    /// JSON解析错误
    | JsonParseError(String)
    /// 数据验证错误
    | ValidationError(String)
    /// 404错误
    | NotFoundError(String)
    /// 401/403错误
    | UnauthorizedError(String)
    /// 500服务器错误
    | ServerError(String)
    
    /// 获取错误消息
    public func getMessage(): String {
        return match (this) {
            case NetworkError(msg) => "网络错误: ${msg}"
            case HttpError(code, msg) => "HTTP错误 ${code}: ${msg}"
            case JsonParseError(msg) => "JSON解析错误: ${msg}"
            case ValidationError(msg) => "验证错误: ${msg}"
            case NotFoundError(msg) => "未找到: ${msg}"
            case UnauthorizedError(msg) => "未授权: ${msg}"
            case ServerError(msg) => "服务器错误: ${msg}"
        }
    }
}

/// HTTP响应结果类型
public enum Result<T, E> {
    | Ok(T)
    | Err(E)
    
    /// 是否成功
    public func isOk(): Bool {
        return match (this) {
            case Ok(_) => true
            case Err(_) => false
        }
    }
    
    /// 是否失败
    public func isErr(): Bool {
        !this.isOk()
    }
    
    /// 解包，失败时抛出异常
    public func getOrThrow(): T {
        return match (this) {
            case Ok(value) => value
            case Err(e) =>
                // 简化实现：返回默认值或抛出错误
                // 注：实际应使用异常机制
                throw Exception("Result unwrap failed")
        }
    }
}

/// HTTP状态码映射
public func mapHttpError(code: Int32, message: String): AgentMemError {
    if (code >= 400 && code < 500) {
        if (code == 401 || code == 403) {
            return UnauthorizedError(message)
        } else if (code == 404) {
            return NotFoundError(message)
        } else {
            return ValidationError(message)
        }
    } else if (code >= 500 && code < 600) {
        return ServerError(message)
    } else {
        return HttpError(code, message)
    }
}
