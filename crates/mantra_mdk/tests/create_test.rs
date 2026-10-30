use mantra_mdk::scaffold::{ArgumentParser, CreateCommand, ProjectOptionsValidator};
use std::fs;
use std::path::Path;

fn cleanup(path: &str) {
    let p = Path::new(path);
    if p.exists() {
        let _ = fs::remove_dir_all(p);
    }
}

fn assert_no_mr_files(dir: &Path) {
    if !dir.exists() {
        return;
    }
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            assert_no_mr_files(&path);
        } else if let Some(ext) = path.extension() {
            assert_ne!(
                ext, "mr",
                "Found prohibited .mr extension in file: {}",
                path.display()
            );
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Frontend Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_frontend_creation_all_platforms() {
    let project = "test_frontend_all";
    cleanup(project);

    let args = vec![
        "--type".to_string(),
        "frontend".to_string(),
        "--org".to_string(),
        "com.testapp".to_string(),
        "--project-name".to_string(),
        project.to_string(),
        "--platforms".to_string(),
        "ios,android,web".to_string(),
        "--description".to_string(),
        "Test frontend all platforms".to_string(),
    ];

    let result = CreateCommand::execute(&args);
    assert!(result.is_ok(), "Failed to create frontend project: {:?}", result);

    let root = Path::new(project);
    assert!(root.join("mantra.toml").exists());
    assert!(root.join("README.md").exists());
    assert!(root.join(".gitignore").exists());
    assert!(root.join("src/main.mantra").exists());
    assert!(root.join("src/app/app.mantra").exists());
    assert!(root.join("src/components/button.mantra").exists());
    assert!(root.join("src/components/header.mantra").exists());
    assert!(root.join("src/screens/home_screen.mantra").exists());
    assert!(root.join("src/navigation/router.mantra").exists());
    assert!(root.join("src/services/api_service.mantra").exists());
    assert!(root.join("src/state/app_state.mantra").exists());
    assert!(root.join("src/styles/theme.mantra").exists());
    assert!(root.join("assets/images").exists());
    assert!(root.join("platforms/ios").exists());
    assert!(root.join("platforms/android").exists());
    assert!(root.join("platforms/web/index.html").exists());
    assert!(root.join("platforms/web/manifest.json").exists());
    assert!(root.join("tests/app_test.mantra").exists());

    // Check manifest
    let manifest = fs::read_to_string(root.join("mantra.toml")).unwrap();
    assert!(manifest.contains("type = \"frontend\""));
    assert!(manifest.contains("entry = \"src/main.mantra\""));
    assert!(manifest.contains("ios = \"platforms/ios\""));
    assert!(manifest.contains("android = \"platforms/android\""));
    assert!(manifest.contains("web = \"platforms/web\""));

    // Check README has creation reference
    let readme = fs::read_to_string(root.join("README.md")).unwrap();
    assert!(readme.contains("## Mantra Project Creation Reference"));
    assert!(readme.contains("--type frontend"));
    assert!(readme.contains("--type backend \\\n  --service REST"));
    assert!(readme.contains("--type backend \\\n  --service GRAPHQL"));

    // Verify NO .mr files
    assert_no_mr_files(root);

    cleanup(project);
}

#[test]
fn test_frontend_creation_subset_platforms() {
    let project = "test_frontend_web_only";
    cleanup(project);

    let args = vec![
        "--type".to_string(),
        "frontend".to_string(),
        "--org".to_string(),
        "com.testapp".to_string(),
        "--project-name".to_string(),
        project.to_string(),
        "--platforms".to_string(),
        "web".to_string(),
    ];

    let result = CreateCommand::execute(&args);
    assert!(result.is_ok());

    let root = Path::new(project);
    assert!(root.join("platforms/web").exists());
    assert!(!root.join("platforms/ios").exists());
    assert!(!root.join("platforms/android").exists());

    assert_no_mr_files(root);
    cleanup(project);
}

#[test]
fn test_frontend_rejection_of_unsupported_platform() {
    let args = vec![
        "--type".to_string(),
        "frontend".to_string(),
        "--project-name".to_string(),
        "my_app".to_string(),
        "--platforms".to_string(),
        "ios,windows".to_string(),
    ];

    let raw = ArgumentParser::parse(&args).unwrap();
    let err = ProjectOptionsValidator::validate(raw).unwrap_err();
    assert!(
        err.contains("Unsupported platform 'windows'"),
        "Unexpected error: {}",
        err
    );
    assert!(err.contains("Supported platforms: ios, android, web"));
}

#[test]
fn test_frontend_rejection_of_service_argument() {
    let args = vec![
        "--type".to_string(),
        "frontend".to_string(),
        "--service".to_string(),
        "REST".to_string(),
        "--project-name".to_string(),
        "my_app".to_string(),
    ];

    let raw = ArgumentParser::parse(&args).unwrap();
    let err = ProjectOptionsValidator::validate(raw).unwrap_err();
    assert_eq!(err, "Error: --service is only supported for backend projects.");
}

// ─────────────────────────────────────────────────────────────────────────────
// Backend REST Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_backend_rest_creation() {
    let project = "test_backend_rest";
    cleanup(project);

    let args = vec![
        "--type".to_string(),
        "backend".to_string(),
        "--service".to_string(),
        "REST".to_string(),
        "--org".to_string(),
        "com.testapp".to_string(),
        "--project-name".to_string(),
        project.to_string(),
        "--description".to_string(),
        "TestApp REST API backend".to_string(),
    ];

    let result = CreateCommand::execute(&args);
    assert!(result.is_ok(), "Failed to create REST project: {:?}", result);

    let root = Path::new(project);
    assert!(root.join("mantra.toml").exists());
    assert!(root.join("README.md").exists());
    assert!(root.join(".gitignore").exists());
    assert!(root.join("src/main.mantra").exists());
    assert!(root.join("src/config/app_config.mantra").exists());
    assert!(root.join("src/routes/api_routes.mantra").exists());
    assert!(root.join("src/controllers/health_controller.mantra").exists());
    assert!(root.join("src/controllers/user_controller.mantra").exists());
    assert!(root.join("src/models/user.mantra").exists());
    assert!(root.join("src/services/user_service.mantra").exists());
    assert!(root.join("src/repositories/user_repository.mantra").exists());
    assert!(root.join("src/middleware/auth_middleware.mantra").exists());
    assert!(root.join("src/middleware/logger_middleware.mantra").exists());
    assert!(root.join("src/validators/user_validator.mantra").exists());
    assert!(root.join("src/utils/response_formatter.mantra").exists());
    assert!(root.join("tests/unit/user_service_test.mantra").exists());
    assert!(root.join("tests/integration/health_api_test.mantra").exists());
    assert!(root.join("docs/api_spec.md").exists());

    // Ensure NO frontend / mobile platform directories
    assert!(!root.join("platforms").exists());
    assert!(!root.join("src/screens").exists());
    assert!(!root.join("src/navigation").exists());

    // Check manifest
    let manifest = fs::read_to_string(root.join("mantra.toml")).unwrap();
    assert!(manifest.contains("type = \"backend\""));
    assert!(manifest.contains("service = \"REST\""));
    assert!(manifest.contains("entry = \"src/main.mantra\""));

    let readme = fs::read_to_string(root.join("README.md")).unwrap();
    assert!(readme.contains("## Mantra Project Creation Reference"));
    assert!(readme.contains("--type frontend"));
    assert!(readme.contains("--type backend \\\n  --service REST"));
    assert!(readme.contains("--type backend \\\n  --service GRAPHQL"));

    assert_no_mr_files(root);
    cleanup(project);
}

