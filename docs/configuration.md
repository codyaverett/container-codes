# Configuration Reference - Container Codes

## Overview
Container Codes uses TOML configuration files for all settings. The configuration supports hot-reloading, environment variable overrides, and validation.

## Configuration File Structure

### Main Configuration (`config/server.toml`)
```toml
[server]
# Network binding
host = "0.0.0.0"
port = 8080
workers = 0  # 0 = number of CPU cores

# TLS configuration
[server.tls]
enabled = true
cert_file = "/etc/ssl/certs/server.crt"
key_file = "/etc/ssl/private/server.key"
# Automatic Let's Encrypt
auto_cert = true
domains = ["example.com", "www.example.com"]
acme_email = "admin@example.com"
acme_directory = "https://acme-v02.api.letsencrypt.org/directory"

# Static file serving
[server.static]
enabled = true
root = "./public"
index_files = ["index.html", "index.htm"]
compression = true
compression_types = ["text/html", "text/css", "application/javascript", "application/json"]
cache_control = "public, max-age=3600"
etag = true

# Security settings
[server.security]
# CORS configuration
cors_origins = ["https://example.com"]
cors_methods = ["GET", "POST", "PUT", "DELETE"]
cors_headers = ["Content-Type", "Authorization"]

# Rate limiting
rate_limit_enabled = true
rate_limit_requests = 100
rate_limit_window = "1m"

# Security headers
security_headers = true
hsts_max_age = 31536000
content_type_nosniff = true
frame_options = "DENY"
xss_protection = true

# Logging configuration
[logging]
level = "info"
format = "json"  # json, pretty, compact
output = "stdout"  # stdout, stderr, file
file_path = "/var/log/container-codes/server.log"
rotation = "daily"  # daily, weekly, size
max_files = 30

# Tracing and observability
[logging.tracing]
enabled = true
jaeger_endpoint = "http://localhost:14268/api/traces"
service_name = "container-codes"
sample_rate = 0.1

# Database configuration
[database]
url = "postgresql://user:password@localhost/container_codes"
max_connections = 50
min_connections = 5
connection_timeout = "30s"
idle_timeout = "10m"
max_lifetime = "1h"

# Migrations
auto_migrate = true
migration_path = "./migrations"

# Redis configuration
[redis]
url = "redis://localhost:6379"
pool_size = 50
connection_timeout = "5s"
command_timeout = "30s"
retry_attempts = 3

# Job queue configuration
[redis.queue]
default_queue = "jobs"
retry_queue = "retry"
failed_queue = "failed"
max_retries = 3
```

### Proxy Configuration (`config/proxy.toml`)
```toml
[proxy]
enabled = true
bind_address = "0.0.0.0:80"
https_redirect = true

# SSL termination
[proxy.ssl]
enabled = true
bind_address = "0.0.0.0:443"
cert_dir = "/etc/ssl/certs"
key_dir = "/etc/ssl/private"

# Load balancing
[proxy.balancing]
strategy = "round_robin"  # round_robin, least_connections, ip_hash, random
session_affinity = false
session_cookie = "SESSIONID"

# Health checking
[proxy.health]
enabled = true
interval = "30s"
timeout = "5s"
healthy_threshold = 2
unhealthy_threshold = 3
check_path = "/health"

# Upstream servers
[[proxy.upstreams]]
name = "web-servers"
strategy = "round_robin"

[[proxy.upstreams.servers]]
address = "127.0.0.1:8001"
weight = 1
max_fails = 3
fail_timeout = "30s"

[[proxy.upstreams.servers]]
address = "127.0.0.1:8002"
weight = 2
max_fails = 3
fail_timeout = "30s"

[[proxy.upstreams]]
name = "api-servers"
strategy = "least_connections"

[[proxy.upstreams.servers]]
address = "127.0.0.1:3001"
weight = 1

[[proxy.upstreams.servers]]
address = "127.0.0.1:3002"
weight = 1

# Routing rules
[[proxy.routes]]
path = "/"
method = "GET"
upstream = "web-servers"
rewrite = false

[[proxy.routes]]
path = "/api/*"
method = "ANY"
upstream = "api-servers"
strip_prefix = "/api"
add_headers = { "X-API-Version" = "v1" }
timeout = "30s"
retries = 3

[[proxy.routes]]
path = "/ws/*"
method = "GET"
upstream = "api-servers"
websocket = true

# Middleware configuration
[proxy.middleware]
# Rate limiting
rate_limit_enabled = true
rate_limit_requests = 1000
rate_limit_window = "1m"
rate_limit_key = "ip"  # ip, header, cookie

# Request/Response modification
add_request_headers = { "X-Forwarded-Proto" = "https" }
remove_request_headers = ["X-Internal-Auth"]
add_response_headers = { "X-Served-By" = "container-codes" }
remove_response_headers = ["Server"]

# Compression
compression_enabled = true
compression_level = 6
compression_types = ["text/*", "application/json", "application/javascript"]
```

