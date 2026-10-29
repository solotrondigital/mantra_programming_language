use std::path::PathBuf;

/// Supported application types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppType {
    Frontend,
    Backend,
}

/// Supported backend services.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendService {
    Rest,
    Graphql,
}

impl BackendService {
    pub fn as_str(&self) -> &'static str {
        match self {
            BackendService::Rest => "REST",
            BackendService::Graphql => "GRAPHQL",
        }
    }
}

/// Raw parsed arguments before validation.
#[derive(Debug, Default, Clone)]
pub struct RawProjectOptions {
    pub app_type: Option<String>,
    pub service: Option<String>,
    pub org: Option<String>,
    pub project_name: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub description: Option<String>,
    pub directory: Option<String>,
}

/// Fully validated project creation configuration.
#[derive(Debug, Clone)]
pub struct ValidatedOptions {
    pub app_type: AppType,
    pub service: Option<BackendService>,
    pub org: String,
    pub project_name: String,
    pub platforms: Vec<String>,
    pub description: String,
    pub directory: PathBuf,
}

pub struct ProjectOptionsValidator;

impl ProjectOptionsValidator {
    /// Validates raw options against the strict conditional rules.
    pub fn validate(raw: RawProjectOptions) -> Result<ValidatedOptions, String> {
        // 1. Validate --type (Required: frontend | backend)
        let app_type_raw = match raw.app_type {
            Some(t) if !t.trim().is_empty() => t.trim().to_lowercase(),
            _ => {
                return Err(
                    "Error: Missing required argument --type.\n\nAvailable types: frontend, backend."
                        .to_string(),
                );
            }
        };

        let app_type = match app_type_raw.as_str() {
            "frontend" => AppType::Frontend,
            "backend" => AppType::Backend,
            other => {
                return Err(format!(
                    "Error: Unknown application type '{}'.\n\nAvailable types: frontend, backend.",
                    other
                ));
            }
        };

        // 2. Validate --project-name (Required)
        let project_name = match raw.project_name {
            Some(name) if !name.trim().is_empty() => name.trim().to_string(),
            _ => {
                return Err("Error: Missing required argument --project-name.".to_string());
            }
        };

        crate::project::validate_project_name(&project_name)
            .map_err(|e| format!("Error: {}", e))?;

        // 3. Validate Organization Identifier (Default: com.example)
        let org = match raw.org {
            Some(o) if !o.trim().is_empty() => {
                let trimmed = o.trim().to_string();
                if !trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-')
                    || !trimmed.contains('.')
                {
                    return Err(format!(
                        "Error: \"{}\" is not a valid organization domain identifier (e.g. com.example).",
                        trimmed
                    ));
                }
                trimmed
            }
            _ => "com.example".to_string(),
        };

        // 4. Type-specific validations
        let mut validated_service = None;
        let mut validated_platforms = Vec::new();

        match app_type {
            AppType::Frontend => {
                // Reject --service on frontend
                if raw.service.is_some() {
                    return Err("Error: --service is only supported for backend projects.".to_string());
                }

                // Validate platforms if provided, otherwise default to [ios, android, web]
                if let Some(plats) = raw.platforms {
                    if plats.is_empty() {
                        validated_platforms = vec!["ios".to_string(), "android".to_string(), "web".to_string()];
                    } else {
                        for plat in &plats {
                            let normalized = plat.trim().to_lowercase();
                            match normalized.as_str() {
                                "ios" | "android" | "web" => {
                                    if !validated_platforms.contains(&normalized) {
                                        validated_platforms.push(normalized);
                                    }
                                }
                                other => {
                                    return Err(format!(
                                        "Error: Unsupported platform '{}'.\n\nSupported platforms: ios, android, web.",
                                        other
                                    ));
                                }
                            }
                        }
                    }
                } else {
                    validated_platforms = vec!["ios".to_string(), "android".to_string(), "web".to_string()];
                }
            }
            AppType::Backend => {
                // Reject --platforms on backend
                if raw.platforms.is_some() {
                    return Err(
                        "Error: --platforms is only supported for frontend projects.\n\nBackend projects use --service REST or --service GRAPHQL."
                            .to_string(),
                    );
                }

                // Require --service on backend
                let service_raw = match raw.service {
                    Some(s) if !s.trim().is_empty() => s.trim().to_uppercase(),
                    _ => {
                        return Err(
                            "Error: Missing required argument --service.\n\nAvailable services: REST, GRAPHQL."
                                .to_string(),
                        );
                    }
                };

                let service = match service_raw.as_str() {
                    "REST" => BackendService::Rest,
                    "GRAPHQL" => BackendService::Graphql,
                    other => {
                        return Err(format!(
                            "Error: Unknown backend service '{}'.\n\nAvailable services: REST, GRAPHQL.",
                            other
                        ));
                    }
                };
                validated_service = Some(service);
            }
        }

        // 5. Target Directory & Overwrite Prevention
        let dir_str = raw.directory.unwrap_or_else(|| project_name.clone());
        let directory = PathBuf::from(&dir_str);

        if directory.exists() {
            return Err(format!(
                "Error: Target directory '{}' already exists. Aborting to prevent accidental overwrite.",
                directory.display()
            ));
        }

        // 6. Description
        let description = raw.description.unwrap_or_else(|| match app_type {
            AppType::Frontend => format!("Mantra cross-platform hybrid application: {}", project_name),
            AppType::Backend => match validated_service.unwrap() {
                BackendService::Rest => format!("Mantra REST API backend application: {}", project_name),
                BackendService::Graphql => format!("Mantra GraphQL API backend application: {}", project_name),
            },
        });

        Ok(ValidatedOptions {
            app_type,
            service: validated_service,
            org,
            project_name,
            platforms: validated_platforms,
            description,
            directory,
        })
    }
}
