/// Extracts a clean project name from a window title.
/// Returns `None` if no meaningful project name is found.
pub fn extract_project_from_title(window_title: &str) -> Option<String> {
    let title = window_title.trim();
    let separators = [" — ", " – ", " - ", " · ", " | ", " :: "];

    // Try extracting before known separators (e.g., "my-app — Visual Studio Code")
    for &sep in &separators {
        if let Some(pos) = title.find(sep) {
            let (candidate, suffix) = title.split_at(pos);
            let candidate = candidate.trim();
            let suffix = suffix.trim_start_matches(sep).trim().to_lowercase();

            // If the suffix is a known app name like "firefox", "zsh", etc. → skip
            if is_generic_title(&suffix) {
                if !candidate.is_empty() && !is_generic_title(candidate) {
                    return Some(clean_segment(candidate));
                }
                return None;
            }
        }
    }

    // Handle paths like "user@fedora:~/src/github.com/kasif/clipcontex"
    if let Some((_, path_part)) = title.split_once(':') {
        if let Some(last_segment) = extract_last_path_segment(path_part) {
            return Some(last_segment);
        }
    }

    // Handle plain paths like "~/projects/app — zsh"
    if let Some(last_segment) = extract_last_path_segment(title) {
        return Some(last_segment);
    }

    None
}

/// Extract last meaningful folder name from path-like strings.
fn extract_last_path_segment(text: &str) -> Option<String> {
    let mut clean = text.trim();

    // Cut off known separators after the path (e.g., " — zsh", " - bash")
    for sep in [" — ", " – ", " - ", " · ", " | ", " :: "] {
        if let Some(pos) = clean.find(sep) {
            clean = &clean[..pos];
            break;
        }
    }

    if clean.contains('/') {
        if let Some(seg) = clean.rsplit('/').next() {
            let seg = seg.trim();
            if !seg.is_empty() && !is_generic_title(seg) {
                return Some(clean_segment(seg));
            }
        }
    }

    None
}

/// Trim and normalize path segments.
fn clean_segment(segment: &str) -> String {
    let s = segment.trim().trim_matches(['~', '/']);
    if s.contains('/') {
        s.rsplit('/').next().unwrap_or(s).to_string()
    } else {
        s.to_string()
    }
}
/// Generic names to ignore as project names.
fn is_generic_title(title: &str) -> bool {
    let lower = title.trim().to_lowercase();
    let generic = [
        "terminal",
        "konsole",
        "firefox",
        "chrome",
        "visual studio code",
        "code",
        "new tab",
        "untitled",
        "home",
        "bash",
        "zsh",
        "",
    ];
    generic.contains(&lower.as_str())
}
