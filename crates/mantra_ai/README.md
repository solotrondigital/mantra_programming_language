# mantra_ai (v0.7.0)

Official Native AI Integration Package for the Mantra Programming Language (MVP 1).

## Overview

`mantra_ai` allows Mantra developers to integrate AI models directly into their applications without manually implementing HTTP clients, authentication, prompt handling, response parsing, and provider-specific integrations.

## Architecture & Modules

| Module | Description |
|---|---|
| **Model Registry** | Register, configure, and resolve AI models dynamically (`ModelRegistry`, `ModelSpec`). |
| **Provider Manager** | Pluggable multi-provider manager with built-in adapters for **OpenAI**, **Gemini**, and **Anthropic**. |
| **Authentication** | Secure credentials with automatic secret masking, environment variables, and direct API keys. |
| **Prompt Engine** | Templating engine with variable interpolation (`{{var}}`) and system instructions. |
| **AI Client** | Unified model invocation handle (`AI.model(...)`, `AI.get(...)`). |
| **Structured Output** | Type-safe JSON schemas and response validation against typed objects. |
| **Streaming Engine** | Chunked real-time streaming response engine (`StreamReceiver`, `StreamChunk`). |
| **Token Manager** | Token counters, request usage tracking, quota limits, and cost estimation metadata. |
| **Configuration** | Project manifest parsing for `project.toml` and `mantra.toml`. |

## Project Manifest Configuration (`project.toml` or `mantra.toml`)

Developers can declare their AI models in `project.toml` or `mantra.toml`:

```toml
[project]
name = "smart_assistant"
version = "0.1.0"

[dependencies]
mantra_ai = "0.7.0"

[ai]
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
```

## Mantra Code Usage

### 1. Unified Model Creation
```mantra
import mantra.ai

model assistant = AI.model(
    provider: "openai",
    name: "gpt-4o",
    apiKey: env("OPENAI_API_KEY")
)

async function askAI(question: String): String {
    let response = await assistant.generate(
        prompt: question
    )
    return response.text
}

async function main() {
    let answer = await askAI("Explain Mantra")
    print(answer)
}
```

### 2. Loading Configured Models from Manifest
```mantra
import mantra.ai

async function main() {
    // Automatically binds 'assistant' defined in project.toml
    let assistant = AI.get("assistant")
    let answer = await assistant.generate("Summarize daily reports")
    print(answer)
}
```

### 3. Streaming Responses
```mantra
import mantra.ai

async function main() {
    let assistant = AI.get("assistant")
    let stream = await assistant.stream("Generate step-by-step tutorial")
    
    while true {
        let chunk = stream.nextChunk()
        if chunk == null {
            break
        }
        print(chunk.delta)
    }
}
```

## Verification & Testing

- Run unit & integration tests:
  ```bash
  cargo test -p mantra_ai
  ```
- Validate Mantra standard library definition:
  ```bash
  cargo run -p mantrac -- --check crates/mantra_ai/lib/mantra_ai.mantra
  ```

