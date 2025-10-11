use super::linux::parse_xprop_output;

#[test]
fn parses_vscode_window() {
    let output = r#"WM_NAME(STRING) = "my-app — Visual Studio Code"
WM_CLASS(STRING) = "code", "Code""#;
    let info = parse_xprop_output(output);
    assert_eq!(info.window_title, "my-app — Visual Studio Code");
    assert_eq!(info.app_class, "code");
}

#[test]
fn parses_konsole_window() {
    let output = r#"WM_NAME(STRING) = "user@fedora: ~/projects"
WM_CLASS(STRING) = "konsole", "Konsole""#;
    let info = parse_xprop_output(output);
    assert_eq!(info.window_title, "user@fedora: ~/projects");
    assert_eq!(info.app_class, "konsole");
}

#[test]
fn handles_missing_fields() {
    let output = "WM_CLASS(STRING) = \"firefox\", \"Firefox\"";
    let info = parse_xprop_output(output);
    assert_eq!(info.window_title, "Unknown");
    assert_eq!(info.app_class, "firefox");
}

#[test]
fn parses_utf8_string_variant() {
    let output = r#"WM_NAME(UTF8_STRING) = "Terminal — GNOME"
WM_CLASS(STRING) = "org.gnome.Terminal", "Gnome-terminal""#;
    let info = parse_xprop_output(output);
    assert_eq!(info.window_title, "Terminal — GNOME");
    assert_eq!(info.app_class, "org.gnome.terminal");
}

#[test]
fn handles_extra_whitespace_and_casing() {
    let output = r#"WM_NAME(STRING) = "Files"
WM_CLASS(STRING) =   "Nautilus"  ,  "nautilus" "#;
    let info = parse_xprop_output(output);
    assert_eq!(info.window_title, "Files");
    assert_eq!(info.app_class, "nautilus"); // lowercased first class
}

#[test]
fn handles_malformed_output() {
    let output = r#"WM_NAME(STRING) = 
WM_CLASS(STRING) = "#;
    let info = parse_xprop_output(output);
    assert_eq!(info.window_title, "Unknown");
    assert_eq!(info.app_class, "unknown");
}

#[test]
fn handles_comma_in_title() {
    let output = r#"WM_NAME(STRING) = "Hello, World App"
WM_CLASS(STRING) = "myapp", "MyApp""#;
    let info = parse_xprop_output(output);
    assert_eq!(info.window_title, "Hello, World App");
    assert_eq!(info.app_class, "myapp");
}

#[test]
fn ignores_unrelated_lines() {
    let output = r#"WM_HINTS(WM_HINTS):
        Client accepts input or input focus: True
WM_NAME(STRING) = "Steam"
WM_CLASS(STRING) = "steam", "Steam"
WM_STATE(WM_STATE):
        window state: Normal"#;
    let info = parse_xprop_output(output);
    assert_eq!(info.window_title, "Steam");
    assert_eq!(info.app_class, "steam");
}

#[test]
fn handles_no_quotes() {
    let output = r#"WM_NAME(STRING) = Untitled
WM_CLASS(STRING) = firefox, Firefox"#;
    let info = parse_xprop_output(output);
    assert_eq!(info.window_title, "Untitled");
    assert_eq!(info.app_class, "firefox");
}

#[test]
fn handles_missing_everything() {
    let output = "";
    let info = parse_xprop_output(output);
    assert_eq!(info.window_title, "Unknown");
    assert_eq!(info.app_class, "unknown");
}

#[test]
fn handles_extra_long_lines() {
    let output = format!(
        "WM_NAME(STRING) = \"{}\"\nWM_CLASS(STRING) = \"chromium\", \"Chromium\"",
        "A".repeat(1000)
    );
    let info = parse_xprop_output(&output);
    assert_eq!(info.window_title.len(), 1000);
    assert_eq!(info.app_class, "chromium");
}

// Test cases to check extract project from window title
use super::project::extract_project_from_title;

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
