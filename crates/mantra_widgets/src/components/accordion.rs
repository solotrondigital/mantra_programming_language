//! Collapsible Accordion and ExpansionTile components.

use crate::core::widget::{BuildContext, Widget};

/// Collapsible expansion panel tile.
#[derive(Debug)]
pub struct ExpansionTile {
    pub id: String,
    pub title: String,
    pub child: Box<dyn Widget>,
    pub is_expanded: bool,
}

impl ExpansionTile {
    pub fn new(title: impl Into<String>, child: impl Widget + 'static) -> Self {
        Self {
            id: format!("exp_{}", uuid_simple()),
            title: title.into(),
            child: Box::new(child),
            is_expanded: false,
        }
    }

    pub fn with_expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }
}

impl Widget for ExpansionTile {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "ExpansionTile"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let child_html = self.child.render(ctx);
        let open_attr = if self.is_expanded { " open" } else { "" };

        format!(
            r#"<details id="{}" class="mantra-accordion-item"{} style="border:1px solid var(--mantra-border,#e2e8f0);border-radius:10px;margin-bottom:8px;background:var(--mantra-surface,#fff);overflow:hidden;">
                <summary style="padding:14px 16px;font-weight:600;font-size:15px;color:var(--mantra-text,#0f172a);cursor:pointer;user-select:none;list-style:none;display:flex;justify-content:space-between;align-items:center;">
                    <span>{}</span>
                    <span style="font-size:12px;color:var(--mantra-text-muted,#64748b);">▼</span>
                </summary>
                <div style="padding:0 16px 16px 16px;font-size:14px;color:var(--mantra-text-muted,#475569);line-height:1.6;">
                    {}
                </div>
            </details>"#,
            self.id, open_attr, self.title, child_html
        )
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![self.child.as_ref()]
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(12000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

