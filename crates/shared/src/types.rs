use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub request_id: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub limit: u32,
    pub offset: u32,
    pub has_next: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub checks: HashMap<String, String>,
    pub uptime: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub version: String,
    pub uptime: u64,
    pub memory_usage: u64,
    pub cpu_usage: f64,
    pub active_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContainerStatus {
    Created,
    Running,
    Paused,
    Restarting,
    Removing,
    Exited,
    Dead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: ContainerStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub ports: Vec<String>,
    pub environment: HashMap<String, String>,
    pub labels: HashMap<String, String>,
    pub resource_usage: Option<ResourceUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub memory_limit: u64,
    pub network_rx: u64,
    pub network_tx: u64,
    pub block_read: u64,
    pub block_write: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerCreateRequest {
    pub name: String,
    pub image: String,
    pub command: Option<Vec<String>>,
    pub environment: Option<HashMap<String, String>>,
    pub ports: Option<HashMap<String, String>>,
    pub volumes: Option<HashMap<String, String>>,
    pub resources: Option<ResourceLimits>,
    pub network_mode: Option<String>,
    pub restart_policy: Option<String>,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub cpu_limit: Option<String>,
    pub memory_limit: Option<String>,
    pub disk_limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobInfo {
    pub job_id: String,
    pub name: String,
    pub status: JobStatus,
    pub progress: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub container_id: Option<String>,
    pub resource_usage: Option<ResourceUsage>,
    pub output_files: Vec<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCreateRequest {
    pub name: String,
    pub image: String,
    pub command: Option<Vec<String>>,
    pub environment: Option<HashMap<String, String>>,
    pub input_files: Option<Vec<FileMapping>>,
    pub output_patterns: Option<Vec<String>>,
    pub resources: Option<ResourceLimits>,
    pub priority: Option<String>,
    pub retry_policy: Option<RetryPolicy>,
    pub timeout: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMapping {
    pub source: String,
    pub destination: String,
    pub permissions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff: String,
    pub delay: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub mime_type: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub etag: String,
    pub permissions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamServer {
    pub address: String,
    pub weight: u32,
    pub status: String,
    pub total_requests: u64,
    pub active_connections: u32,
    pub response_time: f64,
    pub error_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyStats {
    pub requests_total: u64,
    pub requests_per_second: f64,
    pub response_times: ResponseTimeStats,
    pub error_rate: f64,
    pub upstreams: HashMap<String, UpstreamStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimeStats {
    pub avg: f64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamStats {
    pub total_requests: u64,
    pub active_connections: u32,
    pub servers: Vec<UpstreamServer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketMessage {
    JobStatus { job_id: String, status: JobStatus, progress: Option<f64> },
    ContainerEvent { container_id: String, event: String, timestamp: DateTime<Utc> },
    SystemMetric { cpu_usage: f64, memory_usage: u64, timestamp: DateTime<Utc> },
    ProxyStats { stats: ProxyStats },
    LogEntry { level: String, message: String, timestamp: DateTime<Utc> },
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
            request_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn error(code: String, message: String) -> Self {
        Self {
            data: None,
            error: Some(ApiError {
                code,
                message,
                details: None,
            }),
            request_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn error_with_details(code: String, message: String, details: serde_json::Value) -> Self {
        Self {
            data: None,
            error: Some(ApiError {
                code,
                message,
                details: Some(details),
            }),
            request_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
        }
    }
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: u64, limit: u32, offset: u32) -> Self {
        let has_next = (offset + limit as u32) < total as u32;
        Self {
            items,
            total,
            limit,
            offset,
            has_next,
        }
    }
}

impl std::fmt::Display for ContainerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerStatus::Created => write!(f, "created"),
            ContainerStatus::Running => write!(f, "running"),
            ContainerStatus::Paused => write!(f, "paused"),
            ContainerStatus::Restarting => write!(f, "restarting"),
            ContainerStatus::Removing => write!(f, "removing"),
            ContainerStatus::Exited => write!(f, "exited"),
            ContainerStatus::Dead => write!(f, "dead"),
        }
    }
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Queued => write!(f, "queued"),
            JobStatus::Running => write!(f, "running"),
            JobStatus::Completed => write!(f, "completed"),
            JobStatus::Failed => write!(f, "failed"),
            JobStatus::Cancelled => write!(f, "cancelled"),
            JobStatus::Timeout => write!(f, "timeout"),
        }
    }
}