use mantra_widgets::prelude::*;

#[test]
fn test_box_constraints_calculation() {
    let unconstrained = BoxConstraints::unconstrained();
    assert!(!unconstrained.is_tight());

    let tight = BoxConstraints::tight(Size::new(200.0, 100.0));
    assert!(tight.is_tight());
    assert_eq!(tight.constrain_width(500.0), 200.0);
    assert_eq!(tight.constrain_height(20.0), 100.0);

    let deflated = tight.deflate(20.0, 10.0);
    assert_eq!(deflated.min_width, 180.0);
    assert_eq!(deflated.min_height, 90.0);
}

#[test]
fn test_edge_insets_css() {
    let all = EdgeInsets::all(16.0);
    assert_eq!(all.to_css_string(), "16px");

    let sym = EdgeInsets::symmetric(8.0, 16.0);
    assert_eq!(sym.to_css_string(), "8px 16px");

    let only = EdgeInsets::only(10.0, 20.0, 30.0, 40.0);
    assert_eq!(only.to_css_string(), "10px 20px 30px 40px");
    assert_eq!(only.horizontal(), 60.0);
    assert_eq!(only.vertical(), 40.0);
}

#[test]
fn test_observable_state() {
    let counter = Observable::new(0);
    let captured = std::sync::Arc::new(std::sync::Mutex::new(0));
    let cap_clone = captured.clone();

    counter.subscribe(move |val| {
        let mut l = cap_clone.lock().unwrap();
        *l = *val;
    });

    counter.set(42);
    assert_eq!(counter.get(), 42);
    assert_eq!(*captured.lock().unwrap(), 42);

    counter.update(|v| *v += 10);
    assert_eq!(counter.get(), 52);
    assert_eq!(*captured.lock().unwrap(), 52);
}

#[test]
fn test_event_dispatch() {
    let ev = Event::click("btn_submit");
    assert_eq!(ev.event_type, EventType::Click);
    assert_eq!(ev.target_id, "btn_submit");

    let input_ev = Event::input("txt_email", "admin@mantra.dev");
    assert_eq!(input_ev.event_type, EventType::Input);
    assert_eq!(input_ev.value.as_deref(), Some("admin@mantra.dev"));
}

