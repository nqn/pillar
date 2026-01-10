mod cli;
mod commands;
mod fs;
mod models;
mod parser;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, CommentCommands, IssueCommands, MilestoneCommands, ProjectCommands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { base_directory } => {
            commands::init(base_directory.as_deref())?;
        }
        Commands::Project(cmd) => match cmd {
            ProjectCommands::Create { name, id, priority } => {
                commands::create_project(&name, id.as_deref(), &priority)?;
            }
            ProjectCommands::List { status, priority } => {
                commands::list_projects(status.as_deref(), priority.as_deref())?;
            }
            ProjectCommands::Show { name } => {
                commands::show_project(&name)?;
            }
            ProjectCommands::Edit {
                name,
                status,
                priority,
            } => {
                commands::edit_project(&name, status.as_deref(), priority.as_deref())?;
            }
        },
        Commands::Milestone(cmd) => match cmd {
            MilestoneCommands::Create {
                project,
                title,
                date,
            } => {
                commands::create_milestone(&project, &title, date.as_deref())?;
            }
            MilestoneCommands::List { project } => {
                commands::list_milestones(project.as_deref())?;
            }
            MilestoneCommands::Edit {
                project,
                title,
                status,
                date,
            } => {
                commands::edit_milestone(&project, &title, status.as_deref(), date.as_deref())?;
            }
        },
        Commands::Issue(cmd) => match cmd {
            IssueCommands::Create {
                project,
                title,
                priority,
                milestone,
                tags,
            } => {
                commands::create_issue(
                    &project,
                    &title,
                    &priority,
                    milestone.as_deref(),
                    tags.as_deref(),
                )?;
            }
            IssueCommands::List {
                status,
                priority,
                project,
                milestone,
                tag,
            } => {
                commands::list_issues(
                    status.as_deref(),
                    priority.as_deref(),
                    project.as_deref(),
                    milestone.as_deref(),
                    tag.as_deref(),
                )?;
            }
            IssueCommands::Show { id } => {
                commands::show_issue(&id)?;
            }
            IssueCommands::Edit {
                id,
                status,
                priority,
                milestone,
                tags,
            } => {
                commands::edit_issue(
                    &id,
                    status.as_deref(),
                    priority.as_deref(),
                    milestone.as_deref(),
                    tags.as_deref(),
                )?;
            }
        },
        Commands::Comment(cmd) => match cmd {
            CommentCommands::Add {
                entity_type,
                project,
                content,
                identifier,
            } => {
                commands::comment::add(&entity_type, &project, identifier.as_deref(), &content)?;
            }
            CommentCommands::List {
                entity_type,
                project,
                identifier,
            } => {
                commands::comment::list(&entity_type, &project, identifier.as_deref())?;
            }
        },
        Commands::Status => {
            commands::status()?;
        }
        Commands::Board { project } => {
            commands::board(project.as_deref())?;
        }
        Commands::Search { query, entity_type } => {
            commands::search::search(&query, &entity_type)?;
        }
        Commands::Export {
            format,
            entity_type,
            output,
        } => {
            commands::export::export(&format, &entity_type, output.as_deref())?;
        }
        Commands::Ui { port } => {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?;
            rt.block_on(commands::run_ui(port))?;
        }
        Commands::Prompts => {
            print_agent_prompts();
        }
    }

    Ok(())
}

fn print_agent_prompts() {
    println!(
        r#"# AI Agent System Prompt for Pillar

Copy and paste the following into your AI agent's system prompt or instructions.

---

1. You coordinate your work using `pillar` cli.

```bash
pillar -h
A file-based task tracker CLI
```

#### Workflow Guidance

You should generally create projects, milestones, and issues via the CLI, and then expand on the generated Markdown files to capture deep design details, requirements, and acceptance criteria.

The normal flow for building a complex feature:
1. **Define data structures** within the issue or a separate design doc.
2. **Define core algorithms/interactions** and invariants.
3. **Build tests** ranging from unit to integration.
4. **Build tools/REPLs** for experimenting and verifying functionality layer by layer.

Always update issue status as you progress (`todo` -> `in-progress` -> `completed`).

---
"#
    );
}
