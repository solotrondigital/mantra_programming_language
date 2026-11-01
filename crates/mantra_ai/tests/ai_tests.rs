use mantra_ai::*;
use std::collections::HashMap;

#[test]
fn test_auth_credentials() {
    let key = api_key("sk-test1234567890abcdef");
    assert_eq!(key.resolve().unwrap(), "sk-test1234567890abcdef");
    assert!(key.masked().starts_with("sk-t"));
    assert!(key.masked().ends_with("cdef"));

    // Empty key error
    let empty = api_key("");
    assert!(empty.resolve().is_err());
}

#[test]
fn test_prompt_template_rendering() {
    let tpl = PromptTemplate::new("greeting", "Hello {{name}}, welcome to {{project}}!")
        .with_system("You are a friendly assistant");

    let mut vars = HashMap::new();
    vars.insert("name".to_string(), "Alice".to_string());
    vars.insert("project".to_string(), "Mantra".to_string());

    let rendered = tpl.render(&vars);
    assert_eq!(rendered, "Hello Alice, welcome to Mantra!");
    assert_eq!(tpl.system_instruction.as_deref(), Some("You are a friendly assistant"));
}

#[test]
fn test_token_manager_limits_and_costs() {
    let mgr = TokenManager::with_limit(100);
    let usage1 = TokenUsage::new(20, 30).with_rates(0.0015, 0.002);
    assert!(mgr.record(&usage1).is_ok());
    assert_eq!(mgr.total_tokens(), 50);
    assert!(usage1.estimated_cost_usd > 0.0);

    // Second usage within limit
    let usage2 = TokenUsage::new(10, 20);
    assert!(mgr.record(&usage2).is_ok());
    assert_eq!(mgr.total_tokens(), 80);

    // Exceed limit (80 + 30 > 100)
    let usage3 = TokenUsage::new(15, 15);
    assert!(mgr.record(&usage3).is_err());
}

#[test]
fn test_multi_provider_generation() {
    let ai = AI::new();

    // 1. OpenAI provider
    let gpt = ai.model("openai", "gpt-4o", api_key("mock-openai-key"));
    let res_gpt = gpt.generate("Explain static typing").unwrap();
    assert_eq!(res_gpt.provider, "openai");
    assert_eq!(res_gpt.model, "gpt-4o");
    assert!(res_gpt.text.contains("[OpenAI:gpt-4o]"));

    // 2. Gemini provider
    let gemini = ai.model("gemini", "gemini-1.5-pro", api_key("mock-gemini-key"));
    let res_gemini = gemini.generate("Explain AST").unwrap();
    assert_eq!(res_gemini.provider, "gemini");
    assert_eq!(res_gemini.model, "gemini-1.5-pro");
    assert!(res_gemini.text.contains("[Gemini:gemini-1.5-pro]"));

    // 3. Anthropic provider
    let claude = ai.model("anthropic", "claude-3-5-sonnet", api_key("mock-claude-key"));
    let res_claude = claude.generate("Explain pattern matching").unwrap();
    assert_eq!(res_claude.provider, "anthropic");
    assert_eq!(res_claude.model, "claude-3-5-sonnet");
    assert!(res_claude.text.contains("[Anthropic:claude-3-5-sonnet]"));
}

#[test]
fn test_streaming_engine() {
    let ai = AI::new();
    let model = ai.model("openai", "gpt-4o", api_key("mock-key"));
    let mut stream = model.stream("Streaming test").unwrap();

    let collected = stream.collect_text();
    assert!(collected.contains("[OpenAI:gpt-4o]"));
    assert!(collected.contains("Streaming test"));
}

#[test]
fn test_structured_output_schema_validation() {
    let mut person_fields = HashMap::new();
    person_fields.insert("name".to_string(), (SchemaType::String, true));
    person_fields.insert("age".to_string(), (SchemaType::Number, true));
    person_fields.insert("active".to_string(), (SchemaType::Boolean, false));

    let schema = StructuredSchema::new("Person", SchemaType::Object(person_fields));

    // Valid object
    let mut valid_map = HashMap::new();
    valid_map.insert("name".to_string(), JsonValue::String("Bob".to_string()));
    valid_map.insert("age".to_string(), JsonValue::Number(28.0));
    assert!(schema.validate(&JsonValue::Object(valid_map)).is_ok());

    // Missing required field 'age'
    let mut invalid_map = HashMap::new();
    invalid_map.insert("name".to_string(), JsonValue::String("Bob".to_string()));
    assert!(schema.validate(&JsonValue::Object(invalid_map)).is_err());
}

#[test]
fn test_project_toml_configuration_parsing() {
    let toml_sample = r#"
[project]
name = "my_smart_app"
version = "0.1.0"

[ai]
default_provider = "gemini"
default_model = "assistant"
timeout_ms = 45000
max_retries = 5

[ai.models.assistant]
provider = "openai"
name = "gpt-4o"
api_key = "sk-test-key-12345678"
temperature = 0.5
max_tokens = 4096

[ai.models.analyst]
provider = "gemini"
name = "gemini-1.5-flash"
env = "GEMINI_API_KEY"
temperature = 0.2
"#;

    let config = AIConfig::parse_toml(toml_sample).unwrap();
    assert_eq!(config.default_provider, "gemini");
    assert_eq!(config.timeout_ms, 45000);
    assert_eq!(config.max_retries, 5);
    assert_eq!(config.models.len(), 2);

    let assistant = config.models.get("assistant").unwrap();
    assert_eq!(assistant.provider, "openai");
    assert_eq!(assistant.model_name, "gpt-4o");
    assert_eq!(assistant.default_temperature, Some(0.5));
    assert_eq!(assistant.default_max_tokens, Some(4096));

    let analyst = config.models.get("analyst").unwrap();
    assert_eq!(analyst.provider, "gemini");
    assert_eq!(analyst.model_name, "gemini-1.5-flash");

    // Initialize AI engine from config
    let ai = AI::from_config(&config);
    let handle = ai.get("assistant").unwrap();
    assert_eq!(handle.spec.provider, "openai");
    assert_eq!(handle.spec.model_name, "gpt-4o");

    let res = handle.generate("Analyze quarterly sales").unwrap();
    assert!(res.text.contains("[OpenAI:gpt-4o]"));
}

