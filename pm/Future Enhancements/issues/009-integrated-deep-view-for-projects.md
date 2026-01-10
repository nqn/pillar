---
title: Integrated Deep View for Projects
status: todo
priority: medium
project: Future Enhancements
milestone: User Feedback Batch 1
tags:
- cli
- ux
created: 2026-01-10T21:38:07.195903Z
updated: 2026-01-10T21:38:07.195903Z
---

# Integrated Deep View for Projects

## Description

When context-switching, I often need to see a project and its immediate active tasks without running three separate commands (`project show`, `milestone list`, `issue list`).

Request: `pillar project show --deep` to recursively display active milestones and their respective TODO issues in a single tree view.

## Acceptance Criteria

- [ ] New `--deep` flag for `pillar project show`.
- [ ] Recursively display projects, their milestones, and related issues.
- [ ] Support for filtering the deep view (e.g., only show `todo` issues).