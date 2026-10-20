/// Applies the runtime logging switch, level, and memory limit.
///
/// Lowering capacity immediately evicts the oldest records and increments the
/// dropped counter. Disabling logging stops new capture but preserves existing
/// records until [`drain`] or [`clear`] is called.
///
/// # Errors
///
/// Returns [`LoggingError::InvalidCapacity`] when capacity is zero or exceeds
/// [`MAX_CAPACITY`].
pub fn configure(
    enabled: bool,
    level: LogLevel,
    capacity: usize,
) -> Result<LoggingConfig, LoggingError> {
    if !(1..=MAX_CAPACITY).contains(&capacity) {
        return Err(LoggingError::InvalidCapacity(capacity));
    }

    ENABLED.store(false, Ordering::Release);
    LEVEL.store(level as u8, Ordering::Relaxed);
    CAPACITY.store(capacity, Ordering::Relaxed);
    trim_to_capacity(capacity);
    ENABLED.store(enabled, Ordering::Release);
    Ok(LoggingConfig {
        enabled,
        level,
        capacity,
    })
}

/// Returns the currently applied configuration.
#[must_use]
pub fn current_config() -> LoggingConfig {
    LoggingConfig {
        enabled: ENABLED.load(Ordering::Acquire),
        level: current_level(),
        capacity: CAPACITY.load(Ordering::Relaxed),
    }
}

/// Captures a sanitized structured record when the runtime filter allows it.
///
/// Target and message lengths are bounded before insertion. Callers must never
/// include private keys, seed phrases, signatures, authentication tokens, full
/// addresses, nonces, or complete transaction payloads.
pub fn emit(level: LogLevel, target: impl AsRef<str>, message: impl AsRef<str>) {
    if !is_enabled(level) {
        return;
    }

    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed).to_string();
    let record = LogRecord {
        sequence,
        timestamp_ms: unix_timestamp_ms(),
        level,
        target: bounded(target.as_ref(), MAX_TARGET_BYTES),
        message: bounded(message.as_ref(), MAX_MESSAGE_BYTES),
    };
    let capacity = CAPACITY.load(Ordering::Relaxed);
    let mut records = records();
    while records.len() >= capacity {
        records.pop_front();
        DROPPED.fetch_add(1, Ordering::Relaxed);
    }
    records.push_back(record);
}

/// Returns a copy of all retained records without removing them.
#[must_use]
pub fn snapshot() -> LogBatch {
    LogBatch {
        dropped: DROPPED.load(Ordering::Relaxed).to_string(),
        records: records().iter().cloned().collect(),
    }
}

/// Removes and returns every retained record.
#[must_use]
pub fn drain() -> LogBatch {
    let mut records = records();
    LogBatch {
        dropped: DROPPED.swap(0, Ordering::Relaxed).to_string(),
        records: records.drain(..).collect(),
    }
}

/// Removes all records and resets the dropped-record counter.
pub fn clear() {
    records().clear();
    DROPPED.store(0, Ordering::Relaxed);
}

fn is_enabled(level: LogLevel) -> bool {
    ENABLED.load(Ordering::Acquire) && level as u8 <= LEVEL.load(Ordering::Relaxed)
}

fn current_level() -> LogLevel {
    match LEVEL.load(Ordering::Relaxed) {
        1 => LogLevel::Error,
        2 => LogLevel::Warn,
        4 => LogLevel::Debug,
        5 => LogLevel::Trace,
        _ => LogLevel::Info,
    }
}

fn trim_to_capacity(capacity: usize) {
    let mut records = records();
    while records.len() > capacity {
        records.pop_front();
        DROPPED.fetch_add(1, Ordering::Relaxed);
    }
}

fn records() -> MutexGuard<'static, VecDeque<LogRecord>> {
    RECORDS
        .get_or_init(|| Mutex::new(VecDeque::with_capacity(DEFAULT_CAPACITY)))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn bounded(value: &str, max_bytes: usize) -> String {
    if value.len() <= max_bytes {
        return value.to_owned();
    }
    let mut boundary = max_bytes.saturating_sub('…'.len_utf8());
    while !value.is_char_boundary(boundary) {
        boundary -= 1;
    }
    let mut output = value[..boundary].to_owned();
    output.push('…');
    output
}

#[cfg(not(target_arch = "wasm32"))]
fn unix_timestamp_ms() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now().duration_since(UNIX_EPOCH).map_or_else(
        |_| "0".to_owned(),
        |duration| duration.as_millis().to_string(),
    )
}

#[cfg(target_arch = "wasm32")]
fn unix_timestamp_ms() -> String {
    format!("{:.0}", js_sys::Date::now())
}

#[cfg(test)]
mod tests;
