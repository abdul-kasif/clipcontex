use std::{
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use tracing::{info, warn};

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

    pub fn start<F>(&mut self, mut on_event: F) -> ClipboardWatcherHandle
    where
        F: FnMut(ClipboardEvent) + Send + 'static,
    {
        let is_running = Arc::clone(&self.is_running);
        is_running.store(true, Ordering::Relaxed);

        let deduplicator = self.deduplicator.clone();
        let thread_is_running = Arc::clone(&is_running);

        let handle = thread::spawn(move || {
            let mut last_content = String::new();
            let mut last_capture: Option<Instant> = None;

            info!("📋 Clipboard watcher thread started...");

            while thread_is_running.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(250));

                match get_clipboard_text() {
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
                    Err(e) => {
                        warn!("Clipboard read error: {}", e);
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }

            info!("🛑 Clipboard watcher stopped.");
        });

        ClipboardWatcherHandle {
            handle: Some(handle),
            is_running,
        }
    }
}

fn get_clipboard_text() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    {
        let is_wayland = std::env::var("WAYLAND_DISPLAY").is_ok();

        if is_wayland {
            if let Ok(output) = Command::new("wl-paste").arg("--no-newline").output() {
                if output.status.success() {
                    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !text.is_empty() {
                        return Ok(text);
                    }
                }
            }
        }

        // Fallback to arboard
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => clipboard.get_text().map_err(|e| e.to_string()),
            Err(e) => Err(format!("Clipboard init failed: {}", e)),
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
        clipboard.get_text().map_err(|e| e.to_string())
    }
}

pub struct ClipboardWatcherHandle {
    handle: Option<thread::JoinHandle<()>>,
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