### Container Configuration (`config/containers.toml`)
```toml
[containers]
# Docker configuration
docker_host = "unix:///var/run/docker.sock"
api_version = "1.41"
timeout = "60s"

# Default resource limits
[containers.defaults]
cpu_limit = "1.0"
memory_limit = "512m"
network_mode = "bridge"
restart_policy = "unless-stopped"
log_driver = "json-file"
log_options = { "max-size" = "10m", "max-file" = "3" }

# Security settings
[containers.security]
# Default capabilities to drop
drop_capabilities = ["ALL"]
# Capabilities to add back
add_capabilities = ["CHOWN", "SETUID", "SETGID"]
# Run as non-root user
user = "1000:1000"
# Read-only root filesystem
read_only = true
# No new privileges
no_new_privileges = true
# Security profiles
seccomp_profile = "default"
apparmor_profile = "docker-default"

# Networking
[containers.network]
# Default network for containers
default_network = "container-codes"
# Network isolation
enable_isolation = true
# Custom DNS
dns_servers = ["8.8.8.8", "8.8.4.4"]

# Volume management
[containers.volumes]
# Base directory for volumes
base_path = "/var/lib/container-codes/volumes"
# Default mount options
default_options = ["rw", "nosuid", "nodev"]
# Cleanup orphaned volumes
cleanup_orphaned = true
cleanup_interval = "24h"

# Image management
[containers.images]
# Auto-pull images
auto_pull = true
pull_policy = "missing"  # always, missing, never
# Image cleanup
cleanup_unused = true
cleanup_interval = "24h"
keep_tagged = true

# Registry configuration
[[containers.registries]]
name = "docker.io"
url = "https://index.docker.io/v1/"
username = ""
password = ""

[[containers.registries]]
name = "ghcr.io"
url = "https://ghcr.io"
username = ""
password = ""
```

### Job Configuration (`config/jobs.toml`)
```toml
[jobs]
# Queue configuration
default_queue = "default"
max_concurrent_jobs = 10
job_timeout = "1h"
cleanup_completed = true
cleanup_after = "24h"

# Worker configuration
[jobs.workers]
count = 4
poll_interval = "5s"
batch_size = 1
max_memory = "1g"
max_cpu = "2.0"

# Container settings for jobs
[jobs.container]
# Base image for job containers
base_image = "alpine:latest"
# Network isolation
network_mode = "none"
# Resource limits
cpu_limit = "1.0"
memory_limit = "512m"
disk_limit = "1g"
# Execution timeout
timeout = "3600s"
# Cleanup containers after completion
cleanup = true

# Security settings
[jobs.security]
# Run as non-root user
user = "65534:65534"  # nobody user
# Read-only filesystem except work directories
read_only = true
# Work directories (writable)
work_dirs = ["/tmp", "/work", "/output"]
# Dropped capabilities
drop_capabilities = ["ALL"]
# No network access
no_network = true
# Process limits
max_processes = 100

# File handling
[jobs.files]
# Input file staging area
input_dir = "/var/lib/container-codes/jobs/input"
# Output file collection area
output_dir = "/var/lib/container-codes/jobs/output"
# Maximum input file size
max_input_size = "100m"
# Maximum output size per job
max_output_size = "1g"
# File retention period
retention_period = "7d"

# Retry configuration
[jobs.retry]
max_attempts = 3
backoff_strategy = "exponential"  # fixed, linear, exponential
base_delay = "30s"
max_delay = "10m"
jitter = true

# Monitoring
[jobs.monitoring]
# Metrics collection
collect_metrics = true
metrics_interval = "10s"
# Resource monitoring
monitor_resources = true
# Log collection
collect_logs = true
log_level = "info"
```

## Environment Variable Overrides

All configuration values can be overridden using environment variables with the format:
`CONTAINER_CODES_<SECTION>_<KEY>`

Examples:
```bash
# Override server port
export CONTAINER_CODES_SERVER_PORT=9090

# Override database URL
export CONTAINER_CODES_DATABASE_URL="postgresql://localhost/mydb"

# Override log level
export CONTAINER_CODES_LOGGING_LEVEL=debug

# Override TLS settings
export CONTAINER_CODES_SERVER_TLS_ENABLED=false
```

For nested configurations, use double underscores:
```bash
# Override TLS certificate file
export CONTAINER_CODES_SERVER_TLS__CERT_FILE="/path/to/cert.pem"

# Override proxy health check interval
export CONTAINER_CODES_PROXY_HEALTH__INTERVAL="60s"
```

## Configuration Validation

The server validates all configuration on startup and when reloading:

```bash
# Validate configuration
container-codes config validate

# Validate specific file
container-codes config validate --file config/proxy.toml

# Show effective configuration (with env overrides)
container-codes config show

# Hot-reload configuration
container-codes config reload
```

## Development Configuration

Example development configuration (`config/dev.toml`):
```toml
[server]
host = "127.0.0.1"
port = 3000
workers = 1

[server.tls]
enabled = false

[server.static]
root = "./frontend/dist"

[logging]
level = "debug"
format = "pretty"

[database]
url = "postgresql://dev:dev@localhost/container_codes_dev"
auto_migrate = true

[redis]
url = "redis://localhost:6379/1"

[jobs]
max_concurrent_jobs = 2

[jobs.container]
cleanup = false  # Keep containers for debugging
```

## Production Configuration

Example production configuration (`config/prod.toml`):
```toml
[server]
host = "0.0.0.0"
port = 443
workers = 0

[server.tls]
enabled = true
auto_cert = true
domains = ["api.example.com"]
acme_email = "admin@example.com"

[server.security]
rate_limit_enabled = true
security_headers = true

[logging]
level = "info"
format = "json"
output = "file"
file_path = "/var/log/container-codes/server.log"

[logging.tracing]
enabled = true
jaeger_endpoint = "http://jaeger:14268/api/traces"

[database]
url = "postgresql://prod_user:secure_password@db-cluster:5432/container_codes"
max_connections = 50

[redis]
url = "redis://redis-cluster:6379"

[jobs]
max_concurrent_jobs = 20

[jobs.security]
no_network = true
drop_capabilities = ["ALL"]
```