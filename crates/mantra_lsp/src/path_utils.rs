// ==============================================================================
// mantra_lsp::path_utils — Cross-platform URI <-> Path conversions
// Supports macOS, Linux, and Windows (file:///C:/... and file:///path/...)
// ==============================================================================

use std::path::{Path, PathBuf};

/// Convert an LSP `file://` URI into a native OS `PathBuf`.
/// Works consistently on macOS, Linux, and Windows.
pub fn uri_to_file_path(uri: &str) -> Option<PathBuf> {
    if !uri.starts_with("file://") {
        return None;
    }

    // Strip scheme
    let raw_path = &uri["file://".len()..];

    // Decode percent-encoded characters (e.g. %20 -> space, %3A -> :, %2F -> /)
    let decoded = url_decode(raw_path);
    let mut path_str = decoded.as_str();

    // Windows handling: file:///C:/path or file:///c:/path or file://C:/path
    // On Windows, raw_path starts with "/C:" or "/c:" or "C:"
    #[cfg(windows)]
    {
        if path_str.starts_with('/') && path_str.len() >= 3 && path_str.as_bytes()[2] == b':' {
            path_str = &path_str[1..];
        }
        return Some(PathBuf::from(path_str.replace('/', "\\")));
    }

    #[cfg(not(windows))]
    {
        // On macOS/Linux, if VS Code sent file:///Users/..., raw_path starts with /Users/...
        // But if someone sent file:///C:/ on non-windows (tests, etc.), strip leading slash if drive letter
        if path_str.len() >= 3 && path_str.starts_with('/') && path_str.as_bytes()[2] == b':' {
            path_str = &path_str[1..];
        }
        Some(PathBuf::from(path_str))
    }
}

/// Convert a native OS `Path` into an LSP `file://` URI string.
/// Works consistently on macOS, Linux, and Windows.
pub fn file_path_to_uri(path: &Path) -> String {
    let path_str = path.to_string_lossy();
    let normalized = path_str.replace('\\', "/");

    if normalized.starts_with('/') {
        // macOS / Linux: /Users/user/... -> file:///Users/user/...
        format!("file://{}", normalized)
    } else {
        // Windows: C:/Users/user/... -> file:///C:/Users/user/...
        format!("file:///{}", normalized)
    }
}

fn url_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            let h1 = chars.next();
            let h2 = chars.next();
            if let (Some(c1), Some(c2)) = (h1, h2) {
                let hex_str = format!("{}{}", c1, c2);
                if let Ok(byte_val) = u8::from_str_radix(&hex_str, 16) {
                    result.push(byte_val as char);
                    continue;
                }
                result.push('%');
                result.push(c1);
                result.push(c2);
            } else {
                result.push('%');
                if let Some(c1) = h1 { result.push(c1); }
            }
        } else {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unix_uri_conversion() {
        let uri = "file:///Users/user/project/main.mantra";
        let path = uri_to_file_path(uri).unwrap();
        assert_eq!(path.to_string_lossy(), "/Users/user/project/main.mantra");
        let back_uri = file_path_to_uri(&path);
        assert_eq!(back_uri, uri);
    }

    #[test]
    fn test_windows_uri_decoding() {
        let uri = "file:///C%3A/Users/dev/app/src/main.mantra";
        let path = uri_to_file_path(uri).unwrap();
        assert!(path.to_string_lossy().contains("C:"));
    }

    #[test]
    fn test_spaces_in_uri() {
        let uri = "file:///Users/user/My%20Projects/main.mantra";
        let path = uri_to_file_path(uri).unwrap();
        assert_eq!(path.to_string_lossy(), "/Users/user/My Projects/main.mantra");
    }
}
