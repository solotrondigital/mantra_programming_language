//! Mantra design system color palette and color definitions.

use std::fmt;

/// Represents an RGBA or HEX color value.
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
    pub hex: String,
}

impl Color {
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0.0, hex: String::new() };

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: 1.0,
            hex: format!("#{:02x}{:02x}{:02x}", r, g, b),
        }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a: a.clamp(0.0, 1.0),
            hex: format!("#{:02x}{:02x}{:02x}", r, g, b),
        }
    }

    pub fn from_hex(hex_str: &str) -> Self {
        let clean = hex_str.trim_start_matches('#');
        let (r, g, b, a) = match clean.len() {
            6 => {
                let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(0);
                (r, g, b, 1.0)
            }
            8 => {
                let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(0);
                let a = u8::from_str_radix(&clean[6..8], 16).unwrap_or(255) as f32 / 255.0;
                (r, g, b, a)
            }
            3 => {
                let r = u8::from_str_radix(&format!("{}{}", &clean[0..1], &clean[0..1]), 16).unwrap_or(0);
                let g = u8::from_str_radix(&format!("{}{}", &clean[1..2], &clean[1..2]), 16).unwrap_or(0);
                let b = u8::from_str_radix(&format!("{}{}", &clean[2..3], &clean[2..3]), 16).unwrap_or(0);
                (r, g, b, 1.0)
            }
            _ => (0, 0, 0, 1.0),
        };

        Self {
            r,
            g,
            b,
            a,
            hex: format!("#{}", clean),
        }
    }

    pub fn to_css(&self) -> String {
        if (self.a - 1.0).abs() < f32::EPSILON {
            if !self.hex.is_empty() {
                self.hex.clone()
            } else {
                format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
            }
        } else {
            format!("rgba({}, {}, {}, {:.2})", self.r, self.g, self.b, self.a)
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css())
    }
}

/// Standardized color palette for Mantra applications.
#[derive(Debug, Clone)]
pub struct MantraPalette {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub info: Color,
    pub background: Color,
    pub surface: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub border: Color,
}

impl MantraPalette {
    pub fn light() -> Self {
        Self {
            primary: Color::from_hex("#4f46e5"),      // Indigo 600
            secondary: Color::from_hex("#64748b"),    // Slate 500
            accent: Color::from_hex("#06b6d4"),       // Cyan 500
            success: Color::from_hex("#10b981"),      // Emerald 500
            warning: Color::from_hex("#f59e0b"),      // Amber 500
            danger: Color::from_hex("#ef4444"),       // Red 500
            info: Color::from_hex("#0ea5e9"),         // Sky 500
            background: Color::from_hex("#f8fafc"),   // Slate 50
            surface: Color::from_hex("#ffffff"),      // White
            text_primary: Color::from_hex("#0f172a"), // Slate 900
            text_secondary: Color::from_hex("#64748b"),// Slate 500
            border: Color::from_hex("#e2e8f0"),       // Slate 200
        }
    }

    pub fn dark() -> Self {
        Self {
            primary: Color::from_hex("#6366f1"),      // Indigo 500
            secondary: Color::from_hex("#94a3b8"),    // Slate 400
            accent: Color::from_hex("#22d3ee"),       // Cyan 400
            success: Color::from_hex("#34d399"),      // Emerald 400
            warning: Color::from_hex("#fbbf24"),      // Amber 400
            danger: Color::from_hex("#f87171"),       // Red 400
            info: Color::from_hex("#38bdf8"),         // Sky 400
            background: Color::from_hex("#0f172a"),   // Slate 900
            surface: Color::from_hex("#1e293b"),      // Slate 800
            text_primary: Color::from_hex("#f8fafc"), // Slate 50
            text_secondary: Color::from_hex("#94a3b8"),// Slate 400
            border: Color::from_hex("#334155"),       // Slate 700
        }
    }
}

