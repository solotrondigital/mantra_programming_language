// ==============================================================================
// mantra_mdk::ide_installer — IDE Integration Installer (mantra ide install)
// ==============================================================================

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub struct IdeInstaller;

impl IdeInstaller {
    pub fn handle_command(args: &[String]) {
        if args.is_empty() {
            Self::print_help();
            return;
        }

        match args[0].as_str() {
            "install" => {
                if args.len() < 2 {
                    eprintln!("Usage: mantra ide install <vscode|intellij|neovim|sublime|emacs>");
                    return;
                }
                match args[1].to_lowercase().as_str() {
                    "vscode" => Self::install_vscode(),
                    "intellij" | "android-studio" | "clion" | "rustrover" => Self::install_intellij(),
                    "neovim" | "nvim" => Self::install_neovim(),
                    "sublime" => Self::install_sublime(),
                    "emacs" => Self::install_emacs(),
                    other => {
                        eprintln!("Unknown IDE '{}'. Supported: vscode, intellij, neovim, sublime, emacs", other);
                    }
                }
            }
            "list" => {
                println!("Supported Mantra IDE integrations:");
                println!("  vscode          Visual Studio Code (Syntax, Themes, LSP, Debugger)");
                println!("  intellij        IntelliJ IDEA & Android Studio (PSI, LSP)");
                println!("  neovim          Neovim (nvim-lspconfig / native vim.lsp)");
                println!("  sublime         Sublime Text 3/4 (SublimeLSP)");
                println!("  emacs           GNU Emacs (Eglot / mantra-mode)");
            }
            "-h" | "--help" => Self::print_help(),
            other => {
                eprintln!("Unknown IDE command '{}'. Run 'mantra ide --help' for details.", other);
            }
        }
    }

    fn print_help() {
        println!(
            r#"Mantra IDE Integration Installer
Usage:
    mantra ide install <IDE>     Install integration files for specified IDE
    mantra ide list              List all supported IDEs and editors

Supported Targets:
    vscode         Visual Studio Code Extension
    intellij       IntelliJ IDEA, Android Studio, CLion, RustRover
    neovim         Neovim configuration
    sublime        Sublime Text 3 & 4
    emacs          GNU Emacs major mode (mantra-mode)
"#
        );
    }

    fn find_tooling_dir() -> Option<PathBuf> {
        let cwd = env::current_dir().ok()?;
        let candidates = [
            cwd.join("tooling"),
            cwd.join("mantra_programming_language/tooling"),
            PathBuf::from("/usr/local/share/mantra/tooling"),
            dirs_home().map(|h| h.join(".mantra/tooling")).unwrap_or_default(),
        ];
        for c in candidates {
            if c.exists() && c.is_dir() {
                return Some(c);
            }
        }
        None
    }

    fn install_vscode() {
        println!("Installing Mantra VS Code Extensions (Language Support & CLI)...");
        let home = match dirs_home() {
            Some(h) => h,
            None => {
                eprintln!("Could not determine home directory.");
                return;
            }
        };

        let mut ext_roots = vec![
            home.join(".vscode/extensions"),
        ];

        let antigravity_ide = home.join(".antigravity-ide/extensions");
        if antigravity_ide.exists() {
            ext_roots.push(antigravity_ide);
        }

        let antigravity = home.join(".antigravity/extensions");
        if antigravity.exists() {
            ext_roots.push(antigravity);
        }

        if let Some(tooling) = Self::find_tooling_dir() {
            let vscode_lang_src = tooling.join("ide/vscode");
            let vscode_cli_src = tooling.join("ide/vscode-cli");
            let grammars_src = tooling.join("grammars");
            let themes_src = tooling.join("themes");

            for root in &ext_roots {
                // 1. Install Mantra Language Support Extension
                let lang_dir = root.join("mantra-lang.mantra-language-support-0.5.0");
                if let Ok(_) = fs::create_dir_all(&lang_dir) {
                    let _ = copy_file_if_exists(&vscode_lang_src.join("package.json"), &lang_dir.join("package.json"));
                    let _ = copy_file_if_exists(&vscode_lang_src.join("language-configuration.json"), &lang_dir.join("language-configuration.json"));
                    let _ = copy_file_if_exists(&vscode_lang_src.join("README.md"), &lang_dir.join("README.md"));
                    let _ = copy_file_if_exists(&vscode_lang_src.join("CHANGELOG.md"), &lang_dir.join("CHANGELOG.md"));
                    let _ = copy_file_if_exists(&vscode_lang_src.join("LICENSE"), &lang_dir.join("LICENSE"));

                    // Assets & themes
                    let _ = copy_dir_all(&vscode_lang_src.join("images"), &lang_dir.join("images"));
                    let _ = copy_dir_all(&vscode_lang_src.join("grammars"), &lang_dir.join("grammars"));
                    let _ = copy_dir_all(&vscode_lang_src.join("themes"), &lang_dir.join("themes"));
                    let _ = copy_file_if_exists(&grammars_src.join("mantra.tmLanguage.json"), &lang_dir.join("grammars/mantra.tmLanguage.json"));
                    let _ = copy_file_if_exists(&themes_src.join("mantra-dark-theme.json"), &lang_dir.join("themes/mantra-dark-theme.json"));
                    let _ = copy_file_if_exists(&themes_src.join("mantra-light-theme.json"), &lang_dir.join("themes/mantra-light-theme.json"));

                    // Compiled code & dependencies
                    let _ = copy_dir_all(&vscode_lang_src.join("out"), &lang_dir.join("out"));
                    let _ = copy_dir_all(&vscode_lang_src.join("node_modules"), &lang_dir.join("node_modules"));
                    println!("✓ Installed Mantra Language Support to {}", lang_dir.display());
                }

                // 2. Install Mantra CLI Extension
                if vscode_cli_src.exists() {
                    let cli_dir = root.join("mantra-lang.mantra-cli-0.6.0");
                    if let Ok(_) = fs::create_dir_all(&cli_dir) {
                        let _ = copy_file_if_exists(&vscode_cli_src.join("package.json"), &cli_dir.join("package.json"));
                        let _ = copy_file_if_exists(&vscode_cli_src.join("README.md"), &cli_dir.join("README.md"));
                        let _ = copy_file_if_exists(&vscode_cli_src.join("CHANGELOG.md"), &cli_dir.join("CHANGELOG.md"));
                        let _ = copy_file_if_exists(&vscode_cli_src.join("LICENSE"), &cli_dir.join("LICENSE"));
                        let _ = copy_dir_all(&vscode_cli_src.join("images"), &cli_dir.join("images"));
                        let _ = copy_dir_all(&vscode_cli_src.join("out"), &cli_dir.join("out"));
                        println!("✓ Installed Mantra CLI Extension to {}", cli_dir.display());
                    }
                }
            }
            println!("  Restart VS Code or run 'Developer: Reload Window' to activate.");
        } else {
            eprintln!("Error: tooling directory not found. Please run within Mantra project or ensure tooling is installed.");
        }
    }

