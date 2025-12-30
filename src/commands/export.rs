use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, Write};

use crate::fs::{get_base_directory, list_issues, list_milestones, list_projects};

pub fn export(format: &str, entity_type: &str, output: Option<&str>) -> Result<()> {
    let base_dir = get_base_directory()?;

    match format.to_lowercase().as_str() {
        "json" => export_json(entity_type, output, &base_dir),
        "csv" => export_csv(entity_type, output, &base_dir),
        _ => Err(anyhow::anyhow!(
            "Unsupported format: {}. Use 'json' or 'csv'",
            format
        )),
    }
}

fn export_json(entity_type: &str, output: Option<&str>, base_dir: &std::path::Path) -> Result<()> {
    let json = match entity_type {
        "project" => {
            let projects = list_projects(base_dir)?;
            serde_json::to_string_pretty(&projects).context("Failed to serialize projects")?
        }
        "milestone" => {
            let mut all_milestones = Vec::new();
            for project in list_projects(base_dir)? {
                let milestones = list_milestones(&project.path)?;
                all_milestones.extend(milestones);
            }
            serde_json::to_string_pretty(&all_milestones)
                .context("Failed to serialize milestones")?
        }
        "issue" => {
            let mut all_issues = Vec::new();
            for project in list_projects(base_dir)? {
                let issues = list_issues(&project.path)?;
                all_issues.extend(issues);
            }
            serde_json::to_string_pretty(&all_issues).context("Failed to serialize issues")?
        }
        "all" => {
            let projects = list_projects(base_dir)?;
            let mut all_milestones = Vec::new();
            let mut all_issues = Vec::new();

            for project in &projects {
                all_milestones.extend(list_milestones(&project.path)?);
                all_issues.extend(list_issues(&project.path)?);
            }

            let data = serde_json::json!({
                "projects": projects,
                "milestones": all_milestones,
                "issues": all_issues,
            });

            serde_json::to_string_pretty(&data).context("Failed to serialize data")?
        }
        _ => return Err(anyhow::anyhow!("Invalid entity type: {}", entity_type)),
    };

    write_output(&json, output)?;
    Ok(())
}

fn export_csv(entity_type: &str, output: Option<&str>, base_dir: &std::path::Path) -> Result<()> {
    let csv = match entity_type {
        "project" => {
            let projects = list_projects(base_dir)?;
            let mut csv = String::from("name,status,priority,created,updated\n");
            for p in projects {
                csv.push_str(&format!(
                    "\"{}\",{},{},{},{}\n",
                    p.metadata.name.replace('"', "\"\""),
                    p.metadata.status,
                    p.metadata.priority,
                    p.metadata
                        .created
                        .map(|d| d.to_rfc3339())
                        .unwrap_or_default(),
                    p.metadata
                        .updated
                        .map(|d| d.to_rfc3339())
                        .unwrap_or_default()
                ));
            }
            csv
        }
        "milestone" => {
            let mut all_milestones = Vec::new();
            for project in list_projects(base_dir)? {
                all_milestones.extend(list_milestones(&project.path)?);
            }

            let mut csv = String::from("title,status,project,target_date,created,updated\n");
            for m in all_milestones {
                csv.push_str(&format!(
                    "\"{}\",{},\"{}\",{},{},{}\n",
                    m.metadata.title.replace('"', "\"\""),
                    m.metadata.status,
                    m.metadata.project.unwrap_or_default().replace('"', "\"\""),
                    m.metadata.target_date.unwrap_or_default(),
                    m.metadata
                        .created
                        .map(|d| d.to_rfc3339())
                        .unwrap_or_default(),
                    m.metadata
                        .updated
                        .map(|d| d.to_rfc3339())
                        .unwrap_or_default()
                ));
            }
            csv
        }
        "issue" => {
            let mut all_issues = Vec::new();
            for project in list_projects(base_dir)? {
                all_issues.extend(list_issues(&project.path)?);
            }

            let mut csv =
                String::from("title,status,priority,project,milestone,tags,created,updated\n");
            for i in all_issues {
                csv.push_str(&format!(
                    "\"{}\",{},{},\"{}\",\"{}\",\"{}\",{},{}\n",
                    i.metadata.title.replace('"', "\"\""),
                    i.metadata.status,
                    i.metadata.priority,
                    i.metadata.project.unwrap_or_default().replace('"', "\"\""),
                    i.metadata
                        .milestone
                        .unwrap_or_default()
                        .replace('"', "\"\""),
                    i.metadata.tags.join(";"),
                    i.metadata
                        .created
                        .map(|d| d.to_rfc3339())
                        .unwrap_or_default(),
                    i.metadata
                        .updated
                        .map(|d| d.to_rfc3339())
                        .unwrap_or_default()
                ));
            }
            csv
        }
        "all" => {
            return Err(anyhow::anyhow!(
                "CSV export does not support 'all' entity type. Please specify: project, milestone, or issue"
            ));
        }
        _ => return Err(anyhow::anyhow!("Invalid entity type: {}", entity_type)),
    };

    write_output(&csv, output)?;
    Ok(())
}

fn write_output(content: &str, output: Option<&str>) -> Result<()> {
    match output {
        Some(path) => {
            let mut file = File::create(path)
                .with_context(|| format!("Failed to create output file: {}", path))?;
            file.write_all(content.as_bytes())
                .with_context(|| format!("Failed to write to file: {}", path))?;
            println!("Exported to: {}", path);
        }
        None => {
            io::stdout()
                .write_all(content.as_bytes())
                .context("Failed to write to stdout")?;
        }
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
    fn test_export_json() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        init(None)?;

        crate::commands::project::create_project("TestProject", "high")?;

        let result = export("json", "project", None);

        env::set_current_dir(&original_dir)?;

        assert!(result.is_ok());
        Ok(())
    }
}
