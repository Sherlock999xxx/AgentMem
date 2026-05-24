//! 记忆作用域 (MemoryScope) 定义
//!
//! 统一的多租户记忆隔离方案，支持灵活的层级访问控制。

use serde::{Deserialize, Serialize};
use std::fmt;

/// 记忆作用域枚举 - 统一的多租户记忆隔离方案
///
/// 层级结构: Global > Organization > User > Agent > Run > Session
/// 向下包容: Agent 作用域可访问 User 及 Organization 的记忆
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MemoryScope {
    /// 全局作用域 - 公共知识和系统配置（所有用户共享）
    Global,
    /// 组织级作用域 - 企业多租户隔离
    Organization {
        /// 组织 ID
        org_id: String,
    },
    /// 用户级作用域 - 单用户 AI 助手
    User {
        /// 可选的组织 ID（如果属于某个组织）
        org_id: Option<String>,
        /// 用户 ID
        user_id: String,
    },
    /// Agent 级作用域 - 多 Agent 系统
    Agent {
        /// 可选的组织 ID（如果属于某个组织）
        org_id: Option<String>,
        /// 用户 ID
        user_id: String,
        /// Agent ID
        agent_id: String,
    },
    /// 运行级作用域 - 特定任务/会话
    Run {
        /// 可选的组织 ID（如果属于某个组织）
        org_id: Option<String>,
        /// 用户 ID
        user_id: String,
        /// Agent ID
        agent_id: String,
        /// Run ID
        run_id: String,
    },
    /// 会话级作用域 - 多窗口对话
    Session {
        /// 可选的组织 ID（如果属于某个组织）
        org_id: Option<String>,
        /// 用户 ID
        user_id: String,
        /// Agent ID
        agent_id: String,
        /// 会话 ID
        session_id: String,
    },
}

impl MemoryScope {
    /// 获取作用域层级深度（用于排序和比较）
    pub fn level(&self) -> u8 {
        match self {
            MemoryScope::Global => 0,
            MemoryScope::Organization { .. } => 1,
            MemoryScope::User { .. } => 2,
            MemoryScope::Agent { .. } => 3,
            MemoryScope::Run { .. } => 4,
            MemoryScope::Session { .. } => 5,
        }
    }

    /// 获取访问路径（向上访问链）
    ///
    /// 例如：Session -> [Session, Run, Agent, User, Organization, Global]
    pub fn access_path(&self) -> Vec<MemoryScope> {
        let mut path = vec![self.clone()];
        let mut current = self.parent();
        while let Some(p) = current {
            path.push(p.clone());
            current = p.parent();
        }
        path
    }

    /// 获取父作用域
    pub fn parent(&self) -> Option<MemoryScope> {
        match self {
            MemoryScope::Global => None,
            MemoryScope::Organization { .. } => Some(MemoryScope::Global),
            MemoryScope::User { org_id, .. } => Some(
                org_id.as_ref().map(|id| MemoryScope::Organization {
                    org_id: id.clone(),
                }),
            ).unwrap_or(Some(MemoryScope::Global)),
            MemoryScope::Agent { org_id, user_id, .. } => Some(
                MemoryScope::User {
                    org_id: org_id.clone(),
                    user_id: user_id.clone(),
                },
            ),
            MemoryScope::Run { org_id, user_id, agent_id, .. } => Some(
                MemoryScope::Agent {
                    org_id: org_id.clone(),
                    user_id: user_id.clone(),
                    agent_id: agent_id.clone(),
                },
            ),
            MemoryScope::Session { org_id, user_id, agent_id, .. } => Some(
                MemoryScope::Run {
                    org_id: org_id.clone(),
                    user_id: user_id.clone(),
                    agent_id: agent_id.clone(),
                    run_id: "default".to_string(),
                },
            ),
        }
    }

    /// 是否可以访问目标作用域的记忆
    pub fn can_access(&self, target: &MemoryScope) -> bool {
        // Global 可以访问所有
        matches!(self, MemoryScope::Global) ||
        // 同级可以互相访问
        self == target ||
        // 检查是否在访问路径上
        target.is_descendant_of(self)
    }

    /// 是否是某作用域的后代
    pub fn is_descendant_of(&self, ancestor: &MemoryScope) -> bool {
        let mut current = self.parent();
        while let Some(p) = current {
            if p == *ancestor {
                return true;
            }
            current = p.parent();
        }
        false
    }

    /// 获取组织 ID（如果有）
    pub fn org_id(&self) -> Option<&String> {
        match self {
            MemoryScope::Organization { org_id } => Some(org_id),
            MemoryScope::User { org_id, .. } => org_id.as_ref(),
            MemoryScope::Agent { org_id, .. } => org_id.as_ref(),
            MemoryScope::Run { org_id, .. } => org_id.as_ref(),
            MemoryScope::Session { org_id, .. } => org_id.as_ref(),
            MemoryScope::Global => None,
        }
    }

    /// 获取用户 ID（如果有）
    pub fn user_id(&self) -> Option<&String> {
        match self {
            MemoryScope::User { user_id, .. } => Some(user_id),
            MemoryScope::Agent { user_id, .. } => Some(user_id),
            MemoryScope::Run { user_id, .. } => Some(user_id),
            MemoryScope::Session { user_id, .. } => Some(user_id),
            _ => None,
        }
    }

