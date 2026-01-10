# AI Agent Guide for Pillar

Pillar is designed to be "Agent-Friendly". This means it provides a clear, structured interface for AI coding agents to manage tasks directly within the repository they are working on.

## Suggested System Prompt

You can provide the following instructions to your AI agent (in their system prompt or instructions file) to enable them to use Pillar effectively:

---

### Pillar Task Tracking Instructions

1. You coordinate your work using `pillar` cli.

```bash
pillar -h
A file-based task tracker CLI

Usage: pillar <COMMAND>
```

#### Commands Overview

- **`pillar init`**: Initialize a workspace.
- **`pillar project create <NAME>`**: Create a new project.
- **`pillar milestone create <PROJECT> <TITLE>`**: Create a milestone.
- **`pillar issue create <PROJECT> <TITLE>`**: Create a new issue.
- **`pillar status`**: Show overview.
- **`pillar board`**: Show Kanban board.

#### Workflow Guidance

You should generally create projects, milestones, and issues via the CLI, and then expand on the generated Markdown files to capture deep design details, requirements, and acceptance criteria.

The normal flow for building a complex feature:
1. **Define data structures** within the issue or a separate design doc.
2. **Define core algorithms/interactions** and invariants.
3. **Build tests** ranging from unit to integration.
4. **Build tools/REPLs** for experimenting and verifying functionality layer by layer.

Always update issue status as you progress (`todo` -> `in-progress` -> `completed`).

---

## Why use an Agent with Pillar?

1. **Context Awareness**: The agent sees the tasks in the same filesystem as the code.
2. **Accountability**: You can see exactly what an agent is working on by glancing at the Kanban board.
3. **Implicit Memory**: The Markdown files serve as long-term memory for the agent across different sessions.
