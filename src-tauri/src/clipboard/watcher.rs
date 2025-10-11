use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use arboard::Clipboard;

use super::dedupe::Deduplicator;

/// Event emitted when a new unique clipboard text is captured.
#[derive(Debug, Clone, PartialEq)]
pub struct ClipboardEvent {
    pub content: String,
    pub captured_at: Instant,
}

/// Watch the system clipboard for changes with debounce and deduplication
pub struct ClipboardWatcher {
    is_running: Arc<AtomicBool>,
    deduplicator: Deduplicator,
}

impl ClipboardWatcher {
    /// Create a new clipboard watcher with default debounce (300ms) and dedupe(10s)
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            deduplicator: Deduplicator::new(Duration::from_secs(10)),
        }
    }

    /// Starts watching clipboard in the background thread and
    /// Returns a handle to stop watcher
    pub fn start<F>(&mut self, mut on_event: F) -> ClipboardWatcherHandle
    where
        F: FnMut(ClipboardEvent) + Send + 'static,
    {
        let is_running = Arc::clone(&self.is_running);
        is_running.store(true, Ordering::Relaxed);

        let thread_is_running = Arc::clone(&is_running);
        let deduplicator = self.deduplicator.clone();
        let handle = thread::spawn(move || {
            let mut last_content = String::new();
            let mut last_capture: Option<Instant> = None;
            let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");

            while thread_is_running.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(100)); // Poll every 100ms

                match clipboard.get_text() {
                    Ok(content) => {
                        if content.is_empty() || content == last_content {
                            continue;
                        }

                        // Debounce: wait 300ms after content change
                        if last_capture.is_none()
                            || last_capture.unwrap().elapsed() >= Duration::from_millis(300)
                        {
                            if deduplicator.should_save(&content) {
                                let event = ClipboardEvent {
                                    content: content.clone(),
                                    captured_at: Instant::now(),
                                };
                                on_event(event);
                                last_capture = Some(Instant::now());
                            }
                            last_content = content;
                        } else {
                            // content changed to quickly - reset debounce window
                            last_content = content;
                            last_capture = Some(Instant::now());
                        }
                    }
                    Err(e) => {
                        eprintln!("Clipboard read error: {}", e);
                        thread::sleep(Duration::from_secs(1)); // Back off on error
                    }
                }
            }
        });

        ClipboardWatcherHandle {
            handle: Some(handle),
            is_running,
        }
    }
}

// Handle to stop the clipboard watcher
pub struct ClipboardWatcherHandle {
    handle: Option<thread::JoinHandle<()>>,
    is_running: Arc<AtomicBool>,
}

impl ClipboardWatcherHandle {
    /// Stop the watcher and wait for the thread to finish
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for ClipboardWatcherHandle {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
