//! # Mantra Widgets
//!
//! Official comprehensive UI component library, design system, and responsive rendering engine
//! for the Mantra Programming Language.

pub mod accessibility;
pub mod components;
pub mod core;
pub mod render;
pub mod responsive;
pub mod theme;

pub mod prelude {
    pub use crate::accessibility::*;
    pub use crate::components::*;
    pub use crate::core::constraints::*;
    pub use crate::core::event::*;
    pub use crate::core::layout::*;
    pub use crate::core::state::*;
    pub use crate::core::tree::*;
    pub use crate::core::widget::*;
    pub use crate::render::*;
    pub use crate::responsive::*;
    pub use crate::theme::colors::*;
    pub use crate::theme::elevation::*;
    pub use crate::theme::shape::*;
    pub use crate::theme::theme::*;
    pub use crate::theme::typography::*;
}
