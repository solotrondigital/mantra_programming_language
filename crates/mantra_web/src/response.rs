//! Response — Fluent HTTP response builder.

/// Common HTTP status codes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    NoContent = 204,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Conflict = 409,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn text(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::NoContent => "No Content",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::Conflict => "Conflict",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
    pub fn code(&self) -> u16 { *self as u16 }
}

/// Content-Type helpers.
#[derive(Debug, Clone)]
pub enum ContentType {
    Json,
    Html,
    Text,
    Xml,
    OctetStream,
}

impl ContentType {
    pub fn mime(&self) -> &str {
        match self {
            ContentType::Json         => "application/json",
            ContentType::Html         => "text/html; charset=utf-8",
            ContentType::Text         => "text/plain",
            ContentType::Xml          => "application/xml",
            ContentType::OctetStream  => "application/octet-stream",
        }
    }
}

/// A fully built HTTP response.
#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub content_type: ContentType,
    pub body: String,
    pub headers: Vec<(String, String)>,
}

impl Response {
    pub fn ok(body: &str) -> Self {
        Response { status: StatusCode::Ok, content_type: ContentType::Json, body: body.into(), headers: vec![] }
    }
    pub fn created(body: &str) -> Self {
        Response { status: StatusCode::Created, content_type: ContentType::Json, body: body.into(), headers: vec![] }
    }
    pub fn not_found(msg: &str) -> Self {
        Response { status: StatusCode::NotFound, content_type: ContentType::Json,
            body: format!("{{\"error\":\"{}\"}}", msg), headers: vec![] }
    }
    pub fn bad_request(msg: &str) -> Self {
        Response { status: StatusCode::BadRequest, content_type: ContentType::Json,
            body: format!("{{\"error\":\"{}\"}}", msg), headers: vec![] }
    }
    pub fn to_http_string(&self) -> String {
        format!("HTTP {} {} | Content-Type: {} | Body: {}",
            self.status.code(), self.status.text(), self.content_type.mime(), self.body)
    }
}

/// Fluent builder for Response objects.
#[derive(Default)]
pub struct ResponseBuilder {
    status: Option<StatusCode>,
    content_type: Option<ContentType>,
    body: String,
    headers: Vec<(String, String)>,
}

impl ResponseBuilder {
    pub fn new() -> Self { ResponseBuilder::default() }
    pub fn status(mut self, s: StatusCode) -> Self { self.status = Some(s); self }
    pub fn content_type(mut self, ct: ContentType) -> Self { self.content_type = Some(ct); self }
    pub fn body(mut self, body: &str) -> Self { self.body = body.into(); self }
    pub fn header(mut self, k: &str, v: &str) -> Self { self.headers.push((k.into(), v.into())); self }
    pub fn build(self) -> Response {
        Response {
            status: self.status.unwrap_or(StatusCode::Ok),
            content_type: self.content_type.unwrap_or(ContentType::Json),
            body: self.body, headers: self.headers,
        }
    }
}
