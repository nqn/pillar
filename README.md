# Pillar

> A file-based task tracker for managing projects, milestones, and issues

[![Crates.io](https://img.shields.io/crates/v/pillar-cli.svg)](https://crates.io/crates/pillar-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/nqn/pillar/actions/workflows/ci.yml/badge.svg)](https://github.com/nqn/pillar/actions/workflows/ci.yml)

Pillar is a command-line task tracker that uses plain text files (Markdown with YAML frontmatter) to manage your projects, milestones, and issues. Perfect for developers who want to track work without leaving the terminal or dealing with complex web interfaces.

## ✨ Features

- **📁 File-based**: All data stored as Markdown files in your repository
- **🎯 Project Management**: Organize work into projects, milestones, and issues
- **💬 Comments**: Add timestamped comments to any entity
- **🏷️ Flexible Filtering**: Filter by status, priority, tags, and more
- **📊 Kanban Board**: Visualize work across status columns
- **🔍 Status Overview**: Get a quick snapshot of all active work
- **⚙️ Configurable**: Separate PM data from code with base directory support
- **🚀 No Database**: Git-friendly plain text files

## 🚀 Quick Start

```bash
# Initialize a new workspace
pillar init

# Create a project
pillar project create "MyApp" --priority high

# Add a milestone
pillar milestone create MyApp "v1.0" --date 2026-03-01

# Create an issue
pillar issue create MyApp "Fix login bug" --priority urgent --milestone "v1.0"

# View board
pillar board

# Add a comment
pillar comment add issue MyApp 1 "Working on this now"
```

## 📦 Installation

### From Crates.io (Recommended)

```bash
cargo install pillar-cli
```

### From Source

```bash
git clone https://github.com/nqn/pillar.git
cd pillar
cargo install --path .
```

### Homebrew (Coming Soon)

```bash
brew install pillar
```

## 📖 Usage

### Initialize a Workspace

```bash
# Standard initialization (stores data in current directory)
pillar init

# Use a base directory (keeps PM data separate from code)
pillar init --path pm
```

### Managing Projects

```bash
# Create a project
pillar project create "MyProject" --priority medium

# List all projects
pillar project list

# Filter projects
pillar project list --status in-progress --priority high

# Show project details
pillar project show "MyProject"

# Update project
pillar project edit "MyProject" --status in-progress --priority high
```

### Managing Milestones

```bash
# Create a milestone
pillar milestone create MyProject "v1.0" --date 2026-06-01

# List milestones
pillar milestone list

# Filter by project
pillar milestone list --project MyProject

# Update milestone
pillar milestone edit MyProject "v1.0" --status in-progress
```

### Managing Issues

```bash
# Create an issue
pillar issue create MyProject "Add user authentication" \
  --priority high \
  --milestone "v1.0" \
  --tags "feature,security"

# List issues
pillar issue list

# Filter issues
pillar issue list --status todo --priority high --project MyProject

# Update issue
pillar issue edit MyProject/001 --status in-progress
```

### Comments

```bash
# Add comment to project
pillar comment add project MyProject "Sprint planning completed"

# Add comment to issue
pillar comment add issue MyProject 1 "Fixed in commit abc123"

# Add comment to milestone
pillar comment add milestone MyProject "v1.0" "All features complete"

# List comments
pillar comment list project MyProject
pillar comment list issue MyProject 1
```

### Viewing Status

```bash
# Overview of all work
pillar status

# Kanban board view
pillar board

# Board for specific project
pillar board MyProject
```

## 📂 File Structure

When you run `pillar init`, it creates the following structure:

```
.pillar/
├── config.toml           # Workspace configuration
└── templates/            # Templates for new entities
    ├── project.md
    ├── milestone.md
    └── issue.md

MyProject/                # Project directory
├── README.md             # Project metadata and description
├── milestones/           # Milestone files
│   └── v1.0.md
└── issues/               # Issue files
    ├── 001-add-auth.md
    └── 002-fix-bug.md
```

### Example Issue File

```markdown
---
title: Add user authentication
status: in-progress
priority: high
project: MyProject
milestone: v1.0
tags: [feature, security]
created: 2025-12-29T10:00:00Z
---

# Add user authentication

## Description

Implement OAuth2 authentication for user login.

## Acceptance Criteria

- [ ] OAuth2 integration
- [ ] Session management
- [ ] Logout functionality

## Comments

### [2025-12-29T14:30:00Z] - Alice
Started implementation, using passport.js

### [2025-12-29T16:45:00Z] - Bob
Added tests for auth flow
```

## 🎯 Status Values

- `backlog` - Not yet prioritized
- `todo` - Ready to work on
- `in-progress` - Currently being worked on
- `completed` - Finished
- `cancelled` - Not doing this

## 🔥 Priority Levels

- `low` - Nice to have
- `medium` - Standard priority
- `high` - Important
- `urgent` - Critical, needs immediate attention

## 🔧 Configuration

### Base Directory

Keep your PM data separate from code:

```bash
# Initialize with base directory
pillar init --path pm

# Now all projects go in ./pm/ directory
pillar project create "MyProject"
# Creates: pm/MyProject/
```

The configuration is stored in `.pillar/config.toml`:

```toml
[workspace]
version = "0.1.0"
base_directory = "pm"

[defaults]
priority = "medium"
status = "backlog"
```

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/pillar.git
cd pillar

# Run tests (single-threaded to avoid race conditions)
cargo test -- --test-threads=1

# Run with logging
RUST_LOG=debug cargo run -- init

# Build release
cargo build --release
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by tools like Linear, Jira, and GitHub Issues
- Built with [Rust](https://www.rust-lang.org/) and [clap](https://github.com/clap-rs/clap)
- Follows the [Keep a Changelog](https://keepachangelog.com/) format

## 📊 Why Pillar?

**Git-Friendly**: All your tasks are in plain text files that can be versioned with Git. No more losing task history or dealing with database backups.

**Developer-Focused**: Designed for developers who live in the terminal. No context switching to a web browser.

**Portable**: Your data is just Markdown files. Easy to read, easy to edit, easy to migrate.

**Flexible**: Works with your workflow. Use it alongside code, keep it separate, or integrate it into your documentation.

## 💡 Use Cases

- **Solo Developers**: Track personal projects and TODOs
- **Small Teams**: Collaborate using Git for task management
- **Open Source**: Manage issues alongside code in the same repository
- **Documentation**: Tasks embedded in your documentation site
- **Offline Work**: No internet required, everything is local

## 🐛 Bug Reports & Feature Requests

Found a bug? Have an idea for a feature? Please [open an issue](https://github.com/yourusername/pillar/issues/new)!

## ⭐ Show Your Support

If you find Pillar useful, please consider:
- Giving it a star on GitHub
- Sharing it with others
- Contributing to the project
- Reporting bugs or suggesting features