    /// 获取 Agent ID（如果有）
    pub fn agent_id(&self) -> Option<&String> {
        match self {
            MemoryScope::Agent { agent_id, .. } => Some(agent_id),
            MemoryScope::Run { agent_id, .. } => Some(agent_id),
            MemoryScope::Session { agent_id, .. } => Some(agent_id),
            _ => None,
        }
    }

    /// 转换为唯一字符串标识
    pub fn as_key(&self) -> String {
        match self {
            MemoryScope::Global => "global".to_string(),
            MemoryScope::Organization { org_id } => format!("org:{}", org_id),
            MemoryScope::User { org_id, user_id } => {
                if let Some(o) = org_id {
                    format!("org:{}:user:{}", o, user_id)
                } else {
                    format!("user:{}", user_id)
                }
            }
            MemoryScope::Agent { org_id, user_id, agent_id } => {
                let mut key = format!("agent:{}", agent_id);
                if let Some(o) = org_id {
                    key = format!("org:{}:{}", o, key);
                }
                key = format!("{}:user:{}", key, user_id);
                key
            }
            MemoryScope::Run { org_id, user_id, agent_id, run_id } => {
                let mut key = format!("run:{}", run_id);
                if let Some(o) = org_id {
                    key = format!("org:{}:{}", o, key);
                }
                key = format!("{}:agent:{}:user:{}", key, agent_id, user_id);
                key
            }
            MemoryScope::Session { org_id, user_id, agent_id, session_id } => {
                let mut key = format!("session:{}", session_id);
                if let Some(o) = org_id {
                    key = format!("org:{}:{}", o, key);
                }
                key = format!("{}:agent:{}:user:{}", key, agent_id, user_id);
                key
            }
        }
    }

    /// 从字符串解析（用于配置和序列化）
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(':').collect();

        match parts[0] {
            "global" if parts.len() == 1 => Some(MemoryScope::Global),
            "org" if parts.len() == 2 => Some(MemoryScope::Organization {
                org_id: parts[1].to_string(),
            }),
            "user" if parts.len() == 2 => Some(MemoryScope::User {
                org_id: None,
                user_id: parts[1].to_string(),
            }),
            "user" if parts.len() == 4 && parts[2] == "org" => Some(MemoryScope::User {
                org_id: Some(parts[3].to_string()),
                user_id: parts[1].to_string(),
            }),
            "agent" if parts.len() == 4 => Some(MemoryScope::Agent {
                org_id: None,
                user_id: parts[1].to_string(),
                agent_id: parts[3].to_string(),
            }),
            "agent" if parts.len() == 6 && parts[2] == "org" => Some(MemoryScope::Agent {
                org_id: Some(parts[3].to_string()),
                user_id: parts[4].to_string(),
                agent_id: parts[5].to_string(),
            }),
            _ => None,
        }
    }
}

impl fmt::Display for MemoryScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_key())
    }
}

impl Default for MemoryScope {
    fn default() -> Self {
        MemoryScope::Global
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_level() {
        assert_eq!(MemoryScope::Global.level(), 0);
        assert_eq!(MemoryScope::Organization { org_id: "org1".to_string() }.level(), 1);
        assert_eq!(MemoryScope::User { org_id: None, user_id: "user1".to_string() }.level(), 2);
        assert_eq!(MemoryScope::Agent { org_id: None, user_id: "user1".to_string(), agent_id: "agent1".to_string() }.level(), 3);
        assert_eq!(MemoryScope::Run { org_id: None, user_id: "user1".to_string(), agent_id: "agent1".to_string(), run_id: "run1".to_string() }.level(), 4);
        assert_eq!(MemoryScope::Session { org_id: None, user_id: "user1".to_string(), agent_id: "agent1".to_string(), session_id: "sess1".to_string() }.level(), 5);
    }

    #[test]
    fn test_access_path() {
        let scope = MemoryScope::Session {
            org_id: Some("org1".to_string()),
            user_id: "user1".to_string(),
            agent_id: "agent1".to_string(),
            session_id: "sess1".to_string(),
        };

        let path = scope.access_path();
        assert_eq!(path.len(), 6); // session, run, agent, user, org, global
    }

    #[test]
    fn test_can_access() {
        let session = MemoryScope::Session {
            org_id: Some("org1".to_string()),
            user_id: "user1".to_string(),
            agent_id: "agent1".to_string(),
            session_id: "sess1".to_string(),
        };

        let user = MemoryScope::User {
            org_id: Some("org1".to_string()),
            user_id: "user1".to_string(),
        };

        assert!(session.can_access(&user));
        assert!(session.can_access(&MemoryScope::Global));
        assert!(!user.can_access(&session));
    }

    #[test]
    fn test_parse_roundtrip() {
        let original = MemoryScope::Session {
            org_id: Some("org1".to_string()),
            user_id: "user1".to_string(),
            agent_id: "agent1".to_string(),
            session_id: "sess1".to_string(),
        };

        let key = original.as_key();
        let parsed = MemoryScope::parse(&key).unwrap();

        assert_eq!(original, parsed);
    }
}