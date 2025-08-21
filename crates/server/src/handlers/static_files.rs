use axum::{
    extract::{Request, State},
    http::{header, StatusCode, Uri},
    response::{Html, Response},
};
use container_codes_shared::Result;
use std::{path::PathBuf, sync::Arc};
use tokio::fs;
use tracing::{debug, warn};

use crate::server::AppState;

pub async fn serve_static(
    State(state): State<Arc<AppState>>,
    uri: Uri,
    request: Request,
) -> Response {
    if !state.config.server.static_files.enabled {
        return not_found_response().await;
    }

    let path = uri.path().trim_start_matches('/');
    let static_root = PathBuf::from(&state.config.server.static_files.root);
    
    // If path is empty, try index files
    let file_path = if path.is_empty() {
        find_index_file(&static_root, &state.config.server.static_files.index_files).await
    } else {
        let requested_path = static_root.join(path);
        
        // Security check: ensure the path is within the static root
        if !requested_path.starts_with(&static_root) {
            warn!("Attempted path traversal: {}", path);
            return forbidden_response().await;
        }
        
        Some(requested_path)
    };

    if let Some(file_path) = file_path {
        match serve_file(&file_path, &state).await {
            Ok(response) => {
                debug!("Served static file: {}", file_path.display());
                response
            }
            Err(_) => {
                // If file doesn't exist and this looks like a SPA route, serve index.html
                if is_spa_route(path) {
                    if let Some(index_path) = find_index_file(&static_root, &state.config.server.static_files.index_files).await {
                        match serve_file(&index_path, &state).await {
                            Ok(response) => response,
                            Err(_) => not_found_response().await,
                        }
                    } else {
                        not_found_response().await
                    }
                } else {
                    not_found_response().await
                }
            }
        }
    } else {
        not_found_response().await
    }
}

async fn serve_file(file_path: &PathBuf, state: &AppState) -> Result<Response> {
    let contents = fs::read(file_path).await.map_err(|e| {
        container_codes_shared::Error::io(e)
    })?;

    let mime_type = mime_guess::from_path(file_path)
        .first_or_octet_stream()
        .to_string();

    let mut response_builder = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime_type.clone());

    // Add caching headers if enabled
    if state.config.server.static_files.etag {
        let etag = generate_etag(&contents);
        response_builder = response_builder.header(header::ETAG, etag);
    }

    // Add cache control header
    response_builder = response_builder.header(
        header::CACHE_CONTROL,
        &state.config.server.static_files.cache_control,
    );

    // Add security headers
    if state.config.server.security.security_headers {
        response_builder = response_builder
            .header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
            .header(header::X_FRAME_OPTIONS, &state.config.server.security.frame_options);
        
        if state.config.server.security.xss_protection {
            response_builder = response_builder.header("X-XSS-Protection", "1; mode=block");
        }
    }

    Ok(response_builder
        .body(contents.into())
        .unwrap())
}

async fn find_index_file(static_root: &PathBuf, index_files: &[String]) -> Option<PathBuf> {
    for index_file in index_files {
        let index_path = static_root.join(index_file);
        if fs::metadata(&index_path).await.is_ok() {
            return Some(index_path);
        }
    }
    None
}

fn is_spa_route(path: &str) -> bool {
    // Simple heuristic: if the path doesn't contain a file extension,
    // it's likely a SPA route
    !path.contains('.') && !path.is_empty()
}

fn generate_etag(contents: &[u8]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    contents.hash(&mut hasher);
    format!("\"{}\"", hasher.finish())
}

async fn not_found_response() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(header::CONTENT_TYPE, "text/html")
        .body(Html("<h1>404 Not Found</h1><p>The requested resource was not found.</p>").to_string().into())
        .unwrap()
}

async fn forbidden_response() -> Response {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header(header::CONTENT_TYPE, "text/html")
        .body(Html("<h1>403 Forbidden</h1><p>Access denied.</p>").to_string().into())
        .unwrap()
}