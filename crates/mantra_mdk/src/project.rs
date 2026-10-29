use crate::manifest::PackageManifest;
use std::fs;
use std::path::Path;

pub struct ProjectScaffolder;

impl ProjectScaffolder {
    pub fn new_app(name: &str) -> Result<(), String> {
        let root = Path::new(name);
        if root.exists() {
            return Err(format!("Directory '{}' already exists", name));
        }

        fs::create_dir_all(root.join("src"))
            .map_err(|e| format!("Failed to create src dir: {}", e))?;
        fs::create_dir_all(root.join("tests"))
            .map_err(|e| format!("Failed to create tests dir: {}", e))?;

        let manifest = PackageManifest {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            edition: "2026".to_string(),
            description: format!("Mantra application: {}", name),
            license: "MIT".to_string(),
            entry: "src/main.mantra".to_string(),
            dependencies: std::collections::HashMap::new(),
        };
        manifest.save_to_dir(root)?;

        let main_src = format!(
            r#"# Welcome to Mantra Application: {name}

class Greeter:
    def __init__(self, name: String):
        self.name = name

    pub def greet(self) -> String:
        return "Hello, " + self.name + "! Welcome to Mantra."

def main() -> Int:
    greeter = Greeter("{name}")
    print(greeter.greet())
    return 0
"#,
            name = name
        );
        fs::write(root.join("src/main.mantra"), main_src)
            .map_err(|e| format!("Failed to write src/main.mantra: {}", e))?;

        let test_src = format!(
            r#"# Tests for {name}

class TestSuite:
    def __init__(self):
        self.tests_run = 0

    pub def test_greeter(self) -> Bool:
        let expected = "Hello, Mantra! Welcome to Mantra."
        let actual = "Hello, Mantra! Welcome to Mantra."
        assert(expected == actual)
        return True

def main() -> Int:
    suite = TestSuite()
    print("Running test_greeter...")
    suite.test_greeter()
    print("All tests passed successfully!")
    return 0
"#,
            name = name
        );
        fs::write(root.join("tests/test_main.mantra"), test_src)
            .map_err(|e| format!("Failed to write tests/test_main.mantra: {}", e))?;

        let gitignore = "target/\n*.o\n*.exe\n.DS_Store\n";
        fs::write(root.join(".gitignore"), gitignore)
            .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

        let readme = format!(
            r#"# {name}

An Object-Oriented application built with Mantra.

## Development

Run development target:
```bash
mantra run
```

Run test suite:
```bash
mantra test
```

Build native executable:
```bash
mantra build --release
```
"#,
            name = name
        );
        fs::write(root.join("README.md"), readme)
            .map_err(|e| format!("Failed to write README.md: {}", e))?;

        println!("Created Mantra application '{}'", name);
        println!("To get started:\n  cd {}\n  mantra run", name);

        Ok(())
    }

    pub fn new_package(name: &str) -> Result<(), String> {
        let root = Path::new(name);
        if root.exists() {
            return Err(format!("Directory '{}' already exists", name));
        }

        fs::create_dir_all(root.join("src"))
            .map_err(|e| format!("Failed to create src dir: {}", e))?;
        fs::create_dir_all(root.join("tests"))
            .map_err(|e| format!("Failed to create tests dir: {}", e))?;

        let manifest = PackageManifest {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            edition: "2026".to_string(),
            description: format!("Mantra package library: {}", name),
            license: "MIT".to_string(),
            entry: "src/lib.mantra".to_string(),
            dependencies: std::collections::HashMap::new(),
        };
        manifest.save_to_dir(root)?;

        let lib_src = format!(
            r#"# Mantra Package: {name}

interface DataProcessor:
    def process(value: String) -> String

pub class DefaultProcessor implements DataProcessor:
    pub def process(self, value: String) -> String:
        return "Processed: " + value
"#,
            name = name
        );
        fs::write(root.join("src/lib.mantra"), lib_src)
            .map_err(|e| format!("Failed to write src/lib.mantra: {}", e))?;

        let test_src = format!(
            r#"# Tests for package {name}

def main() -> Int:
    print("Testing package {name}...")
    assert(True)
    print("Package tests passed!")
    return 0
"#,
            name = name
        );
        fs::write(root.join("tests/test_lib.mantra"), test_src)
            .map_err(|e| format!("Failed to write tests/test_lib.mantra: {}", e))?;

        let gitignore = "target/\n*.o\n*.exe\n.DS_Store\n";
        fs::write(root.join(".gitignore"), gitignore)
            .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

        println!("Created Mantra package '{}'", name);
        Ok(())
    }
}

