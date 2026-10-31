use mantra_widgets::prelude::*;

#[test]
fn test_mantra_button_rendering() {
    let btn = MantraButton::primary("Get Started")
        .with_size(ButtonSize::Large)
        .with_shape(ShapeVariant::Pill)
        .on_click("navigate_home");

    let ctx = BuildContext::root(false);
    let html = btn.render(&ctx);

    assert!(html.contains("mantra-btn-primary"));
    assert!(html.contains("Get Started"));
    assert!(html.contains("border-radius: 9999px;"));
    assert!(html.contains("navigate_home"));
}

#[test]
fn test_mantra_card_rendering() {
    let card = MantraCard::new(Text::new("Card Content"))
        .with_elevation(Elevation::High)
        .with_interactive(true);

    let ctx = BuildContext::root(false);
    let html = card.render(&ctx);

    assert!(html.contains("mantra-card"));
    assert!(html.contains("Card Content"));
    assert!(html.contains("mantra-card-interactive"));
}

#[test]
fn test_statistics_card_rendering() {
    let stat = StatisticsCard::new("Total Revenue", "$124,500")
        .with_trend(12.5, true)
        .with_icon("💰");

    let ctx = BuildContext::root(false);
    let html = stat.render(&ctx);

    assert!(html.contains("Total Revenue"));
    assert!(html.contains("$124,500"));
    assert!(html.contains("12.5%"));
    assert!(html.contains("💰"));
}

#[test]
fn test_avatar_rendering() {
    let av = circle_avatar(48.0, "ST").with_status(OnlineStatus::Online);
    let ctx = BuildContext::root(false);
    let html = av.render(&ctx);

    assert!(html.contains("ST"));
    assert!(html.contains("width:48px;height:48px;"));
    assert!(html.contains("#10b981")); // Online status color
}

#[test]
fn test_layout_row_and_column() {
    let row = Row::new(vec![
        Box::new(Text::new("Left")),
        Box::new(Text::new("Right")),
    ]).with_gap(16.0);

    let ctx = BuildContext::root(false);
    let html = row.render(&ctx);

    assert!(html.contains("display:flex;flex-direction:row;"));
    assert!(html.contains("gap:16px;"));
    assert!(html.contains("Left"));
    assert!(html.contains("Right"));
}

#[test]
fn test_forms_inputs() {
    let field = TextField::new("Enter password...")
        .with_label("Password")
        .as_password()
        .on_change("validate_pass");

    let ctx = BuildContext::root(false);
    let html = field.render(&ctx);

    assert!(html.contains(r#"type="password""#));
    assert!(html.contains("Password"));
    assert!(html.contains("validate_pass"));

    let sw = Switch::new(true).with_label("Push Notifications");
    let sw_html = sw.render(&ctx);
    assert!(sw_html.contains("Push Notifications"));
}

#[test]
fn test_loaders() {
    let loader = CircularLoader::new().with_size(40.0);
    let ctx = BuildContext::root(false);
    let html = loader.render(&ctx);
    assert!(html.contains("width:40px;height:40px;"));
    assert!(html.contains("mantra-spin"));

    let prog = ProgressBar::new(0.75);
    let prog_html = prog.render(&ctx);
    assert!(prog_html.contains("width:75%;"));
}

