//! CSS-accelerated animation wrappers.

use crate::core::widget::{BuildContext, Widget};

/// Wraps child in a smooth fade-in animation.
#[derive(Debug)]
pub struct FadeAnimation {
    pub id: String,
    pub duration_ms: u32,
    pub child: Box<dyn Widget>,
}

impl FadeAnimation {
    pub fn new(duration_ms: u32, child: impl Widget + 'static) -> Self {
        Self {
            id: format!("fade_{}", uuid_simple()),
            duration_ms,
            child: Box::new(child),
        }
    }
}

impl Widget for FadeAnimation {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "FadeAnimation"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let child_html = self.child.render(ctx);
        format!(
            r#"<div id="{}" class="mantra-fade" style="animation:mantra-fade-in {}ms ease forwards;">{}</div>"#,
            self.id, self.duration_ms, child_html
        )
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![self.child.as_ref()]
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(15000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

