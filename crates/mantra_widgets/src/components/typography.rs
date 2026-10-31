//! Typography components (Text, Heading, Paragraph, Caption, GradientText).

use crate::core::widget::{BuildContext, Widget};
use crate::theme::typography::TextStyle;

/// General purpose text widget.
#[derive(Debug, Clone)]
pub struct Text {
    pub id: String,
    pub content: String,
    pub style: Option<TextStyle>,
    pub max_lines: Option<usize>,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            id: format!("txt_{}", uuid_simple()),
            content: content.into(),
            style: None,
            max_lines: None,
        }
    }

    pub fn with_style(mut self, style: TextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Widget for Text {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Text"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let style_attr = self
            .style
            .as_ref()
            .map(|s| format!(r#" style="{}""#, s.to_css()))
            .unwrap_or_default();

        format!(r#"<span id="{}" class="mantra-text"{}>{}</span>"#, self.id, style_attr, self.content)
    }
}

/// Headings (H1 to H6).
#[derive(Debug, Clone)]
pub struct Heading {
    pub id: String,
    pub level: usize,
    pub text: String,
}

impl Heading {
    pub fn h1(text: impl Into<String>) -> Self {
        Self { id: format!("h_{}", uuid_simple()), level: 1, text: text.into() }
    }

    pub fn h2(text: impl Into<String>) -> Self {
        Self { id: format!("h_{}", uuid_simple()), level: 2, text: text.into() }
    }

    pub fn h3(text: impl Into<String>) -> Self {
        Self { id: format!("h_{}", uuid_simple()), level: 3, text: text.into() }
    }
}

impl Widget for Heading {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Heading"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let tag = match self.level {
            1 => "h1",
            2 => "h2",
            _ => "h3",
        };

        let font_size = match self.level {
            1 => "32px",
            2 => "24px",
            _ => "18px",
        };

        format!(
            r#"<{} id="{}" class="mantra-heading" style="font-size:{};font-weight:700;color:var(--mantra-text,#0f172a);margin-bottom:8px;line-height:1.25;">{}</{}>"#,
            tag, self.id, font_size, self.text, tag
        )
    }
}

/// Gradient text for hero banners and headlines.
#[derive(Debug, Clone)]
pub struct GradientText {
    pub id: String,
    pub text: String,
    pub gradient: String,
    pub font_size: f64,
}

impl GradientText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: format!("gtxt_{}", uuid_simple()),
            text: text.into(),
            gradient: "linear-gradient(135deg, #4f46e5 0%, #06b6d4 100%)".to_string(),
            font_size: 28.0,
        }
    }
}

impl Widget for GradientText {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "GradientText"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        format!(
            r#"<span id="{}" class="mantra-gradient-text" style="background:{};-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:{}px;font-weight:800;display:inline-block;">{}</span>"#,
            self.id, self.gradient, self.font_size, self.text
        )
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(13000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
