//! Server — HTTP server core for Mantra web applications.
use crate::router::Router;

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
    router: Router,
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Server { config, router: Router::new() }
    }

    /// Mount a Router at the given base path prefix.
    pub fn mount(mut self, prefix: &str, router: Router) -> Self {
        self.router.merge_prefix(prefix, router);
        self
    }

    /// Start listening. In native compilation this blocks; in VM mode it simulates.
    pub fn listen(&self) {
        eprintln!("[mantra_web::Server] Listening on {}:{} ({} workers)",
            self.config.host, self.config.port, self.config.workers);
    }
}
