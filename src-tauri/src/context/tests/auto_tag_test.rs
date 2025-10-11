// Test cases for auto-tagging
use super::super::auto_tag::generate_auto_tags;

#[test]
fn project_name_tag() {
    assert_eq!(
        generate_auto_tags("some content", Some("My-App")),
        vec!["#my-app"]
    );
    assert_eq!(
        generate_auto_tags("some content", Some("My App!")),
        vec!["#my-app"]
    );
    assert_eq!(
        generate_auto_tags("some content", Some("")),
        Vec::<String>::new()
    );
}

#[test]
fn code_like_detection() {
    let python_code = "def my_function():\n    pass";
    let rust_code = "fn main() { println!(\"Hello\"); }";
    let js_code = "const x = 10;";
    let yaml_content = "- key: value";

    assert!(generate_auto_tags(python_code, None).contains(&"#code".to_string()));
    assert!(generate_auto_tags(rust_code, None).contains(&"#code".to_string()));
    assert!(generate_auto_tags(js_code, None).contains(&"#code".to_string()));
    assert!(generate_auto_tags(yaml_content, None).contains(&"#code".to_string()));
}

#[test]
fn url_detection() {
    let url = "https://example.com/path";
    let ftp_url = "ftp://example.com/file";
    let mailto = "mailto:user@example.com";
    let data_url = "data:text/plain;base64,SGVsbG8sIFdvcmxkIQ==";

    assert!(generate_auto_tags(url, None).contains(&"#url".to_string()));
    assert!(generate_auto_tags(ftp_url, None).contains(&"#url".to_string()));
    assert!(generate_auto_tags(mailto, None).contains(&"#url".to_string()));
    assert!(generate_auto_tags(data_url, None).contains(&"#url".to_string()));

    // Non-URL should not produce #url
    assert!(!generate_auto_tags("not a url", None).contains(&"#url".to_string()));
}

#[test]
fn email_detection() {
    let valid_email = "user@example.com";
    let invalid_email = "user@@example.com";

    assert!(generate_auto_tags(valid_email, None).contains(&"#email".to_string()));
    assert!(!generate_auto_tags(invalid_email, None).contains(&"#email".to_string()));
}

#[test]
fn terminal_command_detection() {
    let commands = [
        "git status",
        "npm install",
        "pnpm update",
        "yarn add package",
        "cargo build",
        "docker ps",
        "ssh user@host",
        "sudo apt update",
        "./script.sh",
        "~/bin/run",
    ];

    for cmd in commands {
        assert!(generate_auto_tags(cmd, None).contains(&"#terminal".to_string()));
    }

    // Non-command text
    assert!(!generate_auto_tags("this is not a command", None).contains(&"#terminal".to_string()));
}

#[test]
fn combined_tags() {
    let content = "def main(): pass";
    let project = "MyProject";
    let tags = generate_auto_tags(content, Some(project));

    assert!(tags.contains(&"#code".to_string()));
    assert!(tags.contains(&"#myproject".to_string()));
    assert_eq!(tags.len(), 2);
}

#[test]
fn empty_content_and_none_project() {
    let tags = generate_auto_tags("", None);
    assert!(tags.is_empty());
}

#[test]
fn sanitize_project_names() {
    let project_name = "My *Cool* Project!";
    let tags = generate_auto_tags("", Some(project_name));
    assert_eq!(tags, vec!["#my-cool-project"]);
}

#[test]
fn multiple_lines_code_detection() {
    let content = r#"
        import os
        def func():
            print("Hello")
        "#;
    let tags = generate_auto_tags(content, None);
    assert!(tags.contains(&"#code".to_string()));
}

#[test]
fn tags_sorted_and_unique() {
    let content = "def my_func(): pass";
    let project = "MyProject";
    let tags = generate_auto_tags(content, Some(project));

    let mut sorted = tags.clone();
    sorted.sort();
    assert_eq!(tags, sorted);

    // Ensure no duplicates
    let unique: std::collections::HashSet<_> = tags.iter().collect();
    assert_eq!(unique.len(), tags.len());
}

#[test]
fn edge_case_urls_and_emails() {
    let content = "user.name+tag+sorting@example.com";
    let tags = generate_auto_tags(content, None);
    assert!(tags.contains(&"#email".to_string()));

    let url = "https://sub.domain.example.com/path?query=1";
    let tags = generate_auto_tags(url, None);
    assert!(tags.contains(&"#url".to_string()));
}
