# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.3] - 2026-01-10

### Added
- **Project Detail View**: 
  - Rendered Markdown `README.md` support for project documentation
  - Integrated Project Milestone Timeline (Gantt chart)
  - Detailed Issue list within the project context with metadata pills
  - Premium navigation with a dedicated "Back" system
- **Navigational Enhancements**:
  - Sidebar Project Filters: Quickly toggle between **Active**, **Done**, and **All** projects
  - Sidebar metadata: Project status and priority indicators now visible in the navigation
  - Enhanced Project Sorting (by Status, Priority, or Name)
- **Stability & Performance**:
  - Replaced Radix ScrollArea with native vertical scrolling for improved stability and layout predictability
  - Resolved persistent horizontal overflow and grid wrapping issues
  - Fixed project sorting logic in the dashboard list view

## [0.2.2] - 2026-01-10

### Added
- Complete UI migration to Radix UI component library and iconography (Radix Icons)
- New collapsible Sidebar system:
  - Sidebar can now be fully minimized to increase workspace area
  - Inlined "Hamburger" expand trigger in the main header
  - Improved theme toggle and navigation persistence
- Enhanced Projects Dashboard:
  - Toggle between **Grid** and **List** views for better oversight
  - Integrated Project Filtering (Status, Priority, Search) directly into the view
  - Project Grouping support (by Status/Priority) with dynamic counts
  - Compact project cards with inline milestones and progress indicators
- Refined responsive layout with optimized grid systems for different screen sizes

### Fixed
- Improved header alignment and breadcrumb spacing
- Fixed project card width issues in the grid view
- Corrected multiple structural and linting issues in the UI codebase
- Removed unused `lucide-react` dependency in favor of Radix Icons

## [0.2.1] - 2026-01-10

### Added
- Interactive Web UI dashboard launched with `pillar ui`
  - Stateless, file-system-first architecture
  - Bundled directly into the binary for zero-config usage
  - Support for project/issue search, filtering, sorting, and grouping
  - Responsive dark/light theme supporting both master and detail views
  - Built-in Axum server for API and static asset delivery
- New `pillar prompts` command to provide ready-to-use system prompts for AI agents.

## [0.1.2] - 2025-12-30

### Added
- Project ID / short mnemonic feature for easier command usage
  - Optional `--id` flag when creating projects for custom short IDs
  - Auto-generated project IDs from project names (acronyms or first 4 chars)
  - All issue, milestone, and comment commands accept project IDs
  - Project IDs displayed in list and show commands
- Full backward compatibility with existing projects without IDs

### Changed
- `find_project` function now accepts either full project name or project ID
- Enhanced display output to show project IDs where available

## [0.1.1] - 2025-12-30

### Added
- Search command to find text across projects, milestones, and issues
- Export command with JSON and CSV formats for data portability
- Pre-commit hook for automatic code formatting checks
- Self-hosted project management using Pillar itself

### Fixed
- CI test failures by adding `--test-threads=1` to coverage job
- Clippy warnings for Rust 1.92 (use `is_some_and` and `strip_prefix`)
- Code formatting issues across all Rust files

### Changed
- Repository now uses Pillar to track its own development
- Improved documentation with hook setup instructions

## [0.1.0] - 2025-12-29

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

[Unreleased]: https://github.com/nqn/pillar/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/nqn/pillar/releases/tag/v0.1.0
