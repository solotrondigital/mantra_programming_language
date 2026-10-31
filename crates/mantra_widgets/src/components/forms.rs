//! Forms and input controls (TextField, PasswordField, Checkbox, Switch, Dropdown).

use crate::core::widget::{BuildContext, Widget};

/// Standard textual input field.
#[derive(Debug, Clone)]
pub struct TextField {
    pub id: String,
    pub label: Option<String>,
    pub placeholder: String,
    pub value: String,
    pub is_password: bool,
    pub error_text: Option<String>,
    pub helper_text: Option<String>,
    pub is_disabled: bool,
    pub on_change: Option<String>,
}

impl TextField {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            id: format!("input_{}", uuid_simple()),
            label: None,
            placeholder: placeholder.into(),
            value: String::new(),
            is_password: false,
            error_text: None,
            helper_text: None,
            is_disabled: false,
            on_change: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn as_password(mut self) -> Self {
        self.is_password = true;
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error_text = Some(error.into());
        self
    }

    pub fn on_change(mut self, action: impl Into<String>) -> Self {
        self.on_change = Some(action.into());
        self
    }
}

impl Widget for TextField {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "TextField"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let input_type = if self.is_password { "password" } else { "text" };
        let border_color = if self.error_text.is_some() {
            "var(--mantra-danger, #ef4444)"
        } else {
            "var(--mantra-border, #cbd5e1)"
        };

        let label_html = if let Some(lbl) = &self.label {
            format!(
                r#"<label for="{}" style="display:block;font-size:13px;font-weight:600;margin-bottom:6px;color:var(--mantra-text,#0f172a);">{}</label>"#,
                self.id, lbl
            )
        } else {
            String::new()
        };

        let error_html = if let Some(err) = &self.error_text {
            format!(
                r#"<div style="font-size:12px;color:var(--mantra-danger,#ef4444);margin-top:4px;">{}</div>"#,
                err
            )
        } else if let Some(help) = &self.helper_text {
            format!(
                r#"<div style="font-size:12px;color:var(--mantra-text-muted,#64748b);margin-top:4px;">{}</div>"#,
                help
            )
        } else {
            String::new()
        };

        let on_input_attr = if let Some(act) = &self.on_change {
            format!(
                r#" oninput="window.MantraUI && window.MantraUI.emit('input', '{}', '{}|' + this.value)""#,
                self.id, act
            )
        } else {
            String::new()
        };

        format!(
            r#"<div class="mantra-form-field" style="margin-bottom:16px;">
                {}
                <input id="{}" type="{}" placeholder="{}" value="{}"{} style="width:100%;height:42px;padding:0 12px;font-size:14px;border:1px solid {};border-radius:8px;background:var(--mantra-surface,#fff);color:var(--mantra-text,#0f172a);outline:none;box-sizing:border-box;transition:border-color 0.2s;"/>
                {}
            </div>"#,
            label_html, self.id, input_type, self.placeholder, self.value, on_input_attr, border_color, error_html
        )
    }
}

/// Switch toggle button.
#[derive(Debug, Clone)]
pub struct Switch {
    pub id: String,
    pub is_checked: bool,
    pub label: Option<String>,
}

impl Switch {
    pub fn new(is_checked: bool) -> Self {
        Self {
            id: format!("switch_{}", uuid_simple()),
            is_checked,
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Widget for Switch {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Switch"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let (bg, knob_x) = if self.is_checked {
            ("var(--mantra-primary, #4f46e5)", "22px")
        } else {
            ("#cbd5e1", "2px")
        };

        let label_html = if let Some(l) = &self.label {
            format!(
                r#"<span style="margin-left:10px;font-size:14px;color:var(--mantra-text,#0f172a);user-select:none;">{}</span>"#,
                l
            )
        } else {
            String::new()
        };

        format!(
            r#"<label id="{}" class="mantra-switch" style="display:inline-flex;align-items:center;cursor:pointer;">
                <div style="width:44px;height:24px;background:{};border-radius:9999px;position:relative;transition:background 0.2s ease;">
                    <div style="width:20px;height:20px;background:#fff;border-radius:50%;position:absolute;top:2px;left:{};transition:left 0.2s ease;box-shadow:0 1px 3px rgba(0,0,0,0.2);"></div>
                </div>
                {}
            </label>"#,
            self.id, bg, knob_x, label_html
        )
    }
}

/// Checkbox control.
#[derive(Debug, Clone)]
pub struct Checkbox {
    pub id: String,
    pub is_checked: bool,
    pub label: Option<String>,
}

impl Checkbox {
    pub fn new(is_checked: bool) -> Self {
        Self {
            id: format!("chk_{}", uuid_simple()),
            is_checked,
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Widget for Checkbox {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Checkbox"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let (bg, border, check) = if self.is_checked {
            ("var(--mantra-primary, #4f46e5)", "none", "✓")
        } else {
            ("transparent", "2px solid #94a3b8", "")
        };

        let label_html = if let Some(l) = &self.label {
            format!(
                r#"<span style="margin-left:8px;font-size:14px;color:var(--mantra-text,#0f172a);user-select:none;">{}</span>"#,
                l
            )
        } else {
            String::new()
        };

        format!(
            r#"<label id="{}" class="mantra-checkbox" style="display:inline-flex;align-items:center;cursor:pointer;">
                <div style="width:18px;height:18px;border-radius:4px;background:{};border:{};display:flex;align-items:center;justify-content:center;color:#fff;font-size:12px;font-weight:700;">{}</div>
                {}
            </label>"#,
            self.id, bg, border, check, label_html
        )
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(8000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
