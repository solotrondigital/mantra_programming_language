# ai_assistant_app

Mantra standalone application: ai_assistant_app

A standalone application built with the **Mantra Programming Language**.

## Architecture Overview
```text
ai_assistant_app/
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
