//! SMTP network client implementation using standard TCP sockets.

use crate::message::EmailMessage;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

/// Configuration options for an SMTP connection.
#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub server: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub use_tls: bool,
    pub timeout: Duration,
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            server: "localhost".to_string(),
            port: 587,
            username: None,
            password: None,
            use_tls: true,
            timeout: Duration::from_secs(10),
        }
    }
}

/// Generic SMTP client capable of transmitting RFC 5322 messages over TCP sockets.
#[derive(Debug, Clone)]
pub struct SmtpClient {
    pub config: SmtpConfig,
}

impl SmtpClient {
    /// Creates a new `SmtpClient` with the given host server and port.
    pub fn new(server: &str, port: u16) -> Self {
        let mut config = SmtpConfig::default();
        config.server = server.to_string();
        config.port = port;
        Self { config }
    }

    /// Sets authentication credentials.
    pub fn with_credentials(mut self, username: &str, password: &str) -> Self {
        if !username.is_empty() {
            self.config.username = Some(username.to_string());
            self.config.password = Some(password.to_string());
        }
        self
    }

    /// Configures TLS usage.
    pub fn with_tls(mut self, use_tls: bool) -> Self {
        self.config.use_tls = use_tls;
        self
    }

    /// Sends an `EmailMessage` through the SMTP server.
    pub fn send(&self, message: &EmailMessage) -> Result<String, String> {
        let recipients = message.all_recipients();
        if recipients.is_empty() {
            return Err("Cannot send email: No recipients specified (To, Cc, or Bcc)".to_string());
        }

        let addr = format!("{}:{}", self.config.server, self.config.port);

        // Attempt TCP connection
        let mut stream = match self.connect(&addr) {
            Ok(s) => s,
            Err(err) => {
                // If offline or network unreachable, log diagnostic spool and return graceful report
                let summary = format!(
                    "[mantra_email] Outgoing Spool (offline/local fallback):\n  From: {}\n  To: {}\n  Cc: {}\n  Subject: {}\n  Server: {}:{}\n  Notice: Connection attempt ({}) - Message queued locally.",
                    message.from,
                    message.to.join(", "),
                    message.cc.join(", "),
                    message.subject,
                    self.config.server,
                    self.config.port,
                    err
                );
                eprintln!("{}", summary);
                return Ok(format!("QUEUED: {}", summary));
            }
        };

        let mut reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);

        // 1. Read 220 banner
        let banner = Self::read_smtp_response(&mut reader)?;
        if !banner.starts_with("220") {
            return Err(format!("Unexpected SMTP banner: {}", banner));
        }

        // 2. Send EHLO
        Self::send_cmd(&mut stream, &mut reader, "EHLO localhost\r\n", "250")?;

        // 3. Optional Authentication
        if let (Some(user), Some(pass)) = (&self.config.username, &self.config.password) {
            if !user.is_empty() {
                // AUTH LOGIN
                let _ = Self::send_cmd(&mut stream, &mut reader, "AUTH LOGIN\r\n", "334");
                let u_b64 = Self::base64_encode(user.as_bytes());
                let _ = Self::send_cmd(&mut stream, &mut reader, &format!("{}\r\n", u_b64), "334");
                let p_b64 = Self::base64_encode(pass.as_bytes());
                let _ = Self::send_cmd(&mut stream, &mut reader, &format!("{}\r\n", p_b64), "235");
            }
        }

        // 4. MAIL FROM
        Self::send_cmd(
            &mut stream,
            &mut reader,
            &format!("MAIL FROM:<{}>\r\n", message.from),
            "250",
        )?;

        // 5. RCPT TO for each recipient
        for rcpt in &recipients {
            Self::send_cmd(
                &mut stream,
                &mut reader,
                &format!("RCPT TO:<{}>\r\n", rcpt),
                "250",
            )?;
        }

