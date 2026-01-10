---
name: Pillar UI
project_id: pillar-ui
status: completed
priority: high
created: 2026-01-10T13:19:28.941765Z
updated: 2026-01-10T13:19:28.941765Z
---

# Pillar UI

A self-hosted, local WebUI for managing Pillar projects, milestones, and issues directly from the file system.

## Goals

- Provide a "stateless" WebUI that operates directly on the local file system.
- Enable `pillar ui` command for instant local browsing and editing.
- Ensure zero-dependency on external databases (Prisma, SQL) or syncing for the local mode.
- Deliver a premium, responsive interface for project management.

## Overview

The Pillar UI is designed to be a lightweight but powerful companion to the Pillar CLI. It leverages the file-based nature of Pillar to provide a rich visual interface without the overhead of a centralized server or database. When running in local mode, it identifies the current workstation's Pillar projects and allows for seamless interaction.