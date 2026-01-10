---
title: Remove database and syncing dependencies from UI bundle
status: completed
priority: high
project: Pillar UI
milestone: Stateless Web UI
tags: []
created: 2026-01-10T13:20:19.546523Z
updated: 2026-01-10T13:26:50.926220Z
---

# Remove database and syncing dependencies from UI bundle

## Description

The local version of Pillar UI should be as lean as possible. We should strip out heavy dependencies like Prisma (if used in the frontend/client), SQL clients, or syncing logic (e.g., CRDTs if applicable) that are only needed for the cloud version.

## Acceptance Criteria

- [ ] Audit `package.json` and remove dependencies not needed for local mode.
- [ ] Use build-time flags (e.g., Vite env variables) to exclude cloud-only code paths.
- [ ] Ensure the final build artifact is small and fast to load.
- [ ] Verify that no background network requests (other than those to the local server) are made by the UI.