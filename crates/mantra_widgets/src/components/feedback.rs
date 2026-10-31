//! Star ratings, reviews, and interactive feedback widgets.

use crate::core::widget::{BuildContext, Widget};

/// 5-Star Rating widget.
#[derive(Debug, Clone)]
pub struct StarRating {
    pub id: String,
    pub rating: f32,
    pub max_stars: usize,
    pub star_size: f64,
}

impl StarRating {
    pub fn new(rating: f32) -> Self {
        Self {
            id: format!("star_{}", uuid_simple()),
            rating,
            max_stars: 5,
            star_size: 20.0,
        }
    }
}

impl Widget for StarRating {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "StarRating"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let stars_html = (1..=self.max_stars)
            .map(|i| {
                let color = if (i as f32) <= self.rating { "#f59e0b" } else { "#cbd5e1" };
                format!(r#"<span style="color:{};font-size:{}px;margin-right:2px;">★</span>"#, color, self.star_size)
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<div id="{}" class="mantra-star-rating" style="display:inline-flex;align-items:center;">{}</div>"#,
            self.id, stars_html
        )
    }
}

/// Interactive Like button.
#[derive(Debug, Clone)]
pub struct LikeButton {
    pub id: String,
    pub is_liked: bool,
    pub count: usize,
}

impl LikeButton {
    pub fn new(count: usize, is_liked: bool) -> Self {
        Self {
            id: format!("like_{}", uuid_simple()),
            count,
            is_liked,
        }
    }
}

impl Widget for LikeButton {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "LikeButton"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let (icon, color) = if self.is_liked { ("❤️", "#ef4444") } else { ("🤍", "#64748b") };

        format!(
            r#"<button id="{}" class="mantra-like-btn" style="display:inline-flex;align-items:center;gap:6px;padding:6px 12px;border-radius:9999px;border:1px solid #e2e8f0;background:#fff;cursor:pointer;font-size:13px;font-weight:600;color:{};">
                <span>{}</span>
                <span>{}</span>
            </button>"#,
            self.id, color, icon, self.count
        )
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(14000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

