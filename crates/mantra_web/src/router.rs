//! Router — URL pattern matching and handler registration.

/// Supported HTTP methods.
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET, POST, PUT, PATCH, DELETE, OPTIONS, HEAD,
}

impl HttpMethod {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "POST"    => HttpMethod::POST,
            "PUT"     => HttpMethod::PUT,
            "PATCH"   => HttpMethod::PATCH,
            "DELETE"  => HttpMethod::DELETE,
            "OPTIONS" => HttpMethod::OPTIONS,
            "HEAD"    => HttpMethod::HEAD,
            _         => HttpMethod::GET,
        }
    }
}

/// A single registered route with its method, path pattern, and handler name.
#[derive(Debug, Clone)]
pub struct Route {
    pub method: HttpMethod,
    pub pattern: String,
    pub handler: String,
}

/// URL Router: matches incoming request paths to registered handlers.
#[derive(Debug, Default)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self { Router::default() }

    pub fn get(mut self, pattern: &str, handler: &str) -> Self {
        self.routes.push(Route { method: HttpMethod::GET, pattern: pattern.into(), handler: handler.into() });
        self
    }
    pub fn post(mut self, pattern: &str, handler: &str) -> Self {
        self.routes.push(Route { method: HttpMethod::POST, pattern: pattern.into(), handler: handler.into() });
        self
    }
    pub fn put(mut self, pattern: &str, handler: &str) -> Self {
        self.routes.push(Route { method: HttpMethod::PUT, pattern: pattern.into(), handler: handler.into() });
        self
    }
    pub fn delete(mut self, pattern: &str, handler: &str) -> Self {
        self.routes.push(Route { method: HttpMethod::DELETE, pattern: pattern.into(), handler: handler.into() });
        self
    }

    /// Resolve a path+method to a handler name. Returns None if no match.
    pub fn resolve(&self, method: &HttpMethod, path: &str) -> Option<&str> {
        self.routes.iter().find(|r| &r.method == method && r.pattern == path)
            .map(|r| r.handler.as_str())
    }

    /// Merge another router's routes under a prefix.
    pub fn merge_prefix(&mut self, prefix: &str, other: Router) {
        for mut route in other.routes {
            route.pattern = format!("{}{}", prefix, route.pattern);
            self.routes.push(route);
        }
    }

    pub fn routes(&self) -> &[Route] { &self.routes }
}
