//! High-level facade for sending emails from Mantra.

use crate::message::EmailMessage;
use crate::smtp::SmtpClient;

/// Global helper function to dispatch an email via SMTP.
pub fn send_email_full(
    from: &str,
    to: &str,
    cc: &str,
    bcc: &str,
    subject: &str,
    body: &str,
    server: &str,
    port: u16,
    username: &str,
    password: &str,
    use_tls: bool,
) -> Result<String, String> {
    let mut msg = EmailMessage::new();
    msg.set_from(from);
    msg.set_to_str(to);
    msg.set_cc_str(cc);
    msg.set_bcc_str(bcc);
    msg.set_subject(subject);
    msg.set_body(body);

    let client = SmtpClient::new(server, port)
        .with_credentials(username, password)
        .with_tls(use_tls);

    client.send(&msg)
}

/// Convenience function to quickly send an email to a single or comma-separated list of recipients.
pub fn send_email_simple(
    from: &str,
    to: &str,
    subject: &str,
    body: &str,
    server: &str,
    port: u16,
) -> Result<String, String> {
    send_email_full(from, to, "", "", subject, body, server, port, "", "", true)
}
