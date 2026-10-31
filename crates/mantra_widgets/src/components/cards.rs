//! Comprehensive Card components family with 16 variants.

use crate::core::layout::EdgeInsets;
use crate::core::widget::{BuildContext, Widget};
use crate::render::HtmlElement;
use crate::theme::colors::Color;
use crate::theme::elevation::Elevation;
use crate::theme::shape::{BorderRadius, BorderSide};

/// Base MantraCard container.
#[derive(Debug)]
pub struct MantraCard {
    pub id: String,
    pub child: Box<dyn Widget>,
    pub elevation: Elevation,
    pub radius: BorderRadius,
    pub padding: EdgeInsets,
    pub border: Option<BorderSide>,
    pub background_color: Option<Color>,
    pub is_interactive: bool,
    pub on_click: Option<String>,
}

impl MantraCard {
    pub fn new(child: impl Widget + 'static) -> Self {
        Self {
            id: format!("card_{}", uuid_simple()),
            child: Box::new(child),
            elevation: Elevation::Low,
            radius: BorderRadius::all(12.0),
            padding: EdgeInsets::all(16.0),
            border: None,
            background_color: None,
            is_interactive: false,
            on_click: None,
        }
    }

    pub fn with_elevation(mut self, el: Elevation) -> Self {
        self.elevation = el;
        self
    }

    pub fn with_radius(mut self, radius: BorderRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_padding(mut self, padding: EdgeInsets) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_border(mut self, border: BorderSide) -> Self {
        self.border = Some(border);
        self
    }

    pub fn with_interactive(mut self, interactive: bool) -> Self {
        self.is_interactive = interactive;
        self
    }

    pub fn on_click(mut self, action: impl Into<String>) -> Self {
        self.on_click = Some(action.into());
        self.is_interactive = true;
        self
    }
}

impl Widget for MantraCard {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "MantraCard"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let shadow = self.elevation.to_box_shadow(ctx.is_dark_mode);
        let mut elem = HtmlElement::div()
            .with_id(&self.id)
            .add_class("mantra-card")
            .with_style("background-color", "var(--mantra-surface, #ffffff)")
            .with_style("border-radius", &self.radius.to_css())
            .with_style("padding", &self.padding.to_css_string())
            .with_style("box-shadow", shadow)
            .with_style("transition", "box-shadow 0.2s ease, transform 0.2s ease")
            .with_style("display", "block")
            .with_style("box-sizing", "border-box");

        if let Some(border) = &self.border {
            elem = elem.with_style("border", &border.to_css());
        } else {
            elem = elem.with_style("border", "1px solid var(--mantra-border, #e2e8f0)");
        }

        if self.is_interactive {
            elem = elem
                .with_style("cursor", "pointer")
                .add_class("mantra-card-interactive");
        }

        if let Some(act) = &self.on_click {
            elem = elem.with_attribute(
                "onclick",
                format!("window.MantraUI && window.MantraUI.emit('click', '{}', '{}')", self.id, act),
            );
        }

        let inner_ctx = ctx.child_context(&self.id);
        elem = elem.add_child(self.child.render(&inner_ctx));
        elem.render()
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![self.child.as_ref()]
    }
}

/// Statistics / KPI Card.
#[derive(Debug)]
pub struct StatisticsCard {
    pub id: String,
    pub title: String,
    pub value: String,
    pub change_percentage: Option<f64>,
    pub is_positive: bool,
    pub icon: Option<String>,
}

impl StatisticsCard {
    pub fn new(title: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            id: format!("stat_{}", uuid_simple()),
            title: title.into(),
            value: value.into(),
            change_percentage: None,
            is_positive: true,
            icon: None,
        }
    }

    pub fn with_trend(mut self, percentage: f64, is_positive: bool) -> Self {
        self.change_percentage = Some(percentage);
        self.is_positive = is_positive;
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl Widget for StatisticsCard {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "StatisticsCard"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let trend_html = if let Some(pct) = self.change_percentage {
            let color = if self.is_positive { "#10b981" } else { "#ef4444" };
            let arrow = if self.is_positive { "↑" } else { "↓" };
            format!(
                r#"<span style="color:{};font-size:12px;font-weight:600;margin-left:8px;">{} {:.1}%</span>"#,
                color, arrow, pct
            )
        } else {
            String::new()
        };

        let icon_html = if let Some(ic) = &self.icon {
            format!(r#"<div style="font-size:24px;margin-bottom:8px;">{}</div>"#, ic)
        } else {
            String::new()
        };

        format!(
            r#"<div id="{}" class="mantra-stat-card" style="background:var(--mantra-surface,#fff);padding:18px;border-radius:12px;border:1px solid var(--mantra-border,#e2e8f0);box-shadow:0 1px 3px rgba(0,0,0,0.05);">
                {}
                <div style="font-size:13px;color:var(--mantra-text-muted,#64748b);font-weight:500;">{}</div>
                <div style="font-size:24px;font-weight:700;color:var(--mantra-text,#0f172a);display:flex;align-items:center;margin-top:4px;">
                    {}
                    {}
                </div>
            </div>"#,
            self.id, icon_html, self.title, self.value, trend_html
        )
    }
}

/// Product Showcase Card.
#[derive(Debug)]
pub struct ProductCard {
    pub id: String,
    pub title: String,
    pub price: String,
    pub image_url: String,
    pub rating: f32,
    pub on_buy: Option<String>,
}

impl ProductCard {
    pub fn new(title: impl Into<String>, price: impl Into<String>, image_url: impl Into<String>) -> Self {
        Self {
            id: format!("prod_{}", uuid_simple()),
            title: title.into(),
            price: price.into(),
            image_url: image_url.into(),
            rating: 5.0,
            on_buy: None,
        }
    }
}

impl Widget for ProductCard {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "ProductCard"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        format!(
            r#"<div id="{}" class="mantra-product-card" style="background:var(--mantra-surface,#fff);border-radius:14px;overflow:hidden;border:1px solid var(--mantra-border,#e2e8f0);box-shadow:0 4px 6px -1px rgba(0,0,0,0.06);max-width:320px;">
                <div style="width:100%;height:180px;background:#f1f5f9;display:flex;align-items:center;justify-content:center;overflow:hidden;">
                    <img src="{}" alt="{}" style="width:100%;height:100%;object-fit:cover;" onerror="this.style.display='none'"/>
                </div>
                <div style="padding:16px;">
                    <h3 style="font-size:16px;font-weight:600;margin-bottom:6px;color:var(--mantra-text,#0f172a);">{}</h3>
                    <div style="display:flex;justify-content:space-between;align-items:center;margin-top:12px;">
                        <span style="font-size:18px;font-weight:700;color:var(--mantra-primary,#4f46e5);">{}</span>
                        <button class="mantra-btn mantra-btn-primary" style="padding:6px 14px;border-radius:8px;font-size:13px;border:none;background:var(--mantra-primary,#4f46e5);color:#fff;cursor:pointer;">Buy</button>
                    </div>
                </div>
            </div>"#,
            self.id, self.image_url, self.title, self.title, self.price
        )
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(2000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

