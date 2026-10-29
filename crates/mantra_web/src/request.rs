//! Request — Represents an incoming HTTP request.
use std::collections::HashMap;

pub type QueryParams = HashMap<String, String>;

/// HTTP request body variants.
#[derive(Debug, Clone)]
pub enum RequestBody {
    Json(String),
    Form(HashMap<String, String>),
    Raw(Vec<u8>),
    Empty,
}

/// Represents a parsed incoming HTTP request.
#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query: QueryParams,
    pub path_params: HashMap<String, String>,
    pub body: RequestBody,
}

impl Request {
    pub fn new(method: &str, path: &str) -> Self {
        Request {
            method: method.into(), path: path.into(),
            headers: HashMap::new(), query: HashMap::new(),
            path_params: HashMap::new(), body: RequestBody::Empty,
        }
    }

    /// Get a header value by name (case-insensitive).
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(&name.to_lowercase()).map(|s| s.as_str())
    }

    /// Get a path parameter (e.g. `{id}` capture).
    pub fn param(&self, name: &str) -> Option<&str> {
        self.path_params.get(name).map(|s| s.as_str())
    }

    /// Get a query string parameter.
    pub fn query_param(&self, name: &str) -> Option<&str> {
        self.query.get(name).map(|s| s.as_str())
    }

    /// Read the JSON body as a raw string.
    pub fn json_body(&self) -> Option<&str> {
        if let RequestBody::Json(ref s) = self.body { Some(s) } else { None }
    }
}
