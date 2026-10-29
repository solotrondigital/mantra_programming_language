//! RouteHandler — Typed route-to-handler mapping for REST APIs.
use mantra_web::request::Request;
use crate::api_response::ApiResponse;

/// Function pointer type for route handlers.
pub type HandlerFn = fn(&Request) -> ApiResponse;

/// A registered route-handler pair.
pub struct RouteHandler {
    pub method: String,
    pub path: String,
    pub handler: HandlerFn,
    pub description: String,
}

impl RouteHandler {
    pub fn get(path: &str, handler: HandlerFn, desc: &str) -> Self {
        RouteHandler { method: "GET".into(), path: path.into(), handler, description: desc.into() }
    }
    pub fn post(path: &str, handler: HandlerFn, desc: &str) -> Self {
        RouteHandler { method: "POST".into(), path: path.into(), handler, description: desc.into() }
    }
    pub fn put(path: &str, handler: HandlerFn, desc: &str) -> Self {
        RouteHandler { method: "PUT".into(), path: path.into(), handler, description: desc.into() }
    }
    pub fn delete(path: &str, handler: HandlerFn, desc: &str) -> Self {
        RouteHandler { method: "DELETE".into(), path: path.into(), handler, description: desc.into() }
    }
}

/// Registry of all route handlers in a REST API.
#[derive(Default)]
pub struct RouteRegistry {
    handlers: Vec<RouteHandler>,
}

impl RouteRegistry {
    pub fn new() -> Self { RouteRegistry::default() }
    pub fn register(mut self, handler: RouteHandler) -> Self {
        self.handlers.push(handler);
        self
    }
    pub fn dispatch(&self, method: &str, path: &str, req: &Request) -> Option<ApiResponse> {
        self.handlers.iter()
            .find(|h| h.method == method && h.path == path)
            .map(|h| (h.handler)(req))
    }
    pub fn list_routes(&self) -> Vec<String> {
        self.handlers.iter().map(|h| format!("{} {} — {}", h.method, h.path, h.description)).collect()
    }
}
