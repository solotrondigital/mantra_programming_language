use mantra_widgets::prelude::*;

#[test]
fn test_screen_size_categorization() {
    let mobile = ScreenSize::new(375.0, 667.0);
    assert_eq!(mobile.category(), ScreenCategory::Mobile);
    assert!(mobile.is_mobile());
    assert_eq!(mobile.orientation(), Orientation::Portrait);

    let tablet = ScreenSize::new(820.0, 1180.0);
    assert_eq!(tablet.category(), ScreenCategory::Tablet);
    assert!(tablet.is_tablet());

    let desktop = ScreenSize::new(1440.0, 900.0);
    assert_eq!(desktop.category(), ScreenCategory::Desktop);
    assert!(desktop.is_desktop());
    assert_eq!(desktop.orientation(), Orientation::Landscape);
}

#[test]
fn test_responsive_value_resolution() {
    let cols = ResponsiveValue::new(1)
        .with_tablet(2)
        .with_desktop(4);

    let mobile = ScreenSize::new(390.0, 844.0);
    let tablet = ScreenSize::new(800.0, 1200.0);
    let desktop = ScreenSize::new(1920.0, 1080.0);

    assert_eq!(*cols.resolve(&mobile), 1);
    assert_eq!(*cols.resolve(&tablet), 2);
    assert_eq!(*cols.resolve(&desktop), 4);
}

