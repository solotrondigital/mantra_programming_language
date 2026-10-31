//! Navigation components (AppBar, BottomNavigationBar, Drawer, Tabs).

use crate::core::widget::{BuildContext, Widget};

/// Application Header Bar / Top App Bar.
#[derive(Debug)]
pub struct AppBar {
    pub id: String,
    pub title: String,
    pub leading: Option<Box<dyn Widget>>,
    pub actions: Vec<Box<dyn Widget>>,
    pub background_color: Option<String>,
}

impl AppBar {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: format!("appbar_{}", uuid_simple()),
            title: title.into(),
            leading: None,
            actions: Vec::new(),
            background_color: None,
        }
    }

    pub fn with_leading(mut self, leading: impl Widget + 'static) -> Self {
        self.leading = Some(Box::new(leading));
        self
    }

    pub fn add_action(mut self, action: impl Widget + 'static) -> Self {
        self.actions.push(Box::new(action));
        self
    }
}

impl Widget for AppBar {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "AppBar"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let leading_html = self.leading.as_ref().map(|l| l.render(ctx)).unwrap_or_default();
        let actions_html = self.actions.iter().map(|a| a.render(ctx)).collect::<Vec<_>>().join("");

        format!(
            r#"<header id="{}" class="mantra-appbar" style="height:56px;padding:0 16px;display:flex;align-items:center;background:var(--mantra-surface,#fff);border-bottom:1px solid var(--mantra-border,#e2e8f0);position:sticky;top:0;z-index:100;box-sizing:border-box;">
                <div style="display:flex;align-items:center;margin-right:12px;">{}</div>
                <div style="flex:1;font-size:18px;font-weight:700;color:var(--mantra-text,#0f172a);white-space:nowrap;overflow:hidden;text-overflow:ellipsis;">{}</div>
                <div style="display:flex;align-items:center;gap:8px;">{}</div>
            </header>"#,
            self.id, leading_html, self.title, actions_html
        )
    }

    fn children(&self) -> Vec<&dyn Widget> {
        let mut list: Vec<&dyn Widget> = Vec::new();
        if let Some(l) = &self.leading {
            list.push(l.as_ref());
        }
        for a in &self.actions {
            list.push(a.as_ref());
        }
        list
    }
}

/// Item inside BottomNavigationBar.
#[derive(Debug, Clone)]
pub struct BottomNavItem {
    pub label: String,
    pub icon: String,
    pub route: String,
}

/// Mobile/Web Bottom Navigation Bar.
#[derive(Debug, Clone)]
pub struct BottomNavigationBar {
    pub id: String,
    pub items: Vec<BottomNavItem>,
    pub current_index: usize,
}

impl BottomNavigationBar {
    pub fn new(items: Vec<BottomNavItem>, current_index: usize) -> Self {
        Self {
            id: format!("botnav_{}", uuid_simple()),
            items,
            current_index,
        }
    }
}

impl Widget for BottomNavigationBar {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "BottomNavigationBar"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let items_html = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_active = i == self.current_index;
                let color = if is_active { "var(--mantra-primary, #4f46e5)" } else { "#64748b" };
                let font_weight = if is_active { "600" } else { "400" };

                format!(
                    r#"<div class="mantra-bottom-nav-item" style="flex:1;display:flex;flex-direction:column;align-items:center;justify-content:center;cursor:pointer;color:{};user-select:none;">
                        <span style="font-size:20px;">{}</span>
                        <span style="font-size:11px;font-weight:{};margin-top:2px;">{}</span>
                    </div>"#,
                    color, item.icon, font_weight, item.label
                )
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<nav id="{}" class="mantra-bottom-nav" style="height:60px;display:flex;background:var(--mantra-surface,#fff);border-top:1px solid var(--mantra-border,#e2e8f0);position:fixed;bottom:0;left:0;right:0;z-index:100;box-sizing:border-box;">
                {}
            </nav>"#,
            self.id, items_html
        )
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(7000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

