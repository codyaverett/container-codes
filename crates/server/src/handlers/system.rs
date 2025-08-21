use axum::{extract::State, response::Json};
use container_codes_shared::{
    types::{ApiResponse, SystemInfo},
    Result,
};
use std::sync::Arc;
use tracing::instrument;

use crate::server::AppState;

#[instrument(skip(state))]
pub async fn system_info(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<SystemInfo>>> {
    let system_info = SystemInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: get_uptime(),
        memory_usage: get_memory_usage(),
        cpu_usage: get_cpu_usage(),
        active_connections: get_active_connections(),
    };

    Ok(Json(ApiResponse::success(system_info)))
}

fn get_uptime() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn get_memory_usage() -> u64 {
    // Placeholder implementation - in production use sysinfo or similar
    64 * 1024 * 1024 // 64MB
}

fn get_cpu_usage() -> f64 {
    // Placeholder implementation - in production use sysinfo or similar
    15.2
}

fn get_active_connections() -> u32 {
    // Placeholder implementation - would track actual connections
    42
}