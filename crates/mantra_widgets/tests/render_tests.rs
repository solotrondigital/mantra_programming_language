use mantra_widgets::prelude::*;

#[test]
fn test_html_element_builder() {
    let elem = HtmlElement::div()
        .with_id("test_div")
        .add_class("box")
        .add_class("shadow")
        .with_style("color", "#4f46e5")
        .with_content("Hello Mantra");

    let out = elem.render();
    assert!(out.contains(r#"id="test_div""#));
    assert!(out.contains(r#"class="box shadow""#));
    assert!(out.contains(r#"style="color: #4f46e5;""#));
    assert!(out.contains("Hello Mantra"));
}

#[test]
fn test_widget_tree_full_document_generation() {
    let tree = WidgetTree::new(Box::new(Container::new().with_child(Heading::h1("Mantra App"))));

    assert_eq!(tree.count_nodes(), 2);

    let doc = tree.render_full_document("My App", "/* Custom CSS */", false);
    assert!(doc.contains("<!DOCTYPE html>"));
    assert!(doc.contains("<title>My App</title>"));
    assert!(doc.contains("Mantra App"));
    assert!(doc.contains("mantra-theme-light"));
}

#[test]
fn test_platform_bridge_script() {
    let script = PlatformBridge::client_script();
    assert!(script.contains("window.MantraUI"));
    assert!(script.contains("MantraAndroidBridge"));
    assert!(script.contains("webkit.messageHandlers.mantraBridge"));
}