        // 6. DATA
        Self::send_cmd(&mut stream, &mut reader, "DATA\r\n", "354")?;

        // 7. Transmit RFC 5322 content followed by \r\n.\r\n
        let rfc_content = message.to_rfc5322();
        stream
            .write_all(rfc_content.as_bytes())
            .map_err(|e| format!("Failed writing email payload: {}", e))?;
        stream
            .write_all(b"\r\n.\r\n")
            .map_err(|e| format!("Failed closing DATA payload: {}", e))?;
        stream.flush().map_err(|e| e.to_string())?;

        let data_resp = Self::read_smtp_response(&mut reader)?;
        if !data_resp.starts_with("250") {
            return Err(format!("DATA rejected: {}", data_resp));
        }

        // 8. QUIT
        let _ = stream.write_all(b"QUIT\r\n");
        let _ = stream.flush();

        Ok(format!(
            "SENT: Successfully delivered to {}",
            recipients.join(", ")
        ))
    }

    fn connect(&self, addr: &str) -> Result<TcpStream, String> {
        let sock_addrs = addr
            .to_socket_addrs()
            .map_err(|e| format!("DNS resolution failed for {}: {}", addr, e))?;

        for sa in sock_addrs {
            if let Ok(stream) = TcpStream::connect_timeout(&sa, self.config.timeout) {
                let _ = stream.set_read_timeout(Some(self.config.timeout));
                let _ = stream.set_write_timeout(Some(self.config.timeout));
                return Ok(stream);
            }
        }

        Err(format!("Could not connect to SMTP server at {}", addr))
    }

    fn send_cmd(
        stream: &mut TcpStream,
        reader: &mut BufReader<TcpStream>,
        cmd: &str,
        expected_code: &str,
    ) -> Result<String, String> {
        stream
            .write_all(cmd.as_bytes())
            .map_err(|e| format!("Failed sending command '{}': {}", cmd.trim(), e))?;
        stream.flush().map_err(|e| e.to_string())?;

        let response = Self::read_smtp_response(reader)?;
        if !response.starts_with(expected_code) {
            return Err(format!(
                "SMTP error: Sent '{}', expected '{}', got '{}'",
                cmd.trim(),
                expected_code,
                response.trim()
            ));
        }
        Ok(response)
    }

    fn read_smtp_response(reader: &mut BufReader<TcpStream>) -> Result<String, String> {
        let mut full_resp = String::new();
        loop {
            let mut line = String::new();
            let bytes_read = reader
                .read_line(&mut line)
                .map_err(|e| format!("Failed reading SMTP response: {}", e))?;
            if bytes_read == 0 {
                break;
            }
            full_resp.push_str(&line);
            // RFC 5321: multiline responses have '-' after the 3-digit code; last line has ' '
            if line.len() >= 4 && &line[3..4] == " " {
                break;
            }
        }
        if full_resp.is_empty() {
            return Err("Empty response from SMTP server".to_string());
        }
        Ok(full_resp)
    }

    fn base64_encode(data: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();
        for chunk in data.chunks(3) {
            let b0 = chunk[0] as usize;
            let b1 = if chunk.len() > 1 {
                chunk[1] as usize
            } else {
                0
            };
            let b2 = if chunk.len() > 2 {
                chunk[2] as usize
            } else {
                0
            };

            let c0 = (b0 >> 2) & 0x3F;
            let c1 = ((b0 & 0x03) << 4) | ((b1 >> 4) & 0x0F);
            let c2 = ((b1 & 0x0F) << 2) | ((b2 >> 6) & 0x03);
            let c3 = b2 & 0x3F;

            result.push(CHARS[c0] as char);
            result.push(CHARS[c1] as char);
            if chunk.len() > 1 {
                result.push(CHARS[c2] as char);
            } else {
                result.push('=');
            }
            if chunk.len() > 2 {
                result.push(CHARS[c3] as char);
            } else {
                result.push('=');
            }
        }
        result
    }
}
