use super::frontend::FrontendTemplateGenerator;
use super::fs_service::FilesystemService;
use super::graphql::GraphQLTemplateGenerator;
use super::metadata::ProjectMetadataGenerator;
use super::rest::RestTemplateGenerator;
use super::validator::{AppType, BackendService, ValidatedOptions};

pub struct GenerationSummary {
    pub project_name: String,
    pub app_type: AppType,
    pub service: Option<BackendService>,
    pub platforms: Vec<String>,
    pub directory: String,
}

pub struct ProjectGenerator;

impl ProjectGenerator {
    pub fn generate(opts: &ValidatedOptions) -> Result<GenerationSummary, String> {
        let root = &opts.directory;

        // 1. Create root directory
        FilesystemService::create_dir_all(root)?;

        // 2. Generate Metadata (mantra.toml, .gitignore, README.md)
        let manifest_content = ProjectMetadataGenerator::generate_manifest(opts);
        FilesystemService::write_file(&root.join("mantra.toml"), &manifest_content)?;

        let gitignore_content = ProjectMetadataGenerator::generate_gitignore();
        FilesystemService::write_file(&root.join(".gitignore"), gitignore_content)?;

        let readme_content = ProjectMetadataGenerator::generate_readme(opts);
        FilesystemService::write_file(&root.join("README.md"), &readme_content)?;

        // 3. Dispatch to specific template generator
        match opts.app_type {
            AppType::Frontend => {
                FrontendTemplateGenerator::generate(root, opts)?;
            }
            AppType::Backend => match opts.service.unwrap() {
                BackendService::Rest => {
                    RestTemplateGenerator::generate(root, opts)?;
                }
                BackendService::Graphql => {
                    GraphQLTemplateGenerator::generate(root, opts)?;
                }
            },
        }

        Ok(GenerationSummary {
            project_name: opts.project_name.clone(),
            app_type: opts.app_type,
            service: opts.service,
            platforms: opts.platforms.clone(),
            directory: format!("./{}", opts.project_name),
        })
    }
}
