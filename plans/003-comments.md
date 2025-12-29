# Plan: Comment System

**Status**: Draft  
**Created**: 2025-12-29  
**Updated**: 2025-12-29

## Overview

Add support for comments on projects, milestones, and issues. Comments enable team collaboration by allowing users to add timestamped notes, discussions, and updates to any entity without modifying the main content.

## Goals

1. Enable adding comments to projects, milestones, and issues
2. Store comments in the entity's markdown file for simplicity
3. Display comments chronologically with author and timestamp
4. Support listing and viewing comments
5. Maintain backward compatibility with existing entities

## Non-Goals

- Comment threading/replies (can be added later)
- Comment reactions/likes
- Rich text formatting in comments (markdown in comments is future work)
- Comment notifications
- Comment search (covered by existing grep/semantic search)

## Use Cases

```bash
# Add a comment to a project
pillar comment add project "MyProject" "Initial planning phase complete"

# Add a comment to an issue
pillar comment add issue "MyProject" 1 "Fixed in commit abc123"

# Add a comment to a milestone
pillar comment add milestone "MyProject" "v1.0" "All features implemented, ready for testing"

# List comments on an entity
pillar comment list project "MyProject"
pillar comment list issue "MyProject" 1
pillar comment list milestone "MyProject" "v1.0"

# Show entity with comments included
pillar project show "MyProject"  # includes comments section
pillar issue show "MyProject" 1   # includes comments section
```

## Technical Design

### Data Model

Add a `Comment` struct to `models.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,           // UUID or sequential ID
    pub author: String,       // System username or git user.name
    pub timestamp: String,    // ISO 8601 format
    pub content: String,      // Comment text
}
```

### Storage Format

Store comments in a dedicated `## Comments` section at the end of each entity's markdown file:

```markdown
---
title: My Issue
status: in-progress
---

# My Issue

Issue description here.

## Comments

### [2025-12-29T10:30:00Z] - Alice
Started working on this issue

### [2025-12-29T14:15:00Z] - Bob
Found a related bug in module X

### [2025-12-29T16:45:00Z] - Alice
Fixed in commit abc123
```

### Parser Changes

Update `parser.rs`:

1. Add `read_comments()` function to extract comments from markdown body
2. Add `write_comments()` function to append/update comments section
3. Comments section is everything after `## Comments` heading
4. Each comment is a level-3 heading with format: `### [ISO_TIMESTAMP] - AUTHOR`
5. Comment content is the text following the heading until next heading or EOF

### Command Structure

Add new `src/commands/comment.rs` module:

```rust
pub fn add(
    entity_type: &str,      // "project", "milestone", or "issue"
    project_name: &str,
    identifier: Option<&str>, // milestone title or issue ID
    content: &str,
) -> Result<()>

pub fn list(
    entity_type: &str,
    project_name: &str,
    identifier: Option<&str>,
) -> Result<()>
```

### CLI Interface

Add to `cli.rs`:

```rust
#[derive(Subcommand)]
pub enum CommentCommand {
    /// Add a comment to a project, milestone, or issue
    Add {
        /// Entity type: project, milestone, or issue
        #[arg(value_parser = ["project", "milestone", "issue"])]
        entity_type: String,
        
        /// Project name
        project: String,
        
        /// Milestone title or issue ID (not needed for projects)
        identifier: Option<String>,
        
        /// Comment content
        content: String,
    },
    
    /// List comments on a project, milestone, or issue
    List {
        /// Entity type: project, milestone, or issue
        #[arg(value_parser = ["project", "milestone", "issue"])]
        entity_type: String,
        
        /// Project name
        project: String,
        
        /// Milestone title or issue ID (not needed for projects)
        identifier: Option<String>,
    },
}
```

### Author Detection

Implement `get_author()` helper in `fs.rs`:

1. Try `git config user.name` in workspace
2. Fall back to `$USER` environment variable
3. Fall back to "Unknown"

### Display Integration

Update existing show commands to display comments:

1. `project show` - display comments at bottom
2. `issue show` - display comments at bottom
3. Add `--no-comments` flag to suppress comments in show output

## Implementation Steps

1. **Add Comment model** (models.rs)
   - Add `Comment` struct with serde support
   - Add tests for serialization

2. **Add comment parsing** (parser.rs)
   - Implement `read_comments()` to extract from markdown
   - Implement `write_comments()` to append/update section
   - Add tests for parsing and writing

3. **Add author detection** (fs.rs)
   - Implement `get_author()` helper
   - Add test with mocked git config

4. **Create comment command module** (commands/comment.rs)
   - Implement `add()` function
   - Implement `list()` function
   - Add tests for both commands

5. **Update CLI** (cli.rs, main.rs)
   - Add `CommentCommand` enum
   - Wire up to main command handler

6. **Update show commands** (optional enhancement)
   - Add comment display to `project show`
   - Add comment display to `issue show`
   - Add `--no-comments` flag

7. **Documentation**
   - Update README with comment commands
   - Add examples to documentation

## Testing Strategy

1. **Unit Tests**
   - Comment parsing from markdown
   - Comment writing to markdown
   - Author detection fallback chain
   - Comment ID generation

2. **Integration Tests**
   - Add comment to project/milestone/issue
   - List comments returns correct data
   - Multiple comments in correct order
   - Comments survive entity edits

3. **Manual Testing**
   - Add comments to all entity types
   - Verify markdown format is readable
   - Test with missing git config
   - Verify backward compatibility with entities without comments

## Edge Cases

1. **Missing Comments Section**: If entity has no `## Comments`, create it
2. **Empty Comments**: Display appropriate message
3. **Malformed Comments**: Skip invalid comment blocks, log warning
4. **Concurrent Adds**: Last write wins (no locking for now)
5. **Long Comments**: Support multi-line content
6. **Special Characters**: Handle quotes, newlines in comment content

## Future Enhancements

1. **Comment Threading**: Add `reply_to` field for nested discussions
2. **Comment Editing**: `pillar comment edit <id>` command
3. **Comment Deletion**: `pillar comment delete <id>` command
4. **Markdown Support**: Allow markdown formatting in comment content
5. **Mentions**: Support @username mentions in comments
6. **Comment Filtering**: Filter comments by author or date range

## Backward Compatibility

- Entities without comments section work normally
- Reading entities ignores missing comments
- Comments section is optional in markdown
- Existing files continue to work without modification

## Success Criteria

- [ ] Can add comments to all three entity types
- [ ] Comments persist across command invocations
- [ ] Comments display with author and timestamp
- [ ] List command shows all comments in order
- [ ] Existing entities without comments still work
- [ ] All tests pass
- [ ] Documentation updated

## Open Questions

1. Should comments be editable? (Decision: No, not in MVP)
2. Should we support comment deletion? (Decision: Not in MVP)
3. How to handle multi-line comments? (Decision: Support with proper formatting)
4. Should comments appear in issue lists? (Decision: No, only in show/detail views)

## Changelog

- 2025-12-29: Initial plan created