    fn install_neovim() {
        println!("Installing Mantra Neovim configuration...");
        let home = match dirs_home() {
            Some(h) => h,
            None => return,
        };

        let nvim_lua = home.join(".config/nvim/lua");
        let _ = fs::create_dir_all(&nvim_lua);
        let target_file = nvim_lua.join("mantra.lua");

        if let Some(tooling) = Self::find_tooling_dir() {
            let src = tooling.join("ide/neovim/mantra.lua");
            if src.exists() {
                if let Ok(_) = fs::copy(&src, &target_file) {
                    println!("✓ Copied mantra.lua to {}", target_file.display());
                    println!("  Add `require('mantra').setup()` to your init.lua.");
                    return;
                }
            }
        }

        println!("✓ Neovim directory prepared at {}", nvim_lua.display());
        println!("  Ensure `mantra-lsp` is on your PATH and require 'mantra' in your Neovim config.");
    }

    fn install_sublime() {
        println!("Installing Mantra Sublime Text package...");
        let home = match dirs_home() {
            Some(h) => h,
            None => return,
        };

        #[cfg(target_os = "macos")]
        let packages_dir = home.join("Library/Application Support/Sublime Text/Packages/Mantra");
        #[cfg(target_os = "linux")]
        let packages_dir = home.join(".config/sublime-text/Packages/Mantra");
        #[cfg(target_os = "windows")]
        let packages_dir = home.join("AppData/Roaming/Sublime Text/Packages/Mantra");
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        let packages_dir = home.join(".config/sublime-text/Packages/Mantra");

        let _ = fs::create_dir_all(&packages_dir);

        if let Some(tooling) = Self::find_tooling_dir() {
            let sublime_src = tooling.join("ide/sublime");
            let _ = copy_file_if_exists(&sublime_src.join("Mantra.sublime-syntax"), &packages_dir.join("Mantra.sublime-syntax"));
            let _ = copy_file_if_exists(&sublime_src.join("Mantra.sublime-settings"), &packages_dir.join("Mantra.sublime-settings"));
        }

        println!("✓ Mantra Sublime package installed to {}", packages_dir.display());
    }

    fn install_emacs() {
        println!("Installing Mantra Emacs package...");
        let home = match dirs_home() {
            Some(h) => h,
            None => return,
        };

        let lisp_dir = home.join(".emacs.d/lisp");
        let _ = fs::create_dir_all(&lisp_dir);
        let target_file = lisp_dir.join("mantra-mode.el");

        if let Some(tooling) = Self::find_tooling_dir() {
            let src = tooling.join("ide/emacs/mantra-mode.el");
            if src.exists() {
                let _ = fs::copy(&src, &target_file);
            }
        }

        println!("✓ Installed mantra-mode.el to {}", target_file.display());
        println!("  Add `(require 'mantra-mode)` and `(add-hook 'mantra-mode-hook #'eglot-ensure)` to your init.el.");
    }

    fn install_intellij() {
        println!("Mantra IntelliJ & Android Studio Integration:");
        println!("  To install the Mantra JetBrains plugin:");
        println!("  1. Build the plugin: cd tooling/ide/intellij && ./gradlew buildPlugin");
        println!("  2. Open IDE > Settings > Plugins > ⚙️ > Install Plugin from Disk...");
        println!("  3. Select the built ZIP from tooling/ide/intellij/build/distributions/");
        println!("  4. Restart your IDE.");
    }
}

fn dirs_home() -> Option<PathBuf> {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .ok()
        .map(PathBuf::from)
}

fn copy_file_if_exists(from: &Path, to: &Path) -> std::io::Result<()> {
    if from.exists() {
        fs::copy(from, to)?;
    }
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !src.exists() {
        return Ok(());
    }
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

