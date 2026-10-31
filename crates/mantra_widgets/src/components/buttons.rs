//! Complete Button component family with 19 variants, loading states, and custom styling.

use crate::accessibility::{Role, Semantics};
use crate::core::widget::{BuildContext, Widget};
use crate::render::HtmlElement;
use crate::theme::shape::ShapeVariant;

/// Visual button style types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Info,
    Outline,
    Ghost,
    Gradient,
}

impl ButtonType {
    pub fn to_css_class(&self) -> &'static str {
        match self {
            ButtonType::Primary => "mantra-btn-primary",
            ButtonType::Secondary => "mantra-btn-secondary",
            ButtonType::Success => "mantra-btn-success",
            ButtonType::Warning => "mantra-btn-warning",
            ButtonType::Danger => "mantra-btn-danger",
            ButtonType::Info => "mantra-btn-info",
            ButtonType::Outline => "mantra-btn-outline",
            ButtonType::Ghost => "mantra-btn-ghost",
            ButtonType::Gradient => "mantra-btn-gradient",
        }
    }
}

/// Sizing variants for buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
    XLarge,
}

impl ButtonSize {
    pub fn padding(&self) -> &'static str {
        match self {
            ButtonSize::Small => "6px 12px",
            ButtonSize::Medium => "10px 20px",
            ButtonSize::Large => "14px 28px",
            ButtonSize::XLarge => "18px 36px",
        }
    }

    pub fn font_size(&self) -> f64 {
        match self {
            ButtonSize::Small => 12.0,
            ButtonSize::Medium => 14.0,
            ButtonSize::Large => 16.0,
            ButtonSize::XLarge => 18.0,
        }
    }
}

/// Core MantraButton component.
#[derive(Debug, Clone)]
pub struct MantraButton {
    pub id: String,
    pub text: String,
    pub button_type: ButtonType,
    pub size: ButtonSize,
    pub shape: ShapeVariant,
    pub icon: Option<String>,
    pub is_loading: bool,
    pub is_disabled: bool,
    pub on_click: Option<String>,
}

