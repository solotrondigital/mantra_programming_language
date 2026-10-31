//! HTML element renderer and tag builder.

use super::css_builder::CssBuilder;
use crate::accessibility::Semantics;

/// Fast, safe HTML element builder.
#[derive(Debug, Clone)]
pub struct HtmlElement {
    pub tag: &'static str,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub styles: CssBuilder,
    pub semantics: Semantics,
    pub attributes: Vec<(String, String)>,
    pub content: String,
    pub children: Vec<String>,
}

impl HtmlElement {
    pub fn new(tag: &'static str) -> Self {
        Self {
            tag,
            id: None,
            classes: Vec::new(),
            styles: CssBuilder::new(),
            semantics: Semantics::default(),
            attributes: Vec::new(),
            content: String::new(),
            children: Vec::new(),
        }
    }

    pub fn div() -> Self {
        Self::new("div")
    }

    pub fn button() -> Self {
        Self::new("button")
    }

    pub fn span() -> Self {
        Self::new("span")
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn add_class(mut self, class_name: impl Into<String>) -> Self {
        self.classes.push(class_name.into());
        self
    }

    pub fn with_style(mut self, property: &str, value: &str) -> Self {
        self.styles = self.styles.set(property, value);
        self
    }

    pub fn with_semantics(mut self, semantics: Semantics) -> Self {
        self.semantics = semantics;
        self
    }

    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push((key.into(), value.into()));
        self
    }

    pub fn with_content(mut self, text: impl Into<String>) -> Self {
        self.content = text.into();
        self
    }

    pub fn add_child(mut self, child_html: impl Into<String>) -> Self {
        self.children.push(child_html.into());
        self
    }

    pub fn render(&self) -> String {
        let mut out = format!("<{}", self.tag);

        if let Some(id) = &self.id {
            out.push_str(&format!(r#" id="{}""#, id));
        }

        if !self.classes.is_empty() {
            out.push_str(&format!(r#" class="{}""#, self.classes.join(" ")));
        }

        out.push_str(&self.styles.build());
        out.push_str(&self.semantics.to_html_attributes());

        for (k, v) in &self.attributes {
            out.push_str(&format!(r#" {}="{}""#, k, v));
        }

        out.push('>');

        if !self.content.is_empty() {
            out.push_str(&self.content);
        }

        for child in &self.children {
            out.push_str(child);
        }

        out.push_str(&format!("</{}>", self.tag));
        out
    }
}

