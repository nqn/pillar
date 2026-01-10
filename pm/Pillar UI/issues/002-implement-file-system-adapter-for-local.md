---
title: Implement File System Adapter for local mode
status: completed
priority: high
project: Pillar UI
milestone: Stateless Web UI
tags: []
created: 2026-01-10T13:20:19.326616Z
updated: 2026-01-10T13:26:50.779469Z
---

# Implement File System Adapter for local mode

## Description

Instead of fetching data from a backend API backed by Prisma/SQL, the local WebUI needs a "File System Adapter". This adapter will interface with the `pillar` rust library (or use a small local server) to read/write markdown files directly.

## Acceptance Criteria

- [ ] Create a `LocalDataProvider` that implements the same interface as the current Cloud provider.
- [ ] Implement `read` operations for Projects, Milestones, and Issues by parsing `.pillar` and `pm/` directories.
- [ ] Implement `write` operations that update markdown frontmatter and content.
- [ ] Ensure the UI handles file system latency and errors gracefully.
- [ ] Support "hot reloading" if a file is changed externally (optional but recommended).