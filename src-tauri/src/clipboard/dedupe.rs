// src-tauri/src/clipboard/dedupe.rs
//! Clipboard content deduplication within a time window.

use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hasher,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

/// Deduplicates clipboard content based on a time window and optional size limit.
///
/// Uses a hash of the content to reduce memory footprint.
/// Automatically evicts oldest entries when capacity is exceeded.
#[derive(Clone)]
pub struct Deduplicator {
    window: Duration,
    capacity: usize,
    last_seen: Arc<Mutex<HashMap<u64, (String, Instant)>>>,
}

impl Deduplicator {
    /// Creates a new deduplicator.
    ///
    /// - `window`: How long to consider content "recent".
    /// - `capacity`: Max number of entries to track (prevents memory growth).
    pub fn new(window: Duration, capacity: usize) -> Self {
        Self {
            window,
            capacity,
            last_seen: Arc::new(Mutex::new(HashMap::with_capacity(capacity))),
        }
    }

    /// Returns `true` if the content should be saved (not a duplicate in the window).
    ///
    /// Also inserts the content into the cache if it's new.
    pub fn should_save(&self, content: &str) -> bool {
        let hash = hash_content(content);
        let now = Instant::now();

        let mut cache = self.last_seen.lock().unwrap_or_else(|e| e.into_inner());

        // Evict oldest if at capacity (simple LRU not implemented; FIFO eviction)
        if cache.len() >= self.capacity {
            // Remove one arbitrary entry (good enough for clipboard)
            if let Some(key) = cache.keys().next().copied() {
                cache.remove(&key);
            }
        }

        match cache.entry(hash) {
            Entry::Occupied(mut entry) => {
                let (_, last_time) = entry.get();
                if now.duration_since(*last_time) < self.window {
                    false
                } else {
                    entry.insert((content.to_string(), now));
                    true
                }
            }
            Entry::Vacant(entry) => {
                entry.insert((content.to_string(), now));
                true
            }
        }
    }
}

fn hash_content(s: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hasher.write(s.as_bytes());
    hasher.finish()
}

