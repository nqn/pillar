---
title: Extract core UI components and types
status: completed
priority: high
project: Pillar UI
milestone: Stateless Web UI
tags: []
created: 2026-01-10T13:20:19.252554Z
updated: 2026-01-10T13:26:50.727412Z
---

# Extract core UI components and types

## Description

The current UI code in `pillar-cloud` (likely a React/Vite app) contains components that are tightly coupled with the "cloud" features (syncing, multi-repo). We need to extract the core presentation components (Project cards, Milestone lists, Issue editors) and the TypeScript types representing the Pillar domain model.

## Acceptance Criteria

- [ ] Identify and isolate core React components from `pillar-cloud`.
- [ ] Define shared TypeScript interfaces for `Project`, `Milestone`, `Issue`, and `Comment` that match the markdown frontmatter schema.
- [ ] Ensure components are "dumb" and receive data via props, making them easy to reuse in a stateless environment.
- [ ] Extract shared CSS/styling variables (using vanilla CSS as per tech stack requirements).