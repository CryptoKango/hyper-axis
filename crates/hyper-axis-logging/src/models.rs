/// Severity assigned to a structured diagnostic record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum LogLevel {
    /// Operation cannot continue.
    Error = 1,
    /// Recoverable or suspicious condition.
    Warn = 2,
    /// High-level lifecycle event.
    Info = 3,
    /// Development diagnostic.
    Debug = 4,
    /// Fine-grained protocol trace.
    Trace = 5,
}

impl FromStr for LogLevel {
    type Err = LoggingError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "error" => Ok(Self::Error),
            "warn" | "warning" => Ok(Self::Warn),
            "info" => Ok(Self::Info),
            "debug" => Ok(Self::Debug),
            "trace" => Ok(Self::Trace),
            _ => Err(LoggingError::InvalidLevel(value.to_owned())),
        }
    }
}

/// Applied logging configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingConfig {
    /// Whether new records are captured.
    pub enabled: bool,
    /// Most verbose captured severity.
    pub level: LogLevel,
    /// Maximum retained record count.
    pub capacity: usize,
}

/// One sanitized diagnostic event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogRecord {
    /// Monotonically increasing process-local sequence, encoded as a string for
    /// JavaScript integer safety.
    pub sequence: String,
    /// Unix epoch milliseconds, encoded as a string for cross-language safety.
    pub timestamp_ms: String,
    /// Event severity.
    pub level: LogLevel,
    /// Stable subsystem identifier.
    pub target: String,
    /// Sanitized diagnostic text.
    pub message: String,
}

/// Batch returned to a host logger or support-bundle exporter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogBatch {
    /// Number of old records discarded since the previous drain or clear.
    pub dropped: String,
    /// Records in production order.
    pub records: Vec<LogRecord>,
}

/// Logging configuration failures.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LoggingError {
    /// Unsupported textual level.
    #[error("invalid log level `{0}`; expected error, warn, info, debug, or trace")]
    InvalidLevel(String),
    /// Capacity is outside the protected range.
    #[error("log capacity must be between 1 and {MAX_CAPACITY}, received {0}")]
    InvalidCapacity(usize),
}
