use crate::models::{Issue, Milestone, Project};
use anyhow::Result;
use axum::{
    extract::Path,
    http::{header, StatusCode, Uri},
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[derive(RustEmbed)]
#[folder = "services/ui/dist/"]
struct Assets;

#[derive(Serialize)]
struct UIProject {
    #[serde(flatten)]
    inner: Project,
    id: String,
}

#[derive(Serialize)]
struct UIMilestone {
    #[serde(flatten)]
    inner: Milestone,
    id: String,
}

#[derive(Serialize)]
struct UIIssue {
    #[serde(flatten)]
    inner: Issue,
    id: String,
    number: String,
}

#[derive(Serialize)]
struct UIData {
    projects: Vec<UIProject>,
    milestones: Vec<UIMilestone>,
    issues: Vec<UIIssue>,
}

#[derive(Deserialize)]
struct UpdateIssueRequest {
    status: Option<String>,
    priority: Option<String>,
    milestone: Option<String>,
    tags: Option<String>,
    description: Option<String>,
}

#[derive(Deserialize)]
struct CreateIssueRequest {
    project: String,
    title: String,
    priority: String,
    milestone: Option<String>,
    tags: Option<String>,
}

#[derive(Deserialize)]
struct UpdateProjectRequest {
    status: Option<String>,
    priority: Option<String>,
    description: Option<String>,
}

#[derive(Deserialize)]
struct CreateProjectRequest {
    name: String,
    id: Option<String>,
    priority: String,
}

#[derive(Deserialize)]
struct UpdateMilestoneRequest {
    status: Option<String>,
    target_date: Option<String>,
    description: Option<String>,
}

#[derive(Deserialize)]
struct CreateMilestoneRequest {
    project: String,
    title: String,
    date: Option<String>,
}

pub async fn run_ui(port: u16) -> Result<()> {
    // Assets are embedded at compile time.
    // If the UI isn't built, Assets::iter() will be empty or folder won't exist.
    if Assets::iter().count() == 0 {
        println!("Warning: No UI assets found. Did you run 'npm run build' in services/ui before compiling?");
    }

    let app = Router::new()
        .route(
            "/api/data",
            get(move || async move {
                match get_ui_data() {
                    Ok(data) => Json(data).into_response(),
                    Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
                }
            }),
        )
        .route("/api/issues/:project/:number", patch(update_issue_handler))
        .route("/api/issues", post(create_issue_handler))
        .route("/api/projects/:id", patch(update_project_handler))
        .route("/api/projects", post(create_project_handler))
        .route(
            "/api/milestones/:project/:title",
            patch(update_milestone_handler),
        )
        .route("/api/milestones", post(create_milestone_handler))
        .fallback(get(static_handler))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Starting Pillar UI on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn update_issue_handler(
    Path((project, number)): Path<(String, String)>,
    Json(payload): Json<UpdateIssueRequest>,
) -> impl IntoResponse {
    let id = format!("{}/{}", project, number);
    match crate::commands::edit_issue(
        &id,
        payload.status.as_deref(),
        payload.priority.as_deref(),
        payload.milestone.as_deref(),
        payload.tags.as_deref(),
    ) {
        Ok(_) => {
            // If description is provided, we need to update it separately since edit_issue doesn't support it yet
            if let Some(content) = payload.description {
                if let Err(e) = update_issue_description(&id, &content) {
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
            }
            StatusCode::OK.into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

fn update_issue_description(id: &str, content: &str) -> Result<()> {
    let base_dir = crate::fs::get_base_directory()?;
    let (project_name, issue_id) = id
        .split_once('/')
        .ok_or_else(|| anyhow::anyhow!("Invalid ID"))?;
    let project_path = base_dir.join(project_name);
    let issues = crate::fs::list_issues(&project_path)?;
    let issue = issues
        .into_iter()
        .find(|i| {
            i.path
                .file_stem()
                .and_then(|s| s.to_str())
                .and_then(|s| s.split('-').next())
                == Some(issue_id)
        })
        .ok_or_else(|| anyhow::anyhow!("Issue not found"))?;

    crate::parser::write_with_frontmatter(&issue.path, &issue.metadata, content)?;
    Ok(())
}

async fn create_issue_handler(Json(payload): Json<CreateIssueRequest>) -> impl IntoResponse {
    match crate::commands::create_issue(
        &payload.project,
        &payload.title,
        &payload.priority,
        payload.milestone.as_deref(),
        payload.tags.as_deref(),
    ) {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn update_project_handler(
    Path(id): Path<String>,
    Json(payload): Json<UpdateProjectRequest>,
) -> impl IntoResponse {
    match crate::commands::edit_project(&id, payload.status.as_deref(), payload.priority.as_deref())
    {
        Ok(_) => {
            if let Some(content) = payload.description {
                if let Err(e) = update_project_description(&id, &content) {
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
            }
            StatusCode::OK.into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

fn update_project_description(name: &str, content: &str) -> Result<()> {
    let base_dir = crate::fs::get_base_directory()?;
    let project = crate::fs::find_project(&base_dir, name)?;
    crate::parser::write_with_frontmatter(
        project.path.join("README.md"),
        &project.metadata,
        content,
    )?;
    Ok(())
}

async fn create_project_handler(Json(payload): Json<CreateProjectRequest>) -> impl IntoResponse {
    match crate::commands::create_project(&payload.name, payload.id.as_deref(), &payload.priority) {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn update_milestone_handler(
    Path((project, title)): Path<(String, String)>,
    Json(payload): Json<UpdateMilestoneRequest>,
) -> impl IntoResponse {
    match crate::commands::edit_milestone(
        &project,
        &title,
        payload.status.as_deref(),
        payload.target_date.as_deref(),
    ) {
        Ok(_) => {
            if let Some(content) = payload.description {
                if let Err(e) = update_milestone_description(&project, &title, &content) {
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
            }
            StatusCode::OK.into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

fn update_milestone_description(project_name: &str, title: &str, content: &str) -> Result<()> {
    let base_dir = crate::fs::get_base_directory()?;
    let project = crate::fs::find_project(&base_dir, project_name)?;
    let milestones = crate::fs::list_milestones(&project.path)?;
    let milestone = milestones
        .into_iter()
        .find(|m| m.metadata.title == title)
        .ok_or_else(|| anyhow::anyhow!("Milestone not found"))?;

    crate::parser::write_with_frontmatter(&milestone.path, &milestone.metadata, content)?;
    Ok(())
}

async fn create_milestone_handler(
    Json(payload): Json<CreateMilestoneRequest>,
) -> impl IntoResponse {
    match crate::commands::create_milestone(
        &payload.project,
        &payload.title,
        payload.date.as_deref(),
    ) {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path();

    // Trim leading slash and default to index.html for root or SPA fallback
    let path = path.trim_start_matches('/');
    let asset_path = if path.is_empty() { "index.html" } else { path };

    match Assets::get(asset_path).or_else(|| Assets::get("index.html")) {
        Some(content) => {
            let mime = mime_guess::from_path(asset_path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Not Found").into_response(),
    }
}

fn get_ui_data() -> Result<UIData> {
    let base_dir = crate::fs::get_base_directory()?;
    let projects = crate::fs::list_projects(&base_dir)?;

    let mut ui_projects = Vec::new();
    let mut ui_milestones = Vec::new();
    let mut ui_issues = Vec::new();

    for project in projects {
        let project_id = project.metadata.name.clone();
        let project_path = project.path.clone();

        ui_projects.push(UIProject {
            id: project_id.clone(),
            inner: project,
        });

        if let Ok(p_milestones) = crate::fs::list_milestones(&project_path) {
            for m in p_milestones {
                ui_milestones.push(UIMilestone {
                    id: m.metadata.title.clone(),
                    inner: m,
                });
            }
        }

        if let Ok(p_issues) = crate::fs::list_issues(&project_path) {
            for i in p_issues {
                let filename = i.path.file_name().and_then(|f| f.to_str()).unwrap_or("");
                let number = filename.split('-').next().unwrap_or("000").to_string();

                ui_issues.push(UIIssue {
                    id: format!("{}/{}", project_id, number),
                    number,
                    inner: i,
                });
            }
        }
    }

    Ok(UIData {
        projects: ui_projects,
        milestones: ui_milestones,
        issues: ui_issues,
    })
}
