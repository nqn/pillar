---
title: Dependency Tracking (Blocking/Needs)
status: todo
priority: high
project: Future Enhancements
milestone: User Feedback Batch 1
tags:
- core
- cli
created: 2026-01-10T21:38:07.201378Z
updated: 2026-01-10T21:38:07.201378Z
---

# Dependency Tracking (Blocking/Needs)

## Description

As projects get complex, knowing which issue blocks another is crucial.

Request: Add `--blocks <issue-id>` and `--needs <issue-id>` to the CLI, which automatically updates the frontmatter of both issues and allows `pillar board` to visualize dependencies.

## Acceptance Criteria

- [ ] Support for `--blocks <issue-id>` and `--needs <issue-id>` flags in `pillar issue create` and `pillar issue edit`.
- [ ] Automatically update the corresponding frontmatter values in the related files.
- [ ] Visualization of dependencies in the Kanban board UI.
- [ ] Warning or validation for circular dependencies.