//! MantraTheme system with light, dark, Cupertino, Material, and custom palettes.

use super::colors::MantraPalette;
use super::shape::BorderRadius;
use super::typography::MantraTypography;

/// Overall design theme configuration.
#[derive(Debug, Clone)]
pub struct MantraTheme {
    pub name: String,
    pub is_dark: bool,
    pub palette: MantraPalette,
    pub typography: MantraTypography,
    pub default_radius: BorderRadius,
}

impl MantraTheme {
    pub fn light() -> Self {
        Self {
            name: "Mantra Light".to_string(),
            is_dark: false,
            palette: MantraPalette::light(),
            typography: MantraTypography::standard(),
            default_radius: BorderRadius::all(8.0),
        }
    }

    pub fn dark() -> Self {
        Self {
            name: "Mantra Dark".to_string(),
            is_dark: true,
            palette: MantraPalette::dark(),
            typography: MantraTypography::standard(),
            default_radius: BorderRadius::all(8.0),
        }
    }

    pub fn cupertino() -> Self {
        let mut t = Self::light();
        t.name = "Mantra Cupertino".to_string();
        t.palette.primary = super::colors::Color::from_hex("#007aff"); // iOS Blue
        t.default_radius = BorderRadius::all(10.0);
        t
    }

    pub fn material() -> Self {
        let mut t = Self::light();
        t.name = "Mantra Material".to_string();
        t.palette.primary = super::colors::Color::from_hex("#6200ee"); // Material Purple
        t.default_radius = BorderRadius::all(4.0);
        t
    }

    /// Emits standardized CSS custom properties (variables) for this theme.
    pub fn to_css_variables(&self) -> String {
        let p = &self.palette;
        format!(
            r#":root, .mantra-theme-{} {{
    --mantra-primary: {};
    --mantra-secondary: {};
    --mantra-accent: {};
    --mantra-success: {};
    --mantra-warning: {};
    --mantra-danger: {};
    --mantra-info: {};
    --mantra-bg: {};
    --mantra-surface: {};
    --mantra-text: {};
    --mantra-text-muted: {};
    --mantra-border: {};
    --mantra-radius: {};
}}"#,
            if self.is_dark { "dark" } else { "light" },
            p.primary.to_css(),
            p.secondary.to_css(),
            p.accent.to_css(),
            p.success.to_css(),
            p.warning.to_css(),
            p.danger.to_css(),
            p.info.to_css(),
            p.background.to_css(),
            p.surface.to_css(),
            p.text_primary.to_css(),
            p.text_secondary.to_css(),
            p.border.to_css(),
            self.default_radius.to_css(),
        )
    }
}

