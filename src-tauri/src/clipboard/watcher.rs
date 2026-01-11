// src-tauri/src/clipboard/watcher.rs
//! Background clipboard monitoring with deduplication and self-trigger prevention.
//!
//! The [`ClipboardWatcher`] runs a dedicated thread that periodically polls the system clipboard,
//! filters out duplicates and self-induced changes, and emits [`ClipboardEvent`]s for new content.
//!
//! ## Key Features
//!
//! - **Deduplication**: Uses a time-windowed cache to avoid saving identical clips too frequently.
//! - **Ignore Window**: When the app writes to the clipboard (e.g., during paste), it can call
//!   [`mark_ignore_next_clipboard_update`] to suppress the resulting self-triggered event.
//! - **Error Resilience**: Recovers from transient clipboard access failures with exponential backoff.
//! - **Resource Efficiency**: Limits memory usage via bounded deduplication cache.
//!
//! ## Threading Model
//!
//! The watcher runs in a single background thread. It does **not** use async I/O because
//! the Tauri clipboard plugin is synchronous and no native clipboard change events are exposed.
//! Polling every ~300ms provides a good balance between responsiveness and CPU usage.

use super::dedupe::Deduplicator;
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
use tracing::{debug, error, info, warn};

/// Instructs the clipboard watcher to ignore the next update matching the given content.
///
/// This is used to prevent **self-triggering**: when ClipContex itself writes to the clipboard
/// (e.g., during a "paste" action), the subsequent clipboard change should not be recorded
/// as a new clip.
///
/// The ignore window lasts for **1.5 seconds** and only applies to the exact content provided.
pub fn mark_ignore_next_clipboard_update(content: String) {
    IgnoreWindow::global().mark(content);
}

// ===== Domain Types =====

/// Represents a captured clipboard event.
#[derive(Debug, Clone, PartialEq)]
pub struct ClipboardEvent {
    /// The trimmed text content of the clipboard.
    pub content: String,
    /// The monotonic timestamp when the content was captured.
    ///
    /// Use this for ordering and rate-limiting; do not convert to wall-clock time.
    pub captured_at: Instant,
}

/// A builder for starting a clipboard monitoring thread.
///
/// This type is consumed when [`start`](ClipboardWatcher::start) is called.
/// It holds no state beyond configuration defaults.
pub struct ClipboardWatcher {}

/// A handle to a running clipboard watcher thread.
///
/// Dropping this handle will automatically stop the thread.
/// You may also call [`stop`](ClipboardWatcherHandle::stop) explicitly.
pub struct ClipboardWatcherHandle {
    handle: Option<std::thread::JoinHandle<()>>,
    is_running: Arc<AtomicBool>,
}

impl ClipboardWatcher {
    /// Creates a new clipboard watcher with default configuration.
    ///
    /// Default settings:
    /// - Deduplication window: 10 seconds
    /// - Deduplication cache size: 1,000 entries
    /// - Poll interval: ~300ms (with adaptive sleep)
    pub fn new() -> Self {
        Self {}
    }

