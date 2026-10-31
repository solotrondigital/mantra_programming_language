//! Typography hierarchy, weights, and styles.

use super::colors::Color;
use std::fmt;

/// CSS font weights.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl FontWeight {
    pub fn to_numeric(&self) -> u16 {
        match self {
            FontWeight::Thin => 100,
            FontWeight::Light => 300,
            FontWeight::Regular => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Black => 900,
        }
    }
}

/// Text alignment within parent container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

impl fmt::Display for TextAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextAlign::Left => write!(f, "left"),
            TextAlign::Center => write!(f, "center"),
            TextAlign::Right => write!(f, "right"),
            TextAlign::Justify => write!(f, "justify"),
        }
    }
}

/// Detailed text styling properties.
#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    pub font_family: Option<String>,
    pub font_size: f64,
    pub font_weight: FontWeight,
    pub line_height: f64,
    pub letter_spacing: f64,
    pub color: Option<Color>,
    pub text_align: TextAlign,
}

impl TextStyle {
    pub fn new(size: f64, weight: FontWeight) -> Self {
        Self {
            font_family: None,
            font_size: size,
            font_weight: weight,
            line_height: 1.5,
            letter_spacing: 0.0,
            color: None,
            text_align: TextAlign::Left,
        }
    }

    pub fn to_css(&self) -> String {
        let mut css = format!(
            "font-size: {}px; font-weight: {}; line-height: {}; text-align: {};",
            self.font_size,
            self.font_weight.to_numeric(),
            self.line_height,
            self.text_align
        );

        if self.letter_spacing != 0.0 {
            css.push_str(&format!(" letter-spacing: {}px;", self.letter_spacing));
        }

        if let Some(c) = &self.color {
            css.push_str(&format!(" color: {};", c.to_css()));
        }

        if let Some(ff) = &self.font_family {
            css.push_str(&format!(" font-family: {};", ff));
        }

        css
    }
}

/// Mantra standard typography hierarchy.
#[derive(Debug, Clone)]
pub struct MantraTypography {
    pub display: TextStyle,
    pub headline: TextStyle,
    pub title: TextStyle,
    pub body_large: TextStyle,
    pub body: TextStyle,
    pub caption: TextStyle,
    pub code: TextStyle,
}

impl MantraTypography {
    pub fn standard() -> Self {
        Self {
            display: TextStyle::new(36.0, FontWeight::Bold),
            headline: TextStyle::new(28.0, FontWeight::Bold),
            title: TextStyle::new(20.0, FontWeight::SemiBold),
            body_large: TextStyle::new(16.0, FontWeight::Regular),
            body: TextStyle::new(14.0, FontWeight::Regular),
            caption: TextStyle::new(12.0, FontWeight::Medium),
            code: TextStyle {
                font_family: Some("SFMono-Regular, Menlo, Monaco, Consolas, monospace".to_string()),
                font_size: 13.0,
                font_weight: FontWeight::Regular,
                line_height: 1.4,
                letter_spacing: 0.0,
                color: None,
                text_align: TextAlign::Left,
            },
        }
    }
}

