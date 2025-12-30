use anyhow::Result;
use colored::Colorize;

use crate::fs::{get_base_directory, list_issues, list_milestones, list_projects};

pub fn search(query: &str, entity_type: &str) -> Result<()> {
    let base_dir = get_base_directory()?;
    let query_lower = query.to_lowercase();
    let mut found_any = false;

    // Search projects
    if entity_type == "all" || entity_type == "project" {
        let projects = list_projects(&base_dir)?;
        let mut found_projects = false;

        for project in &projects {
            let matches = project.metadata.name.to_lowercase().contains(&query_lower)
                || project.description.to_lowercase().contains(&query_lower);

            if matches {
                if !found_projects {
                    println!("{}", "Projects:".bold().green());
                    found_projects = true;
                    found_any = true;
                }
                println!(
                    "  {} [{}] [{}]",
                    project.metadata.name.bold(),
                    project.metadata.status,
                    project.metadata.priority
                );
            }
        }

        if found_projects {
            println!();
        }
    }

    // Search milestones
    if entity_type == "all" || entity_type == "milestone" {
        let projects = list_projects(&base_dir)?;
        let mut found_milestones = false;

        for project in &projects {
            let milestones = list_milestones(&project.path)?;

            for milestone in milestones {
                let matches = milestone
                    .metadata
                    .title
                    .to_lowercase()
                    .contains(&query_lower)
                    || milestone.description.to_lowercase().contains(&query_lower);

                if matches {
                    if !found_milestones {
                        println!("{}", "Milestones:".bold().green());
                        found_milestones = true;
                        found_any = true;
                    }
                    println!(
                        "  {} / {} [{}]",
                        project.metadata.name,
                        milestone.metadata.title.bold(),
                        milestone.metadata.status
                    );
                    if let Some(date) = &milestone.metadata.target_date {
                        println!("    Target: {}", date);
                    }
                }
            }
        }

        if found_milestones {
            println!();
        }
    }

    // Search issues
    if entity_type == "all" || entity_type == "issue" {
        let projects = list_projects(&base_dir)?;
        let mut found_issues = false;

        for project in &projects {
            let issues = list_issues(&project.path)?;

            for issue in issues {
                let matches = issue.metadata.title.to_lowercase().contains(&query_lower)
                    || issue.description.to_lowercase().contains(&query_lower)
                    || issue
                        .metadata
                        .tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query_lower));

                if matches {
                    if !found_issues {
                        println!("{}", "Issues:".bold().green());
                        found_issues = true;
                        found_any = true;
                    }

                    // Extract ID from path
                    let filename = issue.path.file_stem().unwrap().to_str().unwrap();
                    let id_parts: Vec<&str> = filename.splitn(2, '-').collect();
                    let id = id_parts[0];

                    let project_name = issue.metadata.project.as_deref().unwrap_or("Unknown");

                    println!(
                        "  {}/{} - {} [{}] [{}]",
                        project_name,
                        id,
                        issue.metadata.title.bold(),
                        issue.metadata.status,
                        issue.metadata.priority
                    );

                    if let Some(milestone) = &issue.metadata.milestone {
                        println!("    Milestone: {}", milestone);
                    }

                    if !issue.metadata.tags.is_empty() {
                        println!("    Tags: {}", issue.metadata.tags.join(", "));
                    }
                }
            }
        }

        if found_issues {
            println!();
        }
    }

    if !found_any {
        println!("No results found for query: {}", query.bold());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::init;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_search_finds_entities() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        init(None)?;

        // Create test data
        crate::commands::project::create_project("TestProject", None, "high")?;
        crate::commands::issue::create_issue(
            "TestProject",
            "Fix bug in search",
            "high",
            None,
            Some("bug,search"),
        )?;

        // Test search
        let result = search("search", "all");

        env::set_current_dir(&original_dir)?;

        assert!(result.is_ok());
        Ok(())
    }
}
