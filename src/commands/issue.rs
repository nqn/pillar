use anyhow::{Context, Result};
use chrono::Utc;
use std::str::FromStr;

use crate::fs::{ensure_dir, generate_issue_id, get_base_directory, list_all_issues};
use crate::models::{IssueMetadata, Priority, Status};
use crate::parser::write_with_frontmatter;

pub fn create_issue(
    project_name: &str,
    title: &str,
    priority: &str,
    milestone: Option<&str>,
    tags: Option<&str>,
) -> Result<()> {
    let base_dir = get_base_directory()?;
    let project_path = base_dir.join(project_name);

    if !project_path.exists() {
        return Err(anyhow::anyhow!("Project '{}' does not exist", project_name));
    }

    let issues_dir = project_path.join("issues");
    ensure_dir(&issues_dir)?;

    // Generate issue ID
    let issue_id = generate_issue_id(&project_path)?;
    let sanitized_title = sanitize_filename(title);
    let issue_path = issues_dir.join(format!("{}-{}.md", issue_id, sanitized_title));

    // Parse priority
    let priority =
        Priority::from_str(priority).with_context(|| format!("Invalid priority: {}", priority))?;

    // Parse tags
    let tag_list = if let Some(t) = tags {
        t.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };

    // Create issue metadata
    let metadata = IssueMetadata {
        title: title.to_string(),
        status: Status::Todo,
        priority,
        project: Some(project_name.to_string()),
        milestone: milestone.map(|s| s.to_string()),
        tags: tag_list,
        created: Some(Utc::now()),
        updated: Some(Utc::now()),
    };

    // Create issue description
    let description = format!(
        "# {}\n\n## Description\n\nDetailed issue description.\n\n## Acceptance Criteria\n\n- [ ] Criterion 1\n- [ ] Criterion 2\n",
        title
    );
    write_with_frontmatter(&issue_path, &metadata, &description)?;

    println!(
        "✓ Created issue '{}/{}' - {}",
        project_name, issue_id, title
    );
    if let Some(m) = milestone {
        println!("  Milestone: {}", m);
    }

    Ok(())
}

pub fn list_issues(
    status_filter: Option<&str>,
    priority_filter: Option<&str>,
    project_filter: Option<&str>,
    milestone_filter: Option<&str>,
    tag_filter: Option<&str>,
) -> Result<()> {
    let base_dir = get_base_directory()?;
    let mut issues = if let Some(proj) = project_filter {
        let project_path = base_dir.join(proj);
        crate::fs::list_issues(&project_path)?
    } else {
        list_all_issues(&base_dir)?
    };

    // Parse filters
    let status_filter = if let Some(s) = status_filter {
        Some(Status::from_str(s)?)
    } else {
        None
    };

    let priority_filter = if let Some(p) = priority_filter {
        Some(Priority::from_str(p)?)
    } else {
        None
    };

    // Apply filters
    if let Some(status) = status_filter {
        issues.retain(|i| i.metadata.status == status);
    }

    if let Some(priority) = priority_filter {
        issues.retain(|i| i.metadata.priority == priority);
    }

    if let Some(milestone) = milestone_filter {
        issues.retain(|i| i.metadata.milestone.as_deref() == Some(milestone));
    }

    if let Some(tag) = tag_filter {
        issues.retain(|i| i.metadata.tags.contains(&tag.to_string()));
    }

    if issues.is_empty() {
        println!("No issues found.");
        return Ok(());
    }

    // Sort by priority (descending) then title
    issues.sort_by(|a, b| {
        b.metadata
            .priority
            .cmp(&a.metadata.priority)
            .then_with(|| a.metadata.title.cmp(&b.metadata.title))
    });

    println!("Issues:\n");
    for issue in issues {
        let project = issue.metadata.project.as_deref().unwrap_or("unknown");
        let issue_id = extract_issue_id(&issue.path);

        println!(
            "  {}/{} - {} [{}] [{}]",
            project,
            issue_id,
            issue.metadata.title,
            format_status(&issue.metadata.status),
            format_priority(&issue.metadata.priority)
        );

        if let Some(milestone) = &issue.metadata.milestone {
            println!("    Milestone: {}", milestone);
        }

        if !issue.metadata.tags.is_empty() {
            println!("    Tags: {}", issue.metadata.tags.join(", "));
        }
    }

    Ok(())
}

pub fn show_issue(id: &str) -> Result<()> {
    let base_dir = get_base_directory()?;

    // Parse ID as "project-name/001" or just "001" if in project dir
    let (project_name, issue_id) = if let Some((proj, id)) = id.split_once('/') {
        (proj, id)
    } else {
        return Err(anyhow::anyhow!(
            "Issue ID must be in format 'project-name/001'"
        ));
    };

    let project_path = base_dir.join(project_name);
    if !project_path.exists() {
        return Err(anyhow::anyhow!("Project '{}' does not exist", project_name));
    }

    // Find issue file by ID
    let issues = crate::fs::list_issues(&project_path)?;
    let issue = issues
        .into_iter()
        .find(|i| extract_issue_id(&i.path) == issue_id)
        .ok_or_else(|| anyhow::anyhow!("Issue '{}' not found", id))?;

    println!(
        "Issue: {}/{} - {}",
        project_name, issue_id, issue.metadata.title
    );
    println!("Status: {}", format_status(&issue.metadata.status));
    println!("Priority: {}", format_priority(&issue.metadata.priority));

    if let Some(milestone) = &issue.metadata.milestone {
        println!("Milestone: {}", milestone);
    }

    if !issue.metadata.tags.is_empty() {
        println!("Tags: {}", issue.metadata.tags.join(", "));
    }

    println!("\n{}", issue.description);

    Ok(())
}

