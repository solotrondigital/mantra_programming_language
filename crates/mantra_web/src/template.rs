//! Template — lightweight template engine for HTML rendering.

/// A compiled template ready for rendering.
pub struct Template {
    pub name: String,
    source: String,
}

impl Template {
    pub fn new(name: &str, source: &str) -> Self {
        Template { name: name.into(), source: source.into() }
    }

    /// Render the template substituting {{key}} placeholders with values.
    pub fn render(&self, vars: &[(&str, &str)]) -> String {
        let mut out = self.source.clone();
        for (key, val) in vars {
            out = out.replace(&format!("{{{{{}}}}}", key), val);
        }
        out
    }
}

/// Manages a collection of named templates.
#[derive(Default)]
pub struct TemplateEngine {
    templates: std::collections::HashMap<String, Template>,
}

impl TemplateEngine {
    pub fn new() -> Self { TemplateEngine::default() }
    pub fn register(mut self, t: Template) -> Self {
        self.templates.insert(t.name.clone(), t);
        self
    }
    pub fn render(&self, name: &str, vars: &[(&str, &str)]) -> Option<String> {
        self.templates.get(name).map(|t| t.render(vars))
    }
}
