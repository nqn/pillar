---
title: Direct Description/Body Input for Create
status: todo
priority: high
project: Future Enhancements
milestone: User Feedback Batch 1
tags:
- cli
created: 2026-01-10T21:38:07.198664Z
updated: 2026-01-10T21:38:07.198665Z
---

# Direct Description/Body Input for Create

## Description

Currently, `pillar issue create` only takes a title in the command line. To add a description, I have to perform a follow-up file write.

Request: Support for a `--description` (or `-d`) flag, or reading the description from stdin (e.g., `echo "desc" | pillar issue create ...`).

## Acceptance Criteria

- [ ] New `--description` or `-d` flag for `pillar issue create`.
- [ ] Support for reading the description from standard input (stdin).
- [ ] Proper handling of multi-line descriptions via the new flag or stdin.