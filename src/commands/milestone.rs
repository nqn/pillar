use anyhow::Result;
use chrono::Utc;
use std::str::FromStr;

use crate::fs::{ensure_dir, get_base_directory};
use crate::models::{MilestoneMetadata, Status};
use crate::parser::write_with_frontmatter;

pub fn create_milestone(project_name: &str, title: &str, date: Option<&str>) -> Result<()> {
    let base_dir = get_base_directory()?;
    let project_path = base_dir.join(project_name);

    if !project_path.exists() {
        return Err(anyhow::anyhow!("Project '{}' does not exist", project_name));
    }

    let milestones_dir = project_path.join("milestones");
    ensure_dir(&milestones_dir)?;

    // Create sanitized filename from title
    let filename = sanitize_filename(title);
    let milestone_path = milestones_dir.join(format!("{}.md", filename));

    if milestone_path.exists() {
        return Err(anyhow::anyhow!("Milestone '{}' already exists", title));
    }

    // Create milestone metadata
    let metadata = MilestoneMetadata {
        title: title.to_string(),
        status: Status::Backlog,
        target_date: date.map(|s| s.to_string()),
        project: Some(project_name.to_string()),
        created: Some(Utc::now()),
        updated: Some(Utc::now()),
    };

    // Create milestone description
    let description = format!("# {}\n\nMilestone description and objectives.\n", title);
    write_with_frontmatter(&milestone_path, &metadata, &description)?;

    println!(
        "✓ Created milestone '{}' in project '{}'",
        title, project_name
    );
    if let Some(d) = date {
        println!("  Target date: {}", d);
    }

    Ok(())
}

pub fn list_milestones(project_filter: Option<&str>) -> Result<()> {
    let base_dir = get_base_directory()?;
    let projects = if let Some(name) = project_filter {
        vec![crate::fs::find_project(&base_dir, name)?]
    } else {
        crate::fs::list_projects(&base_dir)?
    };

    let mut all_milestones = Vec::new();

    for project in projects {
        let milestones = crate::fs::list_milestones(&project.path)?;
        for milestone in milestones {
            all_milestones.push((project.metadata.name.clone(), milestone));
        }
    }

    if all_milestones.is_empty() {
        println!("No milestones found.");
        return Ok(());
    }

    // Sort by target date then title
    all_milestones.sort_by(|a, b| {
        let date_a = a.1.metadata.target_date.as_deref().unwrap_or("9999-12-31");
        let date_b = b.1.metadata.target_date.as_deref().unwrap_or("9999-12-31");
        date_a
            .cmp(date_b)
            .then_with(|| a.1.metadata.title.cmp(&b.1.metadata.title))
    });

    println!("Milestones:\n");
    for (project_name, milestone) in all_milestones {
        let target = milestone
            .metadata
            .target_date
            .as_deref()
            .unwrap_or("no date");
        println!(
            "  {} / {} [{}]",
            project_name,
            milestone.metadata.title,
            format_status(&milestone.metadata.status)
        );
        println!("    Target: {}", target);
    }

    Ok(())
}

pub fn edit_milestone(
    project_name: &str,
    title: &str,
    status: Option<&str>,
    date: Option<&str>,
) -> Result<()> {
    let base_dir = get_base_directory()?;
    let project_path = base_dir.join(project_name);

    if !project_path.exists() {
        return Err(anyhow::anyhow!("Project '{}' does not exist", project_name));
    }

    // Find the milestone by title
    let milestones = crate::fs::list_milestones(&project_path)?;
    let milestone = milestones
        .into_iter()
        .find(|m| m.metadata.title == title)
        .ok_or_else(|| anyhow::anyhow!("Milestone '{}' not found", title))?;

    let mut metadata = milestone.metadata;
    let mut changed = false;

    if let Some(s) = status {
        metadata.status = Status::from_str(s)?;
        changed = true;
        println!("Updated status to: {}", format_status(&metadata.status));
    }

    if let Some(d) = date {
        metadata.target_date = Some(d.to_string());
        changed = true;
        println!("Updated target date to: {}", d);
    }

    if !changed {
        return Err(anyhow::anyhow!(
            "No changes specified. Use --status or --date"
        ));
    }

    metadata.updated = Some(Utc::now());

    // Write back to file
    write_with_frontmatter(&milestone.path, &metadata, &milestone.description)?;

    println!("✓ Updated milestone '{}'", title);

    Ok(())
}

fn sanitize_filename(s: &str) -> String {
    s.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "-")
        .replace("--", "-")
        .trim_matches('-')
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
    fn test_create_milestone() -> Result<()> {
        let (temp_dir, project_name) = setup_workspace_with_project()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        let result = create_milestone(&project_name, "v1.0", Some("2025-12-31"));
        env::set_current_dir(&original_dir)?;

        result?;

        let milestone_path = temp_dir
            .path()
            .join(&project_name)
            .join("milestones")
            .join("v1-0.md");

        assert!(milestone_path.exists());

        let milestone = crate::parser::read_milestone(&milestone_path)?;
        assert_eq!(milestone.metadata.title, "v1.0");
        assert_eq!(
            milestone.metadata.target_date,
            Some("2025-12-31".to_string())
        );
        assert_eq!(milestone.metadata.status, Status::Backlog);

        Ok(())
    }

    #[test]
    fn test_edit_milestone() -> Result<()> {
        let (temp_dir, project_name) = setup_workspace_with_project()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        create_milestone(&project_name, "v1.0", Some("2025-12-31"))?;
        let result = edit_milestone(
            &project_name,
            "v1.0",
            Some("in-progress"),
            Some("2026-01-15"),
        );
        env::set_current_dir(&original_dir)?;

        result?;

        let milestone_path = temp_dir
            .path()
            .join(&project_name)
            .join("milestones")
            .join("v1-0.md");

        let milestone = crate::parser::read_milestone(&milestone_path)?;
        assert_eq!(milestone.metadata.status, Status::InProgress);
        assert_eq!(
            milestone.metadata.target_date,
            Some("2026-01-15".to_string())
        );

        Ok(())
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("v1.0"), "v1-0");
        assert_eq!(sanitize_filename("Version 2.0 Beta"), "version-2-0-beta");
        assert_eq!(sanitize_filename("Q1 2025"), "q1-2025");
    }

    #[test]
    fn test_list_milestones_empty() -> Result<()> {
        let (temp_dir, _) = setup_workspace_with_project()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        let result = list_milestones(None);
        env::set_current_dir(&original_dir)?;

        result?;
        Ok(())
    }
}
