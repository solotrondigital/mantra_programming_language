//! Accessibility semantics, ARIA roles, and screen reader annotations.

use std::fmt;

/// Standard W3C ARIA accessibility roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Button,
    Checkbox,
    Radio,
    Switch,
    Tab,
    TabList,
    TabPanel,
    Dialog,
    AlertDialog,
    Navigation,
    Banner,
    Main,
    Form,
    Search,
    Tooltip,
    ProgressBar,
    Slider,
    Img,
    Heading,
    List,
    ListItem,
    Alert,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Button => write!(f, "button"),
            Role::Checkbox => write!(f, "checkbox"),
            Role::Radio => write!(f, "radio"),
            Role::Switch => write!(f, "switch"),
            Role::Tab => write!(f, "tab"),
            Role::TabList => write!(f, "tablist"),
            Role::TabPanel => write!(f, "tabpanel"),
            Role::Dialog => write!(f, "dialog"),
            Role::AlertDialog => write!(f, "alertdialog"),
            Role::Navigation => write!(f, "navigation"),
            Role::Banner => write!(f, "banner"),
            Role::Main => write!(f, "main"),
            Role::Form => write!(f, "form"),
            Role::Search => write!(f, "search"),
            Role::Tooltip => write!(f, "tooltip"),
            Role::ProgressBar => write!(f, "progressbar"),
            Role::Slider => write!(f, "slider"),
            Role::Img => write!(f, "img"),
            Role::Heading => write!(f, "heading"),
            Role::List => write!(f, "list"),
            Role::ListItem => write!(f, "listitem"),
            Role::Alert => write!(f, "alert"),
        }
    }
}

/// ARIA semantics attributes for screen readers and assistive technology.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Semantics {
    pub role: Option<Role>,
    pub label: Option<String>,
    pub hint: Option<String>,
    pub is_expanded: Option<bool>,
    pub is_selected: Option<bool>,
    pub is_checked: Option<bool>,
    pub is_disabled: bool,
    pub is_required: bool,
    pub value_now: Option<f64>,
    pub value_min: Option<f64>,
    pub value_max: Option<f64>,
}

impl Semantics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn button(label: impl Into<String>) -> Self {
        Self {
            role: Some(Role::Button),
            label: Some(label.into()),
            ..Default::default()
        }
    }

    pub fn to_html_attributes(&self) -> String {
        let mut attrs = Vec::new();

        if let Some(r) = self.role {
            attrs.push(format!(r#"role="{}""#, r));
        }
        if let Some(l) = &self.label {
            attrs.push(format!(r#"aria-label="{}""#, l));
        }
        if let Some(h) = &self.hint {
            attrs.push(format!(r#"aria-description="{}""#, h));
        }
        if let Some(exp) = self.is_expanded {
            attrs.push(format!(r#"aria-expanded="{}""#, exp));
        }
        if let Some(sel) = self.is_selected {
            attrs.push(format!(r#"aria-selected="{}""#, sel));
        }
        if let Some(chk) = self.is_checked {
            attrs.push(format!(r#"aria-checked="{}""#, chk));
        }
        if self.is_disabled {
            attrs.push(r#"aria-disabled="true""#.to_string());
        }
        if self.is_required {
            attrs.push(r#"aria-required="true""#.to_string());
        }
        if let Some(vn) = self.value_now {
            attrs.push(format!(r#"aria-valuenow="{}""#, vn));
        }
        if let Some(vmin) = self.value_min {
            attrs.push(format!(r#"aria-valuemin="{}""#, vmin));
        }
        if let Some(vmax) = self.value_max {
            attrs.push(format!(r#"aria-valuemax="{}""#, vmax));
        }

        if attrs.is_empty() {
            String::new()
        } else {
            format!(" {}", attrs.join(" "))
        }
    }
}

