use super::super::linux::parse_xprop_output;

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