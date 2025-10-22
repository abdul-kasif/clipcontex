use std::string::String;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};
use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tracing::info;

use super::dedupe::Deduplicator;

/// Event emitted when a new unique clipboard text is captured.
#[derive(Debug, Clone, PartialEq)]
pub struct ClipboardEvent {
    pub content: String,
    pub captured_at: Instant,
}

/// Watches the system clipboard for changes.
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
            let mut last_content = String::new();
            let mut last_capture: Option<Instant> = None;

            info!("Clipboard watcher thread started...");

            while thread_is_running.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(250));

                match get_clipboard_text(&app_handle) {
                    Ok(content) => {
                        if content.is_empty() || content == last_content {
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

/// Uses Tauri clipboard plugin
fn get_clipboard_text(app: &AppHandle) -> Result<String, String> {
    // read_text() -> Result<String, Error>
    let text = app
        .clipboard()
        .read_text()
        .map_err(|e| format!("Clipboard read failed: {}", e))?; // yields String
    let trimmed = text.trim().to_string();
    if trimmed.is_empty() {
        Err("Clipboard empty".to_string())
    } else {
        Ok(trimmed)
    }
}
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
