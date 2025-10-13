use chrono::{Utc};
use tempfile::tempdir;
use rusqlite::Connection;

use crate::storage::{clip::Clip, clip_store::ClipStore};

fn make_clip(content: &str) -> Clip {
    Clip::new(
        content.to_string(),
        "App".to_string(),
        "Window".to_string(),
        vec!["#tag1".into()],
        vec!["#manual".into()],
        false,
    )
}

#[test]
fn test_init_and_save_clip() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    assert!(db_path.exists(), "Database file should be created");

    let clip = make_clip("Hello clipboard");
    let saved = store.save_clip(&clip).unwrap();
    assert!(saved.id.is_some());
    assert_eq!(saved.auto_tags, "#tag1");
}

#[test]
fn test_clip_tag_helpers() {
    let clip = Clip::new(
        "content".into(),
        "App".into(),
        "Window".into(),
        vec!["#code".into(), "#rust".into()],
        vec!["#study".into()],
        false,
    );

    assert_eq!(clip.auto_tags_vec(), vec!["#code", "#rust"]);
    assert_eq!(clip.manual_tags_vec(), vec!["#study"]);
}

#[test]
fn test_empty_tags_should_not_break() {
    let clip = Clip::new(
        "data".into(),
        "App".into(),
        "Title".into(),
        vec![],
        vec![],
        false,
    );
    assert!(clip.auto_tags_vec().is_empty());
    assert!(clip.manual_tags_vec().is_empty());
}

#[test]
fn test_unicode_and_long_content_handling() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    let long = "🐍 Rust ❤️ SQLite 🚀".repeat(100);
    let clip = make_clip(&long);
    let saved = store.save_clip(&clip).unwrap();

    let clips = store.get_recent_clips(1).unwrap();
    assert_eq!(clips[0].content, long);
    assert_eq!(clips[0].id, saved.id);
}

#[test]
fn test_get_recent_clips_returns_in_desc_order() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    for i in 0..3 {
        let clip = make_clip(&format!("Clip {}", i));
        store.save_clip(&clip).unwrap();
    }

    let clips = store.get_recent_clips(10).unwrap();
    assert_eq!(clips.len(), 3);
    assert!(clips[0].created_at >= clips[1].created_at);
}

#[test]
fn test_clear_history_removes_all_clips() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    store.save_clip(&make_clip("A")).unwrap();
    assert_eq!(store.get_recent_clips(10).unwrap().len(), 1);

    store.clear_history().unwrap();
    assert_eq!(store.get_recent_clips(10).unwrap().len(), 0);
}

#[test]
fn test_search_clips_finds_matches_in_content_app_and_window() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    let mut clip1 = make_clip("My unique content");
    clip1.app_name = "Firefox".to_string();
    clip1.window_title = "StackOverflow".to_string();

    let mut clip2 = make_clip("Something else");
    clip2.app_name = "VSCode".to_string();
    clip2.window_title = "Rust project".to_string();

    store.save_clip(&clip1).unwrap();
    store.save_clip(&clip2).unwrap();

    let found_by_content = store.search_clips("unique", 10).unwrap();
    let found_by_app = store.search_clips("vscode", 10).unwrap();
    let found_by_window = store.search_clips("stack", 10).unwrap();

    assert_eq!(found_by_content.len(), 1);
    assert_eq!(found_by_app.len(), 1);
    assert_eq!(found_by_window.len(), 1);
}

#[test]
fn test_set_pin_status_and_delete_clip() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    let clip = make_clip("Pin me");
    let saved = store.save_clip(&clip).unwrap();

    store.set_pin_status(saved.id.unwrap(), true).unwrap();
    let pinned = store.get_recent_clips(1).unwrap()[0].clone();
    assert!(pinned.is_pinned);

    store.set_pin_status(saved.id.unwrap(), false).unwrap();
    let unpinned = store.get_recent_clips(1).unwrap()[0].clone();
    assert!(!unpinned.is_pinned);

    store.delete_clip(saved.id.unwrap()).unwrap();
    assert!(store.get_recent_clips(10).unwrap().is_empty());
}

#[test]
fn test_invalid_timestamp_fallback() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    let conn = Connection::open(&db_path).unwrap();
    conn.execute(
        "INSERT INTO clips (content, app_name, window_title, auto_tags, manual_tags, is_pinned, created_at, updated_at)
         VALUES ('broken', 'App', 'Win', '', '', 0, 'INVALID', 'INVALID')",
        [],
    )
    .unwrap();

    let clips = store.get_recent_clips(5).unwrap();
    assert_eq!(clips.len(), 1);
    assert!(clips[0].created_at <= Utc::now());
}

#[test]
fn test_timestamp_is_recent() {
    let clip = make_clip("Test timestamp");
    let now = Utc::now();
    assert!((clip.created_at - now).num_seconds().abs() < 5);
    assert_eq!(clip.created_at, clip.updated_at);
}

#[test]
fn test_concurrent_inserts_dont_conflict() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = Arc::new(ClipStore::new(&db_path));
    store.init().unwrap();

    let threads = 5;
    let barrier = Arc::new(Barrier::new(threads));
    let mut handles = vec![];

    for i in 0..threads {
        let store = store.clone();
        let barrier = barrier.clone();
        handles.push(thread::spawn(move || {
            barrier.wait();
            let clip = make_clip(&format!("Clip {}", i));
            store.save_clip(&clip).unwrap();
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let all = store.get_recent_clips(10).unwrap();
    assert_eq!(all.len(), threads);
}
