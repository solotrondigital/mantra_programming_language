//! Box constraints and sizing engine for Mantra Widgets.

/// Represents a 2D size with width and height.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub const ZERO: Size = Size { width: 0.0, height: 0.0 };
    pub const INFINITY: Size = Size { width: f64::INFINITY, height: f64::INFINITY };

    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn square(dimension: f64) -> Self {
        Self { width: dimension, height: dimension }
    }
}

/// Immutable layout constraints passed from parent widget to child widget.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxConstraints {
    pub min_width: f64,
    pub max_width: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl BoxConstraints {
    /// Creates unbounded constraints where child can be any size.
    pub fn unconstrained() -> Self {
        Self {
            min_width: 0.0,
            max_width: f64::INFINITY,
            min_height: 0.0,
            max_height: f64::INFINITY,
        }
    }

    /// Creates tight constraints where child must be exactly given size.
    pub fn tight(size: Size) -> Self {
        Self {
            min_width: size.width,
            max_width: size.width,
            min_height: size.height,
            max_height: size.height,
        }
    }

    /// Creates tight constraints for specific width and height.
    pub fn tight_for(width: Option<f64>, height: Option<f64>) -> Self {
        Self {
            min_width: width.unwrap_or(0.0),
            max_width: width.unwrap_or(f64::INFINITY),
            min_height: height.unwrap_or(0.0),
            max_height: height.unwrap_or(f64::INFINITY),
        }
    }

    /// Creates loose constraints where child can be from 0 up to given max size.
    pub fn loose(size: Size) -> Self {
        Self {
            min_width: 0.0,
            max_width: size.width,
            min_height: 0.0,
            max_height: size.height,
        }
    }

    /// Creates constraints that expand to match the maximum allowable dimensions.
    pub fn expand(width: Option<f64>, height: Option<f64>) -> Self {
        Self {
            min_width: width.unwrap_or(f64::INFINITY),
            max_width: width.unwrap_or(f64::INFINITY),
            min_height: height.unwrap_or(f64::INFINITY),
            max_height: height.unwrap_or(f64::INFINITY),
        }
    }

    /// Checks if width is tightly constrained.
    pub fn has_tight_width(&self) -> bool {
        self.min_width >= self.max_width
    }

    /// Checks if height is tightly constrained.
    pub fn has_tight_height(&self) -> bool {
        self.min_height >= self.max_height
    }

    /// Checks if both width and height are tightly constrained.
    pub fn is_tight(&self) -> bool {
        self.has_tight_width() && self.has_tight_height()
    }

    /// Checks if minimum constraints are zero and maximum is finite.
    pub fn is_loose(&self) -> bool {
        self.min_width == 0.0 && self.min_height == 0.0 && self.max_width.is_finite() && self.max_height.is_finite()
    }

    /// Clamps given width within min and max width constraints.
    pub fn constrain_width(&self, width: f64) -> f64 {
        width.clamp(self.min_width, self.max_width)
    }

    /// Clamps given height within min and max height constraints.
    pub fn constrain_height(&self, height: f64) -> f64 {
        height.clamp(self.min_height, self.max_height)
    }

    /// Clamps given size within constraints.
    pub fn constrain_size(&self, size: Size) -> Size {
        Size {
            width: self.constrain_width(size.width),
            height: self.constrain_height(size.height),
        }
    }

    /// Deflates constraints by insets (e.g. padding).
    pub fn deflate(&self, horizontal: f64, vertical: f64) -> Self {
        Self {
            min_width: (self.min_width - horizontal).max(0.0),
            max_width: (self.max_width - horizontal).max(0.0),
            min_height: (self.min_height - vertical).max(0.0),
            max_height: (self.max_height - vertical).max(0.0),
        }
    }
}

