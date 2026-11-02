//! Email message representation and RFC 5322 formatting.

use std::collections::HashMap;

/// Represents an email message with sender, recipients, headers, and body.
#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub from: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body: String,
    pub is_html: bool,
    pub headers: HashMap<String, String>,
}

impl EmailMessage {
    /// Creates a new empty `EmailMessage`.
    pub fn new() -> Self {
        Self {
            from: String::new(),
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            subject: String::new(),
            body: String::new(),
            is_html: false,
            headers: HashMap::new(),
        }
    }

    /// Creates an `EmailMessage` with common fields.
    pub fn simple(from: &str, to: &str, subject: &str, body: &str) -> Self {
        let mut msg = Self::new();
        msg.from = from.to_string();
        msg.set_to_str(to);
        msg.subject = subject.to_string();
        msg.body = body.to_string();
        msg
    }

    /// Sets the sender address.
    pub fn set_from(&mut self, from: &str) -> &mut Self {
        self.from = from.trim().to_string();
        self
    }

    /// Sets comma-separated To recipients.
    pub fn set_to_str(&mut self, to_str: &str) -> &mut Self {
        self.to = Self::parse_email_list(to_str);
        self
    }

    /// Adds a single To recipient.
    pub fn add_to(&mut self, email: &str) -> &mut Self {
        let clean = email.trim();
        if !clean.is_empty() && !self.to.iter().any(|e| e == clean) {
            self.to.push(clean.to_string());
        }
        self
    }

    /// Sets comma-separated CC recipients.
    pub fn set_cc_str(&mut self, cc_str: &str) -> &mut Self {
        self.cc = Self::parse_email_list(cc_str);
        self
    }

    /// Adds a single CC recipient.
    pub fn add_cc(&mut self, email: &str) -> &mut Self {
        let clean = email.trim();
        if !clean.is_empty() && !self.cc.iter().any(|e| e == clean) {
            self.cc.push(clean.to_string());
        }
        self
    }

    /// Sets comma-separated BCC recipients.
    pub fn set_bcc_str(&mut self, bcc_str: &str) -> &mut Self {
        self.bcc = Self::parse_email_list(bcc_str);
        self
    }

    /// Adds a single BCC recipient.
    pub fn add_bcc(&mut self, email: &str) -> &mut Self {
        let clean = email.trim();
        if !clean.is_empty() && !self.bcc.iter().any(|e| e == clean) {
            self.bcc.push(clean.to_string());
        }
        self
    }

    /// Sets the email subject line.
    pub fn set_subject(&mut self, subject: &str) -> &mut Self {
        self.subject = subject.to_string();
        self
    }

    /// Sets the text or HTML body.
    pub fn set_body(&mut self, body: &str) -> &mut Self {
        self.body = body.to_string();
        self
    }

    /// Toggles HTML formatting header.
    pub fn set_html(&mut self, is_html: bool) -> &mut Self {
        self.is_html = is_html;
        self
    }

    /// Adds a custom MIME header.
    pub fn set_header(&mut self, name: &str, value: &str) -> &mut Self {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    /// Collects all envelope recipients (To + CC + BCC).
    pub fn all_recipients(&self) -> Vec<String> {
        let mut all = Vec::new();
        for r in &self.to {
            if !all.contains(r) {
                all.push(r.clone());
            }
        }
        for r in &self.cc {
            if !all.contains(r) {
                all.push(r.clone());
            }
        }
        for r in &self.bcc {
            if !all.contains(r) {
                all.push(r.clone());
            }
        }
        all
    }

    /// Formats the email into RFC 5322 compliant text.
    pub fn to_rfc5322(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("From: {}\r\n", self.from));
        if !self.to.is_empty() {
            s.push_str(&format!("To: {}\r\n", self.to.join(", ")));
        }
        if !self.cc.is_empty() {
            s.push_str(&format!("Cc: {}\r\n", self.cc.join(", ")));
        }
        s.push_str(&format!("Subject: {}\r\n", self.subject));
        s.push_str("MIME-Version: 1.0\r\n");

        if self.is_html {
            s.push_str("Content-Type: text/html; charset=UTF-8\r\n");
        } else {
            s.push_str("Content-Type: text/plain; charset=UTF-8\r\n");
        }

        for (k, v) in &self.headers {
            s.push_str(&format!("{}: {}\r\n", k, v));
        }

        s.push_str("\r\n");
        s.push_str(&self.body);
        s
    }

    fn parse_email_list(s: &str) -> Vec<String> {
        s.split(&[',', ';'][..])
            .map(|e| e.trim().to_string())
            .filter(|e| !e.is_empty())
            .collect()
    }
}
