use std::fs;
use std::path::Path;

/// Filesystem service responsible for safe directory and file operations.
pub struct FilesystemService;

impl FilesystemService {
    /// Creates a directory and any parent directories if they don't exist.
    pub fn create_dir_all(path: &Path) -> Result<(), String> {
        fs::create_dir_all(path)
            .map_err(|e| format!("Failed to create directory '{}': {}", path.display(), e))
    }

    /// Safely writes content to a file, returning an error if writing fails.
    pub fn write_file(path: &Path, content: &str) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                Self::create_dir_all(parent)?;
            }
        }
        fs::write(path, content)
            .map_err(|e| format!("Failed to write file '{}': {}", path.display(), e))
    }

    /// Checks if a directory or file exists.
    pub fn exists(path: &Path) -> bool {
        path.exists()
    }
}

