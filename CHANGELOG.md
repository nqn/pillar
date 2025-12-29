# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of Pillar task tracker
- Project management commands (create, list, show, edit)
- Milestone management commands (create, list, edit)
- Issue management commands (create, list, show, edit)
- Comment system for projects, milestones, and issues
- Kanban board view
- Status overview command
- Configurable base directory support
- File-based storage using Markdown with YAML frontmatter
- Comprehensive test suite (47 tests)
- Full documentation and examples

### Features
- Filter projects by status and priority
- Filter issues by status, priority, project, milestone, and tags
- Automatic issue ID generation
- Author detection from git config
- Template system for new entities
- Color-coded terminal output

## [0.1.0] - 2025-12-29

### Added
- Initial release
- Core functionality for project, milestone, and issue management
- Comment system
- Base directory configuration
- Kanban board and status views
- MIT License
- Comprehensive README
- Contributing guidelines

[Unreleased]: https://github.com/yourusername/pillar/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/pillar/releases/tag/v0.1.0
