//! Advanced composite components (EmptyState, PricingCard, SearchBar).

use crate::core::widget::{BuildContext, Widget};

/// Standard Empty State placeholder.
#[derive(Debug)]
pub struct EmptyState {
    pub id: String,
    pub icon: String,
    pub title: String,
    pub description: String,
    pub action: Option<Box<dyn Widget>>,
}

impl EmptyState {
    pub fn new(icon: impl Into<String>, title: impl Into<String>, desc: impl Into<String>) -> Self {
        Self {
            id: format!("empty_{}", uuid_simple()),
            icon: icon.into(),
            title: title.into(),
            description: desc.into(),
            action: None,
        }
    }

    pub fn with_action(mut self, action: impl Widget + 'static) -> Self {
        self.action = Some(Box::new(action));
        self
    }
}

impl Widget for EmptyState {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "EmptyState"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let action_html = self.action.as_ref().map(|a| a.render(ctx)).unwrap_or_default();

        format!(
            r#"<div id="{}" class="mantra-empty-state" style="text-align:center;padding:48px 24px;">
                <div style="font-size:48px;margin-bottom:16px;">{}</div>
                <h3 style="font-size:18px;font-weight:700;color:var(--mantra-text,#0f172a);margin-bottom:8px;">{}</h3>
                <p style="font-size:14px;color:var(--mantra-text-muted,#64748b);max-width:320px;margin:0 auto 20px auto;line-height:1.5;">{}</p>
                <div style="display:flex;justify-content:center;">{}</div>
            </div>"#,
            self.id, self.icon, self.title, self.description, action_html
        )
    }

    fn children(&self) -> Vec<&dyn Widget> {
        self.action.as_ref().map(|a| vec![a.as_ref()]).unwrap_or_default()
    }
}

/// SaaS Pricing Card with feature list and popular badge.
#[derive(Debug)]
pub struct PricingCard {
    pub id: String,
    pub tier: String,
    pub price: String,
    pub billing_period: String,
    pub features: Vec<String>,
    pub is_popular: bool,
    pub button_text: String,
}

impl PricingCard {
    pub fn new(tier: impl Into<String>, price: impl Into<String>, features: Vec<String>) -> Self {
        Self {
            id: format!("price_{}", uuid_simple()),
            tier: tier.into(),
            price: price.into(),
            billing_period: "/month".to_string(),
            features,
            is_popular: false,
            button_text: "Get Started".to_string(),
        }
    }

    pub fn with_popular(mut self, pop: bool) -> Self {
        self.is_popular = pop;
        self
    }
}

impl Widget for PricingCard {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "PricingCard"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let popular_badge = if self.is_popular {
            r#"<div style="background:var(--mantra-primary,#4f46e5);color:#fff;font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:1px;padding:4px 12px;border-radius:9999px;position:absolute;top:-12px;right:20px;">Most Popular</div>"#
        } else {
            ""
        };

        let border_css = if self.is_popular {
            "2px solid var(--mantra-primary, #4f46e5)"
        } else {
            "1px solid var(--mantra-border, #e2e8f0)"
        };

        let features_html = self
            .features
            .iter()
            .map(|f| {
                format!(
                    r#"<li style="display:flex;align-items:center;font-size:14px;color:var(--mantra-text,#0f172a);margin-bottom:10px;"><span style="color:#10b981;margin-right:8px;font-weight:700;">✓</span>{}</li>"#,
                    f
                )
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<div id="{}" class="mantra-pricing-card" style="position:relative;background:var(--mantra-surface,#fff);border-radius:16px;border:{};padding:32px 24px;box-shadow:0 4px 6px -1px rgba(0,0,0,0.05);max-width:320px;width:100%;box-sizing:border-box;">
                {}
                <h4 style="font-size:16px;font-weight:600;color:var(--mantra-text-muted,#64748b);margin-bottom:12px;">{}</h4>
                <div style="display:flex;align-items:baseline;margin-bottom:24px;">
                    <span style="font-size:36px;font-weight:800;color:var(--mantra-text,#0f172a);">{}</span>
                    <span style="font-size:14px;color:var(--mantra-text-muted,#64748b);margin-left:4px;">{}</span>
                </div>
                <ul style="list-style:none;padding:0;margin:0 0 28px 0;">
                    {}
                </ul>
                <button class="mantra-btn mantra-btn-primary" style="width:100%;padding:12px;border-radius:10px;font-size:14px;font-weight:600;border:none;background:var(--mantra-primary,#4f46e5);color:#fff;cursor:pointer;">{}</button>
            </div>"#,
            self.id, border_css, popular_badge, self.tier, self.price, self.billing_period, features_html, self.button_text
        )
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(16000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

