//! Dialogs, Modals, Bottom Sheets, Toasts, and Tooltips.

use crate::core::widget::{BuildContext, Widget};

/// Modal confirmation and alert dialog.
#[derive(Debug)]
pub struct AlertDialog {
    pub id: String,
    pub title: String,
    pub message: String,
    pub confirm_text: String,
    pub cancel_text: Option<String>,
}

impl AlertDialog {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            id: format!("dlg_{}", uuid_simple()),
            title: title.into(),
            message: message.into(),
            confirm_text: "OK".to_string(),
            cancel_text: Some("Cancel".to_string()),
        }
    }
}

impl Widget for AlertDialog {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "AlertDialog"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let cancel_btn = self
            .cancel_text
            .as_ref()
            .map(|c| {
                format!(
                    r#"<button class="mantra-btn mantra-btn-ghost" style="padding:8px 16px;border-radius:8px;font-size:14px;border:none;background:transparent;color:var(--mantra-text-muted,#64748b);cursor:pointer;">{}</button>"#,
                    c
                )
            })
            .unwrap_or_default();

        format!(
            r#"<div id="{}" class="mantra-modal-backdrop" style="position:fixed;inset:0;background:rgba(0,0,0,0.5);display:flex;align-items:center;justify-content:center;z-index:9999;">
                <div class="mantra-dialog" style="background:var(--mantra-surface,#fff);border-radius:16px;padding:24px;width:90%;max-width:400px;box-shadow:0 20px 25px -5px rgba(0,0,0,0.2);">
                    <h3 style="font-size:18px;font-weight:700;margin-bottom:8px;color:var(--mantra-text,#0f172a);">{}</h3>
                    <p style="font-size:14px;color:var(--mantra-text-muted,#64748b);line-height:1.5;margin-bottom:24px;">{}</p>
                    <div style="display:flex;justify-content:flex-end;gap:12px;">
                        {}
                        <button class="mantra-btn mantra-btn-primary" style="padding:8px 16px;border-radius:8px;font-size:14px;border:none;background:var(--mantra-primary,#4f46e5);color:#fff;font-weight:600;cursor:pointer;">{}</button>
                    </div>
                </div>
            </div>"#,
            self.id, self.title, self.message, cancel_btn, self.confirm_text
        )
    }
}

/// Floating Toast notification.
#[derive(Debug, Clone)]
pub struct Toast {
    pub id: String,
    pub message: String,
    pub is_success: bool,
}

impl Toast {
    pub fn success(msg: impl Into<String>) -> Self {
        Self {
            id: format!("toast_{}", uuid_simple()),
            message: msg.into(),
            is_success: true,
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            id: format!("toast_{}", uuid_simple()),
            message: msg.into(),
            is_success: false,
        }
    }
}

impl Widget for Toast {
    fn id(&self) -> &str {
        &self.id
    }

    fn widget_name(&self) -> &'static str {
        "Toast"
    }

    fn render(&self, _ctx: &BuildContext) -> String {
        let (icon, bg) = if self.is_success {
            ("✓", "#10b981")
        } else {
            ("✕", "#ef4444")
        };

        format!(
            r#"<div id="{}" class="mantra-toast" style="position:fixed;bottom:24px;left:50%;transform:translateX(-50%);background:#1e293b;color:#fff;padding:12px 20px;border-radius:10px;box-shadow:0 10px 15px -3px rgba(0,0,0,0.3);display:flex;align-items:center;gap:10px;font-size:14px;font-weight:500;z-index:10000;">
                <span style="background:{};width:20px;height:20px;border-radius:50%;display:flex;align-items:center;justify-content:center;font-size:12px;font-weight:700;">{}</span>
                <span>{}</span>
            </div>"#,
            self.id, bg, icon, self.message
        )
    }
}

fn uuid_simple() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(10000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
