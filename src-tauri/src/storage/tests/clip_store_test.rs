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
fn test_remove_older_than_days_with_no_old_clips() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    let clip = make_clip("recent clip");
    store.save_clip(&clip).unwrap();

    // Remove clips older than 100 days (should remove nothing)
    let deleted_count = store.remove_older_than_days(100).unwrap();
    assert_eq!(deleted_count, 0);

    let remaining_clips = store.get_recent_clips(10).unwrap();
    assert_eq!(remaining_clips.len(), 1);
}

#[test]
fn test_remove_older_than_days_removes_all() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    // Create old clips manually with old timestamps
    let conn = Connection::open(&db_path).unwrap();
    conn.execute(
        "INSERT INTO clips (content, app_name, window_title, auto_tags, manual_tags, is_pinned, created_at, updated_at)
         VALUES ('very old', 'App', 'Win', '', '', 0, datetime('now', '-2 days'), datetime('now', '-2 days'))",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO clips (content, app_name, window_title, auto_tags, manual_tags, is_pinned, created_at, updated_at)
         VALUES ('also old', 'App', 'Win', '', '', 0, datetime('now', '-3 days'), datetime('now', '-3 days'))",
        [],
    )
    .unwrap();

    // Remove clips older than 1 day (should remove both)
    let deleted_count = store.remove_older_than_days(1).unwrap();
    assert_eq!(deleted_count, 2);

    let remaining_clips = store.get_recent_clips(10).unwrap();
    assert_eq!(remaining_clips.len(), 0);
}

#[test]
fn test_enforce_max_size_exact_limit() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    // Add 5 clips
    for i in 0..5 {
        let clip = make_clip(&format!("Clip {}", i));
        store.save_clip(&clip).unwrap();
    }

    // Enforce max size of 3 (should remove 2 oldest)
    let deleted_count = store.enforce_max_size(3).unwrap();
    assert_eq!(deleted_count, 2);

    let remaining_clips = store.get_recent_clips(10).unwrap();
    assert_eq!(remaining_clips.len(), 3);
    
    // Should keep the most recent 3 clips (Clip 4, Clip 3, Clip 2 in that order)
    assert_eq!(remaining_clips[0].content, "Clip 4");
    assert_eq!(remaining_clips[1].content, "Clip 3");
    assert_eq!(remaining_clips[2].content, "Clip 2");
}

#[test]
fn test_enforce_max_size_no_change_needed() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    // Add 3 clips
    for i in 0..3 {
        let clip = make_clip(&format!("Clip {}", i));
        store.save_clip(&clip).unwrap();
    }

    // Enforce max size of 5 (should remove nothing)
    let deleted_count = store.enforce_max_size(5).unwrap();
    assert_eq!(deleted_count, 0);

    let remaining_clips = store.get_recent_clips(10).unwrap();
    assert_eq!(remaining_clips.len(), 3);
}

#[test]
fn test_enforce_max_size_zero() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    // Add 3 clips
    for i in 0..3 {
        let clip = make_clip(&format!("Clip {}", i));
        store.save_clip(&clip).unwrap();
    }

    // Enforce max size of 0 (should remove all)
    let deleted_count = store.enforce_max_size(0).unwrap();
    assert_eq!(deleted_count, 3);

    let remaining_clips = store.get_recent_clips(10).unwrap();
    assert_eq!(remaining_clips.len(), 0);
}

#[test]
fn test_perform_cleanup() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    // Create old clips manually
    let conn = Connection::open(&db_path).unwrap();
    conn.execute(
        "INSERT INTO clips (content, app_name, window_title, auto_tags, manual_tags, is_pinned, created_at, updated_at)
         VALUES ('old1', 'App', 'Win', '', '', 0, datetime('now', '-10 days'), datetime('now', '-10 days'))",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO clips (content, app_name, window_title, auto_tags, manual_tags, is_pinned, created_at, updated_at)
         VALUES ('old2', 'App', 'Win', '', '', 0, datetime('now', '-5 days'), datetime('now', '-5 days'))",
        [],
    )
    .unwrap();
    
    // Add some recent clips
    let recent_clip = make_clip("recent");
    store.save_clip(&recent_clip).unwrap();

    // Perform cleanup: remove older than 7 days, max size 1
    store.perform_cleanup(7, 1).unwrap();

    let remaining_clips = store.get_recent_clips(10).unwrap();
    assert_eq!(remaining_clips.len(), 1);
    assert_eq!(remaining_clips[0].content, "recent");
}

