//! Responsive screen breakpoints and device categories.

/// Device viewport categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenCategory {
    Mobile,
    Tablet,
    Desktop,
    WideScreen,
}

/// Device orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

/// Viewport dimensions and characteristics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenSize {
    pub width: f64,
    pub height: f64,
}

impl ScreenSize {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn orientation(&self) -> Orientation {
        if self.width >= self.height {
            Orientation::Landscape
        } else {
            Orientation::Portrait
        }
    }

    pub fn category(&self) -> ScreenCategory {
        if self.width < 768.0 {
            ScreenCategory::Mobile
        } else if self.width < 1024.0 {
            ScreenCategory::Tablet
        } else if self.width <= 1440.0 {
            ScreenCategory::Desktop
        } else {
            ScreenCategory::WideScreen
        }
    }

    pub fn is_mobile(&self) -> bool {
        self.category() == ScreenCategory::Mobile
    }

    pub fn is_tablet(&self) -> bool {
        self.category() == ScreenCategory::Tablet
    }

    pub fn is_desktop(&self) -> bool {
        matches!(self.category(), ScreenCategory::Desktop | ScreenCategory::WideScreen)
    }
}
