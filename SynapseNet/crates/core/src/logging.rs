use tracing::{Level, Metadata};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Log level
    pub level: LogLevel,
    /// Enable JSON formatting
    pub json: bool,
    /// Enable file logging
    pub file_logging: bool,
    /// Log file path
    pub log_file: Option<String>,
    /// Enable performance spans
    pub performance_spans: bool,
}

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            json: false,
            file_logging: false,
            log_file: None,
            performance_spans: false,
        }
    }
}

impl LogConfig {
    /// Create development logging config
    pub fn development() -> Self {
        Self {
            level: LogLevel::Debug,
            json: false,
            file_logging: true,
            log_file: Some("synapsenet-dev.log".to_string()),
            performance_spans: true,
        }
    }

    /// Create production logging config
    pub fn production() -> Self {
        Self {
            level: LogLevel::Info,
            json: true,
            file_logging: true,
            log_file: Some("synapsenet.log".to_string()),
            performance_spans: false,
        }
    }
}

/// Initialize logging with configuration
pub fn init_logging(config: &LogConfig) -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let level: Level = config.level.into();
        EnvFilter::new(format!("synapsenet={},warn", level))
    });

    let fmt_layer = if config.json {
        // JSON formatting for production
        fmt::layer()
            .json()
            .with_span_events(if config.performance_spans {
                FmtSpan::CLOSE
            } else {
                FmtSpan::NONE
            })
            .boxed()
    } else {
        // Pretty formatting for development
        fmt::layer()
            .pretty()
            .with_span_events(if config.performance_spans {
                FmtSpan::CLOSE
            } else {
                FmtSpan::NONE
            })
            .boxed()
    };

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    // Add file logging if enabled
    if config.file_logging {
        if let Some(log_file) = &config.log_file {
            let file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file)?;

            let file_layer = fmt::layer()
                .with_writer(std::sync::Arc::new(file))
                .with_ansi(false)
                .boxed();

            registry.with(file_layer).init();
        } else {
            registry.init();
        }
    } else {
        registry.init();
    }

    Ok(())
}

/// Performance metrics logger
pub struct PerformanceLogger {
    operation: String,
    start: std::time::Instant,
}

impl PerformanceLogger {
    /// Start performance logging
    pub fn start(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            start: std::time::Instant::now(),
        }
    }

    /// Log completion with duration
    pub fn complete(self) {
        let duration = self.start.elapsed();
        tracing::info!(
            operation = %self.operation,
            duration_ms = duration.as_millis(),
            "Operation completed"
        );
    }

    /// Log completion with custom message
    pub fn complete_with(self, message: &str) {
        let duration = self.start.elapsed();
        tracing::info!(
            operation = %self.operation,
            duration_ms = duration.as_millis(),
            message = message,
            "Operation completed"
        );
    }
}

/// Structured logging macros
#[macro_export]
macro_rules! log_operation {
    ($op:expr, $($key:ident = $value:expr),*) => {
        tracing::info!(
            operation = $op,
            $($key = $value),*
        );
    };
}

#[macro_export]
macro_rules! log_error {
    ($op:expr, $err:expr, $($key:ident = $value:expr),*) => {
        tracing::error!(
            operation = $op,
            error = %$err,
            $($key = $value),*
        );
    };
}

/// Log rotation configuration
#[derive(Debug, Clone)]
pub struct LogRotationConfig {
    /// Maximum log file size in MB
    pub max_size_mb: u64,
    /// Maximum number of rotated files to keep
    pub max_files: usize,
    /// Rotation check interval
    pub check_interval: std::time::Duration,
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self {
            max_size_mb: 100,
            max_files: 5,
            check_interval: std::time::Duration::from_secs(3600), // 1 hour
        }
    }
}

/// Simple log rotation manager
pub struct LogRotationManager {
    config: LogRotationConfig,
    log_file: String,
}

impl LogRotationManager {
    /// Create new log rotation manager
    pub fn new(log_file: String, config: LogRotationConfig) -> Self {
        Self { config, log_file }
    }

    /// Check and rotate logs if needed
    pub fn check_and_rotate(&self) -> anyhow::Result<()> {
        let metadata = std::fs::metadata(&self.log_file)?;
        let size_mb = metadata.len() / (1024 * 1024);

        if size_mb >= self.config.max_size_mb {
            self.rotate()?;
        }

        Ok(())
    }

    /// Rotate log files
    fn rotate(&self) -> anyhow::Result<()> {
        // Rotate existing files
        for i in (1..self.config.max_files).rev() {
            let old_file = format!("{}.{}", self.log_file, i);
            let new_file = format!("{}.{}", self.log_file, i + 1);

            if std::path::Path::new(&old_file).exists() {
                std::fs::rename(&old_file, &new_file)?;
            }
        }

        // Move current log to .1
        let rotated_file = format!("{}.1", self.log_file);
        std::fs::rename(&self.log_file, &rotated_file)?;

        // Delete oldest file if exceeds max_files
        let oldest_file = format!("{}.{}", self.log_file, self.config.max_files + 1);
        if std::path::Path::new(&oldest_file).exists() {
            std::fs::remove_file(&oldest_file)?;
        }

        tracing::info!("Log file rotated: {}", self.log_file);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_config() {
        let dev_config = LogConfig::development();
        assert_eq!(dev_config.level, LogLevel::Debug);
        assert!(dev_config.performance_spans);

        let prod_config = LogConfig::production();
        assert_eq!(prod_config.level, LogLevel::Info);
        assert!(prod_config.json);
    }

    #[test]
    fn test_performance_logger() {
        let logger = PerformanceLogger::start("test_operation");
        std::thread::sleep(std::time::Duration::from_millis(10));
        logger.complete();
    }

    #[test]
    fn test_log_rotation_config() {
        let config = LogRotationConfig::default();
        assert_eq!(config.max_size_mb, 100);
        assert_eq!(config.max_files, 5);
    }
}
