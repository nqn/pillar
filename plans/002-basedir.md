# Base Directory Support

## Overview

Enable Pillar to store project management data in a subdirectory separate from the workspace root, allowing cleaner separation of code and project management files in the same repository.

## Problem Statement

Currently, Pillar expects projects to be stored directly under the workspace root (where `.pillar/` exists). For repositories that contain both code and project management data, this can create clutter. 

**Current structure:**
```
repo/
├── .pillar/
├── src/              # Code
├── tests/            # Code
├── project-alpha/    # Project management (mixed with code)
└── project-beta/     # Project management (mixed with code)
```

**Desired structure:**
```
repo/
├── .pillar/
├── src/              # Code
├── tests/            # Code
└── pm/               # All project management data
    ├── project-alpha/
    └── project-beta/
```

## User Experience

### Initialization

```bash
# Initialize with default (current directory)
pillar init

# Initialize with custom base directory
pillar init --path pm
pillar init -p project-management
pillar init --path ./planning

# The base directory will be created if it doesn't exist
```

### Running Commands

All commands work the same way regardless of where you run them from:

```bash
# From repository root
cd /path/to/repo
pillar project create backend-api

# From within code directory
cd /path/to/repo/src
pillar project create backend-api

# From within the base directory itself
cd /path/to/repo/pm
pillar project create backend-api

# All three create the project in /path/to/repo/pm/backend-api
```

### Viewing Current Configuration

```bash
# Show workspace info including base directory
pillar config show

# Output:
# Workspace: /path/to/repo
# Base Directory: pm
# Absolute Path: /path/to/repo/pm
# Projects: 5
```

## Technical Design

### 1. Configuration Storage

**File**: `.pillar/config.toml`

**Current format:**
```toml
[workspace]
version = "0.1.0"

[defaults]
priority = "medium"
status = "backlog"
```

**New format:**
```toml
[workspace]
version = "0.1.0"
base_directory = "pm"  # NEW: relative path from workspace root

[defaults]
priority = "medium"
status = "backlog"
```

**Notes:**
- `base_directory` is optional - defaults to "." (current behavior)
- Path is stored as relative to workspace root
- Can be ".", "pm", "project-management", "planning/issues", etc.

### 2. Data Model Changes

**New struct in `src/models.rs`:**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub version: String,
    #[serde(default = "default_base_directory")]
    pub base_directory: String,
}

fn default_base_directory() -> String {
    ".".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub workspace: WorkspaceConfig,
    pub defaults: DefaultConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    pub priority: String,
    pub status: String,
}
```

### 3. Core Function Changes

#### `src/fs.rs`

**Change `find_workspace_root()`:**
```rust
pub fn find_workspace_root() -> Result<PathBuf> {
    // Current: returns the directory containing .pillar
    // No change needed - still finds workspace root
}
```

**Add `get_base_directory()`:**
```rust
/// Get the base directory path where projects are stored
pub fn get_base_directory() -> Result<PathBuf> {
    let workspace_root = find_workspace_root()?;
    let config = read_config(&workspace_root)?;
    
    let base_path = workspace_root.join(&config.workspace.base_directory);
    
    // Ensure base directory exists
    ensure_dir(&base_path)?;
    
    Ok(base_path)
}
```

**Add `read_config()`:**
```rust
/// Read and parse the workspace configuration
pub fn read_config(workspace_root: &Path) -> Result<Config> {
    let config_path = workspace_root.join(".pillar/config.toml");
    let content = fs::read_to_string(&config_path)
        .context("Failed to read config.toml")?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config.toml")?;
    
    Ok(config)
}
```

**Update all functions that list/find projects:**
```rust
pub fn list_projects() -> Result<Vec<Project>> {
    let base_dir = get_base_directory()?;  // Changed from find_workspace_root()
    // Rest remains the same
}

pub fn find_project(name: &str) -> Result<Project> {
    let base_dir = get_base_directory()?;  // Changed from find_workspace_root()
    // Rest remains the same
}
```

### 4. Command Changes

#### `src/commands/init.rs`

**Update signature:**
```rust
pub fn init(base_directory: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let pillar_dir = current_dir.join(".pillar");
    
    if pillar_dir.exists() {
        return Err(anyhow::anyhow!(
            "Pillar workspace already initialized in this directory"
        ));
    }

    // Create .pillar directory
    ensure_dir(&pillar_dir)?;
    
    // Determine base directory
    let base_dir = base_directory.unwrap_or(".");
    
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
    
    let config_content = toml::to_string_pretty(&config)?;
    fs::write(pillar_dir.join("config.toml"), config_content)?;
    
    // Create templates directory
    let templates_dir = pillar_dir.join("templates");
    ensure_dir(&templates_dir)?;
    
    // Write templates (unchanged)
    // ...
    
    // Create base directory if not current dir
    if base_dir != "." {
        let base_path = current_dir.join(base_dir);
        ensure_dir(&base_path)?;
        println!("  Created base directory: {}", base_dir);
    }
    
    println!("✓ Initialized Pillar workspace in {}", current_dir.display());
    println!("  Base directory: {}", base_dir);
    println!("  Created .pillar/config.toml");
    println!("  Created .pillar/templates/");
    // ...
}
```

#### `src/commands/project.rs`

**Update all functions:**
```rust
pub fn create_project(name: &str, priority: &str) -> Result<()> {
    let base_dir = get_base_directory()?;  // Changed
    let project_path = base_dir.join(name);  // Now relative to base dir
    // Rest remains the same
}

