//! Prompt engine: templates, variable interpolation, and system instructions.

use std::collections::HashMap;

/// Reusable prompt template with placeholder variables like `{{variable_name}}`.
#[derive(Debug, Clone, PartialEq)]
pub struct PromptTemplate {
    pub name: String,
    pub template: String,
    pub system_instruction: Option<String>,
}

impl PromptTemplate {
    pub fn new(name: impl Into<String>, template: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            template: template.into(),
            system_instruction: None,
        }
    }

    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system_instruction = Some(system.into());
        self
    }

    /// Render template with the provided key-value parameters.
    pub fn render(&self, vars: &HashMap<String, String>) -> String {
        let mut rendered = self.template.clone();
        for (k, v) in vars {
            let pattern = format!("{{{{{}}}}}", k);
            rendered = rendered.replace(&pattern, v);
        }
        rendered
    }
}

/// A fully formulated prompt ready for execution.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Prompt {
    pub system_instruction: Option<String>,
    pub user_prompt: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
    pub stop_sequences: Vec<String>,
}

impl Prompt {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            system_instruction: None,
            user_prompt: prompt.into(),
            temperature: None,
            max_tokens: None,
            stop_sequences: Vec::new(),
        }
    }

    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system_instruction = Some(system.into());
        self
    }

    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn with_max_tokens(mut self, tokens: usize) -> Self {
        self.max_tokens = Some(tokens);
        self
    }

    pub fn add_stop(mut self, seq: impl Into<String>) -> Self {
        self.stop_sequences.push(seq.into());
        self
    }
}
