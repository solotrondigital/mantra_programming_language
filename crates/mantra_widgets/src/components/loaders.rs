//! Progress bars, circular spinners, and shimmering skeleton loaders.

use crate::core::widget::{BuildContext, Widget};

/// Circular progress spinner.
#[derive(Debug, Clone)]
pub struct CircularLoader {
    pub id: String,
    pub size: f64,
    pub stroke_width: f64,
    pub color: &'static str,
}

impl CircularLoader {
    pub fn new() -> Self {
        Self {
            id: format!("loader_{}", uuid_simple()),
            size: 32.0,
            stroke_width: 3.0,
            color: "var(--mantra-primary, #4f46e5)",
        }
    }

    pub fn with_size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }
}

impl Default for CircularLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for CircularLoader {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "CircularLoader"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        format!(
            r#"<div id="{}" class="mantra-circular-loader" style="width:{}px;height:{}px;border:{}px solid #e2e8f0;border-top-color:{};border-radius:50%;animation:mantra-spin 0.8s linear infinite;display:inline-block;" role="progressbar"></div>"#,
            self.id, self.size, self.size, self.stroke_width, self.color
        )
    }
}

/// Linear progress bar.
#[derive(Debug, Clone)]
pub struct ProgressBar {
    pub id: String,
    pub progress: f64, // 0.0 to 1.0
    pub height: f64,
    pub color: &'static str,
}

impl ProgressBar {
    pub fn new(progress: f64) -> Self {
        Self {
            id: format!("prog_{}", uuid_simple()),
            progress: progress.clamp(0.0, 1.0),
            height: 6.0,
            color: "var(--mantra-primary, #4f46e5)",
        }
    }
}

impl Widget for ProgressBar {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "ProgressBar"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let pct = (self.progress * 100.0).round();
        format!(
            r#"<div id="{}" class="mantra-progress-bar" style="width:100%;height:{}px;background:#e2e8f0;border-radius:9999px;overflow:hidden;" role="progressbar" aria-valuenow="{}" aria-valuemin="0" aria-valuemax="100">
                <div style="width:{}%;height:100%;background:{};border-radius:9999px;transition:width 0.3s ease;"></div>
            </div>"#,
            self.id, self.height, pct, pct, self.color
        )
    }
}

/// Shimmer placeholder loader for content skeletons.
#[derive(Debug, Clone)]
pub struct Shimmer {
    pub id: String,
    pub width: String,
    pub height: f64,
    pub border_radius: f64,
}

impl Shimmer {
    pub fn new(width: impl Into<String>, height: f64) -> Self {
        Self {
            id: format!("shim_{}", uuid_simple()),
            width: width.into(),
            height,
            border_radius: 8.0,
        }
    }
}

impl Widget for Shimmer {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Shimmer"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        format!(
            r#"<div id="{}" class="mantra-shimmer" style="width:{};height:{}px;border-radius:{}px;background:linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);background-size:200% 100%;animation:mantra-shimmer 1.5s infinite;"></div>"#,
            self.id, self.width, self.height, self.border_radius
        )
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(11000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
