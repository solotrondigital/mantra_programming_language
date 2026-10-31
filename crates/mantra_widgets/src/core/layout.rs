//! Layout geometry, alignments, edge insets, and flex properties.

use std::fmt;

/// Defines insets for padding, margins, and border widths across 4 edges.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeInsets {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl EdgeInsets {
    pub const ZERO: EdgeInsets = EdgeInsets { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 };

    pub fn all(value: f64) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub fn symmetric(vertical: f64, horizontal: f64) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    pub fn only(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self { top, right, bottom, left }
    }

    pub fn horizontal(&self) -> f64 {
        self.left + self.right
    }

    pub fn vertical(&self) -> f64 {
        self.top + self.bottom
    }

    pub fn to_css_string(&self) -> String {
        if self.top == self.bottom && self.right == self.left {
            if self.top == self.right {
                format!("{}px", self.top)
            } else {
                format!("{}px {}px", self.top, self.right)
            }
        } else {
            format!("{}px {}px {}px {}px", self.top, self.right, self.bottom, self.left)
        }
    }
}

/// Point in a 2D Cartesian coordinate plane relative to an element bounds.
/// (-1.0, -1.0) is top-left, (0.0, 0.0) is center, (1.0, 1.0) is bottom-right.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Alignment {
    pub x: f64,
    pub y: f64,
}

impl Alignment {
    pub const TOP_LEFT: Alignment = Alignment { x: -1.0, y: -1.0 };
    pub const TOP_CENTER: Alignment = Alignment { x: 0.0, y: -1.0 };
    pub const TOP_RIGHT: Alignment = Alignment { x: 1.0, y: -1.0 };
    pub const CENTER_LEFT: Alignment = Alignment { x: -1.0, y: 0.0 };
    pub const CENTER: Alignment = Alignment { x: 0.0, y: 0.0 };
    pub const CENTER_RIGHT: Alignment = Alignment { x: 1.0, y: 0.0 };
    pub const BOTTOM_LEFT: Alignment = Alignment { x: -1.0, y: 1.0 };
    pub const BOTTOM_CENTER: Alignment = Alignment { x: 0.0, y: 1.0 };
    pub const BOTTOM_RIGHT: Alignment = Alignment { x: 1.0, y: 1.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Axis direction for layout flows (Flex, Wrap, Scroll).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Axis::Horizontal => write!(f, "row"),
            Axis::Vertical => write!(f, "column"),
        }
    }
}

/// Main axis distribution in flex layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl MainAxisAlignment {
    pub fn to_css(&self) -> &'static str {
        match self {
            MainAxisAlignment::Start => "flex-start",
            MainAxisAlignment::Center => "center",
            MainAxisAlignment::End => "flex-end",
            MainAxisAlignment::SpaceBetween => "space-between",
            MainAxisAlignment::SpaceAround => "space-around",
            MainAxisAlignment::SpaceEvenly => "space-evenly",
        }
    }
}

/// Cross axis alignment in flex layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl CrossAxisAlignment {
    pub fn to_css(&self) -> &'static str {
        match self {
            CrossAxisAlignment::Start => "flex-start",
            CrossAxisAlignment::Center => "center",
            CrossAxisAlignment::End => "flex-end",
            CrossAxisAlignment::Stretch => "stretch",
            CrossAxisAlignment::Baseline => "baseline",
        }
    }
}

/// Controls how multi-line flex items wrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl FlexWrap {
    pub fn to_css(&self) -> &'static str {
        match self {
            FlexWrap::NoWrap => "nowrap",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
        }
    }
}

