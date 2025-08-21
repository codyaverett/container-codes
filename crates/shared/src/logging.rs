use crate::config::LoggingConfig;
use crate::Result;
use tracing::{info, Level};
use tracing_subscriber::{
    fmt::{self, format::Format, time::UtcTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Registry,
};

pub fn init_logging(config: &LoggingConfig) -> Result<()> {
    let level = parse_log_level(&config.level)?;
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("container_codes={}", level)));

    let registry = Registry::default().with(env_filter);

    match config.format.as_str() {
        "json" => {
            let fmt_layer = fmt::layer()
                .json()
                .with_timestamp(UtcTime::rfc_3339())
                .with_current_span(true)
                .with_span_list(true);

            registry.with(fmt_layer).init();
        }
        "pretty" => {
            let fmt_layer = fmt::layer()
                .pretty()
                .with_timestamp(UtcTime::rfc_3339())
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true);

            registry.with(fmt_layer).init();
        }
        "compact" => {
            let fmt_layer = fmt::layer()
                .compact()
                .with_timestamp(UtcTime::rfc_3339())
                .with_target(false);

            registry.with(fmt_layer).init();
        }
        _ => {
            return Err(crate::Error::config_invalid(
                "logging.format",
                &config.format,
            ));
        }
    }

    info!(
        "Logging initialized with level={} format={}",
        config.level, config.format
    );

    Ok(())
}

fn parse_log_level(level: &str) -> Result<Level> {
    match level.to_lowercase().as_str() {
        "trace" => Ok(Level::TRACE),
        "debug" => Ok(Level::DEBUG),
        "info" => Ok(Level::INFO),
        "warn" => Ok(Level::WARN),
        "error" => Ok(Level::ERROR),
        _ => Err(crate::Error::config_invalid("logging.level", level)),
    }
}

#[macro_export]
macro_rules! log_error {
    ($err:expr) => {
        tracing::error!(error = %$err, "Error occurred");
    };
    ($err:expr, $msg:expr) => {
        tracing::error!(error = %$err, message = $msg, "Error occurred");
    };
    ($err:expr, $msg:expr, $($field:tt)*) => {
        tracing::error!(error = %$err, message = $msg, $($field)*, "Error occurred");
    };
}

#[macro_export]
macro_rules! log_request {
    ($method:expr, $path:expr, $status:expr, $duration:expr) => {
        tracing::info!(
            method = $method,
            path = $path,
            status = $status,
            duration_ms = $duration,
            "HTTP request completed"
        );
    };
}

#[macro_export]
macro_rules! log_container_event {
    ($container_id:expr, $event:expr) => {
        tracing::info!(
            container_id = $container_id,
            event = $event,
            "Container event"
        );
    };
    ($container_id:expr, $event:expr, $($field:tt)*) => {
        tracing::info!(
            container_id = $container_id,
            event = $event,
            $($field)*,
            "Container event"
        );
    };
}

#[macro_export]
macro_rules! log_job_event {
    ($job_id:expr, $event:expr) => {
        tracing::info!(
            job_id = $job_id,
            event = $event,
            "Job event"
        );
    };
    ($job_id:expr, $event:expr, $($field:tt)*) => {
        tracing::info!(
            job_id = $job_id,
            event = $event,
            $($field)*,
            "Job event"
        );
    };
}