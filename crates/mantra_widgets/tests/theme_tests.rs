use mantra_widgets::prelude::*;

#[test]
fn test_color_parsing_and_css() {
    let hex6 = Color::from_hex("#4f46e5");
    assert_eq!(hex6.r, 0x4f);
    assert_eq!(hex6.g, 0x46);
    assert_eq!(hex6.b, 0xe5);
    assert_eq!(hex6.to_css(), "#4f46e5");

    let rgba = Color::from_rgba(255, 0, 0, 0.5);
    assert_eq!(rgba.to_css(), "rgba(255, 0, 0, 0.50)");
}

#[test]
fn test_theme_css_variables() {
    let light = MantraTheme::light();
    let css = light.to_css_variables();
    assert!(css.contains("--mantra-primary: #4f46e5;"));
    assert!(css.contains("--mantra-bg: #f8fafc;"));

    let dark = MantraTheme::dark();
    let dark_css = dark.to_css_variables();
    assert!(dark.is_dark);
    assert!(dark_css.contains(".mantra-theme-dark"));
}

#[test]
fn test_elevation_shadows() {
    assert_eq!(Elevation::None.to_box_shadow(false), "none");
    assert!(Elevation::Medium.to_box_shadow(false).contains("rgba(0, 0, 0, 0.1)"));
    assert!(Elevation::Medium.to_box_shadow(true).contains("rgba(0, 0, 0, 0.4)"));
}