#[test]
fn test_backend_rest_requires_service() {
    let args = vec![
        "--type".to_string(),
        "backend".to_string(),
        "--project-name".to_string(),
        "my_app".to_string(),
    ];

    let raw = ArgumentParser::parse(&args).unwrap();
    let err = ProjectOptionsValidator::validate(raw).unwrap_err();
    assert_eq!(
        err,
        "Error: Missing required argument --service.\n\nAvailable services: REST, GRAPHQL."
    );
}

#[test]
fn test_backend_rejects_platforms() {
    let args = vec![
        "--type".to_string(),
        "backend".to_string(),
        "--platforms".to_string(),
        "ios,android".to_string(),
        "--project-name".to_string(),
        "my_app".to_string(),
    ];

    let raw = ArgumentParser::parse(&args).unwrap();
    let err = ProjectOptionsValidator::validate(raw).unwrap_err();
    assert_eq!(
        err,
        "Error: --platforms is only supported for frontend projects.\n\nBackend projects use --service REST or --service GRAPHQL."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Backend GraphQL Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_backend_graphql_creation() {
    let project = "test_backend_graphql";
    cleanup(project);

    let args = vec![
        "--type".to_string(),
        "backend".to_string(),
        "--service".to_string(),
        "GRAPHQL".to_string(),
        "--org".to_string(),
        "com.testapp".to_string(),
        "--project-name".to_string(),
        project.to_string(),
        "--description".to_string(),
        "TestApp GraphQL backend".to_string(),
    ];

    let result = CreateCommand::execute(&args);
    assert!(result.is_ok(), "Failed to create GraphQL project: {:?}", result);

    let root = Path::new(project);
    assert!(root.join("mantra.toml").exists());
    assert!(root.join("README.md").exists());
    assert!(root.join(".gitignore").exists());
    assert!(root.join("src/main.mantra").exists());
    assert!(root.join("src/config/app_config.mantra").exists());
    assert!(root.join("src/graphql/schema/schema.mantra").exists());
    assert!(root.join("src/graphql/resolvers/query_resolver.mantra").exists());
    assert!(root.join("src/graphql/resolvers/mutation_resolver.mantra").exists());
    assert!(root.join("src/graphql/types/user_type.mantra").exists());
    assert!(root.join("src/graphql/queries/user_queries.mantra").exists());
    assert!(root.join("src/graphql/mutations/user_mutations.mantra").exists());
    assert!(root.join("src/graphql/scalars/date_time.mantra").exists());
    assert!(root.join("src/services/user_service.mantra").exists());
    assert!(root.join("src/models/user.mantra").exists());
    assert!(root.join("src/repositories/user_repository.mantra").exists());
    assert!(root.join("src/middleware/auth_middleware.mantra").exists());
    assert!(root.join("src/middleware/logging_middleware.mantra").exists());
    assert!(root.join("src/utils/graphql_formatter.mantra").exists());
    assert!(root.join("tests/unit/resolver_test.mantra").exists());
    assert!(root.join("tests/integration/graphql_api_test.mantra").exists());
    assert!(root.join("docs/schema.graphql").exists());

    // Ensure NO REST-specific controllers or routes
    assert!(!root.join("src/controllers").exists());
    assert!(!root.join("src/routes").exists());
    assert!(!root.join("platforms").exists());

    // Check manifest
    let manifest = fs::read_to_string(root.join("mantra.toml")).unwrap();
    assert!(manifest.contains("type = \"backend\""));
    assert!(manifest.contains("service = \"GRAPHQL\""));
    assert!(manifest.contains("entry = \"src/main.mantra\""));

    assert_no_mr_files(root);
    cleanup(project);
}

// ─────────────────────────────────────────────────────────────────────────────
// General CLI Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_missing_type() {
    let args = vec!["--project-name".to_string(), "my_app".to_string()];
    let raw = ArgumentParser::parse(&args).unwrap();
    let err = ProjectOptionsValidator::validate(raw).unwrap_err();
    assert_eq!(
        err,
        "Error: Missing required argument --type.\n\nAvailable types: frontend, backend."
    );
}

#[test]
fn test_unknown_type() {
    let args = vec![
        "--type".to_string(),
        "desktop".to_string(),
        "--project-name".to_string(),
        "my_app".to_string(),
    ];
    let raw = ArgumentParser::parse(&args).unwrap();
    let err = ProjectOptionsValidator::validate(raw).unwrap_err();
    assert_eq!(
        err,
        "Error: Unknown application type 'desktop'.\n\nAvailable types: frontend, backend."
    );
}

#[test]
fn test_missing_project_name() {
    let args = vec!["--type".to_string(), "frontend".to_string()];
    let raw = ArgumentParser::parse(&args).unwrap();
    let err = ProjectOptionsValidator::validate(raw).unwrap_err();
    assert_eq!(err, "Error: Missing required argument --project-name.");
}

#[test]
fn test_invalid_project_name() {
    let args = vec![
        "--type".to_string(),
        "frontend".to_string(),
        "--project-name".to_string(),
        "Invalid-Name".to_string(),
    ];
    let raw = ArgumentParser::parse(&args).unwrap();
    let err = ProjectOptionsValidator::validate(raw).unwrap_err();
    assert!(err.contains("is not a valid Mantra project name"));
}

#[test]
fn test_case_insensitive_service() {
    let args = vec![
        "--type".to_string(),
        "backend".to_string(),
        "--service".to_string(),
        "graphql".to_string(),
        "--project-name".to_string(),
        "graphql_app".to_string(),
    ];
    let raw = ArgumentParser::parse(&args).unwrap();
    let opts = ProjectOptionsValidator::validate(raw).unwrap();
    assert_eq!(
        opts.service.unwrap(),
        mantra_mdk::scaffold::BackendService::Graphql
    );
}

#[test]
fn test_overwrite_prevention() {
    let project = "test_existing_dir";
    cleanup(project);
    fs::create_dir_all(project).unwrap();

    let args = vec![
        "--type".to_string(),
        "frontend".to_string(),
        "--project-name".to_string(),
        project.to_string(),
    ];

    let raw = ArgumentParser::parse(&args).unwrap();
    let err = ProjectOptionsValidator::validate(raw).unwrap_err();
    assert!(err.contains("already exists. Aborting to prevent accidental overwrite."));

    cleanup(project);
}

#[test]
fn test_mobile_build_apk_and_appbundle() {
    let project = "test_mobile_build";
    cleanup(project);

    let args = vec![
        "--type".to_string(),
        "frontend".to_string(),
        "--org".to_string(),
        "com.testapp".to_string(),
        "--project-name".to_string(),
        project.to_string(),
        "--platforms".to_string(),
        "android,web".to_string(),
    ];

    let result = CreateCommand::execute(&args);
    assert!(result.is_ok());

    let root = Path::new(project);

    // 1. Test build APK release
    let apk_res = mantra_mdk::MobileBuilder::build_apk(root, true, None);
    assert!(apk_res.is_ok(), "build_apk failed: {:?}", apk_res);

    let apk_path = root.join("target/android/app/build/outputs/apk/release/app-release-unsigned.apk");
    assert!(apk_path.exists(), "APK not found at {}", apk_path.display());
    let apk_bytes = fs::read(&apk_path).unwrap();
    assert!(!apk_bytes.is_empty());
    // Verify PK header
    assert_eq!(&apk_bytes[0..2], b"PK");

    // 2. Test build AppBundle release
    let aab_res = mantra_mdk::MobileBuilder::build_appbundle(root, true, None);
    assert!(aab_res.is_ok(), "build_appbundle failed: {:?}", aab_res);

    let aab_path = root.join("target/android/app/build/outputs/bundle/release/app-release.aab");
    assert!(aab_path.exists(), "AAB not found at {}", aab_path.display());

    let aab_mirror = root.join("target/android/app/build/outputs/apk/release/app-release.aab");
    assert!(aab_mirror.exists(), "AAB mirror not found at {}", aab_mirror.display());

    cleanup(project);
}


