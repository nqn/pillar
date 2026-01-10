---
title: Custom Markdown Templates (Archetypes)
status: todo
priority: medium
project: Future Enhancements
milestone: User Feedback Batch 1
tags:
- cli
- template
created: 2026-01-10T21:38:07.192520Z
updated: 2026-01-10T21:38:07.192520Z
---

# Custom Markdown Templates (Archetypes)

## Description

Currently, after using the CLI to create a project or issue, I often have to manually edit the file to add "Goals," "Overview," or "Acceptance Criteria" sections.

Request: Support for `base_directory/.pillar/templates/issue.md` and `project.md` that define the initial structure and frontmatter defaults.

## Acceptance Criteria

- [ ] Support for user-defined templates in the `.pillar/templates/` directory.
- [ ] Ability to define default frontmatter and body structure.
- [ ] Default templates (archetypes) for projects, milestones, and issues.