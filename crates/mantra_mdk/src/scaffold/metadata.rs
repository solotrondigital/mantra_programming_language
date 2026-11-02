use super::validator::{AppType, BackendService, ValidatedOptions};

pub struct ProjectMetadataGenerator;

impl ProjectMetadataGenerator {
    /// Generates the standard mantra.toml manifest.
    pub fn generate_manifest(opts: &ValidatedOptions) -> String {
        let type_str = match opts.app_type {
            AppType::Frontend => "frontend",
            AppType::Backend => "backend",
            AppType::Standalone => "standalone",
            AppType::Ai => "ai",
        };

        let mut manifest = format!(
            r#"[project]
name = "{name}"
org = "{org}"
version = "0.1.0"
type = "{app_type}"
"#,
            name = opts.project_name,
            org = opts.org,
            app_type = type_str
        );

        if let Some(service) = opts.service {
            manifest.push_str(&format!("service = \"{}\"\n", service.as_str()));
        }

        manifest.push_str(&format!(
            "description = \"{}\"\nentry = \"src/main.mantra\"\n\n",
            opts.description
        ));

        if opts.app_type == AppType::Frontend {
            manifest.push_str("[platforms]\n");
            for p in &opts.platforms {
                manifest.push_str(&format!("{} = \"platforms/{}\"\n", p, p));
            }
            manifest.push('\n');
        }

        manifest.push_str("[dependencies]\n");
        let runtime_version = env!("CARGO_PKG_VERSION");
        match opts.app_type {
            AppType::Frontend => {
                manifest.push_str(&format!("\"mantra.web\" = \"{runtime_version}\"\n"));
                manifest.push_str(&format!("\"mantra.mobile\" = \"{runtime_version}\"\n"));
                manifest.push_str(&format!("\"mantra.widgets\" = \"{runtime_version}\"\n"));
            }
            AppType::Backend => match opts.service {
                Some(BackendService::Rest) => {
                    manifest.push_str(&format!("\"mantra.rest\" = \"{runtime_version}\"\n"));
                    manifest.push_str(&format!("\"mantra.sql_orm\" = \"{runtime_version}\"\n"));
                }
                Some(BackendService::Graphql) => {
                    manifest.push_str(&format!("\"mantra.graphql\" = \"{runtime_version}\"\n"));
                    manifest.push_str(&format!("\"mantra.sql_orm\" = \"{runtime_version}\"\n"));
                }
                None => {
                    manifest.push_str(&format!("\"mantra.rest\" = \"{runtime_version}\"\n"));
                }
            },
            // `mantra.core` is provided by the compiler/runtime itself, not
            // downloaded as a registry package, so it must not be locked as an
            // external dependency with an archive checksum.
            AppType::Standalone => {}
            AppType::Ai => {
                manifest.push_str(&format!("\"mantra.ai\" = \"{runtime_version}\"\n\n"));
                manifest.push_str(
                    r#"[ai]
default_provider = "openai"
default_model = "assistant"
timeout_ms = 30000
max_retries = 3

[ai.models.assistant]
provider = "openai"
name = "gpt-4o"
env = "OPENAI_API_KEY"
temperature = 0.7
max_tokens = 2048

[ai.models.analyst]
provider = "gemini"
name = "gemini-1.5-pro"
env = "GEMINI_API_KEY"
temperature = 0.2
max_tokens = 4096

[ai.models.coder]
provider = "anthropic"
name = "claude-3-5-sonnet"
env = "ANTHROPIC_API_KEY"
temperature = 0.1
max_tokens = 8192
"#,
                );
            }
        }

        manifest
    }

