//! Avatar and user profile components.

use crate::core::widget::{BuildContext, Widget};
use crate::theme::shape::ShapeVariant;

/// Status indicators for users (Online, Busy, Away, Offline).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnlineStatus {
    Online,
    Busy,
    Away,
    Offline,
}

impl OnlineStatus {
    pub fn color(&self) -> &'static str {
        match self {
            OnlineStatus::Online => "#10b981",
            OnlineStatus::Busy => "#ef4444",
            OnlineStatus::Away => "#f59e0b",
            OnlineStatus::Offline => "#94a3b8",
        }
    }
}

/// Avatar component supporting images, initials, and presence indicators.
#[derive(Debug, Clone)]
pub struct Avatar {
    pub id: String,
    pub size: f64,
    pub image_url: Option<String>,
    pub initials: Option<String>,
    pub shape: ShapeVariant,
    pub status: Option<OnlineStatus>,
}

impl Avatar {
    pub fn new(size: f64) -> Self {
        Self {
            id: format!("avatar_{}", uuid_simple()),
            size,
            image_url: None,
            initials: None,
            shape: ShapeVariant::Circle,
            status: None,
        }
    }

    pub fn with_image(mut self, url: impl Into<String>) -> Self {
        self.image_url = Some(url.into());
        self
    }

    pub fn with_initials(mut self, initials: impl Into<String>) -> Self {
        self.initials = Some(initials.into());
        self
    }

    pub fn with_shape(mut self, shape: ShapeVariant) -> Self {
        self.shape = shape;
        self
    }

    pub fn with_status(mut self, status: OnlineStatus) -> Self {
        self.status = Some(status);
        self
    }
}

impl Widget for Avatar {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Avatar"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let radius = self.shape.to_border_radius().to_css();
        let status_html = if let Some(st) = self.status {
            format!(
                r#"<span class="mantra-avatar-status" style="position:absolute;bottom:0;right:0;width:{:.0}px;height:{:.0}px;background-color:{};border:2px solid #fff;border-radius:50%;"></span>"#,
                self.size * 0.28,
                self.size * 0.28,
                st.color()
            )
        } else {
            String::new()
        };

        let inner_html = if let Some(url) = &self.image_url {
            format!(
                r#"<img src="{}" alt="Avatar" style="width:100%;height:100%;object-fit:cover;border-radius:{};" onerror="this.style.display='none'"/>"#,
                url, radius
            )
        } else if let Some(inits) = &self.initials {
            format!(
                r#"<span style="font-size:{:.0}px;font-weight:600;color:#fff;">{}</span>"#,
                self.size * 0.4,
                inits
            )
        } else {
            r#"<span style="font-size:20px;">👤</span>"#.to_string()
        };

        format!(
            r#"<div id="{}" class="mantra-avatar" style="position:relative;display:inline-flex;align-items:center;justify-content:center;width:{:.0}px;height:{:.0}px;border-radius:{};background:var(--mantra-primary,#4f46e5);flex-shrink:0;">
                {}
                {}
            </div>"#,
            self.id, self.size, self.size, radius, inner_html, status_html
        )
    }
}

pub fn circle_avatar(size: f64, initials: impl Into<String>) -> Avatar {
    Avatar::new(size).with_initials(initials).with_shape(ShapeVariant::Circle)
}

pub fn rounded_avatar(size: f64, initials: impl Into<String>) -> Avatar {
    Avatar::new(size).with_initials(initials).with_shape(ShapeVariant::Rounded)
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(3000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

