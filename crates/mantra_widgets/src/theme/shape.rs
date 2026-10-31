//! Shape variants, border radius, and edge borders.

use super::colors::Color;

/// Border radius geometry for corners.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorderRadius {
    pub top_left: f64,
    pub top_right: f64,
    pub bottom_right: f64,
    pub bottom_left: f64,
}

impl BorderRadius {
    pub const ZERO: BorderRadius = BorderRadius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 };

    pub fn all(radius: f64) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }

    pub fn circular(radius: f64) -> Self {
        Self::all(radius)
    }

    pub fn only(top_left: f64, top_right: f64, bottom_right: f64, bottom_left: f64) -> Self {
        Self { top_left, top_right, bottom_right, bottom_left }
    }

    pub fn pill() -> Self {
        Self::all(9999.0)
    }

    pub fn to_css(&self) -> String {
        if self.top_left == self.top_right && self.top_left == self.bottom_right && self.top_left == self.bottom_left {
            format!("{}px", self.top_left)
        } else {
            format!("{}px {}px {}px {}px", self.top_left, self.top_right, self.bottom_right, self.bottom_left)
        }
    }
}

/// Common visual shape geometry variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeVariant {
    Square,
    Rounded,
    Pill,
    Circle,
}

impl ShapeVariant {
    pub fn to_border_radius(&self) -> BorderRadius {
        match self {
            ShapeVariant::Square => BorderRadius::ZERO,
            ShapeVariant::Rounded => BorderRadius::all(8.0),
            ShapeVariant::Pill => BorderRadius::pill(),
            ShapeVariant::Circle => BorderRadius::all(9999.0),
        }
    }
}

/// Border style configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct BorderSide {
    pub width: f64,
    pub color: Color,
    pub style: &'static str,
}

impl BorderSide {
    pub fn solid(width: f64, color: Color) -> Self {
        Self { width, color, style: "solid" }
    }

    pub fn dashed(width: f64, color: Color) -> Self {
        Self { width, color, style: "dashed" }
    }

    pub fn dotted(width: f64, color: Color) -> Self {
        Self { width, color, style: "dotted" }
    }

    pub fn to_css(&self) -> String {
        format!("{}px {} {}", self.width, self.style, self.color.to_css())
    }
}

