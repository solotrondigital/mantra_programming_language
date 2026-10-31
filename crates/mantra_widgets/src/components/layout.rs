//! Structural layout components (Container, Row, Column, Stack, Wrap, Center, Padding, Divider, ListView, GridView, ListTile).

use crate::core::layout::{CrossAxisAlignment, EdgeInsets, MainAxisAlignment};
use crate::core::widget::{BuildContext, Widget};
use crate::render::HtmlElement;
use crate::theme::colors::Color;
use crate::theme::shape::BorderRadius;

/// Universal layout Container.
#[derive(Debug)]
pub struct Container {
    pub id: String,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub color: Option<Color>,
    pub radius: BorderRadius,
    pub child: Option<Box<dyn Widget>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            id: format!("cnt_{}", uuid_simple()),
            width: None,
            height: None,
            padding: EdgeInsets::ZERO,
            margin: EdgeInsets::ZERO,
            color: None,
            radius: BorderRadius::ZERO,
            child: None,
        }
    }

    pub fn with_child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_padding(mut self, padding: EdgeInsets) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_margin(mut self, margin: EdgeInsets) -> Self {
        self.margin = margin;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_radius(mut self, radius: BorderRadius) -> Self {
        self.radius = radius;
        self
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Container {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Container"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let mut elem = HtmlElement::div()
            .with_id(&self.id)
            .add_class("mantra-container")
            .with_style("box-sizing", "border-box");

        if let Some(w) = self.width {
            elem = elem.with_style("width", &format!("{}px", w));
        }
        if let Some(h) = self.height {
            elem = elem.with_style("height", &format!("{}px", h));
        }
        if self.padding != EdgeInsets::ZERO {
            elem = elem.with_style("padding", &self.padding.to_css_string());
        }
        if self.margin != EdgeInsets::ZERO {
            elem = elem.with_style("margin", &self.margin.to_css_string());
        }
        if let Some(c) = &self.color {
            elem = elem.with_style("background-color", &c.to_css());
        }
        if self.radius != BorderRadius::ZERO {
            elem = elem.with_style("border-radius", &self.radius.to_css());
        }

        if let Some(ch) = &self.child {
            let inner_ctx = ctx.child_context(&self.id);
            elem = elem.add_child(ch.render(&inner_ctx));
        }

        elem.render()
    }

    fn children(&self) -> Vec<&dyn Widget> {
        self.child.as_ref().map(|c| vec![c.as_ref()]).unwrap_or_default()
    }
}

/// Horizontal Flex layout.
#[derive(Debug)]
pub struct Row {
    pub id: String,
    pub children: Vec<Box<dyn Widget>>,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
    pub gap: f64,
}

impl Row {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            id: format!("row_{}", uuid_simple()),
            children,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Center,
            gap: 8.0,
        }
    }

    pub fn with_alignment(mut self, main: MainAxisAlignment, cross: CrossAxisAlignment) -> Self {
        self.main_axis_alignment = main;
        self.cross_axis_alignment = cross;
        self
    }

    pub fn with_gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }
}

impl Widget for Row {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Row"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let items_html = self
            .children
            .iter()
            .map(|ch| ch.render(ctx))
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<div id="{}" class="mantra-row" style="display:flex;flex-direction:row;justify-content:{};align-items:{};gap:{}px;box-sizing:border-box;width:100%;">{}</div>"#,
            self.id,
            self.main_axis_alignment.to_css(),
            self.cross_axis_alignment.to_css(),
            self.gap,
            items_html
        )
    }

    fn children(&self) -> Vec<&dyn Widget> {
        self.children.iter().map(|b| b.as_ref()).collect()
    }
}

/// Vertical Flex layout.
#[derive(Debug)]
pub struct Column {
    pub id: String,
    pub children: Vec<Box<dyn Widget>>,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
    pub gap: f64,
}

impl Column {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            id: format!("col_{}", uuid_simple()),
            children,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Start,
            gap: 8.0,
        }
    }

    pub fn with_alignment(mut self, main: MainAxisAlignment, cross: CrossAxisAlignment) -> Self {
        self.main_axis_alignment = main;
        self.cross_axis_alignment = cross;
        self
    }

    pub fn with_gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }
}

