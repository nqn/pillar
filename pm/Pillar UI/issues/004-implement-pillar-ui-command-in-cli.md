---
title: Implement pillar ui command in CLI
status: completed
priority: high
project: Pillar UI
milestone: Stateless Web UI
tags: []
created: 2026-01-10T13:20:19.470812Z
updated: 2026-01-10T13:26:50.878683Z
---

# Implement pillar ui command in CLI

## Description

The `pillar` CLI needs a new subcommand `ui` that starts a local web server (likely serving the built SPA) and opens the user's browser.

## Acceptance Criteria

- [ ] Add `ui` subcommand to the Rust CLI (`src/main.rs`).
- [ ] Implement a lightweight HTTP server (e.g., using `axum` or `actix-web`) that:
    - Serves the static UI assets.
    - Provides a minimal JSON API for the File System Adapter to consume.
- [ ] The command should detect the current `.pillar` configuration and serve data from the `base_directory`.
- [ ] Automatically open the default browser to `localhost:<port>`.
- [ ] Ensure the server shuts down cleanly on Ctrl+C.