pub fn list_projects(status_filter: Option<&str>, priority_filter: Option<&str>) -> Result<()> {
    let base_dir = get_base_directory()?;  // Changed
    let mut projects = list_all(&base_dir)?;  // Pass base_dir instead of workspace_root
    // Rest remains the same
}

pub fn show_project(name: &str) -> Result<()> {
    let base_dir = get_base_directory()?;  // Changed
    let project = crate::fs::find_project(&base_dir, name)?;
    // Rest remains the same
}

pub fn edit_project(name: &str, status: Option<&str>, priority: Option<&str>) -> Result<()> {
    let base_dir = get_base_directory()?;  // Changed
    let project = crate::fs::find_project(&base_dir, name)?;
    // Rest remains the same
}
```

**Pattern applies to:**
- `src/commands/milestone.rs` - All functions
- `src/commands/issue.rs` - All functions
- `src/commands/view.rs` - All functions

### 5. CLI Changes

#### `src/cli.rs`

**Update Init command:**
```rust
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Pillar workspace
    Init {
        /// Base directory for project data (relative to workspace root)
        #[arg(short = 'p', long = "path")]
        base_directory: Option<String>,
    },
    // ... rest unchanged
}
```

#### `src/main.rs`

**Update match statement:**
```rust
match cli.command {
    Commands::Init { base_directory } => {
        commands::init(base_directory.as_deref())?;
    }
    // ... rest unchanged
}
```

### 6. New Command: Config

Add a new command to view/manage configuration:

```rust
// In src/cli.rs
#[derive(Subcommand)]
pub enum Commands {
    // ... existing commands
    
    /// Show workspace configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Show current configuration
    Show,
}
```

```rust
// In src/commands/config.rs
pub fn show_config() -> Result<()> {
    let workspace_root = find_workspace_root()?;
    let config = read_config(&workspace_root)?;
    let base_dir = get_base_directory()?;
    
    println!("Workspace Configuration\n");
    println!("Workspace Root: {}", workspace_root.display());
    println!("Base Directory: {}", config.workspace.base_directory);
    println!("Absolute Path: {}", base_dir.display());
    println!("Version: {}", config.workspace.version);
    
    // Count projects
    let projects = list_projects(&base_dir)?;
    println!("\nProjects: {}", projects.len());
    
    Ok(())
}
```

## Implementation Checklist

### Core Changes
- [ ] Add `Config`, `WorkspaceConfig`, `DefaultConfig` structs to `models.rs`
- [ ] Add `get_base_directory()` function to `fs.rs`
- [ ] Add `read_config()` function to `fs.rs`
- [ ] Update `list_projects()` to use base directory
- [ ] Update `find_project()` to use base directory
- [ ] Update `list_all_issues()` to use base directory

### Init Command
- [ ] Add `--path` / `-p` flag to init command in `cli.rs`
- [ ] Update `init()` to accept base_directory parameter
- [ ] Update `init()` to write base_directory to config
- [ ] Update `init()` to create base directory if needed
- [ ] Update init success message to show base directory

### Project Commands
- [ ] Update `create_project()` to use `get_base_directory()`
- [ ] Update `list_projects()` to use `get_base_directory()`
- [ ] Update `show_project()` to use `get_base_directory()`
- [ ] Update `edit_project()` to use `get_base_directory()`

### Milestone Commands
- [ ] Update `create_milestone()` to use `get_base_directory()`
- [ ] Update `list_milestones()` to use `get_base_directory()`
- [ ] Update `edit_milestone()` to use `get_base_directory()`

### Issue Commands
- [ ] Update `create_issue()` to use `get_base_directory()`
- [ ] Update `list_issues()` to use `get_base_directory()`
- [ ] Update `show_issue()` to use `get_base_directory()`
- [ ] Update `edit_issue()` to use `get_base_directory()`

### View Commands
- [ ] Update `status()` to use `get_base_directory()`
- [ ] Update `board()` to use `get_base_directory()`

### New Features
- [ ] Create `src/commands/config.rs`
- [ ] Add `config show` command
- [ ] Export config functions from commands module
- [ ] Wire up config command in main.rs

### Testing
- [ ] Add tests for `read_config()` with default base directory
- [ ] Add tests for `read_config()` with custom base directory
- [ ] Add tests for `get_base_directory()` 
- [ ] Update `setup_workspace()` test helper to support base directory
- [ ] Add test for `init()` with custom base directory
- [ ] Add integration test: init with base dir → create project → verify location
- [ ] Add test for running commands from different directories
- [ ] Update all existing tests to work with base directory concept

### Documentation
- [ ] Update README.md with `--path` flag documentation
- [ ] Add "Base Directory" section to README
- [ ] Update Quick Start with base directory example
- [ ] Add examples of repo structure with separate PM directory
- [ ] Update config file format documentation

### Backwards Compatibility
- [ ] Ensure old workspaces (without base_directory in config) default to "."
- [ ] Add migration note in CHANGELOG
- [ ] Test that existing workspaces work without modification

## Example Usage Scenarios

### Scenario 1: Code + PM in Same Repo

```bash
# Developer initializes repo with code
git init
mkdir src tests docs

