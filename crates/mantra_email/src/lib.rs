//! mantra_email — Generic email package for the Mantra Programming Language.
//!
//! Provides generic SMTP transport, RFC 5322 message composition, To/Cc/Bcc distribution,
//! and high-level static mail dispatching.

pub mod client;
pub mod message;
pub mod smtp;

pub use client::{send_email_full, send_email_simple};
pub use message::EmailMessage;
pub use smtp::{SmtpClient, SmtpConfig};

/// Embedded Mantra source for package `mantra.email` / `mantra_email`.
pub const MANTRA_EMAIL_SOURCE: &str = include_str!("../lib/mantra_email.mantra");
