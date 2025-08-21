use crate::config::{parse_duration, DatabaseConfig};
use crate::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing::{info, instrument};

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    #[instrument(skip(config))]
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let connection_timeout = parse_duration(&config.connection_timeout)?;
        let idle_timeout = parse_duration(&config.idle_timeout)?;
        let max_lifetime = parse_duration(&config.max_lifetime)?;

        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(connection_timeout)
            .idle_timeout(idle_timeout)
            .max_lifetime(max_lifetime)
            .connect(&config.url)
            .await?;

        info!(
            max_connections = config.max_connections,
            min_connections = config.min_connections,
            "Database connection pool established"
        );

        if config.auto_migrate {
            info!("Running database migrations");
            Self::migrate(&pool).await?;
        }

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    #[instrument(skip(pool))]
    pub async fn migrate(pool: &PgPool) -> Result<()> {
        sqlx::migrate!("./migrations").run(pool).await?;
        info!("Database migrations completed successfully");
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn close(&self) {
        self.pool.close().await;
        info!("Database connection pool closed");
    }
}