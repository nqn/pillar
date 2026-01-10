---
title: Multi-select filters for status and priority
status: todo
priority: medium
project: Web UI
milestone: Projects and Milestones View
tags:
- ui
- filters
created: 2026-01-10T14:22:46.788014Z
updated: 2026-01-10T14:22:46.788014Z
---

# Multi-select filters for status and priority

## Description

Refactor the existing status and priority filters in the Web UI to support multiple selection. This will allow users to filter for multiple states simultaneously (e.g., "Todo" AND "In Progress") or exclude specific states (e.g., "All except Completed").

## Acceptance Criteria

- [ ] Status filter supports selecting multiple values.
- [ ] Priority filter supports selecting multiple values.
- [ ] UI reflects active filters clearly (e.g., tag-like indicators).
- [ ] Filter logic correctly combines multiple selections (OR logic within a category).
- [ ] Ability to easily clear all filters or specific selections.