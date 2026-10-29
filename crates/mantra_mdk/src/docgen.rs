use mantrac_ast::*;
use mantrac_lexer::Lexer;
use mantrac_parser::Parser;
use std::fs;
use std::path::Path;

pub struct DocGenerator;

impl DocGenerator {
    pub fn generate_docs_for_file(path: &Path) -> Result<String, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        let mut lexer = Lexer::new(&content);
        let tokens = lexer.tokenize().map_err(|e| e.message)?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse().map_err(|e| e.message)?;

        let mut doc = String::new();
        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        doc.push_str(&format!("# Documentation for `{}`\n\n", filename));

        for decl in &program.declarations {
            match decl {
                Declaration::Class(cls) => {
                    doc.push_str(&format!("## Class `{}`\n\n", cls.name));
                    if let Some(ref base) = cls.base_class {
                        doc.push_str(&format!("*Extends:* `{}`  \n", base));
                    }
                    if !cls.interfaces.is_empty() {
                        doc.push_str(&format!("*Implements:* `{}`  \n", cls.interfaces.join(", ")));
                    }
                    doc.push('\n');

                    for member in &cls.members {
                        match member {
                            ClassMember::Method(m) => {
                                let params: Vec<String> = m.params.iter().map(|p| p.name.clone()).collect();
                                doc.push_str(&format!("- `def {}({})`", m.name, params.join(", ")));
                                if let Some(ref ret) = m.return_type {
                                    doc.push_str(&format!(" -> `{}`", ret));
                                }
                                doc.push_str("\n\n");
                            }
                            ClassMember::StaticMethod(sm) => {
                                let params: Vec<String> = sm.params.iter().map(|p| p.name.clone()).collect();
                                doc.push_str(&format!("- *static* `def {}({})`\n\n", sm.name, params.join(", ")));
                            }
                            ClassMember::Field(f) => {
                                doc.push_str(&format!("- *field* `{}`\n\n", f.name));
                            }
                            _ => {}
                        }
                    }
                }
                Declaration::Interface(iface) => {
                    doc.push_str(&format!("## Interface `{}`\n\n", iface.name));
                    for m in &iface.methods {
                        let params: Vec<String> = m.params.iter().map(|p| p.name.clone()).collect();
                        doc.push_str(&format!("- `def {}({}) -> {}`\n\n", m.name, params.join(", "), m.return_type));
                    }
                }
                Declaration::Function(f) => {
                    let params: Vec<String> = f.params.iter().map(|p| p.name.clone()).collect();
                    doc.push_str(&format!("### Function `{}({})`\n\n", f.name, params.join(", ")));
                }
                _ => {}
            }
        }

        Ok(doc)
    }
}

