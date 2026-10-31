//! Responsive property resolution and media query representations.

use super::breakpoints::{ScreenCategory, ScreenSize};

/// Encapsulates values that vary depending on screen category.
#[derive(Debug, Clone, PartialEq)]
pub struct ResponsiveValue<T> {
    pub mobile: T,
    pub tablet: Option<T>,
    pub desktop: Option<T>,
}

impl<T: Clone> ResponsiveValue<T> {
    pub fn new(mobile: T) -> Self {
        Self { mobile, tablet: None, desktop: None }
    }

    pub fn with_tablet(mut self, tablet: T) -> Self {
        self.tablet = Some(tablet);
        self
    }

    pub fn with_desktop(mut self, desktop: T) -> Self {
        self.desktop = Some(desktop);
        self
    }

    pub fn resolve<'a>(&'a self, screen: &ScreenSize) -> &'a T {
        match screen.category() {
            ScreenCategory::Mobile => &self.mobile,
            ScreenCategory::Tablet => self.tablet.as_ref().unwrap_or(&self.mobile),
            ScreenCategory::Desktop | ScreenCategory::WideScreen => {
                self.desktop.as_ref().or(self.tablet.as_ref()).unwrap_or(&self.mobile)
            }
        }
    }
}

/// Dynamic media query simulator and inspector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MediaQuery {
    pub size: ScreenSize,
    pub device_pixel_ratio: f64,
}

impl MediaQuery {
    pub fn standard_mobile() -> Self {
        Self {
            size: ScreenSize::new(390.0, 844.0), // iPhone 14/15
            device_pixel_ratio: 3.0,
        }
    }

    pub fn standard_tablet() -> Self {
        Self {
            size: ScreenSize::new(820.0, 1180.0), // iPad Air
            device_pixel_ratio: 2.0,
        }
    }

    pub fn standard_desktop() -> Self {
        Self {
            size: ScreenSize::new(1440.0, 900.0), // MacBook Air
            device_pixel_ratio: 2.0,
        }
    }
}

