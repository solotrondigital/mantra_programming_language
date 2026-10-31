//! CSS rule and inline style builder.

/// Helper for constructing CSS inline style strings.
#[derive(Debug, Default, Clone)]
pub struct CssBuilder {
    properties: Vec<(String, String)>,
}

impl CssBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(mut self, property: &str, value: &str) -> Self {
        if !value.is_empty() {
            self.properties.push((property.to_string(), value.to_string()));
        }
        self
    }

    pub fn set_if(self, condition: bool, property: &str, value: &str) -> Self {
        if condition {
            self.set(property, value)
        } else {
            self
        }
    }

    pub fn build(&self) -> String {
        if self.properties.is_empty() {
            return String::new();
        }

        let formatted = self
            .properties
            .iter()
            .map(|(k, v)| format!("{}: {};", k, v))
            .collect::<Vec<_>>()
            .join(" ");

        format!(r#" style="{}""#, formatted)
    }
}

