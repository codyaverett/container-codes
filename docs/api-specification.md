# API Specification - Container Codes

## Overview
Container Codes provides a comprehensive REST API for managing the webserver, containers, jobs, and proxy configurations. All endpoints support JSON request/response format with proper HTTP status codes.

## Authentication
All API endpoints require authentication via JWT tokens or API keys (configurable).

```http
Authorization: Bearer <jwt-token>
# or
X-API-Key: <api-key>
```

## Core Server API

### System Information
```http
GET /api/system/info
```
**Response:**
```json
{
  "version": "1.0.0",
  "uptime": 3600,
  "memory_usage": 67108864,
  "cpu_usage": 12.5,
  "active_connections": 150
}
```

### Health Check
```http
GET /api/health
```
**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z",
  "checks": {
    "database": "healthy",
    "redis": "healthy",
    "docker": "healthy"
  }
}
```

### Configuration Management
```http
GET /api/config
POST /api/config/reload
PUT /api/config/validate
```

## File Management API

### File Upload
```http
POST /api/files/upload
Content-Type: multipart/form-data

{
  "file": <binary-data>,
  "path": "/uploads/document.pdf",
  "permissions": "644"
}
```

### File Download
```http
GET /api/files/download/{path}
```
**Headers:**
- `Range`: bytes=0-1023 (optional, for partial content)
- `If-None-Match`: "etag-value" (optional, for caching)

### File Information
```http
GET /api/files/info/{path}
```
**Response:**
```json
{
  "path": "/uploads/document.pdf",
  "size": 1048576,
  "mime_type": "application/pdf",
  "created_at": "2024-01-01T12:00:00Z",
  "modified_at": "2024-01-01T12:30:00Z",
  "etag": "d41d8cd98f00b204e9800998ecf8427e",
  "permissions": "644"
}
```

## Container Management API

### List Containers
```http
GET /api/containers
```
**Query Parameters:**
- `status`: filter by status (running, stopped, paused)
- `image`: filter by image name
- `limit`: number of results (default: 50)
- `offset`: pagination offset

**Response:**
```json
{
  "containers": [
    {
      "id": "c123456789",
      "name": "web-app-1",
      "image": "nginx:latest",
      "status": "running",
      "created_at": "2024-01-01T12:00:00Z",
      "ports": ["80:8080", "443:8443"],
      "cpu_usage": 15.2,
      "memory_usage": 33554432
    }
  ],
  "total": 1,
  "limit": 50,
  "offset": 0
}
```

### Create Container
```http
POST /api/containers
```
**Request:**
```json
{
  "name": "my-app",
  "image": "node:18-alpine",
  "command": ["npm", "start"],
  "environment": {
    "NODE_ENV": "production",
    "PORT": "3000"
  },
  "ports": {
    "3000": "8080"
  },
  "volumes": {
    "/app/data": "/host/data"
  },
  "resources": {
    "cpu_limit": "0.5",
    "memory_limit": "512m"
  },
  "network_mode": "bridge",
  "restart_policy": "unless-stopped"
}
```

### Container Operations
```http
GET    /api/containers/{id}           # Get container details
POST   /api/containers/{id}/start     # Start container
POST   /api/containers/{id}/stop      # Stop container
POST   /api/containers/{id}/restart   # Restart container
POST   /api/containers/{id}/pause     # Pause container
POST   /api/containers/{id}/unpause   # Unpause container
DELETE /api/containers/{id}           # Remove container
```

### Container Logs
```http
GET /api/containers/{id}/logs
```
**Query Parameters:**
- `follow`: true/false (stream logs)
- `tail`: number of lines from end
- `since`: timestamp filter
- `timestamps`: include timestamps

### Container Stats
```http
GET /api/containers/{id}/stats
```
**Response:**
```json
{
  "cpu_usage": 25.5,
  "memory_usage": 67108864,
  "memory_limit": 134217728,
  "network_rx": 1024,
  "network_tx": 2048,
  "block_read": 4096,
  "block_write": 8192,
  "timestamp": "2024-01-01T12:00:00Z"
}
```

## Job Management API

### Submit Job
```http
POST /api/jobs
```
**Request:**
```json
{
  "name": "data-processing-job",
  "image": "python:3.11-slim",
  "command": ["python", "process.py"],
  "environment": {
    "INPUT_FILE": "/input/data.csv",
    "OUTPUT_DIR": "/output"
  },
  "input_files": [
    {
      "source": "/uploads/data.csv",
      "destination": "/input/data.csv"
    }
  ],
  "output_patterns": [
    "/output/*.json",
    "/output/report.pdf"
  ],
  "resources": {
    "cpu_limit": "1.0",
    "memory_limit": "1g",
    "timeout": "3600s"
  },
  "priority": "normal",
  "retry_policy": {
    "max_attempts": 3,
    "backoff": "exponential"
  }
}
```

**Response:**
```json
{
  "job_id": "job_123456789",
  "status": "queued",
  "created_at": "2024-01-01T12:00:00Z",
  "estimated_start": "2024-01-01T12:05:00Z"
}
```

### Job Status
```http
GET /api/jobs/{id}
```
**Response:**
```json
{
  "job_id": "job_123456789",
  "name": "data-processing-job",
  "status": "running",
  "progress": 45.5,
  "created_at": "2024-01-01T12:00:00Z",
  "started_at": "2024-01-01T12:05:00Z",
  "estimated_completion": "2024-01-01T12:45:00Z",
  "container_id": "c987654321",
  "resource_usage": {
    "cpu_time": 120.5,
    "memory_peak": 67108864,
    "network_io": 1048576
  },
  "output_files": []
}
```

### Job Operations
```http
GET    /api/jobs                     # List jobs
POST   /api/jobs/{id}/cancel         # Cancel job
POST   /api/jobs/{id}/retry          # Retry failed job
GET    /api/jobs/{id}/logs           # Get job logs
GET    /api/jobs/{id}/output         # List output files
GET    /api/jobs/{id}/output/{file}  # Download output file
```

### List Jobs
```http
GET /api/jobs
```
**Query Parameters:**
- `status`: filter by status (queued, running, completed, failed, cancelled)
- `user`: filter by user
- `created_since`: timestamp filter
- `limit`: pagination limit
- `offset`: pagination offset

## Reverse Proxy API

### Proxy Configuration
```http
GET /api/proxy/config
PUT /api/proxy/config
```

### Upstream Management
```http
GET    /api/proxy/upstreams          # List upstreams
POST   /api/proxy/upstreams          # Create upstream
GET    /api/proxy/upstreams/{name}   # Get upstream
PUT    /api/proxy/upstreams/{name}   # Update upstream
DELETE /api/proxy/upstreams/{name}   # Delete upstream
```

**Upstream Configuration:**
```json
{
  "name": "api-servers",
  "strategy": "round_robin",
  "health_check": {
    "enabled": true,
    "path": "/health",
    "interval": "30s",
    "timeout": "5s",
    "healthy_threshold": 2,
    "unhealthy_threshold": 3
  },
  "servers": [
    {
      "address": "127.0.0.1:3001",
      "weight": 1,
      "max_fails": 3,
      "fail_timeout": "30s"
    },
    {
      "address": "127.0.0.1:3002",
      "weight": 2,
      "max_fails": 3,
      "fail_timeout": "30s"
    }
  ]
}
```

### Route Management
```http
GET    /api/proxy/routes             # List routes
POST   /api/proxy/routes             # Create route
GET    /api/proxy/routes/{id}        # Get route
PUT    /api/proxy/routes/{id}        # Update route
DELETE /api/proxy/routes/{id}        # Delete route
```

**Route Configuration:**
```json
{
  "id": "route_123",
  "path": "/api/*",
  "method": "ANY",
  "upstream": "api-servers",
  "middleware": ["rate_limit", "auth"],
  "headers": {
    "X-Forwarded-For": "$remote_addr",
    "X-Real-IP": "$remote_addr"
  },
  "timeout": "30s",
  "retries": 3
}
```

### Proxy Statistics
```http
GET /api/proxy/stats
```
**Response:**
```json
{
  "requests_total": 150000,
  "requests_per_second": 125.5,
  "response_times": {
    "avg": 45.2,
    "p50": 35.0,
    "p95": 120.0,
    "p99": 250.0
  },
  "error_rate": 0.02,
  "upstreams": {
    "api-servers": {
      "total_requests": 75000,
      "active_connections": 25,
      "servers": [
        {
          "address": "127.0.0.1:3001",
          "status": "healthy",
          "requests": 25000,
          "response_time": 42.1
        }
      ]
    }
  }
}
```

## WebSocket API

### Real-time Updates
```http
GET /api/ws/updates
Upgrade: websocket
```
**Message Types:**
```json
{
  "type": "job_status",
  "data": {
    "job_id": "job_123456789",
    "status": "completed",
    "progress": 100
  }
}

{
  "type": "container_event",
  "data": {
    "container_id": "c123456789",
    "event": "start",
    "timestamp": "2024-01-01T12:00:00Z"
  }
}

{
  "type": "system_metric",
  "data": {
    "cpu_usage": 15.2,
    "memory_usage": 67108864,
    "timestamp": "2024-01-01T12:00:00Z"
  }
}
```

## Error Responses

All API endpoints return consistent error responses:

```json
{
  "error": {
    "code": "CONTAINER_NOT_FOUND",
    "message": "Container with ID 'c123456789' not found",
    "details": {
      "container_id": "c123456789",
      "available_containers": ["c987654321", "c456789123"]
    },
    "request_id": "req_123456789"
  }
}
```

**Common Error Codes:**
- `INVALID_REQUEST`: 400 - Malformed request
- `UNAUTHORIZED`: 401 - Authentication required
- `FORBIDDEN`: 403 - Insufficient permissions
- `NOT_FOUND`: 404 - Resource not found
- `CONFLICT`: 409 - Resource conflict
- `RATE_LIMITED`: 429 - Rate limit exceeded
- `INTERNAL_ERROR`: 500 - Internal server error
- `SERVICE_UNAVAILABLE`: 503 - Service temporarily unavailable