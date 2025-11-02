use std::string::String;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant, SystemTime},
};
use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tracing::{info, warn};

use super::dedupe::Deduplicator;

// Ignore window to prevent self-trigger duplication
static IGNORE_UNTIL: Mutex<Option<SystemTime>> = Mutex::new(None);

/// Ignore clipboard updates for a short window (default 500ms)
pub fn mark_ignore_next_clipboard_update() {
    let mut lock = IGNORE_UNTIL.lock().unwrap();
    *lock = Some(SystemTime::now());
}

/// Returns true if we are currently within the ignore window.
pub fn should_ignore_clipboard_update() -> bool {
    let now = SystemTime::now();
    let mut lock = IGNORE_UNTIL.lock().unwrap();

    if let Some(ignore_until) = *lock {
        if now.duration_since(ignore_until).unwrap_or_default() < Duration::from_millis(500) {
            return true;
        }
    }

    // Clear after window expires
    *lock = None;
    false
}

// Clipboard event and watcher
#[derive(Debug, Clone, PartialEq)]
pub struct ClipboardEvent {
    pub content: String,
    pub captured_at: Instant,
}

pub struct ClipboardWatcher {
    is_running: Arc<AtomicBool>,
    deduplicator: Deduplicator,
}

impl ClipboardWatcher {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            deduplicator: Deduplicator::new(Duration::from_secs(10)),
        }
    }

    pub fn start<F>(&mut self, app: AppHandle, mut on_event: F) -> ClipboardWatcherHandle
    where
        F: FnMut(ClipboardEvent) + Send + 'static,
    {
        let is_running = Arc::clone(&self.is_running);
        is_running.store(true, Ordering::Relaxed);

        let deduplicator = self.deduplicator.clone();
        let thread_is_running = Arc::clone(&is_running);
        let app_handle = app.clone();

        let handle = thread::spawn(move || {
            // Initialize last_content to prevent startup duplication
            let mut last_content = match get_clipboard_text(&app_handle) {
                Ok(initial) => {
                    info!("Watcher initialized with existing clipboard content, skipping first capture.");
                    initial
                }
                Err(_) => String::new(),
            };
            let mut last_capture: Option<Instant> = None;

            info!("Clipboard watcher thread started...");

            while thread_is_running.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(300));

                match get_clipboard_text(&app_handle) {
                    Ok(content) => {
                        // Ignore empty or identical clipboard text
                        if content.is_empty() || content == last_content {
                            continue;
                        }

                        // If this update was triggered by our own app (quick picker)
                        if should_ignore_clipboard_update() {
                            warn!("Ignoring clipboard update triggered by the app itself.");
                            last_content = content;
                            last_capture = Some(Instant::now());
                            continue;
                        }

                        let now = Instant::now();
                        let should_trigger = match last_capture {
                            Some(ts) => now.duration_since(ts) >= Duration::from_millis(300),
                            None => true,
                        };

                        // Check deduplication window
                        if should_trigger && deduplicator.should_save(&content) {
                            on_event(ClipboardEvent {
                                content: content.clone(),
                                captured_at: now,
                            });
                            last_capture = Some(now);
                            last_content = content;
                        }
                    }
                    Err(_e) => {
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }

            info!("Clipboard watcher stopped.");
        });

        ClipboardWatcherHandle {
            handle: Some(handle),
            is_running,
        }
    }
}

/// Uses Tauri clipboard plugin to read clipboard text
fn get_clipboard_text(app: &AppHandle) -> Result<String, String> {
    let text = app
        .clipboard()
        .read_text()
        .map_err(|e| format!("Clipboard read failed: {}", e))?;

    let trimmed = text.trim().to_string();
    if trimmed.is_empty() {
        Err("Clipboard empty".to_string())
    } else {
        Ok(trimmed)
    }
}

/// Handle for managing the watcher thread
pub struct ClipboardWatcherHandle {
    handle: Option<std::thread::JoinHandle<()>>,
    is_running: Arc<AtomicBool>,
}

impl ClipboardWatcherHandle {
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for ClipboardWatcherHandle {
    fn drop(&mut self) {
        self.stop();
    }
}
