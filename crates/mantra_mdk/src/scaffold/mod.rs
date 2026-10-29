pub mod frontend;
pub mod fs_service;
pub mod generator;
pub mod graphql;
pub mod metadata;
pub mod rest;
pub mod validator;

pub use frontend::FrontendTemplateGenerator;
pub use fs_service::FilesystemService;
pub use generator::{GenerationSummary, ProjectGenerator};
pub use graphql::GraphQLTemplateGenerator;
pub use metadata::ProjectMetadataGenerator;
pub use rest::RestTemplateGenerator;
pub use validator::{AppType, BackendService, ProjectOptionsValidator, RawProjectOptions, ValidatedOptions};

/// Argument parser for the `mantra create` command.
pub struct ArgumentParser;

impl ArgumentParser {
    pub fn parse(args: &[String]) -> Result<RawProjectOptions, String> {
        let mut raw = RawProjectOptions::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--type" {
                i += 1;
                if i < args.len() {
                    raw.app_type = Some(args[i].clone());
                } else {
                    return Err("Error: --type requires a value (frontend or backend).".to_string());
                }
            } else if let Some(val) = arg.strip_prefix("--type=") {
                raw.app_type = Some(val.to_string());
            } else if arg == "--service" {
                i += 1;
                if i < args.len() {
                    raw.service = Some(args[i].clone());
                } else {
                    return Err("Error: --service requires a value (REST or GRAPHQL).".to_string());
                }
            } else if let Some(val) = arg.strip_prefix("--service=") {
                raw.service = Some(val.to_string());
            } else if arg == "--org" {
                i += 1;
                if i < args.len() {
                    raw.org = Some(args[i].clone());
                } else {
                    return Err("Error: --org requires a domain value (e.g. com.example).".to_string());
                }
            } else if let Some(val) = arg.strip_prefix("--org=") {
                raw.org = Some(val.to_string());
            } else if arg == "--project-name" {
                i += 1;
                if i < args.len() {
                    raw.project_name = Some(args[i].clone());
                } else {
                    return Err("Error: --project-name requires a name value.".to_string());
                }
            } else if let Some(val) = arg.strip_prefix("--project-name=") {
                raw.project_name = Some(val.to_string());
            } else if arg == "--platforms" {
                i += 1;
                if i < args.len() {
                    let parts: Vec<String> = args[i]
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    raw.platforms = Some(parts);
                } else {
                    return Err("Error: --platforms requires a comma-separated list (e.g. ios,android,web).".to_string());
                }
            } else if let Some(val) = arg.strip_prefix("--platforms=") {
                let unquoted = val.trim_matches('"').trim_matches('\'');
                let parts: Vec<String> = unquoted
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                raw.platforms = Some(parts);
            } else if arg == "--description" {
                i += 1;
                if i < args.len() {
                    raw.description = Some(args[i].clone());
                } else {
                    return Err("Error: --description requires a description text.".to_string());
                }
            } else if let Some(val) = arg.strip_prefix("--description=") {
                let unquoted = val.trim_matches('"').trim_matches('\'');
                raw.description = Some(unquoted.to_string());
            } else if !arg.starts_with('-') {
                // Positional argument can serve as project_name if not already set
                if raw.project_name.is_none() {
                    raw.project_name = Some(arg.clone());
                } else if raw.directory.is_none() {
                    raw.directory = Some(arg.clone());
                }
            } else {
                return Err(format!("Error: Unknown option '{}' for 'mantra create'.", arg));
            }

            i += 1;
        }

        Ok(raw)
    }
}

/// Primary entry point for the `mantra create` command.
pub struct CreateCommand;

impl CreateCommand {
    pub fn execute(args: &[String]) -> Result<(), String> {
        let raw = ArgumentParser::parse(args)?;
        let validated = ProjectOptionsValidator::validate(raw)?;
        let summary = ProjectGenerator::generate(&validated)?;

        println!("Mantra project created successfully!\n");
        println!("Project: {}", summary.project_name);
        match summary.app_type {
            AppType::Frontend => {
                println!("Type: frontend");
                println!("Platforms: {}", summary.platforms.join(", "));
            }
            AppType::Backend => {
                println!("Type: backend");
                if let Some(service) = summary.service {
                    println!("Service: {}", service.as_str());
                }
            }
        }
        println!("Location: {}\n", summary.directory);
        println!("Next steps:");
        println!("  cd {}", summary.project_name);
        println!("  mantra build");
        println!("  mantra run");

        Ok(())
    }
}

