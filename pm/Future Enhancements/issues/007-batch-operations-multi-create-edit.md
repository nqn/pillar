---
title: Batch Operations (Multi-Create/Edit)
status: todo
priority: high
project: Future Enhancements
milestone: User Feedback Batch 1
tags:
- cli
- ux
created: 2026-01-10T21:38:07.188234Z
updated: 2026-01-10T21:38:07.188234Z
---

# Batch Operations (Multi-Create/Edit)

## Description

We often find ourselves running long chains of `pillar issue create` or `pillar issue edit` commands. A batch mode would be much more efficient.

Request: `pillar issue batch-create <project> <spec.yaml|json>` or `pillar issue edit --all --status completed`.

## Acceptance Criteria

- [ ] Support for creating multiple issues from a YAML or JSON file.
- [ ] Support for bulk editing issues using filters (e.g., `--all`, `--project`, `--status`).
- [ ] Command line feedback on the results of the batch operations.