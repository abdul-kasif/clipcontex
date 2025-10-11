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
