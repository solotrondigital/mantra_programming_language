//! ApiResponse — Standardised REST API response envelope.
use mantra_web::response::{Response, StatusCode, ContentType};

/// Pagination metadata for list responses.
#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: usize,
    pub page_size: usize,
    pub total: usize,
}

impl Pagination {
    pub fn new(page: usize, page_size: usize, total: usize) -> Self {
        Pagination { page, page_size, total }
    }
    pub fn to_json(&self) -> String {
        format!("{{\"page\":{},\"page_size\":{},\"total\":{},\"pages\":{}}}",
            self.page, self.page_size, self.total,
            (self.total + self.page_size - 1) / self.page_size)
    }
}

/// Structured API error.
#[derive(Debug, Clone)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

impl ApiError {
    pub fn new(code: &str, msg: &str) -> Self {
        ApiError { code: code.into(), message: msg.into() }
    }
    pub fn to_json(&self) -> String {
        format!("{{\"code\":\"{}\",\"message\":\"{}\"}}", self.code, self.message)
    }
}

/// Standard REST API response envelope: { success, data, error, pagination }.
#[derive(Debug)]
pub struct ApiResponse {
    pub status: StatusCode,
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<ApiError>,
    pub pagination: Option<Pagination>,
}

impl ApiResponse {
    pub fn ok(data: &str) -> Self {
        ApiResponse { status: StatusCode::Ok, success: true, data: Some(data.into()), error: None, pagination: None }
    }
    pub fn created(data: &str) -> Self {
        ApiResponse { status: StatusCode::Created, success: true, data: Some(data.into()), error: None, pagination: None }
    }
    pub fn list(data: &str, pagination: Pagination) -> Self {
        ApiResponse { status: StatusCode::Ok, success: true, data: Some(data.into()), error: None, pagination: Some(pagination) }
    }
    pub fn not_found(msg: &str) -> Self {
        ApiResponse { status: StatusCode::NotFound, success: false, data: None,
            error: Some(ApiError::new("NOT_FOUND", msg)), pagination: None }
    }
    pub fn bad_request(msg: &str) -> Self {
        ApiResponse { status: StatusCode::BadRequest, success: false, data: None,
            error: Some(ApiError::new("BAD_REQUEST", msg)), pagination: None }
    }
    pub fn method_not_allowed(msg: &str) -> Self {
        ApiResponse { status: StatusCode::BadRequest, success: false, data: None,
            error: Some(ApiError::new("METHOD_NOT_ALLOWED", msg)), pagination: None }
    }

    /// Serialise to JSON envelope string.
    pub fn to_json(&self) -> String {
        let data_part = match &self.data {
            Some(d) => format!("\"data\":{}", d),
            None    => "\"data\":null".into(),
        };
        let err_part = match &self.error {
            Some(e) => format!("\"error\":{}", e.to_json()),
            None    => "\"error\":null".into(),
        };
        let page_part = match &self.pagination {
            Some(p) => format!("\"pagination\":{}", p.to_json()),
            None    => "\"pagination\":null".into(),
        };
        format!("{{\"success\":{},{},{},{}}}", self.success, data_part, err_part, page_part)
    }

    /// Convert to a mantra_web Response.
    pub fn to_response(&self) -> Response {
        Response {
            status: self.status,
            content_type: ContentType::Json,
            body: self.to_json(),
            headers: vec![],
        }
    }
}
