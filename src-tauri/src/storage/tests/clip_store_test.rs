use chrono::{Duration, Utc};
use rusqlite::Connection;
use tempfile::tempdir;

use crate::storage::{clip::Clip, clip_store::ClipStore};

fn make_clip(content: &str) -> Clip {
    Clip {
        id: None,
        content: content.to_string(),
        app_name: "App".to_string(),
        window_title: "Window".to_string(),
        auto_tags: "#tag1".to_string(),
        is_pinned: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[test]
fn test_save_and_get_recent_clips() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");

    let store = ClipStore::new(&db_path).unwrap();

    let clip = make_clip("hello");
    store.save_clip(&clip).unwrap();

    let clips = store.get_recent_clips(10).unwrap();
    assert_eq!(clips.len(), 1);
    assert_eq!(clips[0].content, "hello");
}

#[test]
fn test_get_recent_clips_limit() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");

    let store = ClipStore::new(&db_path).unwrap();

    for i in 0..5 {
        store.save_clip(&make_clip(&format!("clip {}", i))).unwrap();
    }

    let clips = store.get_recent_clips(2).unwrap();
    assert_eq!(clips.len(), 2);
    assert_eq!(clips[0].content, "clip 4");
    assert_eq!(clips[1].content, "clip 3");
}

#[test]
fn test_set_pin_status() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");

    let store = ClipStore::new(&db_path).unwrap();
    let saved = store.save_clip(&make_clip("pin me")).unwrap();

    store.set_pin_status(saved.id.unwrap(), true).unwrap();

    let clips = store.get_recent_clips(1).unwrap();
    assert!(clips[0].is_pinned);
}

#[test]
fn test_delete_clip() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");

    let store = ClipStore::new(&db_path).unwrap();
    let saved = store.save_clip(&make_clip("delete me")).unwrap();

    store.delete_clip(saved.id.unwrap()).unwrap();

    let clips = store.get_recent_clips(10).unwrap();
    assert!(clips.is_empty());
}

#[test]
fn test_clear_history() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");

    let store = ClipStore::new(&db_path).unwrap();

    store.save_clip(&make_clip("a")).unwrap();
    store.save_clip(&make_clip("b")).unwrap();

    store.clear_history().unwrap();

    let clips = store.get_recent_clips(10).unwrap();
    assert!(clips.is_empty());
}

#[test]
fn test_perform_cleanup_removes_old_clips() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");

    let store = ClipStore::new(&db_path).unwrap();

    // Insert an old clip manually
    let conn = Connection::open(&db_path).unwrap();
    let old_time = (Utc::now() - Duration::days(10)).to_rfc3339();

    conn.execute(
        r#"
        INSERT INTO clips (
            content, app_name, window_title,
            auto_tags, is_pinned,
            created_at, updated_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)
        "#,
        rusqlite::params!["old", "App", "Win", "", false, old_time],
    )
    .unwrap();

    // Recent clip
    store.save_clip(&make_clip("recent")).unwrap();

    store.perform_cleanup(5, 10).unwrap();

    let clips = store.get_recent_clips(10).unwrap();
    assert_eq!(clips.len(), 1);
    assert_eq!(clips[0].content, "recent");
}

#[test]
fn test_perform_cleanup_enforces_max_size() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");

    let store = ClipStore::new(&db_path).unwrap();

    for i in 0..5 {
        store.save_clip(&make_clip(&format!("clip {}", i))).unwrap();
    }

    store.perform_cleanup(100, 3).unwrap();

    let clips = store.get_recent_clips(10).unwrap();
    assert_eq!(clips.len(), 3);
    assert_eq!(clips[0].content, "clip 4");
    assert_eq!(clips[1].content, "clip 3");
    assert_eq!(clips[2].content, "clip 2");
}

#[test]
fn test_nested_db_path_creation() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("a/b/c/clips.db");

    let store = ClipStore::new(&db_path).unwrap();
    store.save_clip(&make_clip("works")).unwrap();

    assert!(db_path.exists());
}
