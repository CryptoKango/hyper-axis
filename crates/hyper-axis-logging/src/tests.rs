
use super::{
    LogLevel, MAX_CAPACITY, MAX_MESSAGE_BYTES, MAX_TARGET_BYTES, clear, configure, drain, emit,
    snapshot,
};

#[test]
fn supports_switch_filter_capacity_and_drain() {
    clear();
    configure(false, LogLevel::Trace, 2).unwrap();
    emit(LogLevel::Error, "test", "disabled");
    assert_eq!(snapshot().records.len(), 0);

    configure(true, LogLevel::Info, 2).unwrap();
    emit(LogLevel::Debug, "test", "filtered");
    emit(LogLevel::Info, "test", "one");
    emit(LogLevel::Warn, "test", "two");
    emit(LogLevel::Error, "test", "three");

    let batch = drain();
    assert_eq!(batch.dropped, "1");
    assert_eq!(batch.records.len(), 2);
    assert_eq!(batch.records[0].message, "two");
    assert_eq!(batch.records[1].message, "three");
    assert_eq!(drain().records.len(), 0);

    emit(LogLevel::Info, "目".repeat(100), "值".repeat(2_000));
    let bounded = snapshot();
    assert!(bounded.records[0].target.len() <= MAX_TARGET_BYTES);
    assert!(bounded.records[0].message.len() <= MAX_MESSAGE_BYTES);

    assert!(configure(true, LogLevel::Info, 0).is_err());
    assert!(configure(true, LogLevel::Info, MAX_CAPACITY + 1).is_err());

    configure(false, LogLevel::Info, 2).unwrap();
}
