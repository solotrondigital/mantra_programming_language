//! AI Runner — executes Mantra AI projects by loading mantra.toml config
//! and dispatching real HTTP calls to LLM providers via mantra_ai.
//!
//! This bypasses the Mantra VM interpreter for `--type ai` projects and
//! instead runs the AI application logic directly in Rust, calling OpenAI,
//! Gemini, and Anthropic APIs with the credentials from the environment.

use mantra_ai::client::AI;
use mantra_ai::config::AIConfig;
use std::path::Path;

/// Entry-point called by `Runner::run_target` for AI project types.
pub struct AiRunner;

impl AiRunner {
    /// Load `mantra.toml` from the project root, build the AI client, and
    /// execute the demonstration scenarios — calling real LLM HTTP APIs.
    pub fn run(project_dir: &Path) -> Result<(), String> {
        // ── Load project AI configuration ────────────────────────────────
        let config = AIConfig::load_from_dir(project_dir)
            .map_err(|e| format!("Failed to load AI configuration: {}", e))?;

        let project_name = {
            // Read mantra.toml for [project] name field
            let toml_path = project_dir.join("mantra.toml");
            let alt_path = project_dir.join("project.toml");
            let content = std::fs::read_to_string(&toml_path)
                .or_else(|_| std::fs::read_to_string(&alt_path))
                .unwrap_or_default();
            // Simple extraction of name = "..." from [project] section
            let mut found = None;
            let mut in_project = false;
            for line in content.lines() {
                let t = line.trim();
                if t == "[project]" {
                    in_project = true;
                    continue;
                }
                if t.starts_with('[') {
                    in_project = false;
                }
                if in_project {
                    if let Some((k, v)) = t.split_once('=') {
                        if k.trim() == "name" {
                            found = Some(v.trim().trim_matches('"').to_string());
                            break;
                        }
                    }
                }
            }
            found.unwrap_or_else(|| {
                project_dir
                    .canonicalize()
                    .ok()
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "AI App".to_string())
            })
        };

        let ai = AI::from_config(&config);

        println!("==================================================================");
        println!(
            "              RUNNING {} (MANTRA AI)                    ",
            project_name.to_uppercase()
        );
        println!("==================================================================");
        println!("Dynamic Configuration Loaded from mantra.toml:");
        println!("------------------------------------------------------------------");
        println!("  Project Name       : {}", project_name);
        println!("  Default Provider   : {}", config.default_provider);
        println!("  Default Model      : {}", config.default_model);
        println!("  Timeout (ms)       : {}", config.timeout_ms);
        println!("  Max Retries        : {}", config.max_retries);
        println!("  Configured Models  : {}", config.models.len());
        println!("------------------------------------------------------------------");
        println!("Models Defined in mantra.toml:");
        for (alias, spec) in &config.models {
            let temp_str = spec
                .default_temperature
                .map_or("0.7".to_string(), |t| t.to_string());
            let max_t_str = spec
                .default_max_tokens
                .map_or("2048".to_string(), |m| m.to_string());
            println!("  • [{}]", alias);
            println!("      provider    = \"{}\"", spec.provider);
            println!("      name        = \"{}\"", spec.model_name);
            println!("      temperature = {}", temp_str);
            println!("      max_tokens  = {}", max_t_str);
            println!("      credential  = \"{}\"", spec.auth.masked());
        }
        println!("------------------------------------------------------------------\n");

        // ── 1. Primary / Default model generation (Dynamic) ─────────────
        let default_alias = if config.models.contains_key(&config.default_model) {
            config.default_model.clone()
        } else if config.models.contains_key("assistant") {
            "assistant".to_string()
        } else {
            config
                .models
                .keys()
                .next()
                .cloned()
                .unwrap_or_else(|| "assistant".to_string())
        };

