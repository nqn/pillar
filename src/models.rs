use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Workspace configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub version: String,
    #[serde(default = "default_base_directory")]
    pub base_directory: String,
}

fn default_base_directory() -> String {
    ".".to_string()
}

/// Default values for new entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    pub priority: String,
    pub status: String,
}

/// Complete configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub workspace: WorkspaceConfig,
    pub defaults: DefaultConfig,
}

/// Status of a project, milestone, or issue
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Backlog,
    Todo,
    InProgress,
    Completed,
    Cancelled,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Backlog => write!(f, "backlog"),
            Status::Todo => write!(f, "todo"),
            Status::InProgress => write!(f, "in-progress"),
            Status::Completed => write!(f, "completed"),
            Status::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl std::str::FromStr for Status {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "backlog" => Ok(Status::Backlog),
            "todo" => Ok(Status::Todo),
            "in-progress" | "inprogress" => Ok(Status::InProgress),
            "completed" | "done" => Ok(Status::Completed),
            "cancelled" | "canceled" => Ok(Status::Cancelled),
            _ => Err(anyhow::anyhow!("Invalid status: {}", s)),
        }
    }
}

/// Priority level for projects and issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
            Priority::Urgent => write!(f, "urgent"),
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "urgent" => Ok(Priority::Urgent),
            _ => Err(anyhow::anyhow!("Invalid priority: {}", s)),
        }
    }
}

/// Metadata for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub status: Status,
    pub priority: Priority,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTime<Utc>>,
}

/// A project with its content and location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub metadata: ProjectMetadata,
    pub description: String,
    #[serde(skip)]
    pub path: PathBuf,
}

/// Metadata for a milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneMetadata {
    pub title: String,
    pub status: Status,
    pub target_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTime<Utc>>,
}

/// A milestone with its content and location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub metadata: MilestoneMetadata,
    pub description: String,
    #[serde(skip)]
    pub path: PathBuf,
}

/// Metadata for an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueMetadata {
    pub title: String,
    pub status: Status,
    pub priority: Priority,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTime<Utc>>,
}

/// An issue with its content and location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub metadata: IssueMetadata,
    pub description: String,
    #[serde(skip)]
    pub path: PathBuf,
}

/// A comment on a project, milestone, or issue
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Comment {
    pub id: String,
    pub author: String,
    pub timestamp: String,
    pub content: String,
}

impl Comment {
    /// Create a new comment with generated UUID and current timestamp
    pub fn new(author: String, content: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().to_rfc3339();
        Comment {
            id,
            author,
            timestamp,
            content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_from_str() {
        assert_eq!("backlog".parse::<Status>().unwrap(), Status::Backlog);
        assert_eq!("todo".parse::<Status>().unwrap(), Status::Todo);
        assert_eq!("in-progress".parse::<Status>().unwrap(), Status::InProgress);
        assert_eq!("inprogress".parse::<Status>().unwrap(), Status::InProgress);
        assert_eq!("completed".parse::<Status>().unwrap(), Status::Completed);
        assert_eq!("done".parse::<Status>().unwrap(), Status::Completed);
        assert_eq!("cancelled".parse::<Status>().unwrap(), Status::Cancelled);
        assert!("invalid".parse::<Status>().is_err());
    }

    #[test]

    #[test]
    fn test_config_default_base_directory() {
        let config = WorkspaceConfig {
            version: "0.1.0".to_string(),
            base_directory: default_base_directory(),
        };
        assert_eq!(config.base_directory, ".");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config {
            workspace: WorkspaceConfig {
                version: "0.1.0".to_string(),
                base_directory: "pm".to_string(),
            },
            defaults: DefaultConfig {
                priority: "medium".to_string(),
                status: "backlog".to_string(),
            },
        };

        let toml = toml::to_string(&config).unwrap();
        assert!(toml.contains("base_directory"));
        assert!(toml.contains("pm"));

        // Test deserialization
        let parsed: Config = toml::from_str(&toml).unwrap();
        assert_eq!(parsed.workspace.base_directory, "pm");
    }

    #[test]
    fn test_config_backwards_compatibility() {
        // Old config without base_directory should default to "."
        let toml = r#"
[workspace]
version = "0.1.0"

[defaults]
priority = "medium"
status = "backlog"
"#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.workspace.base_directory, ".");
    }
    fn test_priority_from_str() {
        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("urgent".parse::<Priority>().unwrap(), Priority::Urgent);
        assert!("invalid".parse::<Priority>().is_err());
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Low < Priority::Medium);
        assert!(Priority::Medium < Priority::High);
        assert!(Priority::High < Priority::Urgent);
    }

    #[test]
    fn test_status_display() {
        assert_eq!(Status::Backlog.to_string(), "backlog");
        assert_eq!(Status::InProgress.to_string(), "in-progress");
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(Priority::Low.to_string(), "low");
        assert_eq!(Priority::Urgent.to_string(), "urgent");
    }
}
