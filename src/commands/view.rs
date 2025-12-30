use anyhow::Result;
use colored::Colorize;

use crate::fs::{get_base_directory, list_all_issues, list_projects};
use crate::models::Status;

pub fn status() -> Result<()> {
    let base_dir = get_base_directory()?;
    let projects = list_projects(&base_dir)?;

    if projects.is_empty() {
        println!("No projects in workspace.");
        return Ok(());
    }

    println!("{}\n", "Workspace Status".bold());

    // Show active projects
    let active_projects: Vec<_> = projects
        .iter()
        .filter(|p| p.metadata.status == Status::InProgress)
        .collect();

    if !active_projects.is_empty() {
        println!("{}", "Active Projects:".bold());
        for project in active_projects {
            let issues = crate::fs::list_issues(&project.path).unwrap_or_default();
            let in_progress = issues
                .iter()
                .filter(|i| i.metadata.status == Status::InProgress)
                .count();
            println!(
                "  • {} ({} issues in progress)",
                project.metadata.name, in_progress
            );
        }
        println!();
    }

    // Show in-progress issues
    let all_issues = list_all_issues(&base_dir)?;
    let in_progress_issues: Vec<_> = all_issues
        .iter()
        .filter(|i| i.metadata.status == Status::InProgress)
        .collect();

    if !in_progress_issues.is_empty() {
        println!("{}", "Issues In Progress:".bold());
        for issue in in_progress_issues {
            let project = issue.metadata.project.as_deref().unwrap_or("unknown");
            println!(
                "  • {} / {} [{}]",
                project,
                issue.metadata.title,
                format_priority(&issue.metadata.priority)
            );
        }
        println!();
    }

    // Show upcoming milestones
    let mut all_milestones = Vec::new();
    for project in &projects {
        let milestones = crate::fs::list_milestones(&project.path)?;
        for milestone in milestones {
            all_milestones.push((project.metadata.name.clone(), milestone));
        }
    }

    // Filter incomplete milestones with dates
    let mut upcoming: Vec<_> = all_milestones
        .iter()
        .filter(|(_, m)| {
            m.metadata.status != Status::Completed
                && m.metadata.status != Status::Cancelled
                && m.metadata.target_date.is_some()
        })
        .collect();

    upcoming.sort_by(|a, b| {
        let date_a = a.1.metadata.target_date.as_deref().unwrap_or("");
        let date_b = b.1.metadata.target_date.as_deref().unwrap_or("");
        date_a.cmp(date_b)
    });

    if !upcoming.is_empty() {
        println!("{}", "Upcoming Milestones:".bold());
        for (project_name, milestone) in upcoming.iter().take(5) {
            let target = milestone.metadata.target_date.as_deref().unwrap_or("");
            println!(
                "  • {} / {} ({})",
                project_name, milestone.metadata.title, target
            );
        }
        println!();
    }

    // Summary stats
    let total_issues = all_issues.len();
    let completed = all_issues
        .iter()
        .filter(|i| i.metadata.status == Status::Completed)
        .count();
    let todo = all_issues
        .iter()
        .filter(|i| i.metadata.status == Status::Todo)
        .count();

    println!("{}", "Summary:".bold());
    println!("  Projects: {}", projects.len());
    println!(
        "  Issues: {} total, {} completed, {} todo",
        total_issues, completed, todo
    );

    Ok(())
}

pub fn board(project_filter: Option<&str>) -> Result<()> {
    let base_dir = get_base_directory()?;

    let issues = if let Some(proj) = project_filter {
        let project_path = base_dir.join(proj);
        crate::fs::list_issues(&project_path)?
    } else {
        list_all_issues(&base_dir)?
    };

    if issues.is_empty() {
        println!("No issues found.");
        return Ok(());
    }

    let title = if let Some(proj) = project_filter {
        format!("Board: {}", proj)
    } else {
        "Board: All Projects".to_string()
    };

    println!("{}\n", title.bold());

    // Group by status
    let statuses = [
        (Status::Backlog, "Backlog"),
        (Status::Todo, "Todo"),
        (Status::InProgress, "In Progress"),
        (Status::Completed, "Completed"),
    ];

    for (status, label) in statuses {
        let status_issues: Vec<_> = issues
            .iter()
            .filter(|i| i.metadata.status == status)
            .collect();

        if status_issues.is_empty() {
            continue;
        }

        println!(
            "{} ({})",
            format_status_label(&status, label),
            status_issues.len()
        );
        println!("{}", "─".repeat(40));

        for issue in status_issues {
            let project = issue.metadata.project.as_deref().unwrap_or("?");
            println!(
                "  • {} / {} [{}]",
                project,
                issue.metadata.title,
                format_priority(&issue.metadata.priority)
            );
        }
        println!();
    }

    Ok(())
}

fn format_status_label(status: &Status, label: &str) -> String {
    use colored::Colorize;

    match status {
        Status::Backlog => label.white(),
        Status::Todo => label.cyan(),
        Status::InProgress => label.yellow(),
        Status::Completed => label.green(),
        Status::Cancelled => label.red(),
    }
    .bold()
    .to_string()
}

fn format_priority(priority: &crate::models::Priority) -> String {
    use crate::models::Priority;
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

    fn setup_workspace_with_data() -> Result<TempDir> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        crate::commands::init(None)?;
        crate::commands::create_project("project-a", "high")?;
        crate::commands::create_issue("project-a", "Issue 1", "high", None, None)?;
        crate::commands::create_issue("project-a", "Issue 2", "medium", None, None)?;
        crate::commands::edit_issue("project-a/001", Some("in-progress"), None, None, None)?;
        env::set_current_dir(&original_dir)?;

        Ok(temp_dir)
    }

    #[test]
    fn test_status_command() -> Result<()> {
        let temp_dir = setup_workspace_with_data()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        let result = status();
        env::set_current_dir(&original_dir)?;

        result?;
        Ok(())
    }

    #[test]
    fn test_board_command() -> Result<()> {
        let temp_dir = setup_workspace_with_data()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        let result = board(Some("project-a"));
        env::set_current_dir(&original_dir)?;

        result?;
        Ok(())
    }

    #[test]
    fn test_board_all_projects() -> Result<()> {
        let temp_dir = setup_workspace_with_data()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        let result = board(None);
        env::set_current_dir(&original_dir)?;

        result?;
        Ok(())
    }
}