    /// Starts the clipboard watcher in a background thread.
    ///
    /// The provided callback `on_event` is invoked once per unique clipboard change.
    /// The callback must be `'static + Send` since it runs on a separate thread.
    ///
    /// # Panics
    ///
    /// Does not panic under normal conditions. Thread panics are logged but do not propagate.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri::AppHandle;
    /// use clipcontex_lib::clipboard::watcher::{ClipboardWatcher, ClipboardEvent};
    ///
    /// fn setup_clipboard_watcher(app_handle: AppHandle) {
    ///     let watcher = ClipboardWatcher::new();
    ///     let _handle = watcher.start(app_handle, |event| {
    ///         println!("Captured clipboard: {}", event.content);
    ///     });
    /// }
    /// // `handle` keeps the watcher alive; drop it to stop.
    /// ```
    ///
    pub fn start<F>(self, app: AppHandle, on_event: F) -> ClipboardWatcherHandle
    where
        F: Fn(ClipboardEvent) + Send + 'static,
    {
        let is_running = Arc::new(AtomicBool::new(true));
        let deduplicator = Deduplicator::new(Duration::from_secs(10), 1000);
        let ignore_window = IgnoreWindow::global();
        let app_handle = app.clone();

        let thread_is_running = Arc::clone(&is_running);

        let handle = thread::spawn(move || {
            let mut last_content = match read_clipboard_text(&app_handle) {
                Ok(initial) => {
                    info!("Watcher initialized with existing clipboard content.");
                    initial
                }
                Err(_) => String::new(),
            };

            let mut last_capture: Option<Instant> = None;
            let base_sleep = Duration::from_millis(300);
            let mut error_backoff = Duration::from_millis(200);

            info!("Clipboard watcher thread started.");

            while thread_is_running.load(Ordering::Relaxed) {
                thread::sleep(base_sleep);

                let content = match read_clipboard_text(&app_handle) {
                    Ok(c) => c,
                    Err(e) => {
                        debug!("Clipboard read failed: {}. Retrying...", e);
                        thread::sleep(error_backoff);
                        error_backoff = (error_backoff * 2).min(Duration::from_secs(2));
                        continue;
                    }
                };

                error_backoff = Duration::from_millis(200);

                if content.is_empty() || content == last_content {
                    continue;
                }

                if ignore_window.should_ignore(&content) {
                    warn!("Ignored self-triggered clipboard update: {}", &content);
                    last_content = content;
                    last_capture = Some(Instant::now());
                    continue;
                }

                let now = Instant::now();
                let min_interval = Duration::from_millis(300);
                let should_trigger = last_capture
                    .map(|ts| now.duration_since(ts) >= min_interval)
                    .unwrap_or(true);

                if should_trigger && deduplicator.should_save(&content) {
                    on_event(ClipboardEvent {
                        content: content.clone(),
                        captured_at: now,
                    });
                    last_content = content;
                    last_capture = Some(now);
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

impl Default for ClipboardWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardWatcherHandle {
    /// Stops the clipboard watcher thread gracefully.
    ///
    /// Waits for the thread to finish. If the thread panicked, the panic payload is logged.
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            if let Err(e) = handle.join() {
                error!("Watcher thread panicked: {:?}", e);
            }
        }
    }
}

impl Drop for ClipboardWatcherHandle {
    fn drop(&mut self) {
        self.stop();
    }
}

// ===== Safe Ignore Window =====

/// Tracks a short-lived ignore window to prevent self-triggers.
///
/// This is a singleton used globally across the application.
/// It is safe to use from any thread.
#[derive(Debug)]
struct IgnoreWindow {
    until: Mutex<Option<SystemTime>>,
    content: Mutex<Option<String>>,
}

impl IgnoreWindow {
    fn new() -> Self {
        Self {
            until: Mutex::new(None),
            content: Mutex::new(None),
        }
    }

    fn mark(&self, content: String) {
        let mut until = self.until.lock().unwrap_or_else(|e| e.into_inner());
        let mut ignored = self.content.lock().unwrap_or_else(|e| e.into_inner());
        *until = Some(SystemTime::now() + Duration::from_millis(1500));
        *ignored = Some(content);
    }

    fn should_ignore(&self, current_content: &str) -> bool {
        let now = SystemTime::now();
        let until = self.until.lock().unwrap_or_else(|e| e.into_inner());
        let ignored = self.content.lock().unwrap_or_else(|e| e.into_inner());

        if let Some(ignore_until) = *until {
            if now < ignore_until {
                if let Some(ref content) = *ignored {
                    if content == current_content {
                        return true;
                    }
                }
            }
        }

        // Cleanup expired state
        if until.is_some() && now >= until.unwrap() {
            *self.until.lock().unwrap() = None;
            *self.content.lock().unwrap() = None;
        }

        false
    }

    /// Returns the global singleton instance.
    fn global() -> &'static Self {
        static INSTANCE: std::sync::OnceLock<IgnoreWindow> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(IgnoreWindow::new)
    }
}

// ===== Clipboard Access =====

/// Reads and trims text from the system clipboard.
///
/// Returns an error if:
/// - The clipboard is inaccessible.
/// - The content is empty or whitespace-only.
fn read_clipboard_text(app: &AppHandle) -> Result<String, String> {
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
