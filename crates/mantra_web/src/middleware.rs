//! Middleware — composable request/response processing pipeline.
use crate::{request::Request, response::Response};

/// A middleware component processes a request and may modify or short-circuit the response.
pub trait Middleware: Send + Sync {
    fn name(&self) -> &str;
    fn before(&self, req: &Request) -> Option<Response>;   // None = continue, Some = short-circuit
    fn after(&self, res: &mut Response);
}

/// Chains multiple middleware in order.
pub struct MiddlewareChain {
    chain: Vec<Box<dyn Middleware>>,
}

impl MiddlewareChain {
    pub fn new() -> Self { MiddlewareChain { chain: vec![] } }
    pub fn add(mut self, mw: impl Middleware + 'static) -> Self {
        self.chain.push(Box::new(mw));
        self
    }
    pub fn run_before(&self, req: &Request) -> Option<Response> {
        for mw in &self.chain {
            if let Some(res) = mw.before(req) { return Some(res); }
        }
        None
    }
    pub fn run_after(&self, res: &mut Response) {
        for mw in &self.chain { mw.after(res); }
    }
}

/// Built-in: request logging middleware.
pub struct LoggingMiddleware;
impl Middleware for LoggingMiddleware {
    fn name(&self) -> &str { "LoggingMiddleware" }
    fn before(&self, req: &Request) -> Option<Response> {
        eprintln!("[LOG] {} {}", req.method, req.path);
        None
    }
    fn after(&self, res: &mut Response) {
        eprintln!("[LOG] → HTTP {}", res.status.code());
    }
}

/// Built-in: CORS middleware — adds cross-origin headers.
pub struct CorsMiddleware {
    pub allowed_origins: Vec<String>,
}
impl Middleware for CorsMiddleware {
    fn name(&self) -> &str { "CorsMiddleware" }
    fn before(&self, _req: &Request) -> Option<Response> { None }
    fn after(&self, res: &mut Response) {
        res.headers.push(("Access-Control-Allow-Origin".into(), "*".into()));
        res.headers.push(("Access-Control-Allow-Methods".into(), "GET,POST,PUT,DELETE,OPTIONS".into()));
    }
}
