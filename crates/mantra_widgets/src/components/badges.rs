//! Badges, notification tags, and chips.

use crate::core::widget::{BuildContext, Widget};

/// Status/Notification Badge.
#[derive(Debug)]
pub struct Badge {
    pub id: String,
    pub content: Option<String>,
    pub is_dot: bool,
    pub color: &'static str,
    pub child: Option<Box<dyn Widget>>,
}

impl Badge {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            id: format!("badge_{}", uuid_simple()),
            content: Some(content.into()),
            is_dot: false,
            color: "#ef4444",
            child: None,
        }
    }

    pub fn dot() -> Self {
        Self {
            id: format!("badge_{}", uuid_simple()),
            content: None,
            is_dot: true,
            color: "#ef4444",
            child: None,
        }
    }

    pub fn with_child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn with_color(mut self, color: &'static str) -> Self {
        self.color = color;
        self
    }
}

impl Widget for Badge {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Badge"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let badge_body = if self.is_dot {
            format!(
                r#"<span class="mantra-badge-dot" style="width:8px;height:8px;border-radius:50%;background-color:{};display:inline-block;"></span>"#,
                self.color
            )
        } else {
            format!(
                r#"<span class="mantra-badge-pill" style="background-color:{};color:#fff;font-size:11px;font-weight:700;padding:2px 6px;border-radius:9999px;display:inline-flex;align-items:center;justify-content:center;line-height:1;">{}</span>"#,
                self.color,
                self.content.as_deref().unwrap_or("")
            )
        };

        if let Some(ch) = &self.child {
            let child_html = ch.render(ctx);
            format!(
                r#"<div id="{}" style="position:relative;display:inline-flex;">
                    {}
                    <div style="position:absolute;top:-6px;right:-6px;">{}</div>
                </div>"#,
                self.id, child_html, badge_body
            )
        } else {
            badge_body
        }
    }
}

/// Interactive or informational Chip.
#[derive(Debug, Clone)]
pub struct Chip {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub is_selected: bool,
    pub on_click: Option<String>,
}

impl Chip {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: format!("chip_{}", uuid_simple()),
            label: label.into(),
            icon: None,
            is_selected: false,
            on_click: None,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_selected(mut self, sel: bool) -> Self {
        self.is_selected = sel;
        self
    }
}

impl Widget for Chip {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Chip"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let (bg, text_color, border) = if self.is_selected {
            ("var(--mantra-primary, #4f46e5)", "#ffffff", "none")
        } else {
            ("var(--mantra-surface, #f1f5f9)", "var(--mantra-text, #0f172a)", "1px solid var(--mantra-border, #cbd5e1)")
        };

        let icon_html = self.icon.as_ref().map(|i| format!(r#"<span style="margin-right:6px;">{}</span>"#, i)).unwrap_or_default();

        format!(
            r#"<div id="{}" class="mantra-chip" style="display:inline-flex;align-items:center;padding:4px 12px;border-radius:9999px;background:{};color:{};border:{};font-size:13px;font-weight:500;cursor:pointer;user-select:none;">
                {}{}
            </div>"#,
            self.id, bg, text_color, border, icon_html, self.label
        )
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(4000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

