use axum::{
    extract::{Multipart, Path, State},
    http::{header, StatusCode},
    response::{Json, Response},
};
use container_codes_shared::{
    security::sanitize_filename,
    types::{ApiResponse, FileInfo},
    Result,
};
use std::{path::PathBuf, sync::Arc};
use tracing::{error, info, instrument};
use tokio::fs;

use crate::server::AppState;

#[instrument(skip(state, multipart))]
pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<String>>> {
    let upload_dir = PathBuf::from(&state.config.server.static_files.root).join("uploads");
    
    // Ensure upload directory exists
    if let Err(e) = fs::create_dir_all(&upload_dir).await {
        error!("Failed to create upload directory: {}", e);
        return Err(container_codes_shared::Error::internal(
            "Failed to create upload directory",
        ));
    }

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        container_codes_shared::Error::http(format!("Failed to read multipart field: {}", e))
    })? {
        let name = field.name().unwrap_or("unknown").to_string();
        
        if name == "file" {
            let file_name = field
                .file_name()
                .map(|n| sanitize_filename(n))
                .unwrap_or_else(|| format!("upload_{}", uuid::Uuid::new_v4()));

            let file_path = upload_dir.join(&file_name);
            
            let data = field.bytes().await.map_err(|e| {
                container_codes_shared::Error::http(format!("Failed to read file data: {}", e))
            })?;

            if let Err(e) = fs::write(&file_path, &data).await {
                error!("Failed to write uploaded file: {}", e);
                return Err(container_codes_shared::Error::internal(
                    "Failed to save uploaded file",
                ));
            }

            info!(
                file_name = %file_name,
                size = data.len(),
                "File uploaded successfully"
            );

            return Ok(Json(ApiResponse::success(format!(
                "File '{}' uploaded successfully",
                file_name
            ))));
        }
    }

    Err(container_codes_shared::Error::validation(
        "No file field found in multipart request",
    ))
}

#[instrument(skip(state))]
pub async fn download_file(
    State(state): State<Arc<AppState>>,
    Path(file_path): Path<String>,
) -> Result<Response> {
    let static_root = PathBuf::from(&state.config.server.static_files.root);
    let full_path = static_root.join(&file_path);

    // Security check: ensure the path is within the static root
    if !full_path.starts_with(&static_root) {
        return Err(container_codes_shared::Error::validation(
            "Invalid file path",
        ));
    }

    match fs::read(&full_path).await {
        Ok(contents) => {
            let mime_type = mime_guess::from_path(&full_path)
                .first_or_octet_stream()
                .to_string();

            let file_name = full_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("download");

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type)
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", file_name),
                )
                .body(contents.into())
                .unwrap())
        }
        Err(_) => Err(container_codes_shared::Error::http("File not found".to_string())),
    }
}

#[instrument(skip(state))]
pub async fn file_info(
    State(state): State<Arc<AppState>>,
    Path(file_path): Path<String>,
) -> Result<Json<ApiResponse<FileInfo>>> {
    let static_root = PathBuf::from(&state.config.server.static_files.root);
    let full_path = static_root.join(&file_path);

    // Security check: ensure the path is within the static root
    if !full_path.starts_with(&static_root) {
        return Err(container_codes_shared::Error::validation(
            "Invalid file path",
        ));
    }

    match fs::metadata(&full_path).await {
        Ok(metadata) => {
            let mime_type = mime_guess::from_path(&full_path)
                .first_or_octet_stream()
                .to_string();

            let created_at = metadata
                .created()
                .unwrap_or_else(|_| std::time::SystemTime::now())
                .into();

            let modified_at = metadata
                .modified()
                .unwrap_or_else(|_| std::time::SystemTime::now())
                .into();

            // Generate simple ETag based on size and modified time
            let etag = format!(
                "\"{}\"",
                format!("{}-{}", metadata.len(), modified_at.timestamp())
            );

            let file_info = FileInfo {
                path: file_path,
                size: metadata.len(),
                mime_type,
                created_at,
                modified_at,
                etag,
                permissions: format!("{:o}", metadata.permissions()),
            };

            Ok(Json(ApiResponse::success(file_info)))
        }
        Err(_) => Err(container_codes_shared::Error::http("File not found".to_string())),
    }
}