        println!("--- 1. Generating Model Response [{}] ---", default_alias);
        match ai.get(&default_alias) {
            Ok(handle) => {
                println!(
                    "[AI Service] Dispatching prompt to {} ({})...",
                    handle.spec.provider, handle.spec.model_name
                );
                match handle
                    .generate("Explain native AI integration in the Mantra Programming Language")
                {
                    Ok(resp) => {
                        println!(
                            "  -> Tokens: {} (Prompt: {}, Completion: {}) | Cost: ${:.4}",
                            resp.usage.total_tokens,
                            resp.usage.prompt_tokens,
                            resp.usage.completion_tokens,
                            resp.usage.estimated_cost_usd
                        );
                        println!("Result:\n{}\n", resp.text);
                    }
                    Err(e) => {
                        eprintln!(
                            "  [Warning] {} model call failed: {}",
                            handle.spec.provider, e
                        );
                        if e.to_string().contains("Authentication Error") {
                            eprintln!("  -> Ensure {} environment variable or api_key in mantra.toml is set.\n", Self::env_for_provider(&handle.spec.provider));
                        } else {
                            eprintln!(
                                "  -> Check your {} account credits / API key validity.\n",
                                handle.spec.provider
                            );
                        }
                    }
                }
            }
            Err(e) => eprintln!(
                "  [Warning] Could not resolve model '{}': {}\n",
                default_alias, e
            ),
        }

        // ── 2. Streaming response demonstration (Dynamic) ───────────────
        println!(
            "--- 2. Streaming Response Demonstration [{}] ---",
            default_alias
        );
        match ai.get(&default_alias) {
            Ok(handle) => {
                println!(
                    "[AI Service] Streaming response from {}...",
                    handle.spec.provider
                );
                match handle.stream("Mantra AI Compiler Pipeline and Streaming Runtime") {
                    Ok(mut receiver) => {
                        print!("  ");
                        loop {
                            match receiver.next_chunk() {
                                Some(chunk) => {
                                    print!("{}", chunk.delta);
                                    if chunk.is_final {
                                        println!();
                                        println!(
                                            "  [finish: {}]",
                                            chunk
                                                .finish_reason
                                                .unwrap_or_else(|| "stop".to_string())
                                        );
                                        break;
                                    }
                                }
                                None => {
                                    println!();
                                    break;
                                }
                            }
                        }
                        println!();
                    }
                    Err(e) => eprintln!("  [Warning] Streaming failed: {}\n", e),
                }
            }
            Err(e) => eprintln!(
                "  [Warning] Could not resolve model '{}': {}\n",
                default_alias, e
            ),
        }

        // ── 3. Iterate over all other models dynamically from mantra.toml ─
        let mut step_num = 3;
        for (alias, spec) in &config.models {
            if alias == &default_alias {
                continue;
            }
            println!(
                "--- {}. Multi-Provider Model Binding: [{}] ---",
                step_num, alias
            );
            println!("Bound provider: {} / {}", spec.provider, spec.model_name);
            match ai.get(alias) {
                Ok(handle) => {
                    let prompt_text = format!(
                        "Provide structural overview of {} model capabilities in Mantra",
                        alias
                    );
                    match handle.generate(&prompt_text) {
                        Ok(resp) => {
                            println!(
                                "  -> Tokens: {} | Cost: ${:.4}",
                                resp.usage.total_tokens, resp.usage.estimated_cost_usd
                            );
                            println!("Result:\n{}\n", resp.text);
                        }
                        Err(e) => {
                            eprintln!(
                                "  [Warning] {} model call failed: {}",
                                handle.spec.provider, e
                            );
                            if e.to_string().contains("Authentication Error") {
                                eprintln!("  -> Ensure {} environment variable or api_key in mantra.toml is set.\n", Self::env_for_provider(&handle.spec.provider));
                            } else {
                                eprintln!(
                                    "  -> Check your {} account credits / API key validity.\n",
                                    handle.spec.provider
                                );
                            }
                        }
                    }
                }
                Err(e) => eprintln!("  [Warning] Could not resolve model '{}': {}\n", alias, e),
            }
            step_num += 1;
        }

        // ── Token summary ────────────────────────────────────────────────
        println!("--- Token Usage Summary ---");
        let total = ai.tokens.total_tokens();
        let prompt = ai.tokens.prompt_tokens();
        let completion = ai.tokens.completion_tokens();
        println!("  Prompt tokens     : {}", prompt);
        println!("  Completion tokens : {}", completion);
        println!("  Total tokens used : {}", total);
        println!();

        println!("==================================================================");
        println!(
            "         {} AI Execution Completed Successfully! 🚀",
            project_name.to_uppercase()
        );
        println!("==================================================================");
        Ok(())
    }

    fn env_for_provider(provider: &str) -> &'static str {
        match provider.to_lowercase().as_str() {
            "openai" => "OPENAI_API_KEY",
            "gemini" => "GEMINI_API_KEY",
            "anthropic" => "ANTHROPIC_API_KEY",
            _ => "API_KEY",
        }
    }
}
