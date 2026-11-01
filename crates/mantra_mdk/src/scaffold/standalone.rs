use super::fs_service::FilesystemService;
use super::validator::ValidatedOptions;
use std::path::Path;

pub struct StandaloneTemplateGenerator;

impl StandaloneTemplateGenerator {
    pub fn generate(root: &Path, opts: &ValidatedOptions) -> Result<(), String> {
        let name = &opts.project_name;

        // 1. Directory Structure
        FilesystemService::create_dir_all(&root.join("src"))?;
        FilesystemService::create_dir_all(&root.join("tests"))?;

        // 2. src/main.mantra
        let main_src = format!(
            r#"# {name} — Standalone Application Entry Point

class App:
    pub name: String
    pub version: String

    def __init__(self, name: String, version: String):
        self.name = name
        self.version = version

    pub def run(self) -> Int:
        print("==================================================================")
        print("          Running " + self.name + " v" + self.version)
        print("==================================================================")
        print("Hello from Mantra standalone application!")
        return 0

def main() -> Int:
    let app = App("{name}", "0.1.0")
    return app.run()
"#,
            name = name
        );
        FilesystemService::write_file(&root.join("src/main.mantra"), &main_src)?;

        // 3. tests/app_test.mantra
        let test_src = format!(
            r#"# Test suite for {name}

def test_app_initialization() -> Bool:
    print("[TEST] Verifying {name} standalone integrity...")
    return True

def main() -> Int:
    assert(test_app_initialization() == True)
    print("All standalone tests passed!")
    return 0
"#,
            name = name
        );
        FilesystemService::write_file(&root.join("tests/app_test.mantra"), &test_src)?;

        Ok(())
    }
}

