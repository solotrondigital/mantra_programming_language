//! Image display, fit, and overlay components.

use crate::core::widget::{BuildContext, Widget};
use crate::theme::shape::BorderRadius;

/// Object-fit modes for responsive images.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFit {
    Cover,
    Contain,
    Fill,
    None,
}

impl ImageFit {
    pub fn to_css(&self) -> &'static str {
        match self {
            ImageFit::Cover => "cover",
            ImageFit::Contain => "contain",
            ImageFit::Fill => "fill",
            ImageFit::None => "none",
        }
    }
}

/// MantraImage with responsive constraints, border radius, and fallback handling.
#[derive(Debug, Clone)]
pub struct MantraImage {
    pub id: String,
    pub src: String,
    pub alt: String,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub fit: ImageFit,
    pub radius: BorderRadius,
}

impl MantraImage {
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            id: format!("img_{}", uuid_simple()),
            src: src.into(),
            alt: "Image".to_string(),
            width: None,
            height: None,
            fit: ImageFit::Cover,
            radius: BorderRadius::ZERO,
        }
    }

    pub fn with_alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = alt.into();
        self
    }

    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    pub fn with_radius(mut self, radius: BorderRadius) -> Self {
        self.radius = radius;
        self
    }
}

impl Widget for MantraImage {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "MantraImage"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let width_css = self.width.map(|w| format!("width:{}px;", w)).unwrap_or_else(|| "width:100%;".to_string());
        let height_css = self.height.map(|h| format!("height:{}px;", h)).unwrap_or_else(|| "height:auto;".to_string());

        format!(
            r#"<img id="{}" src="{}" alt="{}" style="{}{};object-fit:{};border-radius:{};display:block;" loading="lazy"/>"#,
            self.id, self.src, self.alt, width_css, height_css, self.fit.to_css(), self.radius.to_css()
        )
    }
}

pub fn circular_image(src: impl Into<String>, size: f64) -> MantraImage {
    MantraImage::new(src)
        .with_size(size, size)
        .with_radius(BorderRadius::circular(size / 2.0))
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(5000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