impl MantraButton {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: format!("btn_{}", uuid_simple()),
            text: text.into(),
            button_type: ButtonType::Primary,
            size: ButtonSize::Medium,
            shape: ShapeVariant::Rounded,
            icon: None,
            is_loading: false,
            is_disabled: false,
            on_click: None,
        }
    }

    pub fn primary(text: impl Into<String>) -> Self {
        Self::new(text).with_type(ButtonType::Primary)
    }

    pub fn secondary(text: impl Into<String>) -> Self {
        Self::new(text).with_type(ButtonType::Secondary)
    }

    pub fn success(text: impl Into<String>) -> Self {
        Self::new(text).with_type(ButtonType::Success)
    }

    pub fn warning(text: impl Into<String>) -> Self {
        Self::new(text).with_type(ButtonType::Warning)
    }

    pub fn danger(text: impl Into<String>) -> Self {
        Self::new(text).with_type(ButtonType::Danger)
    }

    pub fn outline(text: impl Into<String>) -> Self {
        Self::new(text).with_type(ButtonType::Outline)
    }

    pub fn with_type(mut self, t: ButtonType) -> Self {
        self.button_type = t;
        self
    }

    pub fn with_size(mut self, s: ButtonSize) -> Self {
        self.size = s;
        self
    }

    pub fn with_shape(mut self, s: ShapeVariant) -> Self {
        self.shape = s;
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_loading(mut self, loading: bool) -> Self {
        self.is_loading = loading;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn on_click(mut self, action: impl Into<String>) -> Self {
        self.on_click = Some(action.into());
        self
    }
}

impl Widget for MantraButton {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "MantraButton"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let mut elem = HtmlElement::button()
            .with_id(&self.id)
            .add_class("mantra-btn")
            .add_class(self.button_type.to_css_class())
            .with_style("padding", self.size.padding())
            .with_style("font-size", &format!("{}px", self.size.font_size()))
            .with_style("border-radius", &self.shape.to_border_radius().to_css())
            .with_style("cursor", if self.is_disabled || self.is_loading { "not-allowed" } else { "pointer" })
            .with_style("opacity", if self.is_disabled { "0.6" } else { "1.0" })
            .with_style("display", "inline-flex")
            .with_style("align-items", "center")
            .with_style("justify-content", "center")
            .with_style("gap", "8px")
            .with_style("font-weight", "600")
            .with_style("border", "none")
            .with_style("transition", "all 0.2s ease-in-out")
            .with_semantics(Semantics {
                role: Some(Role::Button),
                label: Some(self.text.clone()),
                is_disabled: self.is_disabled || self.is_loading,
                ..Default::default()
            });

        if let Some(act) = &self.on_click {
            if !self.is_disabled && !self.is_loading {
                elem = elem.with_attribute("onclick", format!("window.MantraUI && window.MantraUI.emit('click', '{}', '{}')", self.id, act));
            }
        }

        if self.is_loading {
            let spinner = r#"<span class="mantra-spinner" style="width:14px;height:14px;border:2px solid currentColor;border-top-color:transparent;border-radius:50%;animation:mantra-spin 0.6s linear infinite;display:inline-block;"></span>"#;
            elem = elem.add_child(spinner);
        } else if let Some(icon) = &self.icon {
            elem = elem.add_child(format!(r#"<span class="mantra-btn-icon">{}</span>"#, icon));
        }

        elem = elem.add_child(format!(r#"<span class="mantra-btn-text">{}</span>"#, self.text));
        elem.render()
    }
}

// Additional Button Specializations:

/// Elevated Button with realistic elevation shadow.
pub fn elevated_button(text: impl Into<String>) -> MantraButton {
    MantraButton::new(text).with_type(ButtonType::Primary)
}

/// Filled Button.
pub fn filled_button(text: impl Into<String>) -> MantraButton {
    MantraButton::new(text).with_type(ButtonType::Primary)
}

/// Outlined Button.
pub fn outlined_button(text: impl Into<String>) -> MantraButton {
    MantraButton::new(text).with_type(ButtonType::Outline)
}

/// Text Button / Flat Button.
pub fn text_button(text: impl Into<String>) -> MantraButton {
    MantraButton::new(text).with_type(ButtonType::Ghost)
}

/// Rounded Button.
pub fn rounded_button(text: impl Into<String>) -> MantraButton {
    MantraButton::new(text).with_shape(ShapeVariant::Pill)
}

/// Square Button.
pub fn square_button(text: impl Into<String>) -> MantraButton {
    MantraButton::new(text).with_shape(ShapeVariant::Square)
}

/// Icon Button with circular outline.
pub fn icon_button(icon: impl Into<String>) -> MantraButton {
    MantraButton::new("")
        .with_icon(icon)
        .with_shape(ShapeVariant::Circle)
        .with_type(ButtonType::Ghost)
}

/// Social Button (Google, Apple, GitHub, Twitter).
pub fn social_button(provider: &str, text: impl Into<String>) -> MantraButton {
    let icon = match provider.to_lowercase().as_str() {
        "google" => "🌐",
        "apple" => "🍎",
        "github" => "🐙",
        _ => "🔗",
    };
    MantraButton::new(text).with_icon(icon).with_type(ButtonType::Outline)
}

/// Floating Action Button (FAB).
pub fn floating_action_button(icon: impl Into<String>) -> MantraButton {
    MantraButton::new("")
        .with_icon(icon)
        .with_shape(ShapeVariant::Circle)
        .with_size(ButtonSize::Large)
        .with_type(ButtonType::Primary)
}

/// Loading Button wrapper.
pub fn loading_button(text: impl Into<String>, is_loading: bool) -> MantraButton {
    MantraButton::new(text).with_loading(is_loading)
}

/// Gradient Button.
pub fn gradient_button(text: impl Into<String>) -> MantraButton {
    MantraButton::new(text).with_type(ButtonType::Gradient)
}

/// Button Group container.
#[derive(Debug, Clone)]
pub struct ButtonGroup {
    pub id: String,
    pub buttons: Vec<MantraButton>,
}

impl ButtonGroup {
    pub fn new(buttons: Vec<MantraButton>) -> Self {
        Self {
            id: format!("btngrp_{}", uuid_simple()),
            buttons,
        }
    }
}

impl Widget for ButtonGroup {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "ButtonGroup"
    }

    fn render(&self, ctx: &BuildContext) -> String {
        let children_html = self.buttons.iter().map(|b| b.render(ctx)).collect::<Vec<_>>().join("");
        format!(
            r#"<div id="{}" class="mantra-btn-group" style="display:inline-flex;border-radius:8px;overflow:hidden;box-shadow:0 1px 2px rgba(0,0,0,0.05);">{}</div>"#,
            self.id, children_html
        )
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(1000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