pub fn edit_issue(
    id: &str,
    status: Option<&str>,
    priority: Option<&str>,
    milestone: Option<&str>,
    tags: Option<&str>,
) -> Result<()> {
    let base_dir = get_base_directory()?;

    // Parse ID
    let (project_name, issue_id) = if let Some((proj, id)) = id.split_once('/') {
        (proj, id)
    } else {
        return Err(anyhow::anyhow!(
            "Issue ID must be in format 'project-name/001'"
        ));
    };

    let project_path = base_dir.join(project_name);
    if !project_path.exists() {
        return Err(anyhow::anyhow!("Project '{}' does not exist", project_name));
    }

    // Find issue
    let issues = crate::fs::list_issues(&project_path)?;
    let issue = issues
        .into_iter()
        .find(|i| extract_issue_id(&i.path) == issue_id)
        .ok_or_else(|| anyhow::anyhow!("Issue '{}' not found", id))?;

    let mut metadata = issue.metadata;
    let mut changed = false;

    if let Some(s) = status {
        metadata.status = Status::from_str(s)?;
        changed = true;
        println!("Updated status to: {}", format_status(&metadata.status));
    }

    if let Some(p) = priority {
        metadata.priority = Priority::from_str(p)?;
        changed = true;
        println!(
            "Updated priority to: {}",
            format_priority(&metadata.priority)
        );
    }

    if let Some(m) = milestone {
        metadata.milestone = if m.is_empty() {
            None
        } else {
            Some(m.to_string())
        };
        changed = true;
        println!("Updated milestone to: {}", m);
    }

    if let Some(t) = tags {
        metadata.tags = t.split(',').map(|s| s.trim().to_string()).collect();
        changed = true;
        println!("Updated tags to: {}", metadata.tags.join(", "));
    }

    if !changed {
        return Err(anyhow::anyhow!(
            "No changes specified. Use --status, --priority, --milestone, or --tags"
        ));
    }

    metadata.updated = Some(Utc::now());

    // Write back to file
    write_with_frontmatter(&issue.path, &metadata, &issue.description)?;

    println!("✓ Updated issue '{}'", id);

    Ok(())
}

fn sanitize_filename(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .take(40) // Limit filename length
        .collect::<String>()
        .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "-")
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}

fn extract_issue_id(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| s.split('-').next())
        .unwrap_or("unknown")
        .to_string()
}

fn format_status(status: &Status) -> String {
    use colored::Colorize;

    match status {
        Status::Backlog => "backlog".white(),
        Status::Todo => "todo".cyan(),
        Status::InProgress => "in-progress".yellow(),
        Status::Completed => "completed".green(),
        Status::Cancelled => "cancelled".red(),
    }
    .to_string()
}

fn format_priority(priority: &Priority) -> String {
    use colored::Colorize;

    match priority {
        Priority::Low => "low".white(),
        Priority::Medium => "medium".cyan(),
        Priority::High => "high".yellow(),
        Priority::Urgent => "urgent".red(),
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    fn setup_workspace_with_project() -> Result<(TempDir, String)> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        crate::commands::init(None)?;
        crate::commands::create_project("test-project", "medium")?;
        env::set_current_dir(&original_dir)?;

        Ok((temp_dir, "test-project".to_string()))
    }

    #[test]
    fn test_create_issue() -> Result<()> {
        let (temp_dir, project_name) = setup_workspace_with_project()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        let result = create_issue(
            &project_name,
            "Fix critical bug",
            "urgent",
            Some("v1.0"),
            Some("bug,critical"),
        );
        env::set_current_dir(&original_dir)?;

        result?;

        let issues_dir = temp_dir.path().join(&project_name).join("issues");
        assert!(issues_dir.exists());

        let issues = crate::fs::list_issues(&temp_dir.path().join(&project_name))?;
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].metadata.title, "Fix critical bug");
        assert_eq!(issues[0].metadata.priority, Priority::Urgent);
        assert_eq!(issues[0].metadata.milestone, Some("v1.0".to_string()));
        assert_eq!(issues[0].metadata.tags, vec!["bug", "critical"]);

        Ok(())
    }

    #[test]
    fn test_edit_issue() -> Result<()> {
        let (temp_dir, project_name) = setup_workspace_with_project()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        create_issue(&project_name, "Test issue", "medium", None, None)?;
        let result = edit_issue(
            "test-project/001",
            Some("in-progress"),
            Some("high"),
            Some("v2.0"),
            None,
        );
        env::set_current_dir(&original_dir)?;

        result?;

        let issues = crate::fs::list_issues(&temp_dir.path().join(&project_name))?;
        assert_eq!(issues[0].metadata.status, Status::InProgress);
        assert_eq!(issues[0].metadata.priority, Priority::High);
        assert_eq!(issues[0].metadata.milestone, Some("v2.0".to_string()));

        Ok(())
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Fix critical bug"), "fix-critical-bug");
        assert_eq!(
            sanitize_filename("Add new feature: authentication"),
            "add-new-feature-authentication"
        );
        assert_eq!(sanitize_filename("Bug #123"), "bug-123");
    }

    #[test]
    fn test_list_issues_with_filters() -> Result<()> {
        let (temp_dir, project_name) = setup_workspace_with_project()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        create_issue(&project_name, "Issue 1", "high", None, Some("bug"))?;
        create_issue(&project_name, "Issue 2", "low", None, Some("feature"))?;
        let result = list_issues(None, Some("high"), None, None, None);
        env::set_current_dir(&original_dir)?;

        result?;
        Ok(())
    }
}