# Initialize Pillar with PM in subdirectory
pillar init --path planning

# Directory structure:
# .
# ├── .git/
# ├── .pillar/
# │   └── config.toml (base_directory = "planning")
# ├── src/
# ├── tests/
# └── planning/          # Created by pillar init

# Create projects from repo root
pillar project create backend-api
pillar project create mobile-app

# Directory structure:
# .
# ├── .git/
# ├── .pillar/
# ├── src/
# ├── tests/
# └── planning/
#     ├── backend-api/
#     │   ├── README.md
#     │   ├── milestones/
#     │   └── issues/
#     └── mobile-app/
#         ├── README.md
#         ├── milestones/
#         └── issues/

# Developer can work from anywhere
cd src/
pillar status  # Works!

cd ../planning/backend-api/
pillar issue create backend-api "New feature"  # Works!
```

### Scenario 2: PM-Only Repository

```bash
# PM repository for tracking multiple teams
pillar init  # Default base directory "."

# Create projects
pillar project create team-alpha-q1
pillar project create team-beta-q1

# Structure:
# .
# ├── .pillar/
# ├── team-alpha-q1/
# └── team-beta-q1/
```

### Scenario 3: Nested Base Directory

```bash
# Complex organization
pillar init --path project-management/active

# Structure:
# .
# ├── .pillar/
# ├── src/
# └── project-management/
#     └── active/           # Base directory
#         ├── project-a/
#         └── project-b/
```

## Migration Path

### For Existing Users

1. **No action required** - Workspaces without `base_directory` in config will default to "." (current directory behavior)

2. **Optional migration** to subdirectory:
   ```bash
   # Manual approach
   mkdir pm
   mv project-* pm/
   
   # Edit .pillar/config.toml
   # Add: base_directory = "pm"
   
   # Verify
   pillar project list  # Should show all projects
   ```

3. **Future enhancement**: `pillar config migrate --to <path>` command to automate migration

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_read_config_with_base_directory() {
    // Test config parsing with base_directory field
}

#[test]
fn test_read_config_defaults_to_current_dir() {
    // Test backwards compatibility
}

#[test]
fn test_get_base_directory_creates_if_missing() {
    // Test that base directory is created on access
}
```

### Integration Tests

```rust
#[test]
fn test_init_with_custom_base_directory() {
    // pillar init --path pm
    // Verify config contains base_directory = "pm"
    // Verify pm/ directory is created
}

#[test]
fn test_create_project_in_base_directory() {
    // pillar init --path pm
    // pillar project create test
    // Verify project is in pm/test/
}

#[test]
fn test_commands_work_from_subdirectory() {
    // cd into src/
    // Run pillar commands
    // Verify they still work
}
```

## Benefits

1. **Separation of Concerns**: Code and PM data are clearly separated
2. **Cleaner Repository**: Less clutter at repo root
3. **Flexibility**: Users can organize PM data as they prefer
4. **Backwards Compatible**: Existing workspaces continue to work
5. **Works from Anywhere**: Commands work from any subdirectory

## Potential Edge Cases

1. **Relative vs Absolute Paths**: Base directory should always be relative to workspace root
2. **Symlinks**: Should follow symlinks when resolving paths
3. **Moving Workspace**: If user moves `.pillar/`, base directory is still relative
4. **Base Directory = ".pillar"**: Should be prevented (reserved directory)
5. **Deeply Nested Paths**: `project-management/teams/engineering/active` should work

## Future Enhancements

1. **Multiple Base Directories**: Support for different base dirs per project type
   ```toml
   [workspace]
   base_directory = "pm"
   archive_directory = "pm/archive"
   ```

2. **Base Directory Aliases**: 
   ```toml
   [aliases]
   active = "pm/active"
   archived = "pm/archived"
   ```

3. **Migration Command**:
   ```bash
   pillar config migrate --to new-path
   ```

4. **Workspace Info Command**:
   ```bash
   pillar workspace info
   # Shows workspace root, base dir, project count, etc.
   ```

## Success Criteria

- [ ] Users can initialize with custom base directory
- [ ] All commands work transparently with custom base directory
- [ ] Commands work from any directory within workspace
- [ ] Existing workspaces continue to work without modification
- [ ] All tests pass including new base directory tests
- [ ] Documentation updated with examples
- [ ] No breaking changes to existing functionality
