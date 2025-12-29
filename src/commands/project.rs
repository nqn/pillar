use anyhow::{Context, Result};
use chrono::Utc;
use std::str::FromStr;

use crate::fs::{ensure_dir, get_base_directory, list_projects as list_all};
use crate::models::{Priority, ProjectMetadata, Status};
use crate::parser::{read_project, write_with_frontmatter};

pub fn create_project(name: &str, priority: &str) -> Result<()> {
    let base_dir = get_base_directory()?;
    let project_path = base_dir.join(name);
    
    if project_path.exists() {
        return Err(anyhow::anyhow!("Project '{}' already exists", name));
    }

    // Create project directory structure
    ensure_dir(&project_path)?;
    ensure_dir(project_path.join("milestones"))?;
    ensure_dir(project_path.join("issues"))?;
    
    // Parse priority
    let priority = Priority::from_str(priority)
        .with_context(|| format!("Invalid priority: {}", priority))?;
    
    // Create project metadata
    let metadata = ProjectMetadata {
        name: name.to_string(),
        status: Status::Backlog,
        priority,
        created: Some(Utc::now()),
        updated: Some(Utc::now()),
    };
    
    // Create README.md
    let description = format!("# {}\n\nProject description goes here.\n\n## Goals\n\n- Goal 1\n- Goal 2\n", name);
    write_with_frontmatter(project_path.join("README.md"), &metadata, &description)?;
    
    println!("✓ Created project '{}' at {}", name, project_path.display());
    println!("\nNext steps:");
    println!("  pillar milestone create {} <milestone-name>", name);
    println!("  pillar issue create {} <issue-title>", name);
    
    Ok(())
}

pub fn list_projects(status_filter: Option<&str>, priority_filter: Option<&str>) -> Result<()> {
    let base_dir = get_base_directory()?;
    let mut projects = list_all(&base_dir)?;
    
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
        projects.retain(|p| p.metadata.status == status);
    }
    
    if let Some(priority) = priority_filter {
        projects.retain(|p| p.metadata.priority == priority);
    }
    
    if projects.is_empty() {
        println!("No projects found.");
        return Ok(());
    }
    
    // Sort by priority (descending) then name
    projects.sort_by(|a, b| {
        b.metadata.priority.cmp(&a.metadata.priority)
            .then_with(|| a.metadata.name.cmp(&b.metadata.name))
    });
    
    println!("Projects:\n");
    for project in projects {
        let issues = crate::fs::list_issues(&project.path).unwrap_or_default();
        let milestones = crate::fs::list_milestones(&project.path).unwrap_or_default();
        
        println!("  {} [{}] [{}]", 
            project.metadata.name,
            format_status(&project.metadata.status),
            format_priority(&project.metadata.priority)
        );
        println!("    {} issues, {} milestones", issues.len(), milestones.len());
    }
    
    Ok(())
}

pub fn show_project(name: &str) -> Result<()> {
    let base_dir = get_base_directory()?;
    let project = crate::fs::find_project(&base_dir, name)?;
    
    let milestones = crate::fs::list_milestones(&project.path)?;
    let issues = crate::fs::list_issues(&project.path)?;
    
    println!("Project: {}", project.metadata.name);
    println!("Status: {}", format_status(&project.metadata.status));
    println!("Priority: {}", format_priority(&project.metadata.priority));
    println!("\n{}", project.description);
    
    // Show milestones
    if !milestones.is_empty() {
        println!("\nMilestones ({}):", milestones.len());
        for milestone in milestones {
            let target = milestone.metadata.target_date
                .as_deref()
                .unwrap_or("no date");
            println!("  • {} [{}] (target: {})",
                milestone.metadata.title,
                format_status(&milestone.metadata.status),
                target
            );
        }
    }
    
    // Show issues grouped by status
    if !issues.is_empty() {
        let completed = issues.iter().filter(|i| i.metadata.status == Status::Completed).count();
        println!("\nIssues ({}/{} completed):", completed, issues.len());
        
        for status in [Status::InProgress, Status::Todo, Status::Backlog, Status::Completed, Status::Cancelled] {
            let status_issues: Vec<_> = issues.iter()
                .filter(|i| i.metadata.status == status)
                .collect();
            
            if !status_issues.is_empty() {
                println!("\n  {}:", format_status(&status));
                for issue in status_issues {
                    println!("    • {} [{}]",
                        issue.metadata.title,
                        format_priority(&issue.metadata.priority)
                    );
                }
            }
        }
    } else {
        println!("\nNo issues yet.");
    }
    
    Ok(())
}