impl Widget for Column {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Column"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let items_html = self
            .children
            .iter()
            .map(|ch| ch.render(ctx))
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<div id="{}" class="mantra-column" style="display:flex;flex-direction:column;justify-content:{};align-items:{};gap:{}px;box-sizing:border-box;width:100%;">{}</div>"#,
            self.id,
            self.main_axis_alignment.to_css(),
            self.cross_axis_alignment.to_css(),
            self.gap,
            items_html
        )
    }

    fn children(&self) -> Vec<&dyn Widget> {
        self.children.iter().map(|b| b.as_ref()).collect()
    }
}

/// Horizontal divider line.
#[derive(Debug, Clone)]
pub struct Divider {
    pub id: String,
    pub height: f64,
    pub color: &'static str,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            id: format!("div_{}", uuid_simple()),
            height: 1.0,
            color: "var(--mantra-border, #e2e8f0)",
        }
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Divider {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Divider"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        format!(
            r#"<hr id="{}" class="mantra-divider" style="border:none;border-top:{}px solid {};margin:12px 0;width:100%;"/>"#,
            self.id, self.height, self.color
        )
    }
}

/// ListTile component for settings, list items, and contacts.
#[derive(Debug)]
pub struct ListTile {
    pub id: String,
    pub leading: Option<Box<dyn Widget>>,
    pub title: String,
    pub subtitle: Option<String>,
    pub trailing: Option<Box<dyn Widget>>,
    pub on_click: Option<String>,
}

impl ListTile {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: format!("tile_{}", uuid_simple()),
            leading: None,
            title: title.into(),
            subtitle: None,
            trailing: None,
            on_click: None,
        }
    }

    pub fn with_leading(mut self, leading: impl Widget + 'static) -> Self {
        self.leading = Some(Box::new(leading));
        self
    }

    pub fn with_subtitle(mut self, sub: impl Into<String>) -> Self {
        self.subtitle = Some(sub.into());
        self
    }

    pub fn with_trailing(mut self, trailing: impl Widget + 'static) -> Self {
        self.trailing = Some(Box::new(trailing));
        self
    }
}

impl Widget for ListTile {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "ListTile"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let leading_html = self.leading.as_ref().map(|l| l.render(ctx)).unwrap_or_default();
        let trailing_html = self.trailing.as_ref().map(|t| t.render(ctx)).unwrap_or_default();

        let subtitle_html = self
            .subtitle
            .as_ref()
            .map(|s| format!(r#"<div style="font-size:13px;color:var(--mantra-text-muted,#64748b);margin-top:2px;">{}</div>"#, s))
            .unwrap_or_default();

        format!(
            r#"<div id="{}" class="mantra-list-tile" style="display:flex;align-items:center;padding:12px 16px;background:var(--mantra-surface,#fff);border-bottom:1px solid var(--mantra-border,#f1f5f9);cursor:pointer;">
                <div style="margin-right:14px;display:flex;align-items:center;">{}</div>
                <div style="flex:1;">
                    <div style="font-size:15px;font-weight:600;color:var(--mantra-text,#0f172a);">{}</div>
                    {}
                </div>
                <div style="margin-left:14px;display:flex;align-items:center;">{}</div>
            </div>"#,
            self.id, leading_html, self.title, subtitle_html, trailing_html
        )
    }

    fn children(&self) -> Vec<&dyn Widget> {
        let mut list: Vec<&dyn Widget> = Vec::new();
        if let Some(l) = &self.leading {
            list.push(l.as_ref());
        }
        if let Some(t) = &self.trailing {
            list.push(t.as_ref());
        }
        list
    }
}

/// Grid layout with customizable column count and gaps.
#[derive(Debug)]
pub struct GridView {
    pub id: String,
    pub columns: usize,
    pub gap: f64,
    pub children: Vec<Box<dyn Widget>>,
}

impl GridView {
    pub fn new(columns: usize, children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            id: format!("grid_{}", uuid_simple()),
            columns,
            gap: 16.0,
            children,
        }
    }
}

impl Widget for GridView {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "GridView"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let items_html = self.children.iter().map(|ch| ch.render(ctx)).collect::<Vec<_>>().join("");
        format!(
            r#"<div id="{}" class="mantra-grid" style="display:grid;grid-template-columns:repeat({}, minmax(0, 1fr));gap:{}px;width:100%;box-sizing:border-box;">{}</div>"#,
            self.id, self.columns, self.gap, items_html
        )
    }

    fn children(&self) -> Vec<&dyn Widget> {
        self.children.iter().map(|b| b.as_ref()).collect()
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(9000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
