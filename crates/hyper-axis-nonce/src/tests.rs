
use std::{
    collections::HashSet,
    sync::{Arc, Barrier},
    thread,
};

use super::{
    MAX_FUTURE_OFFSET_MS, MAX_PAST_AGE_MS, NonceError, NonceManager, validate_nonce_window,
};

#[test]
fn uses_clock_then_increments_within_the_same_millisecond() {
    let manager = NonceManager::default();
    assert_eq!(manager.next(1_000).unwrap(), 1_000);
    assert_eq!(manager.next(1_000).unwrap(), 1_001);
    assert_eq!(manager.next(999).unwrap(), 1_002);
    assert_eq!(manager.last_issued(), Some(1_002));
}

#[test]
fn fast_forward_never_moves_backwards() {
    let manager = NonceManager::new(Some(10));
    manager.fast_forward(20);
    manager.fast_forward(15);
    assert_eq!(manager.next(1).unwrap(), 21);
}

#[test]
fn issues_unique_values_under_contention() {
    const WORKERS: usize = 16;
    let manager = Arc::new(NonceManager::default());
    let barrier = Arc::new(Barrier::new(WORKERS));
    let handles: Vec<_> = (0..WORKERS)
        .map(|_| {
            let manager = Arc::clone(&manager);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                manager.next(1_000).unwrap()
            })
        })
        .collect();
    let values: HashSet<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();
    assert_eq!(values.len(), WORKERS);
    assert_eq!(values.iter().min(), Some(&1_000));
    assert_eq!(values.iter().max(), Some(&1_015));
}

#[test]
fn validates_documented_time_window_inclusively() {
    let now = 1_000_000_000;
    assert_eq!(validate_nonce_window(now - MAX_PAST_AGE_MS, now), Ok(()));
    assert_eq!(
        validate_nonce_window(now - MAX_PAST_AGE_MS - 1, now),
        Err(NonceError::TooOld)
    );
    assert_eq!(
        validate_nonce_window(now + MAX_FUTURE_OFFSET_MS, now),
        Ok(())
    );
    assert_eq!(
        validate_nonce_window(now + MAX_FUTURE_OFFSET_MS + 1, now),
        Err(NonceError::TooFarInFuture)
    );
}

#[test]
fn reports_exhaustion_without_wrapping() {
    let manager = NonceManager::new(Some(u64::MAX));
    assert_eq!(manager.next(u64::MAX), Err(NonceError::Exhausted));
    assert_eq!(manager.last_issued(), Some(u64::MAX));
}
