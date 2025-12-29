use anyhow::Result;
use std::fs;

use crate::fs::{find_project, get_author, get_base_directory};
use crate::models::Comment;
use crate::parser::{read_comments, write_comments};
use walkdir::WalkDir;

/// Add a comment to a project, milestone, or issue
pub fn add(
    entity_type: &str,
    project_name: &str,
    identifier: Option<&str>,
    content: &str,
) -> Result<()> {
    let base_dir = get_base_directory()?;
    let project = find_project(&base_dir, project_name)?;
    
    // Determine the file path based on entity type
    let file_path = match entity_type {
        "project" => project.path.join("README.md"),
        "milestone" => {
            let milestone_title = identifier
                .ok_or_else(|| anyhow::anyhow!("Milestone title required"))?;
            project.path.join("milestones").join(format!("{}.md", milestone_title))
        }
        "issue" => {
            let issue_id = identifier
                .ok_or_else(|| anyhow::anyhow!("Issue ID required"))?;
            
            // Find the issue file by ID (files are named like "001-title.md")
            let issues_dir = project.path.join("issues");
            let mut found_path = None;
            
            for entry in WalkDir::new(&issues_dir)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                        // Check if filename starts with the issue ID (with or without leading zeros)
                        let padded_id = format!("{:03}", issue_id.parse::<usize>().unwrap_or(0));
                        if filename.starts_with(&format!("{}-", padded_id)) || 
                           filename.starts_with(&format!("{}-", issue_id)) {
                            found_path = Some(path.to_path_buf());
                            break;
                        }
                    }
                }
            }
            
            found_path.ok_or_else(|| anyhow::anyhow!("Issue {} not found", issue_id))?
        }
        _ => return Err(anyhow::anyhow!("Invalid entity type: {}", entity_type)),
    };
    
    if !file_path.exists() {
        return Err(anyhow::anyhow!("{} does not exist", entity_type));
    }
    
    // Read the file
    let file_content = fs::read_to_string(&file_path)?;
    
    // Split frontmatter and body
    let body_start = if let Some(end_pos) = file_content[3..].find("\n---\n") {
        end_pos + 7 // Position after "\n---\n"
    } else {
        return Err(anyhow::anyhow!("Invalid file format"));
    };
    
    let frontmatter = &file_content[..body_start];
    let body = file_content[body_start..].trim();
    
    // Read existing comments
    let mut comments = read_comments(body);
    
    // Create and add new comment
    let author = get_author();
    let new_comment = Comment::new(author.clone(), content.to_string());
    comments.push(new_comment);
    
    // Write back with updated comments
    let updated_body = write_comments(body, &comments);
    let updated_content = format!("{}\n\n{}", frontmatter.trim(), updated_body);
    
    fs::write(&file_path, updated_content)?;
    
    println!("✓ Added comment by {} to {} '{}'", author, entity_type, 
             identifier.unwrap_or(project_name));
    
    Ok(())
}

/// List comments on a project, milestone, or issue
pub fn list(
    entity_type: &str,
    project_name: &str,
    identifier: Option<&str>,
) -> Result<()> {
    let base_dir = get_base_directory()?;
    let project = find_project(&base_dir, project_name)?;
    
    // Determine the file path based on entity type
    let file_path = match entity_type {
        "project" => project.path.join("README.md"),
        "milestone" => {
            let milestone_title = identifier
                .ok_or_else(|| anyhow::anyhow!("Milestone title required"))?;
            project.path.join("milestones").join(format!("{}.md", milestone_title))
        }
        "issue" => {
            let issue_id = identifier
                .ok_or_else(|| anyhow::anyhow!("Issue ID required"))?;
            
            // Find the issue file by ID (files are named like "001-title.md")
            let issues_dir = project.path.join("issues");
            let mut found_path = None;
            
            for entry in WalkDir::new(&issues_dir)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                        // Check if filename starts with the issue ID (with or without leading zeros)
                        let padded_id = format!("{:03}", issue_id.parse::<usize>().unwrap_or(0));
                        if filename.starts_with(&format!("{}-", padded_id)) || 
                           filename.starts_with(&format!("{}-", issue_id)) {
                            found_path = Some(path.to_path_buf());
                            break;
                        }
                    }
                }
            }
            
            found_path.ok_or_else(|| anyhow::anyhow!("Issue {} not found", issue_id))?
        }
        _ => return Err(anyhow::anyhow!("Invalid entity type: {}", entity_type)),
    };
    
    if !file_path.exists() {
        return Err(anyhow::anyhow!("{} does not exist", entity_type));
    }
    
    // Read the file
    let file_content = fs::read_to_string(&file_path)?;
    
    // Extract body (skip frontmatter)
    let body_start = if let Some(end_pos) = file_content[3..].find("\n---\n") {
        end_pos + 7
    } else {
        return Err(anyhow::anyhow!("Invalid file format"));
    };
    
    let body = file_content[body_start..].trim();
    
    // Read comments
    let comments = read_comments(body);
    
    if comments.is_empty() {
        println!("No comments on {} '{}'", entity_type, 
                 identifier.unwrap_or(project_name));
        return Ok(());
    }
    
    println!("Comments on {} '{}':\n", entity_type, 
             identifier.unwrap_or(project_name));
    
    for comment in comments {
        println!("[{}] - {}", comment.timestamp, comment.author);
        println!("{}\n", comment.content);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::init;
    use crate::commands::project;
    use crate::commands::issue;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_add_comment_to_project() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;
        
        env::set_current_dir(temp_dir.path())?;
        init(None)?;
        project::create_project("TestProject", "medium")?;
        
        let result = add("project", "TestProject", None, "This is a test comment");
        
        env::set_current_dir(&original_dir)?;
        
        result?;
        
        // Verify comment was added
        let readme_path = temp_dir.path().join("TestProject/README.md");
        let content = fs::read_to_string(readme_path)?;
        assert!(content.contains("## Comments"));
        assert!(content.contains("This is a test comment"));
        
        Ok(())
    }

    #[test]
    fn test_add_comment_to_issue() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;
        
        env::set_current_dir(temp_dir.path())?;
        init(None)?;
        project::create_project("TestProject", "medium")?;
        issue::create_issue("TestProject", "Test Issue", "medium", None, None)?;
        
        let result = add("issue", "TestProject", Some("1"), "Issue comment");
        
        env::set_current_dir(&original_dir)?;
        
        result?;
        
        // Verify comment was added  
        // Issue filename is sanitized: "Test Issue" becomes "001-test-issue.md"
        let issue_path = temp_dir.path().join("TestProject/issues/001-test-issue.md");
        let content = fs::read_to_string(issue_path)?;
        assert!(content.contains("## Comments"));
        assert!(content.contains("Issue comment"));
        
        Ok(())
    }

    #[test]
    fn test_list_comments_empty() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;
        
        env::set_current_dir(temp_dir.path())?;
        init(None)?;
        project::create_project("TestProject", "medium")?;
        
        let result = list("project", "TestProject", None);
        
        env::set_current_dir(&original_dir)?;
        
        result?;
        
        Ok(())
    }

    #[test]
    fn test_list_comments_with_data() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;
        
        env::set_current_dir(temp_dir.path())?;
        init(None)?;
        project::create_project("TestProject", "medium")?;
        add("project", "TestProject", None, "First comment")?;
        add("project", "TestProject", None, "Second comment")?;
        
        let result = list("project", "TestProject", None);
        
        env::set_current_dir(&original_dir)?;
        
        result?;
        
        Ok(())
    }
}
