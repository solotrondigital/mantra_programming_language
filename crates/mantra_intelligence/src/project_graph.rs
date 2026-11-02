//! Project Understanding and Architecture Graph
//!
//! Indexes multi-crate workspaces and project directories, analyzes module dependencies,
//! and verifies architectural conventions across Mantra applications.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectModule {
    pub name: String,
    pub file_path: String,
    pub classes: Vec<String>,
    pub functions: Vec<String>,
    pub imports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectGraph {
    pub project_name: String,
    pub project_type: String,
    pub root_dir: String,
    pub modules: HashMap<String, ProjectModule>,
    pub dependencies: Vec<String>,
}

impl ProjectGraph {
    /// Discovers and indexes all Mantra source files and manifest configuration in the project.
    pub fn index(project_dir: &Path) -> Result<Self, String> {
        let manifest_path = project_dir.join("mantra.toml");
        let alt_manifest = project_dir.join("project.toml");

        let mut project_name = "mantra_project".to_string();
        let mut project_type = "standard".to_string();
        let mut dependencies = Vec::new();

        let manifest_content = fs::read_to_string(&manifest_path)
            .or_else(|_| fs::read_to_string(&alt_manifest))
            .unwrap_or_default();

        let mut in_project = false;
        let mut in_deps = false;

        for line in manifest_content.lines() {
            let t = line.trim();
            if t == "[project]" || t == "[package]" {
                in_project = true;
                in_deps = false;
                continue;
            }
            if t == "[dependencies]" {
                in_deps = true;
                in_project = false;
                continue;
            }
            if t.starts_with('[') {
                in_project = false;
                in_deps = false;
            }

            if in_project {
                if let Some((k, v)) = t.split_once('=') {
                    if k.trim() == "name" {
                        project_name = v.trim().trim_matches('"').to_string();
                    } else if k.trim() == "type" {
                        project_type = v.trim().trim_matches('"').to_string();
                    }
                }
            } else if in_deps {
                if let Some((k, _)) = t.split_once('=') {
                    dependencies.push(k.trim().trim_matches('"').to_string());
                }
            }
        }

        // Recursively index .mantra files
        let mut modules = HashMap::new();
        let mut files = Vec::new();

        fn scan_dir(dir: &Path, files: &mut Vec<PathBuf>) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if name.starts_with('.')
                        || name == "target"
                        || name == "build"
                        || name == "dist"
                    {
                        continue;
                    }
                    if path.is_dir() {
                        scan_dir(&path, files);
                    } else if path.extension().and_then(|e| e.to_str()) == Some("mantra") {
                        files.push(path);
                    }
                }
            }
        }

        let search_dirs = [
            project_dir.join("src"),
            project_dir.join("lib"),
            project_dir.join("tests"),
        ];
        for d in &search_dirs {
            if d.exists() {
                scan_dir(d, &mut files);
            }
        }

        for f in files {
            if let Ok(content) = fs::read_to_string(&f) {
                let mod_name = f
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("module")
                    .to_string();
                let mut classes = Vec::new();
                let mut functions = Vec::new();
                let mut imports = Vec::new();

                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("class ") {
                        let cname = trimmed
                            .split_whitespace()
                            .nth(1)
                            .unwrap_or("")
                            .trim_end_matches(':');
                        classes.push(cname.to_string());
                    } else if trimmed.starts_with("def ") && !trimmed.contains("def __") {
                        let fname = trimmed
                            .split_whitespace()
                            .nth(1)
                            .unwrap_or("")
                            .split('(')
                            .next()
                            .unwrap_or("");
                        functions.push(fname.to_string());
                    } else if trimmed.starts_with("from ") || trimmed.starts_with("import ") {
                        imports.push(trimmed.to_string());
                    }
                }

                modules.insert(
                    mod_name.clone(),
                    ProjectModule {
                        name: mod_name,
                        file_path: f.to_string_lossy().to_string(),
                        classes,
                        functions,
                        imports,
                    },
                );
            }
        }

        Ok(Self {
            project_name,
            project_type,
            root_dir: project_dir.to_string_lossy().to_string(),
            modules,
            dependencies,
        })
    }
}
