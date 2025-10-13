use super::super::project::extract_project_from_title;

#[test]
fn extracts_from_vscode_title() {
    assert_eq!(
        extract_project_from_title("my-ecommerce — Visual Studio Code"),
        Some("my-ecommerce".to_string())
    );
}

#[test]
fn extracts_from_konsole_title() {
    assert_eq!(
        extract_project_from_title("user@fedora: ~/projects/blog"),
        Some("blog".to_string())
    );
}

#[test]
fn handles_firefox_title() {
    // Firefox often shows just the page title — no project
    assert_eq!(
        extract_project_from_title("Rust - Mozilla Firefox"),
        None
    );
}

#[test]
fn ignores_generic_titles() {
    assert_eq!(extract_project_from_title("Terminal — Konsole"), None);
    assert_eq!(extract_project_from_title("Home - Chrome"), None);
    assert_eq!(extract_project_from_title("Untitled — Code"), None);
}

#[test]
fn handles_different_separators() {
    assert_eq!(
        extract_project_from_title("my-app - Visual Studio Code"),
        Some("my-app".to_string())
    );
    assert_eq!(
        extract_project_from_title("my-app — Visual Studio Code"),
        Some("my-app".to_string())
    );
    assert_eq!(
        extract_project_from_title("my-app · Visual Studio Code"),
        Some("my-app".to_string())
    );
    assert_eq!(
        extract_project_from_title("my-app | Visual Studio Code"),
        Some("my-app".to_string())
    );
    assert_eq!(
        extract_project_from_title("my-app :: Visual Studio Code"),
        Some("my-app".to_string())
    );
}

#[test]
fn returns_none_for_empty_or_invalid() {
    assert_eq!(extract_project_from_title(""), None);
    assert_eq!(extract_project_from_title(" — Visual Studio Code"), None);
    assert_eq!(extract_project_from_title(" - Code"), None);
    assert_eq!(extract_project_from_title(" | Code"), None);
}

#[test]
fn handles_terminal_path_variants() {
    assert_eq!(
        extract_project_from_title("user@fedora: /home/user/projects/cli_tool"),
        Some("cli_tool".to_string())
    );
    assert_eq!(
        extract_project_from_title("user@arch:~/workspace/rust/mycrate"),
        Some("mycrate".to_string())
    );
    assert_eq!(
        extract_project_from_title("user@laptop:~/"),
        None
    );
}

#[test]
fn case_insensitive_generic_titles() {
    assert_eq!(extract_project_from_title("my-app — VISUAL STUDIO CODE"), Some("my-app".to_string()));
    assert_eq!(extract_project_from_title("Visual Studio Code"), None);
    assert_eq!(extract_project_from_title("chrome"), None);
    assert_eq!(extract_project_from_title("TERMINAL"), None);
}

#[test]
fn handles_nested_paths_or_noise() {
    // Should still find last valid folder
    assert_eq!(
        extract_project_from_title("user@fedora:~/src/github.com/kasif/clipcontex"),
        Some("clipcontex".to_string())
    );

    // Should ignore trailing shell name
    assert_eq!(
        extract_project_from_title("~/projects/quiz_app — zsh"),
        Some("quiz_app".to_string())
    );
}

#[test]
fn handles_titles_without_known_separator() {
    // No separator or recognizable path → None
    assert_eq!(extract_project_from_title("MyAppWindowTitle"), None);

    // Single dash in middle, but not a separator pattern
    assert_eq!(extract_project_from_title("realtime-dashboard"), None);
}

#[test]
fn trims_and_ignores_spaces() {
    assert_eq!(
        extract_project_from_title("   myproj   —   Code   "),
        Some("myproj".to_string())
    );
    assert_eq!(
        extract_project_from_title("   user@fedora:   ~/dev/demo   "),
        Some("demo".to_string())
    );
}

#[test]
fn handles_non_ascii_names() {
    // Unicode project names are supported
    assert_eq!(
        extract_project_from_title("日本語プロジェクト — Visual Studio Code"),
        Some("日本語プロジェクト".to_string())
    );
    assert_eq!(
        extract_project_from_title("المشروع - Code"),
        Some("المشروع".to_string())
    );
}