#[test]
fn test_get_recent_clips_with_limit() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    // Add 5 clips
    for i in 0..5 {
        let clip = make_clip(&format!("Clip {}", i));
        store.save_clip(&clip).unwrap();
    }

    // Get only 2 most recent
    let clips = store.get_recent_clips(2).unwrap();
    assert_eq!(clips.len(), 2);
    assert_eq!(clips[0].content, "Clip 4");
    assert_eq!(clips[1].content, "Clip 3");
}

#[test]
fn test_get_recent_clips_empty_db() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    let clips = store.get_recent_clips(10).unwrap();
    assert_eq!(clips.len(), 0);
}

#[test]
fn test_delete_nonexistent_clip() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    // Try to delete a clip that doesn't exist
    let result = store.delete_clip(999);
    assert!(result.is_ok()); // Should not error, just return 0 affected rows
}

#[test]
fn test_set_pin_status_nonexistent_clip() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    // Try to pin a clip that doesn't exist
    let result = store.set_pin_status(999, true);
    assert!(result.is_ok()); // Should not error, just return 0 affected rows
}

#[test]
fn test_pin_status_persistence() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    let clip = make_clip("test pin");
    let saved = store.save_clip(&clip).unwrap();

    // Pin the clip
    store.set_pin_status(saved.id.unwrap(), true).unwrap();
    let retrieved = store.get_recent_clips(10).unwrap();
    assert_eq!(retrieved.len(), 1);
    assert!(retrieved[0].is_pinned);

    // Unpin the clip
    store.set_pin_status(saved.id.unwrap(), false).unwrap();
    let retrieved = store.get_recent_clips(10).unwrap();
    assert_eq!(retrieved.len(), 1);
    assert!(!retrieved[0].is_pinned);
}

#[test]
fn test_multiple_pins() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    let clip1 = make_clip("clip 1");
    let clip2 = make_clip("clip 2");
    let saved1 = store.save_clip(&clip1).unwrap();
    let saved2 = store.save_clip(&clip2).unwrap();

    // Pin both clips
    store.set_pin_status(saved1.id.unwrap(), true).unwrap();
    store.set_pin_status(saved2.id.unwrap(), true).unwrap();

    let all_clips = store.get_recent_clips(10).unwrap();
    assert_eq!(all_clips.len(), 2);
    assert!(all_clips.iter().all(|clip| clip.is_pinned));
}

#[test]
fn test_database_initialization_idempotency() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    
    // Initialize twice - should not cause issues
    let store = ClipStore::new(&db_path);
    store.init().unwrap();
    store.init().unwrap(); // Second init should be safe

    assert!(db_path.exists());

    // Should still work normally
    let clip = make_clip("test");
    let saved = store.save_clip(&clip).unwrap();
    assert!(saved.id.is_some());
}

#[test]
fn test_database_path_with_nested_directories() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("nested").join("deep").join("clips.db");
    
    let store = ClipStore::new(&db_path);
    store.init().unwrap(); // Should create nested directories

    assert!(db_path.exists());
    assert!(db_path.parent().unwrap().exists());

    let clip = make_clip("nested path test");
    let saved = store.save_clip(&clip).unwrap();
    assert!(saved.id.is_some());
}

#[test]
fn test_large_content_handling() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    // Create very large content (10KB)
    let large_content = "A".repeat(10 * 1024);
    let clip = make_clip(&large_content);
    let saved = store.save_clip(&clip).unwrap();

    let retrieved = store.get_recent_clips(1).unwrap();
    assert_eq!(retrieved.len(), 1);
    assert_eq!(retrieved[0].content, large_content);
    assert_eq!(retrieved[0].id, saved.id);
}