pub fn edit_project(name: &str, status: Option<&str>, priority: Option<&str>) -> Result<()> {
    let base_dir = get_base_directory()?;
    let project = crate::fs::find_project(&base_dir, name)?;
    
    let mut metadata = project.metadata;
    let mut changed = false;
    
    if let Some(s) = status {
        metadata.status = Status::from_str(s)?;
        changed = true;
        println!("Updated status to: {}", format_status(&metadata.status));
    }
    
    if let Some(p) = priority {
        metadata.priority = Priority::from_str(p)?;
        changed = true;
        println!("Updated priority to: {}", format_priority(&metadata.priority));
    }
    
    if !changed {
        return Err(anyhow::anyhow!("No changes specified. Use --status or --priority"));
    }
    
    metadata.updated = Some(Utc::now());
    
    // Write back to file
    write_with_frontmatter(
        project.path.join("README.md"),
        &metadata,
        &project.description
    )?;
    
    println!("✓ Updated project '{}'", name);
    
    Ok(())
}

fn format_status(status: &Status) -> String {
    use colored::Colorize;
    
    match status {
        Status::Backlog => "backlog".white(),
        Status::Todo => "todo".cyan(),
        Status::InProgress => "in-progress".yellow(),
        Status::Completed => "completed".green(),
        Status::Cancelled => "cancelled".red(),
    }.to_string()
}

fn format_priority(priority: &Priority) -> String {
    use colored::Colorize;
    
    match priority {
        Priority::Low => "low".white(),
        Priority::Medium => "medium".cyan(),
        Priority::High => "high".yellow(),
        Priority::Urgent => "urgent".red(),
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    fn setup_workspace() -> Result<TempDir> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;
        
        env::set_current_dir(temp_dir.path())?;
        let result = crate::commands::init(None);
        env::set_current_dir(&original_dir)?;
        
        result?;
        Ok(temp_dir)
    }

    #[test]
    fn test_create_project() -> Result<()> {
        let temp_dir = setup_workspace()?;
        let original_dir = env::current_dir()?;
        
        env::set_current_dir(temp_dir.path())?;
        let create_result = create_project("test-project", "high");
        env::set_current_dir(&original_dir)?;
        
        create_result?;
        
        let project_path = temp_dir.path().join("test-project");
        
        assert!(project_path.exists());
        assert!(project_path.join("README.md").exists());
        assert!(project_path.join("milestones").exists());
        assert!(project_path.join("issues").exists());
        
        let project = read_project(&project_path)?;
        assert_eq!(project.metadata.name, "test-project");
        assert_eq!(project.metadata.priority, Priority::High);
        assert_eq!(project.metadata.status, Status::Backlog);
        
        Ok(())
    }

    #[test]
    fn test_create_duplicate_project_fails() -> Result<()> {
        let temp_dir = setup_workspace()?;
        let original_dir = env::current_dir()?;
        
        env::set_current_dir(temp_dir.path())?;
        let _ = create_project("test-project", "medium");
        let result = create_project("test-project", "medium");
        env::set_current_dir(&original_dir)?;
        
        assert!(result.is_err());
        
        Ok(())
    }

    #[test]
    fn test_edit_project() -> Result<()> {
        let temp_dir = setup_workspace()?;
        let original_dir = env::current_dir()?;
        
        env::set_current_dir(temp_dir.path())?;
        let _ = create_project("test-project", "medium");
        let edit_result = edit_project("test-project", Some("in-progress"), Some("urgent"));
        env::set_current_dir(&original_dir)?;
        
        edit_result?;
        
        let project = crate::fs::find_project(temp_dir.path(), "test-project")?;
        
        assert_eq!(project.metadata.status, Status::InProgress);
        assert_eq!(project.metadata.priority, Priority::Urgent);
        
        Ok(())
    }

    #[test]
    fn test_list_empty_projects() -> Result<()> {
        let temp_dir = setup_workspace()?;
        let original_dir = env::current_dir()?;
        
        env::set_current_dir(temp_dir.path())?;
        let result = list_projects(None, None);
        env::set_current_dir(original_dir)?;
        
        // Should not panic with empty workspace
        result?;
        
        Ok(())
    }
}
