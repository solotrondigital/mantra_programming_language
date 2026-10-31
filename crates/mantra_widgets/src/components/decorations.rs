//! Box decorations, gradients, and custom visual borders.

use crate::theme::colors::Color;
use crate::theme::elevation::Elevation;
use crate::theme::shape::{BorderRadius, BorderSide};

/// Full visual decoration specification for containers and surfaces.
#[derive(Debug, Clone, PartialEq)]
pub struct BoxDecoration {
    pub color: Option<Color>,
    pub border: Option<BorderSide>,
    pub radius: BorderRadius,
    pub elevation: Elevation,
    pub gradient: Option<String>,
}

impl BoxDecoration {
    pub fn new() -> Self {
        Self {
            color: None,
            border: None,
            radius: BorderRadius::ZERO,
            elevation: Elevation::None,
            gradient: None,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_radius(mut self, radius: BorderRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_elevation(mut self, elevation: Elevation) -> Self {
        self.elevation = elevation;
        self
    }
}

impl Default for BoxDecoration {
    fn default() -> Self {
        Self::new()
    }
}

