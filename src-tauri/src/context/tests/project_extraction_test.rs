use crate::context::linux::extract_project_from_title_linux;

#[test]
fn extracts_from_vscode_title_linux() {
    assert_eq!(
        extract_project_from_title_linux("my-ecommerce — Visual Studio Code"),
        Some("my-ecommerce".to_string())
    );
}

#[test]
fn extracts_from_konsole_title_linux() {
    assert_eq!(
        extract_project_from_title_linux("user@fedora: ~/projects/blog"),
        Some("blog".to_string())
    );
}

#[test]
fn handles_firefox_title_linux() {
    // Firefox often shows just the page title_linux — no project
    assert_eq!(
        extract_project_from_title_linux("Rust - Mozilla Firefox"),
        None
    );
}

#[test]
fn ignores_generic_title_linuxs() {
    assert_eq!(extract_project_from_title_linux("Terminal — Konsole"), None);
    assert_eq!(extract_project_from_title_linux("Home - Chrome"), None);
    assert_eq!(extract_project_from_title_linux("Untitled — Code"), None);
}

#[test]
fn handles_different_separators() {
    assert_eq!(
        extract_project_from_title_linux("my-app - Visual Studio Code"),
        Some("my-app".to_string())
    );
    assert_eq!(
        extract_project_from_title_linux("my-app — Visual Studio Code"),
        Some("my-app".to_string())
    );
    assert_eq!(
        extract_project_from_title_linux("my-app · Visual Studio Code"),
        Some("my-app".to_string())
    );
    assert_eq!(
        extract_project_from_title_linux("my-app | Visual Studio Code"),
        Some("my-app".to_string())
    );
    assert_eq!(
        extract_project_from_title_linux("my-app :: Visual Studio Code"),
        Some("my-app".to_string())
    );
}

#[test]
fn returns_none_for_empty_or_invalid() {
    assert_eq!(extract_project_from_title_linux(""), None);
    assert_eq!(
        extract_project_from_title_linux(" — Visual Studio Code"),
        None
    );
    assert_eq!(extract_project_from_title_linux(" - Code"), None);
    assert_eq!(extract_project_from_title_linux(" | Code"), None);
}

#[test]
fn handles_terminal_path_variants() {
    assert_eq!(
        extract_project_from_title_linux("user@fedora: /home/user/projects/cli_tool"),
        Some("cli_tool".to_string())
    );
    assert_eq!(
        extract_project_from_title_linux("user@arch:~/workspace/rust/mycrate"),
        Some("mycrate".to_string())
    );
    assert_eq!(extract_project_from_title_linux("user@laptop:~/"), None);
}

#[test]
fn case_insensitive_generic_title_linuxs() {
    assert_eq!(
        extract_project_from_title_linux("my-app — VISUAL STUDIO CODE"),
        Some("my-app".to_string())
    );
    assert_eq!(extract_project_from_title_linux("Visual Studio Code"), None);
    assert_eq!(extract_project_from_title_linux("chrome"), None);
    assert_eq!(extract_project_from_title_linux("TERMINAL"), None);
}

#[test]
fn handles_nested_paths_or_noise() {
    // Should still find last valid folder
    assert_eq!(
        extract_project_from_title_linux("user@fedora:~/src/github.com/kasif/clipcontex"),
        Some("clipcontex".to_string())
    );

    // Should ignore trailing shell name
    assert_eq!(
        extract_project_from_title_linux("~/projects/quiz_app — zsh"),
        Some("quiz_app".to_string())
    );
}

#[test]
fn handles_title_linuxs_without_known_separator() {
    // No separator or recognizable path → None
    assert_eq!(extract_project_from_title_linux("MyAppWindowTitle"), None);

    // Single dash in middle, but not a separator pattern
    assert_eq!(extract_project_from_title_linux("realtime-dashboard"), None);
}

#[test]
fn trims_and_ignores_spaces() {
    assert_eq!(
        extract_project_from_title_linux("   myproj   —   Code   "),
        Some("myproj".to_string())
    );
    assert_eq!(
        extract_project_from_title_linux("   user@fedora:   ~/dev/demo   "),
        Some("demo".to_string())
    );
}

#[test]
fn handles_non_ascii_names() {
    // Unicode project names are supported
    assert_eq!(
        extract_project_from_title_linux("日本語プロジェクト — Visual Studio Code"),
        Some("日本語プロジェクト".to_string())
    );
    assert_eq!(
        extract_project_from_title_linux("المشروع - Code"),
        Some("المشروع".to_string())
    );
}

