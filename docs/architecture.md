# Container Codes - Ultimate Webserver Architecture

## Overview
Container Codes is a high-performance Rust webserver with embedded React frontend, designed for maximum speed, security, and extensibility. It provides file serving with compression/encryption, reverse proxy capabilities, container management, and async job processing.

## Core Design Principles
- **Performance**: Zero-copy file serving, async I/O, optimized for throughput
- **Security**: TLS 1.3, container sandboxing, secure file handling
- **Modularity**: Workspace-based architecture with clear separation of concerns
- **Operability**: CLI management, structured logging, metrics, health checks

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Container Codes                          │
├─────────────────────────────────────────────────────────────┤
│  Frontend (React SPA)          │  Management CLI            │
│  ├─ Static Assets              │  ├─ Service Control        │
│  ├─ API Integration            │  ├─ Configuration          │
│  └─ Hot Reload (dev)           │  └─ Installation           │
├─────────────────────────────────────────────────────────────┤
│                    Core HTTP Server                        │
│  ├─ Axum Framework             │  ├─ Compression            │
│  ├─ File Serving               │  ├─ Caching                │
│  ├─ TLS Termination            │  └─ Range Requests         │
├─────────────────────────────────────────────────────────────┤
│  Reverse Proxy     │  Container Mgmt    │  Async Jobs      │
│  ├─ Load Balancing │  ├─ Docker API     │  ├─ Queue System │
│  ├─ Health Checks  │  ├─ Orchestration  │  ├─ Sandboxing  │
│  └─ SSL Termination│  └─ Networking     │  └─ File Output  │
├─────────────────────────────────────────────────────────────┤
│                    Shared Infrastructure                   │
│  ├─ Configuration Management   │  ├─ Logging & Tracing     │
│  ├─ Database Connections       │  ├─ Metrics & Monitoring  │
│  └─ Security & Auth            │  └─ Error Handling        │
└─────────────────────────────────────────────────────────────┘
```

## Crate Structure

### `crates/shared/`
Common utilities and types used across all crates:
- Configuration management (TOML parsing, validation)
- Error types and handling
- Logging setup and utilities
- Security primitives
- Database connection pooling

### `crates/server/`
Core HTTP server implementation:
- Axum-based async web framework
- Static file serving with compression (Brotli, Gzip)
- TLS 1.3 with automatic certificate management
- Caching with ETags and conditional requests
- Range request support for large files
- WebSocket support

### `crates/cli/`
Command-line management interface:
- Service installation and management
- Configuration validation and hot-reloading
- Log viewing and system status
- Certificate management
- Database migrations

### `crates/proxy/`
Reverse proxy functionality:
- HTTP/HTTPS load balancing
- Health checking and failover
- SSL termination and SNI routing
- Rate limiting and request filtering
- WebSocket proxying

### `crates/containers/`
Container orchestration and management:
- Docker/Podman API integration
- Container lifecycle management
- Network isolation and port mapping
- Resource limits and security policies
- Image building and caching

### `crates/jobs/`
Async job processing system:
- Redis/PostgreSQL-backed job queue
- Container-based job sandboxing
- File generation and secure retrieval
- Job scheduling and retry logic
- Progress tracking and notifications

## Technology Stack

### Core Runtime
- **Tokio**: Async runtime for high-performance I/O
- **Axum**: Modern web framework with excellent ecosystem
- **Tower**: Middleware and service abstractions
- **Hyper**: HTTP/1.1 and HTTP/2 implementation

### Security & TLS
- **rustls**: Memory-safe TLS implementation
- **rustls-acme**: Automatic Let's Encrypt integration
- **ring**: Cryptographic primitives

### Data Storage
- **SQLx**: Async PostgreSQL driver with compile-time query checking
- **Redis**: In-memory cache and job queue
- **Serde**: Serialization framework

### Container Integration
- **Bollard**: Docker API client
- **Sysinfo**: System information and resource monitoring

### Frontend Integration
- **React**: Single-page application framework
- **TypeScript**: Type-safe JavaScript
- **Webpack/Vite**: Build tooling and asset pipeline

## Key Features

### High-Performance File Serving
- Zero-copy sendfile() for large files
- Automatic compression based on content type
- Intelligent caching with proper HTTP headers
- Range request support for video/audio streaming
- Content-Type detection with MIME sniffing

### Security Features
- TLS 1.3 with perfect forward secrecy
- Automatic certificate provisioning and renewal
- Container sandboxing for job execution
- Secure file upload and download
- Rate limiting and DDoS protection

### Operational Excellence
- Structured logging with tracing
- Prometheus metrics integration
- Health check endpoints
- Graceful shutdown handling
- Configuration hot-reloading

### Developer Experience
- Hot-reload development mode
- Generated TypeScript API clients
- Comprehensive error messages
- CLI tools for all operations
- Docker-based development environment