use crate::error::{ConfigError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub proxy: Option<ProxyConfig>,
    pub containers: Option<ContainerConfig>,
    pub jobs: Option<JobConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub tls: TlsConfig,
    pub static_files: StaticConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TlsConfig {
    pub enabled: bool,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
    pub auto_cert: bool,
    pub domains: Vec<String>,
    pub acme_email: Option<String>,
    pub acme_directory: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StaticConfig {
    pub enabled: bool,
    pub root: String,
    pub index_files: Vec<String>,
    pub compression: bool,
    pub compression_types: Vec<String>,
    pub cache_control: String,
    pub etag: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecurityConfig {
    pub cors_origins: Vec<String>,
    pub cors_methods: Vec<String>,
    pub cors_headers: Vec<String>,
    pub rate_limit_enabled: bool,
    pub rate_limit_requests: u32,
    pub rate_limit_window: String,
    pub security_headers: bool,
    pub hsts_max_age: u32,
    pub content_type_nosniff: bool,
    pub frame_options: String,
    pub xss_protection: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
    pub file_path: Option<String>,
    pub rotation: String,
    pub max_files: u32,
    pub tracing: TracingConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TracingConfig {
    pub enabled: bool,
    pub jaeger_endpoint: Option<String>,
    pub service_name: String,
    pub sample_rate: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: String,
    pub idle_timeout: String,
    pub max_lifetime: String,
    pub auto_migrate: bool,
    pub migration_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
    pub connection_timeout: String,
    pub command_timeout: String,
    pub retry_attempts: u32,
    pub queue: QueueConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueueConfig {
    pub default_queue: String,
    pub retry_queue: String,
    pub failed_queue: String,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProxyConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub https_redirect: bool,
    pub ssl: ProxySslConfig,
    pub balancing: BalancingConfig,
    pub health: HealthConfig,
    pub upstreams: Vec<UpstreamConfig>,
    pub routes: Vec<RouteConfig>,
    pub middleware: MiddlewareConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProxySslConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub cert_dir: String,
    pub key_dir: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BalancingConfig {
    pub strategy: String,
    pub session_affinity: bool,
    pub session_cookie: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HealthConfig {
    pub enabled: bool,
    pub interval: String,
    pub timeout: String,
    pub healthy_threshold: u32,
    pub unhealthy_threshold: u32,
    pub check_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpstreamConfig {
    pub name: String,
    pub strategy: String,
    pub servers: Vec<ServerInstanceConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerInstanceConfig {
    pub address: String,
    pub weight: u32,
    pub max_fails: u32,
    pub fail_timeout: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RouteConfig {
    pub path: String,
    pub method: String,
    pub upstream: String,
    pub rewrite: Option<bool>,
    pub strip_prefix: Option<String>,
    pub add_headers: Option<HashMap<String, String>>,
    pub timeout: Option<String>,
    pub retries: Option<u32>,
    pub websocket: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MiddlewareConfig {
    pub rate_limit_enabled: bool,
    pub rate_limit_requests: u32,
    pub rate_limit_window: String,
    pub rate_limit_key: String,
    pub add_request_headers: Option<HashMap<String, String>>,
    pub remove_request_headers: Option<Vec<String>>,
    pub add_response_headers: Option<HashMap<String, String>>,
    pub remove_response_headers: Option<Vec<String>>,
    pub compression_enabled: bool,
    pub compression_level: u32,
    pub compression_types: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContainerConfig {
    pub docker_host: String,
    pub api_version: String,
    pub timeout: String,
    pub defaults: ContainerDefaults,
    pub security: ContainerSecurity,
    pub network: NetworkConfig,
    pub volumes: VolumeConfig,
    pub images: ImageConfig,
    pub registries: Vec<RegistryConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContainerDefaults {
    pub cpu_limit: String,
    pub memory_limit: String,
    pub network_mode: String,
    pub restart_policy: String,
    pub log_driver: String,
    pub log_options: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContainerSecurity {
    pub drop_capabilities: Vec<String>,
    pub add_capabilities: Vec<String>,
    pub user: String,
    pub read_only: bool,
    pub no_new_privileges: bool,
    pub seccomp_profile: String,
    pub apparmor_profile: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub default_network: String,
    pub enable_isolation: bool,
    pub dns_servers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VolumeConfig {
    pub base_path: String,
    pub default_options: Vec<String>,
    pub cleanup_orphaned: bool,
    pub cleanup_interval: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageConfig {
    pub auto_pull: bool,
    pub pull_policy: String,
    pub cleanup_unused: bool,
    pub cleanup_interval: String,
    pub keep_tagged: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegistryConfig {
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobConfig {
    pub default_queue: String,
    pub max_concurrent_jobs: u32,
    pub job_timeout: String,
    pub cleanup_completed: bool,
    pub cleanup_after: String,
    pub workers: WorkerConfig,
    pub container: JobContainerConfig,
    pub security: JobSecurityConfig,
    pub files: FileConfig,
    pub retry: RetryConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkerConfig {
    pub count: u32,
    pub poll_interval: String,
    pub batch_size: u32,
    pub max_memory: String,
    pub max_cpu: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobContainerConfig {
    pub base_image: String,
    pub network_mode: String,
    pub cpu_limit: String,
    pub memory_limit: String,
    pub disk_limit: String,
    pub timeout: String,
    pub cleanup: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobSecurityConfig {
    pub user: String,
    pub read_only: bool,
    pub work_dirs: Vec<String>,
    pub drop_capabilities: Vec<String>,
    pub no_network: bool,
    pub max_processes: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileConfig {
    pub input_dir: String,
    pub output_dir: String,
    pub max_input_size: String,
    pub max_output_size: String,
    pub retention_period: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub backoff_strategy: String,
    pub base_delay: String,
    pub max_delay: String,
    pub jitter: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitoringConfig {
    pub collect_metrics: bool,
    pub metrics_interval: String,
    pub monitor_resources: bool,
    pub collect_logs: bool,
    pub log_level: String,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path).map_err(|_| ConfigError::FileNotFound {
            path: path.as_ref().display().to_string(),
        })?;
        
        let mut config: Config = toml::from_str(&content)?;
        config.apply_env_overrides();
        config.validate()?;
        
        Ok(config)
    }

    pub fn load_from_env() -> Result<Self> {
        let config_path = env::var("CONTAINER_CODES_CONFIG")
            .unwrap_or_else(|_| "config/server.toml".to_string());
        
        Self::load_from_file(config_path)
    }

    fn apply_env_overrides(&mut self) {
        for (key, value) in env::vars() {
            if let Some(config_key) = key.strip_prefix("CONTAINER_CODES_") {
                self.set_from_env_key(config_key, &value);
            }
        }
    }

    fn set_from_env_key(&mut self, key: &str, value: &str) {
        let parts: Vec<&str> = key.split('_').collect();
        
        match parts.as_slice() {
            ["SERVER", "PORT"] => {
                if let Ok(port) = value.parse() {
                    self.server.port = port;
                }
            }
            ["SERVER", "HOST"] => {
                self.server.host = value.to_string();
            }
            ["DATABASE", "URL"] => {
                self.database.url = value.to_string();
            }
            ["REDIS", "URL"] => {
                self.redis.url = value.to_string();
            }
            ["LOGGING", "LEVEL"] => {
                self.logging.level = value.to_string();
            }
            _ => {
                // Handle nested configurations with double underscores
                // Implementation would be more complex for full support
            }
        }
    }

    fn validate(&self) -> Result<()> {
        if self.server.port == 0 {
            return Err(crate::Error::config_invalid("server.port", "0"));
        }

        if self.server.host.is_empty() {
            return Err(crate::Error::config_missing("server.host"));
        }

        if self.database.url.is_empty() {
            return Err(crate::Error::config_missing("database.url"));
        }

        if self.redis.url.is_empty() {
            return Err(crate::Error::config_missing("redis.url"));
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
            database: DatabaseConfig::default(),
            redis: RedisConfig::default(),
            proxy: None,
            containers: None,
            jobs: None,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: 0,
            tls: TlsConfig::default(),
            static_files: StaticConfig::default(),
            security: SecurityConfig::default(),
        }
    }
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_file: None,
            key_file: None,
            auto_cert: false,
            domains: vec![],
            acme_email: None,
            acme_directory: "https://acme-v02.api.letsencrypt.org/directory".to_string(),
        }
    }
}

impl Default for StaticConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            root: "./public".to_string(),
            index_files: vec!["index.html".to_string(), "index.htm".to_string()],
            compression: true,
            compression_types: vec![
                "text/html".to_string(),
                "text/css".to_string(),
                "application/javascript".to_string(),
                "application/json".to_string(),
            ],
            cache_control: "public, max-age=3600".to_string(),
            etag: true,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            cors_origins: vec!["*".to_string()],
            cors_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
            cors_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
            rate_limit_enabled: false,
            rate_limit_requests: 100,
            rate_limit_window: "1m".to_string(),
            security_headers: true,
            hsts_max_age: 31536000,
            content_type_nosniff: true,
            frame_options: "DENY".to_string(),
            xss_protection: true,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
            output: "stdout".to_string(),
            file_path: None,
            rotation: "daily".to_string(),
            max_files: 30,
            tracing: TracingConfig::default(),
        }
    }
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            jaeger_endpoint: None,
            service_name: "container-codes".to_string(),
            sample_rate: 0.1,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgresql://localhost/container_codes".to_string(),
            max_connections: 50,
            min_connections: 5,
            connection_timeout: "30s".to_string(),
            idle_timeout: "10m".to_string(),
            max_lifetime: "1h".to_string(),
            auto_migrate: false,
            migration_path: "./migrations".to_string(),
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            pool_size: 50,
            connection_timeout: "5s".to_string(),
            command_timeout: "30s".to_string(),
            retry_attempts: 3,
            queue: QueueConfig::default(),
        }
    }
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            default_queue: "jobs".to_string(),
            retry_queue: "retry".to_string(),
            failed_queue: "failed".to_string(),
            max_retries: 3,
        }
    }
}

pub fn parse_duration(s: &str) -> Result<Duration> {
    if let Some(stripped) = s.strip_suffix('s') {
        Ok(Duration::from_secs(stripped.parse().map_err(|_| {
            crate::Error::config_invalid("duration", s)
        })?))
    } else if let Some(stripped) = s.strip_suffix('m') {
        Ok(Duration::from_secs(stripped.parse::<u64>().map_err(|_| {
            crate::Error::config_invalid("duration", s)
        })? * 60))
    } else if let Some(stripped) = s.strip_suffix('h') {
        Ok(Duration::from_secs(stripped.parse::<u64>().map_err(|_| {
            crate::Error::config_invalid("duration", s)
        })? * 3600))
    } else {
        Err(crate::Error::config_invalid("duration", s))
    }
}