use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::models::{
    Comment, Issue, IssueMetadata, Milestone, MilestoneMetadata, Project, ProjectMetadata,
};

/// Parse a markdown file with YAML frontmatter
fn parse_frontmatter<T>(content: &str) -> Result<(T, String)>
where
    T: serde::de::DeserializeOwned,
{
    let content = content.trim();
    
    if !content.starts_with("---") {
        return Err(anyhow::anyhow!("File does not start with frontmatter delimiter"));
    }

    let rest = &content[3..];
    let end_pos = rest
        .find("\n---")
        .ok_or_else(|| anyhow::anyhow!("Could not find end of frontmatter"))?;

    let frontmatter = &rest[..end_pos];
    let body = &rest[end_pos + 4..].trim();

    let metadata: T = serde_yaml::from_str(frontmatter)
        .context("Failed to parse frontmatter YAML")?;

    Ok((metadata, body.to_string()))
}

/// Read and parse a project README.md
pub fn read_project<P: AsRef<Path>>(path: P) -> Result<Project> {
    let path = path.as_ref();
    let readme_path = path.join("README.md");
    let content = fs::read_to_string(&readme_path)
        .with_context(|| format!("Failed to read project file: {}", readme_path.display()))?;

    let (metadata, description) = parse_frontmatter::<ProjectMetadata>(&content)
        .context("Failed to parse project metadata")?;

    Ok(Project {
        metadata,
        description,
        path: path.to_path_buf(),
    })
}

/// Read and parse a milestone file
pub fn read_milestone<P: AsRef<Path>>(path: P) -> Result<Milestone> {
    let path = path.as_ref();
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read milestone file: {}", path.display()))?;

    let (metadata, description) = parse_frontmatter::<MilestoneMetadata>(&content)
        .context("Failed to parse milestone metadata")?;

    Ok(Milestone {
        metadata,
        description,
        path: path.to_path_buf(),
    })
}

/// Read and parse an issue file
pub fn read_issue<P: AsRef<Path>>(path: P) -> Result<Issue> {
    let path = path.as_ref();
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read issue file: {}", path.display()))?;

    let (metadata, description) = parse_frontmatter::<IssueMetadata>(&content)
        .context("Failed to parse issue metadata")?;

    Ok(Issue {
        metadata,
        description,
        path: path.to_path_buf(),
    })
}

/// Write a markdown file with YAML frontmatter
pub fn write_with_frontmatter<T, P>(path: P, metadata: &T, body: &str) -> Result<()>
where
    T: serde::Serialize,
    P: AsRef<Path>,
{
    let frontmatter = serde_yaml::to_string(metadata)
        .context("Failed to serialize metadata")?;
    
    let content = format!("---\n{}---\n\n{}", frontmatter, body.trim());
    
    fs::write(path.as_ref(), content)
        .with_context(|| format!("Failed to write file: {}", path.as_ref().display()))?;
    
    Ok(())
}

/// Read comments from a markdown file body
/// Comments are in a ## Comments section with format:
/// ### [timestamp] - author
/// comment content
pub fn read_comments(body: &str) -> Vec<Comment> {
    let mut comments = Vec::new();
    
    // Find the ## Comments section
    let comments_section = if let Some(pos) = body.find("\n## Comments\n") {
        &body[pos + 13..] // Skip "\n## Comments\n"
    } else if let Some(stripped) = body.strip_prefix("## Comments\n") {
        stripped
    } else {
        return comments;
    };
    
    // Split by ### headings
    let mut current_comment: Option<Comment> = None;
    let mut content_lines = Vec::new();
    
    for line in comments_section.lines() {
        if line.starts_with("### [") {
            // Save previous comment if any
            if let Some(mut comment) = current_comment.take() {
                comment.content = content_lines.join("\n").trim().to_string();
                comments.push(comment);
                content_lines.clear();
            }
            
            // Parse new comment header: ### [timestamp] - author
            if let Some(close_bracket) = line.find(']') {
                let timestamp = line[5..close_bracket].to_string(); // Skip "### ["
                let rest = &line[close_bracket + 1..];
                
                let author = if let Some(dash_pos) = rest.find(" - ") {
                    rest[dash_pos + 3..].trim().to_string()
                } else {
                    "Unknown".to_string()
                };
                
                current_comment = Some(Comment {
                    id: uuid::Uuid::new_v4().to_string(), // Generate new ID on read
                    author,
                    timestamp,
                    content: String::new(),
                });
            }
        } else if line.starts_with("## ") {
            // Another section started, stop reading comments
            break;
        } else if current_comment.is_some() {
            // Accumulate content lines
            content_lines.push(line);
        }
    }
    
    // Save last comment if any
    if let Some(mut comment) = current_comment {
        comment.content = content_lines.join("\n").trim().to_string();
        comments.push(comment);
    }
    
    comments
}

