//! Unified event handling and dispatch system for Mantra Widgets.

use std::fmt;

/// Type of UI event received from platform view or browser DOM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    Click,
    DoubleClick,
    LongPress,
    Input,
    Change,
    Submit,
    Focus,
    Blur,
    HoverEnter,
    HoverLeave,
    TouchStart,
    TouchMove,
    TouchEnd,
    Scroll,
    KeyDown,
    KeyUp,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::Click => write!(f, "click"),
            EventType::DoubleClick => write!(f, "dblclick"),
            EventType::LongPress => write!(f, "longpress"),
            EventType::Input => write!(f, "input"),
            EventType::Change => write!(f, "change"),
            EventType::Submit => write!(f, "submit"),
            EventType::Focus => write!(f, "focus"),
            EventType::Blur => write!(f, "blur"),
            EventType::HoverEnter => write!(f, "mouseenter"),
            EventType::HoverLeave => write!(f, "mouseleave"),
            EventType::TouchStart => write!(f, "touchstart"),
            EventType::TouchMove => write!(f, "touchmove"),
            EventType::TouchEnd => write!(f, "touchend"),
            EventType::Scroll => write!(f, "scroll"),
            EventType::KeyDown => write!(f, "keydown"),
            EventType::KeyUp => write!(f, "keyup"),
        }
    }
}

/// Structured cross-platform UI event payload.
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub event_type: EventType,
    pub target_id: String,
    pub value: Option<String>,
    pub client_x: f64,
    pub client_y: f64,
    pub key: Option<String>,
    pub timestamp_ms: u64,
}

impl Event {
    pub fn new(event_type: EventType, target_id: impl Into<String>) -> Self {
        Self {
            event_type,
            target_id: target_id.into(),
            value: None,
            client_x: 0.0,
            client_y: 0.0,
            key: None,
            timestamp_ms: 0,
        }
    }

    pub fn click(target_id: impl Into<String>) -> Self {
        Self::new(EventType::Click, target_id)
    }

    pub fn input(target_id: impl Into<String>, value: impl Into<String>) -> Self {
        let mut ev = Self::new(EventType::Input, target_id);
        ev.value = Some(value.into());
        ev
    }

    pub fn change(target_id: impl Into<String>, value: impl Into<String>) -> Self {
        let mut ev = Self::new(EventType::Change, target_id);
        ev.value = Some(value.into());
        ev
    }
}

