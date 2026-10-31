//! Carousel and image slider components.

use crate::core::widget::{BuildContext, Widget};

/// Touch-friendly swipeable carousel.
#[derive(Debug)]
pub struct Carousel {
    pub id: String,
    pub items: Vec<Box<dyn Widget>>,
    pub show_indicators: bool,
    pub autoplay: bool,
}

impl Carousel {
    pub fn new(items: Vec<Box<dyn Widget>>) -> Self {
        Self {
            id: format!("carousel_{}", uuid_simple()),
            items,
            show_indicators: true,
            autoplay: false,
        }
    }
}

impl Widget for Carousel {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Carousel"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let items_html = self
            .items
            .iter()
            .map(|item| {
                format!(
                    r#"<div class="mantra-carousel-item" style="flex:0 0 100%;scroll-snap-align:start;width:100%;">{}</div>"#,
                    item.render(ctx)
                )
            })
            .collect::<Vec<_>>()
            .join("");

        let indicators_html = if self.show_indicators && !self.items.is_empty() {
            let dots = (0..self.items.len())
                .map(|i| {
                    format!(
                        r#"<span style="width:8px;height:8px;border-radius:50%;background:{};display:inline-block;margin:0 4px;"></span>"#,
                        if i == 0 { "var(--mantra-primary, #4f46e5)" } else { "#cbd5e1" }
                    )
                })
                .collect::<Vec<_>>()
                .join("");
            format!(
                r#"<div style="position:absolute;bottom:10px;left:0;right:0;display:flex;justify-content:center;align-items:center;">{}</div>"#,
                dots
            )
        } else {
            String::new()
        };

        format!(
            r#"<div id="{}" class="mantra-carousel" style="position:relative;width:100%;overflow:hidden;border-radius:12px;">
                <div style="display:flex;overflow-x:auto;scroll-snap-type:x mandatory;scrollbar-width:none;-webkit-overflow-scrolling:touch;">
                    {}
                </div>
                {}
            </div>"#,
            self.id, items_html, indicators_html
        )
    }

    fn children(&self) -> Vec<&dyn Widget> {
        self.items.iter().map(|b| b.as_ref()).collect()
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(6000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

