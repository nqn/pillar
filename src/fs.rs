use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

use crate::models::{Config, Issue, Milestone, Project};
use crate::parser::{read_issue, read_milestone, read_project};

/// Find the workspace root by looking for .pillar directory
pub fn find_workspace_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir()?;
    
    loop {
        let pillar_dir = current.join(".pillar");
        if pillar_dir.exists() && pillar_dir.is_dir() {
            return Ok(current);
        }
        
        if !current.pop() {
            return Err(anyhow::anyhow!(
                "Not in a Pillar workspace. Run 'pillar init' to initialize one."
            ));
        }
    }
}

/// Read and parse the workspace configuration
pub fn read_config(workspace_root: &Path) -> Result<Config> {
    let config_path = workspace_root.join(".pillar/config.toml");
    let content = fs::read_to_string(&config_path)
        .context("Failed to read config.toml")?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config.toml")?;
    
    Ok(config)
}

/// Get the base directory path where projects are stored
pub fn get_base_directory() -> Result<PathBuf> {
    let workspace_root = find_workspace_root()?;
    let config = read_config(&workspace_root)?;
    
    let base_path = workspace_root.join(&config.workspace.base_directory);
    
    // Ensure base directory exists
    ensure_dir(&base_path)?;
    
    Ok(base_path)
}

/// Get the current user's name for comments
/// Tries git config, then $USER environment variable, then falls back to "Unknown"
pub fn get_author() -> String {
    // Try git config user.name first
    if let Ok(output) = Command::new("git")
        .args(["config", "user.name"])
        .output()
    {
        if output.status.success() {
            if let Ok(name) = String::from_utf8(output.stdout) {
                let name = name.trim();
                if !name.is_empty() {
                    return name.to_string();
                }
            }
        }
    }
    
    // Fall back to $USER environment variable
    if let Ok(user) = std::env::var("USER") {
        if !user.is_empty() {
            return user;
        }
    }
    
    // Last resort
    "Unknown".to_string()
}

/// Create directory structure if it doesn't exist
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::create_dir_all(path.as_ref())
        .with_context(|| format!("Failed to create directory: {}", path.as_ref().display()))?;
    Ok(())
}

/// List all projects in the workspace
pub fn list_projects<P: AsRef<Path>>(workspace_root: P) -> Result<Vec<Project>> {
    let workspace_root = workspace_root.as_ref();
    let mut projects = Vec::new();

    for entry in fs::read_dir(workspace_root)? {
        let entry = entry?;
        let path = entry.path();
        
        // Skip .pillar directory and other hidden directories
        if path.is_dir() && !path.file_name().unwrap().to_str().unwrap().starts_with('.') {
            let readme = path.join("README.md");
            if readme.exists() {
                match read_project(&path) {
                    Ok(project) => projects.push(project),
                    Err(e) => eprintln!("Warning: Failed to read project at {}: {}", path.display(), e),
                }
            }
        }
    }

    Ok(projects)
}

/// Find a project by name
pub fn find_project<P: AsRef<Path>>(workspace_root: P, name: &str) -> Result<Project> {
    let project_path = workspace_root.as_ref().join(name);
    
    if !project_path.exists() {
        return Err(anyhow::anyhow!("Project '{}' does not exist", name));
    }

    read_project(&project_path)
}

/// List all milestones in a project
pub fn list_milestones<P: AsRef<Path>>(project_path: P) -> Result<Vec<Milestone>> {
    let milestones_dir = project_path.as_ref().join("milestones");
    
    if !milestones_dir.exists() {
        return Ok(Vec::new());
    }

    let mut milestones = Vec::new();

    for entry in WalkDir::new(&milestones_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            match read_milestone(path) {
                Ok(milestone) => milestones.push(milestone),
                Err(e) => eprintln!("Warning: Failed to read milestone at {}: {}", path.display(), e),
            }
        }
    }

    Ok(milestones)
}

/// List all issues in a project
pub fn list_issues<P: AsRef<Path>>(project_path: P) -> Result<Vec<Issue>> {
    let issues_dir = project_path.as_ref().join("issues");
    
    if !issues_dir.exists() {
        return Ok(Vec::new());
    }

    let mut issues = Vec::new();

    for entry in WalkDir::new(&issues_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            match read_issue(path) {
                Ok(issue) => issues.push(issue),
                Err(e) => eprintln!("Warning: Failed to read issue at {}: {}", path.display(), e),
            }
        }
    }

    Ok(issues)
}

/// List all issues across all projects in the workspace
pub fn list_all_issues<P: AsRef<Path>>(workspace_root: P) -> Result<Vec<Issue>> {
    let projects = list_projects(&workspace_root)?;
    let mut all_issues = Vec::new();

    for project in projects {
        let issues = list_issues(&project.path)?;
        all_issues.extend(issues);
    }

    Ok(all_issues)
}

