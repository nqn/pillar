use crate::models::{Issue, Milestone, Project};
use anyhow::Result;
use axum::{
    http::{header, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use rust_embed::RustEmbed;
use serde::Serialize;
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
        .fallback(get(static_handler))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Starting Pillar UI on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
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