#[test]
fn test_special_characters_in_content() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    let special_content = "Test with 'quotes', \"double quotes\", & symbols! @#$%^&*()";
    let clip = make_clip(special_content);
    let saved = store.save_clip(&clip).unwrap();

    let retrieved = store.get_recent_clips(1).unwrap();
    assert_eq!(retrieved[0].content, special_content);
    assert_eq!(retrieved[0].id, saved.id);
}

#[test]
fn test_empty_content_handling() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    let empty_clip = Clip::new(
        "".to_string(),
        "App".to_string(),
        "Window".to_string(),
        vec![],
        vec![],
        false,
    );
    let saved = store.save_clip(&empty_clip).unwrap();
    assert!(saved.id.is_some());

    let retrieved = store.get_recent_clips(1).unwrap();
    assert_eq!(retrieved.len(), 1);
    assert_eq!(retrieved[0].content, "");
}

#[test]
fn test_timestamp_precision() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    let clip = make_clip("timestamp test");
    let saved = store.save_clip(&clip).unwrap();

    // Should have created_at and updated_at timestamps
    assert!(saved.created_at <= Utc::now());
    assert_eq!(saved.created_at, saved.updated_at);

    // After pinning, updated_at should change
    std::thread::sleep(std::time::Duration::from_millis(10));
    store.set_pin_status(saved.id.unwrap(), true).unwrap();

    let updated_clip = store.get_recent_clips(1).unwrap()[0].clone();
    assert!(updated_clip.updated_at > saved.updated_at);
}

#[test]
fn test_multiple_operations_chaining() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    // Chain multiple operations
    let clip1 = make_clip("clip 1");
    let clip2 = make_clip("clip 2");
    let clip3 = make_clip("clip 3");

    let saved1 = store.save_clip(&clip1).unwrap();
    let saved2 = store.save_clip(&clip2).unwrap();
    store.save_clip(&clip3).unwrap();

    // Pin first clip
    store.set_pin_status(saved1.id.unwrap(), true).unwrap();

    // Delete second clip
    store.delete_clip(saved2.id.unwrap()).unwrap();

    // Get remaining clips
    let remaining = store.get_recent_clips(10).unwrap();
    assert_eq!(remaining.len(), 2);
    assert_eq!(remaining[0].content, "clip 3"); // Most recent
    assert_eq!(remaining[1].content, "clip 1"); // Pinned
    assert!(remaining[1].is_pinned);
}

#[test]
fn test_get_recent_clips_order_with_pins() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    let clip1 = make_clip("first");
    let clip2 = make_clip("second");
    let clip3 = make_clip("third");

    let saved1 = store.save_clip(&clip1).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let _saved2 = store.save_clip(&clip2).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    store.save_clip(&clip3).unwrap();

    // Pin the oldest clip
    store.set_pin_status(saved1.id.unwrap(), true).unwrap();

    let clips = store.get_recent_clips(10).unwrap();
    assert_eq!(clips.len(), 3);
    // Most recent should come first, then pinned items, then others in time order
    assert_eq!(clips[0].content, "third");
    assert_eq!(clips[1].content, "second");
    assert_eq!(clips[2].content, "first");
    assert!(clips[2].is_pinned);
}

#[test]
fn test_database_file_permissions() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    // Test that we can open the database file multiple times
    let store2 = ClipStore::new(&db_path);
    let clip = make_clip("test");
    let saved = store2.save_clip(&clip).unwrap();
    assert!(saved.id.is_some());

    let clips = store2.get_recent_clips(1).unwrap();
    assert_eq!(clips.len(), 1);
}

#[test]
fn test_timestamp_edge_cases() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clips.db");
    let store = ClipStore::new(&db_path);
    store.init().unwrap();

    // Create clips with timestamps very close together
    for i in 0..5 {
        let clip = make_clip(&format!("clip {}", i));
        store.save_clip(&clip).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1)); // Ensure different timestamps
    }

    let clips = store.get_recent_clips(10).unwrap();
    assert_eq!(clips.len(), 5);

    // Verify they're in correct order (most recent first)
    for i in 0..4 {
        assert!(clips[i].created_at >= clips[i + 1].created_at);
    }
}