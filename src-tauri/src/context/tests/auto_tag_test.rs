// src/context/tests/auto_tag_test.rs
#![cfg(test)]

use super::super::auto_tags::generate_auto_tags;

/// Helper: sort tags for deterministic comparison
fn sorted_tags(tags: Vec<String>) -> Vec<String> {
    let mut t = tags;
    t.sort();
    t
}

/// Convenience wrapper to reduce boilerplate — passes `None` for the 3rd argument.
fn gen(content: &str, project: Option<&str>) -> Vec<String> {
    generate_auto_tags(content, project, None)
}

#[test]
fn test_code_like_detection_across_languages() {
    let samples = [
        "def my_function(): pass",              // Python
        "fn main() { println!(\"Hi\"); }",      // Rust
        "const x = 10;",                        // JS
        "class Test {}",                        // Java/C++
        "package main",                         // Go
        "- key: value",                         // YAML
        "{ \"a\": 1 }",                         // JSON
        "<html><body></body></html>",           // HTML
    ];

    for sample in samples {
        let tags = gen(sample, None);
        assert!(
            tags.contains(&"#code".to_string()),
            "Expected '#code' for sample: {}",
            sample
        );
    }
}

#[test]
fn test_url_detection_variants() {
    let urls = [
        "https://example.com",
        "http://test.org",
        "ftp://ftp.server.net/file.txt",
        "mailto:someone@example.com",
        "data:text/plain;base64,SGVsbG8=",
    ];

    for url in urls {
        let tags = gen(url, None);
        assert!(
            tags.contains(&"#url".to_string()),
            "Expected '#url' tag for: {}",
            url
        );
    }

    let non_urls = [
        "example.com",
        "https:/broken.url",
        "http//missingcolon.com",
        "not a url",
    ];

    for text in non_urls {
        let tags = gen(text, None);
        assert!(
            !tags.contains(&"#url".to_string()),
            "Unexpected '#url' tag for: {}",
            text
        );
    }
}

#[test]
fn test_email_detection_strictness() {
    let valid = ["user@example.com", "user.name+tag@example.co.uk"];
    let invalid = ["user@@example.com", "userexample.com", "@nouser.com"];

    for e in valid {
        assert!(
            gen(e, None).contains(&"#email".to_string()),
            "Expected '#email' tag for valid email: {}",
            e
        );
    }

    for e in invalid {
        assert!(
            !gen(e, None).contains(&"#email".to_string()),
            "Unexpected '#email' tag for invalid email: {}",
            e
        );
    }
}

#[test]
fn test_terminal_command_detection() {
    let commands = [
        "git status",
        "npm install",
        "pnpm update",
        "yarn add",
        "cargo build",
        "docker ps",
        "ssh user@host",
        "sudo apt update",
        "./script.sh",
        "~/run.sh",
    ];

    for cmd in commands {
        assert!(
            gen(cmd, None).contains(&"#terminal".to_string()),
            "Expected '#terminal' tag for command: {}",
            cmd
        );
    }

    assert!(
        !gen("echo is builtin command but simple text", None)
            .contains(&"#terminal".to_string())
    );
}

#[test]
fn test_combined_tags_and_sorting() {
    let tags = gen("def main(): pass", Some("MyProject"));
    let expected = sorted_tags(vec!["#code".into(), "#myproject".into()]);
    assert_eq!(sorted_tags(tags.clone()), expected);
    assert_eq!(tags.len(), 2, "Should not contain duplicate tags");
}

#[test]
fn test_empty_inputs_produce_no_tags() {
    assert!(gen("", None).is_empty());
    assert!(gen("   ", None).is_empty());
}

#[test]
fn test_sanitize_complex_project_names() {
    let tags = gen("", Some("My *Cool* Project!"));
    assert_eq!(tags, vec!["#my-cool-project"]);
}

#[test]
fn test_multiline_code_detection() {
    let multiline = r#"
        import os
        def func():
            print("Hello")
    "#;
    let tags = gen(multiline, None);
    assert!(
        tags.contains(&"#code".to_string()),
        "Expected '#code' for multiline content"
    );
}

#[test]
fn test_deterministic_and_unique_output() {
    let tags = gen("fn x() {}", Some("Proj"));
    let sorted = sorted_tags(tags.clone());
    assert_eq!(tags, sorted, "Tags should be sorted");
    let unique: std::collections::HashSet<_> = tags.iter().collect();
    assert_eq!(unique.len(), tags.len(), "Tags must be unique");
}

#[test]
fn test_edge_case_urls_and_emails() {
    let edge_email = "user.name+tag+sorting@example.com";
    let tags = gen(edge_email, None);
    assert!(tags.contains(&"#email".to_string()));

    let complex_url = "https://sub.domain.example.com/path?query=1#anchor";
    let tags = gen(complex_url, None);
    assert!(tags.contains(&"#url".to_string()));
}
