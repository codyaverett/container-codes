use crate::handlers;
use crate::middleware::request_id::RequestIdLayer;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use container_codes_shared::{
    config::Config,
    database::Database,
};
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
};
use tracing::{info, instrument};

pub struct AppState {
    pub config: Config,
    pub database: Option<Database>,
}

#[instrument(skip(config))]
pub async fn start(config: Config) -> anyhow::Result<()> {
    let database = if !config.database.url.is_empty() {
        Some(Database::new(&config.database).await?)
    } else {
        None
    };

    let state = Arc::new(AppState { config: config.clone(), database });

    let app = create_router(state.clone());

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port).parse()?;
    
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: Arc<AppState>) -> Router {
    let api_routes = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/system/info", get(handlers::system::system_info))
        .route("/files/upload", post(handlers::files::upload_file))
        .route("/files/download/*path", get(handlers::files::download_file))
        .route("/files/info/*path", get(handlers::files::file_info));

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(RequestIdLayer::new())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)); // 10MB default

    Router::new()
        .nest("/api", api_routes)
        .fallback(handlers::static_files::serve_static)
        .layer(middleware_stack)
        .with_state(state)
}