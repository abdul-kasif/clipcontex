// src-tauri/src/clipboard/watcher.rs
use std::{
    string::String,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::{Duration, Instant, SystemTime},
};
use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tracing::{debug, error, info, warn};

use super::dedupe::Deduplicator;

// Ignore window to prevent self-trigger duplication
static IGNORE_UNTIL: Mutex<Option<SystemTime>> = Mutex::new(None);

/// Ignore clipboard updates for a short window (default 500ms)
pub fn mark_ignore_next_clipboard_update() {
    let mut lock = IGNORE_UNTIL.lock().unwrap();
    *lock = Some(SystemTime::now() + Duration::from_millis(500)); // Increased duration
}

/// Returns true if we are currently within the ignore window.
pub fn should_ignore_clipboard_update() -> bool {
    let now = SystemTime::now();
    let mut lock = IGNORE_UNTIL.lock().unwrap();

    if let Some(ignore_until) = *lock {
        if now < ignore_until {
            return true;
        }
    }

    // Clear after window expires
    *lock = None;
    false
}

// Clipboard Event + Watcher
#[derive(Debug, Clone, PartialEq)]
pub struct ClipboardEvent {
    pub content: String,
    pub captured_at: Instant,
}

pub struct ClipboardWatcher {
    is_running: Arc<AtomicBool>,
    deduplicator: Deduplicator,
    signal: Arc<(Mutex<bool>, Condvar)>,
}

impl Default for ClipboardWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardWatcher {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            deduplicator: Deduplicator::new(Duration::from_secs(10)),
            signal: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    /// Starts the watcher in a blocking loop (single thread)
    pub fn start<F>(&mut self, app: AppHandle, mut on_event: F) -> ClipboardWatcherHandle
    where
        F: FnMut(ClipboardEvent) + Send + 'static,
    {
        let is_running = Arc::clone(&self.is_running);
        is_running.store(true, Ordering::Relaxed);

        let signal = Arc::clone(&self.signal);
        let deduplicator = self.deduplicator.clone();
        let app_handle = app.clone();

        // Clone for thread use
        let thread_is_running = Arc::clone(&is_running);

        let handle = thread::spawn(move || {
            let mut last_content = match get_clipboard_text(&app_handle) {
                Ok(initial) => {
                    info!("Watcher initialized with existing clipboard content, skipping first capture.");
                    initial
                }
                Err(_) => String::new(),
            };

            let mut last_capture: Option<Instant> = None;
            let mut sleep_duration = Duration::from_millis(300);
            let mut error_backoff = Duration::from_millis(200);

            info!("Clipboard watcher thread started.");

            let (lock, cvar) = &*signal;
            let mut stop_requested = lock.lock().unwrap();

            // Use cloned reference here
            while thread_is_running.load(Ordering::Relaxed) && !*stop_requested {
                let result = cvar.wait_timeout(stop_requested, sleep_duration).unwrap();
                stop_requested = result.0;

                if *stop_requested {
                    break;
                }

                let content = match get_clipboard_text(&app_handle) {
                    Ok(c) => c,
                    Err(e) => {
                        debug!("Clipboard read failed ({}). Retrying...", e);
                        thread::sleep(error_backoff);
                        error_backoff = (error_backoff * 2).min(Duration::from_secs(2));
                        continue;
                    }
                };

                error_backoff = Duration::from_millis(200);

                if content.is_empty() || content == last_content {
                    sleep_duration = (sleep_duration + Duration::from_millis(100))
                        .min(Duration::from_millis(1500));
                    continue;
                }

                sleep_duration = Duration::from_millis(250);

                if should_ignore_clipboard_update() {
                    warn!("Ignored clipboard update triggered by app itself.");
                    last_content = content;
                    last_capture = Some(Instant::now());
                    continue;
                }

                let now = Instant::now();
                let should_trigger = match last_capture {
                    Some(ts) => now.duration_since(ts) >= Duration::from_millis(300),
                    None => true,
                };

                if should_trigger && deduplicator.should_save(&content) {
                    on_event(ClipboardEvent {
                        content: content.clone(),
                        captured_at: now,
                    });

                    last_content = content;
                    last_capture = Some(now);
                }
            }

            info!("Clipboard watcher stopped cleanly.");
        });

        // return original Arc
        ClipboardWatcherHandle {
            handle: Some(handle),
            is_running,
            signal: Arc::clone(&self.signal),
        }
    }
}

/// Reads clipboard text via Tauri clipboard plugin
fn get_clipboard_text(app: &AppHandle) -> Result<String, String> {
    let text = app
        .clipboard()
        .read_text()
        .map_err(|e| format!("Clipboard read failed: {}", e))?;

    let trimmed = text.trim();
    if trimmed.is_empty() {
        Err("Clipboard empty".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

// Handle for managing watcher lifecycle
pub struct ClipboardWatcherHandle {
    handle: Option<std::thread::JoinHandle<()>>,
    is_running: Arc<AtomicBool>,
    signal: Arc<(Mutex<bool>, Condvar)>,
}

impl ClipboardWatcherHandle {
    /// Immediately stop the watcher and wake up the thread if it's sleeping
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        {
            let (lock, cvar) = &*self.signal;
            let mut stop_flag = lock.lock().unwrap();
            *stop_flag = true;
            cvar.notify_all(); // wake up sleeping thread
        }

        if let Some(handle) = self.handle.take() {
            if let Err(e) = handle.join() {
                error!("Failed to join watcher thread: {:?}", e);
            }
        }
    }
}

impl Drop for ClipboardWatcherHandle {
    fn drop(&mut self) {
        self.stop();
    }
}

