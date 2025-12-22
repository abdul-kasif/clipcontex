//src-tauri/src/clipboard/dedupe.rs
// ===== Imports =====
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

// ===== Domain Type =====
#[derive(Clone)]
pub struct Deduplicator {
    window: Duration,
    last_seen: Arc<Mutex<HashMap<String, Instant>>>,
}

// ===== Deduplicator Implementation =====
impl Deduplicator {
    /// Creates a new deduplicator in a given time window
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            last_seen: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Returns `true` if the content should be saved (not a duplicate in the window)
    pub fn should_save(&self, content: &str) -> bool {
        let mut last_seen = self.last_seen.lock().unwrap();
        let now = Instant::now();

        if let Some(last_time) = last_seen.get(content) {
            if now.duration_since(*last_time) < self.window {
                return false;
            }
        }

        last_seen.insert(content.to_string(), now);
        true
    }
}
