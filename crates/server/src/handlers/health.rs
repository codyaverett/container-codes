use axum::{extract::State, response::Json};
use container_codes_shared::{
    types::{ApiResponse, HealthStatus},
    Result,
};
use std::{collections::HashMap, sync::Arc};
use tracing::instrument;

use crate::server::AppState;

#[instrument(skip(state))]
pub async fn health_check(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<HealthStatus>>> {
    let mut checks = HashMap::new();

    // Check database connectivity
    if let Some(ref db) = state.database {
        match db.health_check().await {
            Ok(_) => {
                checks.insert("database".to_string(), "healthy".to_string());
            }
            Err(e) => {
                tracing::warn!("Database health check failed: {}", e);
                checks.insert("database".to_string(), "unhealthy".to_string());
            }
        }
    } else {
        checks.insert("database".to_string(), "disabled".to_string());
    }

    // Check Redis connectivity (placeholder)
    checks.insert("redis".to_string(), "healthy".to_string());

    // Check Docker connectivity (placeholder)
    checks.insert("docker".to_string(), "healthy".to_string());

    let health_status = HealthStatus {
        status: if checks.values().all(|v| v == "healthy" || v == "disabled") {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        },
        timestamp: chrono::Utc::now(),
        checks,
        uptime: get_uptime(),
    };

    Ok(Json(ApiResponse::success(health_status)))
}

fn get_uptime() -> u64 {
    // This is a simple implementation - in production you'd track actual start time
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}