---
title: Refactor Navigation to remove multi-repo search
status: completed
priority: high
project: Pillar UI
milestone: Stateless Web UI
tags: []
created: 2026-01-10T13:20:19.401113Z
updated: 2026-01-10T13:26:50.827877Z
---

# Refactor Navigation to remove multi-repo search

## Description

The "multi-repo" / repo view search in the navigation bar is a "cloud" feature that allows searching across multiple repositories. For the local `pillar ui` experience, we only care about the current workspace. This needs to be removed or simplified.

## Acceptance Criteria

- [ ] Remove the multi-repo search input from the top navigation bar.
- [ ] Replace it with a simpler "Project Search" that only operates on the local workspace.
- [ ] Remove any "Repo switcher" or multi-repository navigation elements.
- [ ] Simplify the sidebar to focus only on Projects, Milestones, and Issues of the current context.