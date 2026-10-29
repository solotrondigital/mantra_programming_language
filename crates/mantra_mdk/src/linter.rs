use std::fs;
use std::path::Path;

pub struct Linter;

impl Linter {
    pub fn lint_file(path: &Path) -> Result<Vec<String>, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        let mut warnings = Vec::new();
        for (idx, line) in content.lines().enumerate() {
            let line_num = idx + 1;
            if line.len() > 120 {
                warnings.push(format!("{}:{}: warning: Line exceeds 120 characters ({})", path.display(), line_num, line.len()));
            }
            if line.ends_with(' ') || line.ends_with('\t') {
                warnings.push(format!("{}:{}: warning: Trailing whitespace detected", path.display(), line_num));
            }
            if line.contains('\t') {
                warnings.push(format!("{}:{}: warning: Tab character used; 4 spaces recommended", path.display(), line_num));
            }
        }

        Ok(warnings)
    }
}

