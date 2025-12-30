use anyhow::Result;
use std::fs;

use crate::fs::ensure_dir;
use crate::models::{Config, DefaultConfig, WorkspaceConfig};

const PROJECT_TEMPLATE: &str = r#"---
name: {{PROJECT_NAME}}
status: backlog
priority: medium
---

# {{PROJECT_NAME}}

Project description goes here.

## Goals

- Goal 1
- Goal 2

## Overview

Detailed project overview.
"#;

const MILESTONE_TEMPLATE: &str = r#"---
title: {{MILESTONE_TITLE}}
status: backlog
target_date: {{TARGET_DATE}}
project: {{PROJECT_NAME}}
---

# {{MILESTONE_TITLE}}

Milestone description and objectives.
"#;

const ISSUE_TEMPLATE: &str = r#"---
title: {{ISSUE_TITLE}}
status: todo
priority: medium
project: {{PROJECT_NAME}}
tags: []
---

# {{ISSUE_TITLE}}

## Description

Detailed issue description.

## Acceptance Criteria

- [ ] Criterion 1
- [ ] Criterion 2
"#;

pub fn init(base_directory: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let pillar_dir = current_dir.join(".pillar");

    if pillar_dir.exists() {
        return Err(anyhow::anyhow!(
            "Pillar workspace already initialized in this directory"
        ));
    }

    // Determine base directory
    let base_dir = base_directory.unwrap_or(".");

    // Validate base directory
    if base_dir == ".pillar" || base_dir.starts_with(".pillar/") {
        return Err(anyhow::anyhow!(
            "Base directory cannot be '.pillar' or inside '.pillar/'"
        ));
    }

    // Create .pillar directory
    ensure_dir(&pillar_dir)?;

    // Create config file with base directory
    let config = Config {
        workspace: WorkspaceConfig {
            version: "0.1.0".to_string(),
            base_directory: base_dir.to_string(),
        },
        defaults: DefaultConfig {
            priority: "medium".to_string(),
            status: "backlog".to_string(),
        },
    };

    let config_str = toml::to_string_pretty(&config)?;
    fs::write(pillar_dir.join("config.toml"), config_str)?;

    // Create templates directory
    let templates_dir = pillar_dir.join("templates");
    ensure_dir(&templates_dir)?;

    // Write template files
    fs::write(templates_dir.join("project.md"), PROJECT_TEMPLATE)?;
    fs::write(templates_dir.join("milestone.md"), MILESTONE_TEMPLATE)?;
    fs::write(templates_dir.join("issue.md"), ISSUE_TEMPLATE)?;

    // Create base directory if not current dir
    if base_dir != "." {
        let base_path = current_dir.join(base_dir);
        ensure_dir(&base_path)?;
    }

    println!(
        "✓ Initialized Pillar workspace in {}",
        current_dir.display()
    );
    if base_dir != "." {
        println!("  Base directory: {}", base_dir);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_init_creates_workspace() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        let result = init(None);

        // Restore original directory first
        env::set_current_dir(&original_dir)?;

        // Now check results
        result?;

        assert!(temp_dir.path().join(".pillar").exists());
        assert!(temp_dir.path().join(".pillar/config.toml").exists());
        assert!(temp_dir.path().join(".pillar/templates").exists());
        assert!(temp_dir
            .path()
            .join(".pillar/templates/project.md")
            .exists());
        assert!(temp_dir
            .path()
            .join(".pillar/templates/milestone.md")
            .exists());
        assert!(temp_dir.path().join(".pillar/templates/issue.md").exists());

        // Verify config format
        let config_content = fs::read_to_string(temp_dir.path().join(".pillar/config.toml"))?;
        assert!(config_content.contains("[workspace]"));
        assert!(config_content.contains("base_directory"));

        Ok(())
    }

    #[test]
    fn test_init_with_custom_base_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        let result = init(Some("pm"));
        env::set_current_dir(&original_dir)?;

        result?;

        // Check base directory was created
        assert!(temp_dir.path().join("pm").exists());

        // Check config contains base_directory
        let config_content = fs::read_to_string(temp_dir.path().join(".pillar/config.toml"))?;
        assert!(config_content.contains("base_directory = \"pm\""));

        Ok(())
    }

    #[test]
    fn test_init_fails_if_already_initialized() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;

        // First init should succeed
        let first_result = init(None);
        assert!(first_result.is_ok());

        // Second init should fail
        let result = init(None);

        env::set_current_dir(original_dir)?;

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_init_rejects_pillar_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        let result = init(Some(".pillar"));
        env::set_current_dir(&original_dir)?;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains(".pillar"));

        Ok(())
    }

    #[test]
    fn test_config_file_content() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;

        env::set_current_dir(temp_dir.path())?;
        let result = init(None);
        env::set_current_dir(&original_dir)?;

        result?;

        let config_content = fs::read_to_string(temp_dir.path().join(".pillar/config.toml"))?;
        assert!(config_content.contains("[workspace]"));
        assert!(config_content.contains("version"));
        assert!(config_content.contains("[defaults]"));
        assert!(config_content.contains("base_directory"));

        Ok(())
    }
}