/// Write comments section to markdown body
/// Returns the body with comments section appended
pub fn write_comments(body: &str, comments: &[Comment]) -> String {
    // Remove existing comments section if present
    let body_without_comments = if let Some(pos) = body.find("\n## Comments\n") {
        body[..pos].trim_end()
    } else if body.starts_with("## Comments\n") {
        ""
    } else {
        body.trim_end()
    };
    
    if comments.is_empty() {
        return body_without_comments.to_string();
    }
    
    let mut result = body_without_comments.to_string();
    result.push_str("\n\n## Comments\n");
    
    for comment in comments {
        result.push_str(&format!("\n### [{}] - {}\n", comment.timestamp, comment.author));
        result.push_str(&comment.content);
        result.push('\n');
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Priority, Status};
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_frontmatter_valid() {
        let content = r#"---
title: "Test Issue"
status: todo
priority: high
tags: [bug, urgent]
---

This is the issue description.
"#;

        let result: Result<(IssueMetadata, String)> = parse_frontmatter(content);
        assert!(result.is_ok());
        
        let (metadata, body) = result.unwrap();
        assert_eq!(metadata.title, "Test Issue");
        assert_eq!(metadata.status, Status::Todo);
        assert_eq!(metadata.priority, Priority::High);
        assert_eq!(metadata.tags, vec!["bug", "urgent"]);
        assert_eq!(body, "This is the issue description.");
    }

    #[test]
    fn test_parse_frontmatter_missing_delimiter() {
        let content = "No frontmatter here";
        let result: Result<(IssueMetadata, String)> = parse_frontmatter(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_frontmatter_invalid_yaml() {
        let content = r#"---
invalid: yaml: structure:
---

Body
"#;
        let result: Result<(IssueMetadata, String)> = parse_frontmatter(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_with_frontmatter() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        
        let metadata = IssueMetadata {
            title: "Test".to_string(),
            status: Status::Todo,
            priority: Priority::Medium,
            project: None,
            milestone: None,
            tags: vec![],
            created: None,
            updated: None,
        };

        write_with_frontmatter(temp_file.path(), &metadata, "Test body")?;

        let content = fs::read_to_string(temp_file.path())?;
        assert!(content.starts_with("---\n"));
        assert!(content.contains("title: Test"));
        assert!(content.contains("Test body"));

        Ok(())
    }

    #[test]
    fn test_read_write_roundtrip() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        
        let original_metadata = IssueMetadata {
            title: "Roundtrip Test".to_string(),
            status: Status::InProgress,
            priority: Priority::High,
            project: Some("test-project".to_string()),
            milestone: Some("v1.0".to_string()),
            tags: vec!["test".to_string(), "roundtrip".to_string()],
            created: None,
            updated: None,
        };

        let body = "This is a test issue.\n\nWith multiple lines.";

        write_with_frontmatter(temp_file.path(), &original_metadata, body)?;
        
        let issue = read_issue(temp_file.path())?;
        
        assert_eq!(issue.metadata.title, original_metadata.title);
        assert_eq!(issue.metadata.status, original_metadata.status);
        assert_eq!(issue.metadata.priority, original_metadata.priority);
        assert_eq!(issue.description, body);

        Ok(())
    }

    #[test]
    fn test_read_comments_empty() {
        let body = "# Issue Description\n\nSome content here.";
        let comments = read_comments(body);
        assert_eq!(comments.len(), 0);
    }

    #[test]
    fn test_read_comments_single() {
        let body = r#"# Issue Description

Some content here.

## Comments

### [2025-12-29T10:30:00Z] - Alice
This is a comment
"#;
        let comments = read_comments(body);
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].author, "Alice");
        assert_eq!(comments[0].timestamp, "2025-12-29T10:30:00Z");
        assert_eq!(comments[0].content, "This is a comment");
    }

    #[test]
    fn test_read_comments_multiple() {
        let body = r#"# Issue Description

## Comments

### [2025-12-29T10:30:00Z] - Alice
First comment

### [2025-12-29T14:15:00Z] - Bob
Second comment
with multiple lines

### [2025-12-29T16:45:00Z] - Charlie
Third comment
"#;
        let comments = read_comments(body);
        assert_eq!(comments.len(), 3);
        assert_eq!(comments[0].author, "Alice");
        assert_eq!(comments[1].author, "Bob");
        assert_eq!(comments[1].content, "Second comment\nwith multiple lines");
        assert_eq!(comments[2].author, "Charlie");
    }

    #[test]
    fn test_write_comments_empty() {
        let body = "# Issue Description\n\nSome content.";
        let result = write_comments(body, &[]);
        assert_eq!(result, "# Issue Description\n\nSome content.");
    }

    #[test]
    fn test_write_comments_adds_section() {
        let body = "# Issue Description\n\nSome content.";
        let comments = vec![
            Comment {
                id: "1".to_string(),
                author: "Alice".to_string(),
                timestamp: "2025-12-29T10:30:00Z".to_string(),
                content: "Test comment".to_string(),
            },
        ];
        let result = write_comments(body, &comments);
        assert!(result.contains("## Comments"));
        assert!(result.contains("### [2025-12-29T10:30:00Z] - Alice"));
        assert!(result.contains("Test comment"));
    }

    #[test]
    fn test_write_comments_replaces_existing() {
        let body = r#"# Issue Description

## Comments

### [2025-12-29T10:00:00Z] - OldUser
Old comment
"#;
        let comments = vec![
            Comment {
                id: "2".to_string(),
                author: "NewUser".to_string(),
                timestamp: "2025-12-29T11:00:00Z".to_string(),
                content: "New comment".to_string(),
            },
        ];
        let result = write_comments(body, &comments);
        assert!(!result.contains("OldUser"));
        assert!(result.contains("NewUser"));
        assert!(result.contains("New comment"));
    }
}
