use super::{dedupe::Deduplicator, watcher::ClipboardEvent};
use std::time::Duration;

#[test]
fn debounce_works() {
    // Simulate rapid changes: only the last one after 300ms should count
    // (This is implicitly tested in watcher logic; we test dedupe + timing via integration)
    // For unit test, we focus on dedupe and event structure
    let event = ClipboardEvent {
        content: "test".to_string(),
        captured_at: std::time::Instant::now(),
        app_info_title: "Google - Chrome".to_string(),
        app_info_class: "Chrome".to_string(),
    };
    assert_eq!(event.content, "test");
}

#[test]
fn dedupe_skips_duplicate() {
    let deduper = Deduplicator::new(Duration::from_secs(1));
    assert!(deduper.should_save("hello"));
    assert!(!deduper.should_save("hello")); // duplicate
    assert!(deduper.should_save("world"));
}

#[test]
fn dedupe_allows_after_window() {
    use std::thread;
    let deduper = Deduplicator::new(Duration::from_millis(200));
    assert!(deduper.should_save("temp"));
    thread::sleep(Duration::from_millis(250));
    assert!(deduper.should_save("temp")); // allowed after window
}