/// Generate a unique issue ID based on existing issues
pub fn generate_issue_id<P: AsRef<Path>>(project_path: P) -> Result<String> {
    let issues_dir = project_path.as_ref().join("issues");
    
    if !issues_dir.exists() {
        return Ok("001".to_string());
    }

    let mut max_id = 0;
    
    for entry in fs::read_dir(&issues_dir)? {
        let entry = entry?;
        let filename = entry.file_name();
        let name = filename.to_str().unwrap_or("");
        
        // Parse ID from filename like "001-description.md"
        if let Some(id_str) = name.split('-').next() {
            if let Ok(id) = id_str.parse::<u32>() {
                max_id = max_id.max(id);
            }
        }
    }

    Ok(format!("{:03}", max_id + 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Priority, ProjectMetadata, Status};
    use crate::parser::write_with_frontmatter;
    use tempfile::TempDir;
    use std::env;

    fn create_test_project(dir: &Path, name: &str) -> Result<()> {
        let project_dir = dir.join(name);
        fs::create_dir_all(&project_dir)?;
        
        let metadata = ProjectMetadata {
            name: name.to_string(),
            status: Status::InProgress,
            priority: Priority::Medium,
            created: None,
            updated: None,
        };

        write_with_frontmatter(
            project_dir.join("README.md"),
            &metadata,
            "Test project description",
        )?;

        Ok(())
    }

    #[test]
    fn test_ensure_dir() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let nested_path = temp_dir.path().join("a").join("b").join("c");
        
        ensure_dir(&nested_path)?;
        assert!(nested_path.exists());
        
        Ok(())
    }

    #[test]
    fn test_list_projects() -> Result<()> {
        let temp_dir = TempDir::new()?;
        
        // Create .pillar directory
        fs::create_dir(temp_dir.path().join(".pillar"))?;
        
        // Create test projects
        create_test_project(temp_dir.path(), "project-a")?;
        create_test_project(temp_dir.path(), "project-b")?;
        
        let projects = list_projects(temp_dir.path())?;
        
        assert_eq!(projects.len(), 2);
        let names: Vec<String> = projects.iter().map(|p| p.metadata.name.clone()).collect();
        assert!(names.contains(&"project-a".to_string()));
        assert!(names.contains(&"project-b".to_string()));
        
        Ok(())
    }

    #[test]
    fn test_find_project() -> Result<()> {
        let temp_dir = TempDir::new()?;
        create_test_project(temp_dir.path(), "test-project")?;
        
        let project = find_project(temp_dir.path(), "test-project")?;
        assert_eq!(project.metadata.name, "test-project");
        
        let result = find_project(temp_dir.path(), "nonexistent");
        assert!(result.is_err());

    #[test]
    fn test_read_config() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let pillar_dir = temp_dir.path().join(".pillar");
        fs::create_dir(&pillar_dir)?;

        let config_content = r#"
[workspace]
version = "0.1.0"
base_directory = "pm"

[defaults]
priority = "medium"
status = "backlog"
"#;
        fs::write(pillar_dir.join("config.toml"), config_content)?;

        let config = read_config(temp_dir.path())?;
        assert_eq!(config.workspace.version, "0.1.0");
        assert_eq!(config.workspace.base_directory, "pm");
        assert_eq!(config.defaults.priority, "medium");

        Ok(())
    }

    #[test]
    fn test_read_config_default_base_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let pillar_dir = temp_dir.path().join(".pillar");
        fs::create_dir(&pillar_dir)?;

        // Config without base_directory
        let config_content = r#"
[workspace]
version = "0.1.0"

[defaults]
priority = "medium"
status = "backlog"
"#;
        fs::write(pillar_dir.join("config.toml"), config_content)?;

        let config = read_config(temp_dir.path())?;
        assert_eq!(config.workspace.base_directory, ".");

        Ok(())
    }

    #[test]
    fn test_get_base_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;
        
        env::set_current_dir(temp_dir.path())?;
        
        // Create workspace with custom base directory
        let pillar_dir = temp_dir.path().join(".pillar");
        fs::create_dir(&pillar_dir)?;
        
        let config_content = r#"
[workspace]
version = "0.1.0"
base_directory = "project-management"

[defaults]
priority = "medium"
status = "backlog"
"#;
        fs::write(pillar_dir.join("config.toml"), config_content)?;

        let base_dir = get_base_directory()?;
        
        env::set_current_dir(original_dir)?;
        
        assert_eq!(base_dir, temp_dir.path().join("project-management"));
        assert!(base_dir.exists()); // Should be created

        Ok(())
    }
        
        Ok(())
    }

    #[test]
    fn test_generate_issue_id() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let project_dir = temp_dir.path().join("project");
        let issues_dir = project_dir.join("issues");
        fs::create_dir_all(&issues_dir)?;
        
        // First issue should be 001
        let id1 = generate_issue_id(&project_dir)?;
        assert_eq!(id1, "001");
        
        // Create some issue files
        fs::write(issues_dir.join("001-first.md"), "test")?;
        fs::write(issues_dir.join("002-second.md"), "test")?;
        
        // Next should be 003
        let id2 = generate_issue_id(&project_dir)?;
        assert_eq!(id2, "003");
        
        Ok(())
    }
}
