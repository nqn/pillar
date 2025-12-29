use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pillar")]
#[command(about = "A file-based task tracker CLI", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Pillar workspace
    Init {
        /// Base directory for project data (relative to workspace root)
        #[arg(short = 'p', long = "path")]
        base_directory: Option<String>,
    },
    
    /// Manage projects
    #[command(subcommand)]
    Project(ProjectCommands),
    
    /// Manage milestones
    #[command(subcommand)]
    Milestone(MilestoneCommands),
    
    /// Manage issues
    #[command(subcommand)]
    Issue(IssueCommands),
    
    /// Manage comments
    #[command(subcommand)]
    Comment(CommentCommands),
    
    /// Show workspace status overview
    Status,
    
    /// Display Kanban board view
    Board {
        /// Optional project name to filter by
        project: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    /// Create a new project
    Create {
        /// Name of the project
        name: String,
        
        /// Priority (low, medium, high, urgent)
        #[arg(short, long, default_value = "medium")]
        priority: String,
    },
    
    /// List all projects
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
        
        /// Filter by priority
        #[arg(short, long)]
        priority: Option<String>,
    },
    
    /// Show project details
    Show {
        /// Name of the project
        name: String,
    },
    
    /// Edit project metadata
    Edit {
        /// Name of the project
        name: String,
        
        /// Update status
        #[arg(long)]
        status: Option<String>,
        
        /// Update priority
        #[arg(long)]
        priority: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum MilestoneCommands {
    /// Create a new milestone
    Create {
        /// Project name
        project: String,
        
        /// Milestone title
        title: String,
        
        /// Target date (YYYY-MM-DD)
        #[arg(short, long)]
        date: Option<String>,
    },
    
    /// List milestones
    List {
        /// Optional project name to filter by
        project: Option<String>,
    },
    
    /// Edit milestone metadata
    Edit {
        /// Project name
        project: String,
        
        /// Milestone title
        title: String,
        
        /// Update status
        #[arg(long)]
        status: Option<String>,
        
        /// Update target date
        #[arg(long)]
        date: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum IssueCommands {
    /// Create a new issue
    Create {
        /// Project name
        project: String,
        
        /// Issue title
        title: String,
        
        /// Priority (low, medium, high, urgent)
        #[arg(short, long, default_value = "medium")]
        priority: String,
        
        /// Milestone to assign to
        #[arg(short, long)]
        milestone: Option<String>,
        
        /// Tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
    },
    
    /// List issues
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
        
        /// Filter by priority
        #[arg(short, long)]
        priority: Option<String>,
        
        /// Filter by project
        #[arg(short = 'P', long)]
        project: Option<String>,
        
        /// Filter by milestone
        #[arg(short, long)]
        milestone: Option<String>,
        
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
    },
    
    /// Show issue details
    Show {
        /// Issue ID (e.g., project-name/001)
        id: String,
    },
    
    /// Edit issue metadata
    Edit {
        /// Issue ID (e.g., project-name/001)
        id: String,
        
        /// Update status
        #[arg(long)]
        status: Option<String>,
        
        /// Update priority
        #[arg(long)]
        priority: Option<String>,
        
        /// Update milestone
        #[arg(long)]
        milestone: Option<String>,
        
        /// Update tags (comma-separated, replaces existing)
        #[arg(long)]
        tags: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum CommentCommands {
    /// Add a comment to a project, milestone, or issue
    Add {
        /// Entity type: project, milestone, or issue
        #[arg(value_parser = ["project", "milestone", "issue"])]
        entity_type: String,
        
        /// Project name
        project: String,
        
        /// Comment content
        content: String,
        
        /// Milestone title or issue ID (not needed for projects)
        identifier: Option<String>,
    },
    
    /// List comments on a project, milestone, or issue
    List {
        /// Entity type: project, milestone, or issue
        #[arg(value_parser = ["project", "milestone", "issue"])]
        entity_type: String,
        
        /// Project name
        project: String,
        
        /// Milestone title or issue ID (not needed for projects)
        identifier: Option<String>,
    },
}
