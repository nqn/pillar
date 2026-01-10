---
title: UI Daemon Mode
status: todo
priority: high
project: Future Enhancements
milestone: User Feedback Batch 1
tags:
- ui
- cli
created: 2026-01-10T21:38:07.209304Z
updated: 2026-01-10T21:38:07.209304Z
---

# UI "Daemon" Mode

## Description

The `pillar ui` command is great, but as we saw, it’s an interactive process that blocks the terminal.

Request: A `pillar ui --daemon` or `pillar ui start` command that runs the server in the background and provides a URL, allowing it to stay alive across different agent turns or terminal sessions.

## Acceptance Criteria

- [ ] New `--daemon` flag or `start` subcommand for `pillar ui`.
- [ ] Ability to run the UI server as a background process.
- [ ] A `pillar ui stop` command to shut down the daemon.
- [ ] A `pillar ui status` command to check if the server is running.
- [ ] Automated logging of the daemon's background output.