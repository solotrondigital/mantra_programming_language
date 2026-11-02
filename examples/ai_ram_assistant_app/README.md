# ai_ram_assistant_app

Mantra AI application: ai_ram_assistant_app

A native AI application built with the **Mantra Programming Language** and `mantra.ai`.

## Architecture Overview
```text
ai_ram_assistant_app/
├── mantra.toml        # Project manifest with multi-provider AI model declarations
├── README.md
├── .gitignore
├── src/
│   └── main.mantra    # Application entry point & AI assistant workflow
└── tests/
    └── app_test.mantra # Test suite verifying model invocation & tokens
```

## Configured AI Providers & Email Service
Models and email transport are declared in `mantra.toml`:
- **Assistant**: OpenAI `gpt-4o`
- **Analyst**: Google Gemini `gemini-1.5-pro`
- **Coder**: Anthropic Claude `claude-3-5-sonnet`
- **Email Service**: `mantra_email` SMTP client (`smtp.solotrontech.com:587`) for dispatching automated AI execution reports

## Automated Email Dispatching (`AssistantEmailNotifier`)
The `AssistantEmailNotifier` class loads SMTP server settings and recipient routing from `mantra.toml` using `PropertiesTOML` and `ENV`:
```mantra
let email_notifier = AssistantEmailNotifier("mantra.toml")
email_notifier.send_ai_summary(
    "Mantra AI Execution & Native Integration",
    "openai:gpt-4o",
    "Analysis completed."
)
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

## Mantra Project Creation Reference
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
```
