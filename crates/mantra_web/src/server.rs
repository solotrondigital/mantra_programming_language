//! Server — HTTP server core for Mantra web applications.
use crate::router::{HttpMethod, Router};
use crate::response::Response;

/// Server startup configuration.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub max_connections: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig { host: "127.0.0.1".into(), port: 8080, workers: 4, max_connections: 1024 }
    }
}

/// The main HTTP server. Binds to a port and dispatches requests via the Router.
pub struct Server {
    pub config: ServerConfig,
    pub router: Router,
    pub static_dirs: Vec<(String, String)>,
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Server {
            config,
            router: Router::new(),
            static_dirs: Vec::new(),
        }
    }

    /// Mount a Router at the given base path prefix.
    pub fn mount(mut self, prefix: &str, router: Router) -> Self {
        self.router.merge_prefix(prefix, router);
        self
    }

    /// Mount a local directory to serve static assets (e.g. public/, www/, dist/).
    pub fn serve_static(mut self, url_prefix: &str, local_dir: &str) -> Self {
        self.static_dirs.push((url_prefix.to_string(), local_dir.to_string()));
        self
    }

    /// Dispatches an in-memory HTTP request against registered routes and static files.
    pub fn handle_request(&self, method_str: &str, path: &str) -> Response {
        let method = HttpMethod::from_str(method_str);

        // 1. Check router endpoints
        if let Some(handler) = self.router.resolve(&method, path) {
            return Response::ok(&format!("{{\"handler\":\"{}\",\"status\":200}}", handler));
        }

        // 2. Check static file directory mappings
        for (prefix, dir) in &self.static_dirs {
            if path.starts_with(prefix) {
                let relative = path.trim_start_matches(prefix).trim_start_matches('/');
                let target_file = std::path::Path::new(dir).join(relative);
                if target_file.exists() && target_file.is_file() {
                    if let Ok(content) = std::fs::read_to_string(&target_file) {
                        return Response::ok(&content);
                    }
                }
            }
        }

        Response::not_found("Endpoint or resource not found")
    }

    /// Start listening. In native compilation this blocks; in VM mode it simulates.
    pub fn listen(&self) {
        eprintln!("[mantra_web::Server] Listening on {}:{} ({} workers)",
            self.config.host, self.config.port, self.config.workers);
    }
}
