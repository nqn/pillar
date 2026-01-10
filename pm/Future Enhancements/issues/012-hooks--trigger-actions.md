---
title: Hooks / Trigger Actions
status: todo
priority: low
project: Future Enhancements
milestone: User Feedback Batch 1
tags:
- core
- automation
created: 2026-01-10T21:38:07.203833Z
updated: 2026-01-10T21:38:07.203833Z
---

# Hooks / Trigger Actions

## Description

It would be powerful if changing a status could trigger a script.

Request: A hooks section in `.pillar/config.toml` (e.g., `on_issue_complete = "npm run test"`).

## Acceptance Criteria

- [ ] Support for configuring hooks in `.pillar/config.toml`.
- [ ] Support for common triggers such as `on_issue_complete`, `on_project_create`, etc.
- [ ] Ability to pass context to the scripts (e.g., the ID of the completed issue).
- [ ] Safe execution of shell commands.