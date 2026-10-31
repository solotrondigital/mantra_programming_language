//! Widget base traits, build context, and lifecycle abstractions.

use super::constraints::BoxConstraints;
use std::fmt::Debug;

/// Build context passed to `build` methods during component tree resolution.
#[derive(Debug, Clone)]
pub struct BuildContext {
    pub current_path: String,
    pub depth: usize,
    pub is_dark_mode: bool,
    pub constraints: BoxConstraints,
}

impl BuildContext {
    pub fn root(is_dark_mode: bool) -> Self {
        Self {
            current_path: "root".to_string(),
            depth: 0,
            is_dark_mode,
            constraints: BoxConstraints::unconstrained(),
        }
    }

    pub fn child_context(&self, id: &str) -> Self {
        Self {
            current_path: format!("{}/{}", self.current_path, id),
            depth: self.depth + 1,
            is_dark_mode: self.is_dark_mode,
            constraints: self.constraints,
        }
    }
}

/// Core trait implemented by all Mantra UI components.
pub trait Widget: Debug + Send + Sync {
    /// Returns the unique or assigned ID for this widget instance.
    fn id(&self) -> &str;

    /// Returns the type identifier of the widget (e.g. "MantraButton", "Container").
    fn widget_name(&self) -> &'static str;

    /// Compiles this widget into clean, semantic HTML5/CSS representation.
    fn render(&self, ctx: &BuildContext) -> String;

    /// Returns references to direct child widgets, if any.
    fn children(&self) -> Vec<&dyn Widget> {
        Vec::new()
    }
}

/// A widget that does not require mutable state.
pub trait StatelessWidget: Widget {
    fn build(&self, ctx: &BuildContext) -> String;
}

/// A widget that possesses mutable lifecycle state.
pub trait State<T: Widget>: Debug + Send + Sync {
    fn init_state(&mut self) {}
    fn build(&self, widget: &T, ctx: &BuildContext) -> String;
    fn dispose(&mut self) {}
}