    /// Generates .gitignore file.
    pub fn generate_gitignore() -> &'static str {
        r#"# Mantra Build Artifacts
target/
dist/
*.wasm
*.wat

# Local Environment Overrides
.env
*.local

# Platform specific
android/app/build/
android/.gradle/
ios/build/
*.DS_Store
"#
    }

    /// Generates project-specific README.md.
    pub fn generate_readme(opts: &ValidatedOptions) -> String {
        match opts.app_type {
            AppType::Frontend => format!(
                r#"# {name}

{description}

A cross-platform hybrid application built with the **Mantra Programming Language**.

## Target Platforms
{platforms}

## Architecture Overview
```text
{name}/
├── mantra.toml
├── README.md
├── .gitignore
├── src/
│   ├── app/           # Application root and bootstrap
│   ├── components/    # Reusable UI components
│   ├── screens/       # Application screens / views
│   ├── navigation/    # Screen routing and navigation stack
│   ├── services/      # Cross-platform HTTP/API services
│   ├── state/         # State management store
│   ├── styles/        # Global theme and styling
│   └── main.mantra    # Application entry point
├── assets/            # Static assets (images, icons)
├── platforms/         # Platform bridge projects
│   ├── ios/           # iOS Swift bridge (WebKit WKWebView)
│   ├── android/       # Android Kotlin bridge (WebView)
│   └── web/           # WebAssembly & PWA shell
└── tests/             # Unit and integration test suite
```

## Getting Started

### Development & Running
```bash
# Web browser runtime
mantra run -d chrome --web-port 4300

# Android device/emulator runtime
mantra run -d android

# iOS simulator runtime
mantra run -d ios
```

### Building for Release
```bash
# Compile WebAssembly bundle
mantra build web --release

# Build Android APK
mantra build apk --release

# Build Android App Bundle (AAB for Google Play)
mantra build appbundle --release
```

### Running Tests
```bash
mantra test
```

{creation_reference}
"#,
                name = opts.project_name,
                description = opts.description,
                creation_reference = Self::creation_reference(),
                platforms = opts
                    .platforms
                    .iter()
                    .map(|p| format!("- `{}`", p))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            AppType::Backend => match opts.service.unwrap() {
                BackendService::Rest => format!(
                    r#"# {name}

{description}

A modular REST API backend application built with the **Mantra Programming Language**.

## Architecture (MVC API)
```text
{name}/
├── mantra.toml
├── README.md
├── .gitignore
├── src/
│   ├── main.mantra       # Server entry point & route binding
│   ├── config/           # Environment configuration
│   ├── routes/           # Endpoint definitions
│   ├── controllers/      # Request handlers & response dispatch
│   ├── models/           # Domain entities
│   ├── services/         # Core business logic
│   ├── repositories/     # Data access abstraction
│   ├── middleware/       # Auth, logging, CORS middleware
│   ├── validators/       # Request payload validators
│   └── utils/            # Shared utilities & response formatters
├── tests/
│   ├── unit/             # Service & validator unit tests
│   └── integration/      # Endpoint integration tests
└── docs/
    └── api_spec.md       # API endpoint specification
```

## Endpoints
- `GET  /health`    — Health check status
- `GET  /api/users` — Retrieve all users
- `POST /api/users` — Create a new user

## Getting Started

### Running the Server
```bash
mantra run
```

### Running Tests
```bash
mantra test
```

{creation_reference}
"#,
                    name = opts.project_name,
                    description = opts.description,
                    creation_reference = Self::creation_reference()
                ),
                BackendService::Graphql => format!(
                    r#"# {name}

{description}

A modular GraphQL API backend application built with the **Mantra Programming Language**.

## Architecture (GraphQL)
```text
{name}/
├── mantra.toml
├── README.md
├── .gitignore
├── src/
│   ├── main.mantra       # GraphQL server entry point
│   ├── config/           # Server configuration
│   ├── graphql/
│   │   ├── schema/       # GraphQL schema definitions
│   │   ├── resolvers/    # Query and Mutation field resolvers
│   │   ├── types/        # GraphQL object type definitions
│   │   ├── queries/      # Query execution operations
│   │   ├── mutations/    # Mutation execution operations
│   │   └── scalars/      # Custom scalar types (DateTime)
│   ├── services/         # Reusable business logic
│   ├── models/           # Domain entities
│   ├── repositories/     # Data access layer
│   ├── middleware/       # Auth and logging context middleware
│   └── utils/            # GraphQL response formatters
├── tests/
│   ├── unit/             # Resolver unit tests
│   └── integration/      # GraphQL execution integration tests
└── docs/
    └── schema.graphql    # GraphQL SDL schema
```

## Schema Operations
- **Query**: `health: String!`, `users: [User!]!`, `user(id: ID!): User`
- **Mutation**: `createUser(name: String!, email: String!): User!`

## Getting Started

### Running the Server
```bash
mantra run
```

### Running Tests
```bash
mantra test
```

{creation_reference}
"#,
                    name = opts.project_name,
                    description = opts.description,
                    creation_reference = Self::creation_reference()
                ),
            },
            AppType::Standalone => format!(
                r#"# {name}

{description}

A standalone application built with the **Mantra Programming Language**.

## Architecture Overview
```text
{name}/
├── mantra.toml
├── README.md
├── .gitignore
├── src/
│   └── main.mantra       # Application entry point
└── tests/
    └── app_test.mantra   # Test suite
```

## Getting Started

### Running the Application
```bash
mantra run
```

### Running Tests
```bash
mantra test
```

{creation_reference}
"#,
                name = opts.project_name,
                description = opts.description,
                creation_reference = Self::creation_reference()
            ),
            AppType::Ai => format!(
                r#"# {name}

{description}

A native AI application built with the **Mantra Programming Language** and `mantra.ai`.

## Architecture Overview
```text
{name}/
├── mantra.toml        # Project manifest with multi-provider AI model declarations
├── README.md
├── .gitignore
├── src/
│   └── main.mantra    # Application entry point & AI assistant workflow
└── tests/
    └── app_test.mantra # Test suite verifying model invocation & tokens
```

## Configured AI Providers
Models are declared in `mantra.toml` and accessible via `AI.get("<alias>")` or `AI.model(...)`:
- **Assistant**: OpenAI `gpt-4o`
- **Analyst**: Google Gemini `gemini-1.5-pro`
- **Coder**: Anthropic Claude `claude-3-5-sonnet`

## Getting Started

### Running the Application
```bash
mantra run
```

### Running Tests
```bash
mantra test
```

{creation_reference}
"#,
                name = opts.project_name,
                description = opts.description,
                creation_reference = Self::creation_reference()
            ),
        }
    }

    /// Helpful reference commands for creating new Mantra projects
    pub fn creation_reference() -> &'static str {
        r#"## Mantra Project Creation Reference
Use these reference commands to scaffold new Mantra projects:

### 1. Frontend Cross-Platform Application (iOS, Android, Web)
```bash
mantra create \
  --type frontend \
  --org com.testapp \
  --project-name testapp \
  --platforms ios,android,web \
  --description "TestApp cross-platform application"
```

### 2. Backend REST API
```bash
mantra create \
  --type backend \
  --service REST \
  --org com.testapp \
  --project-name testapp \
  --description "TestApp REST API backend"
```

### 3. Backend GraphQL API
```bash
mantra create \
  --type backend \
  --service GRAPHQL \
  --org com.testapp \
  --project-name testapp \
  --description "TestApp GraphQL backend"
```

### 4. Standalone Application
```bash
mantra create \
  --type standalone \
  --org com.raghav \
  --project-name movie_standalone_proj \
  --description "Standalone application"
```

### 5. Native AI Application
```bash
mantra create \
  --type ai \
  --org com.acme \
  --project-name ai_assistant_app \
  --description "Native AI assistant application"
```"#
    }
}
