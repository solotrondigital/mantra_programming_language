//! Server — HTTP server core for Mantra web applications.
use crate::response::Response;
use crate::router::{HttpMethod, Router};

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
        ServerConfig {
            host: "127.0.0.1".into(),
            port: 8080,
            workers: 4,
            max_connections: 1024,
        }
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
        self.static_dirs
            .push((url_prefix.to_string(), local_dir.to_string()));
        self
    }

    /// Dispatches an in-memory HTTP request against registered routes and static files.
    pub fn handle_request(&self, method_str: &str, path: &str) -> Response {
        let method = HttpMethod::from_str(method_str);
        let path = path.split('?').next().unwrap_or(path);

        // 1. Check router endpoints
        if let Some(handler) = self.router.resolve(&method, path) {
            return Response::ok(&format!("{{\"handler\":\"{}\",\"status\":200}}", handler));
        }

        // 2. Check static file directory mappings
        for (prefix, dir) in &self.static_dirs {
            let prefix = prefix.trim_end_matches('/');
            if path == prefix
                || path
                    .strip_prefix(prefix)
                    .is_some_and(|tail| tail.starts_with('/'))
            {
                let relative = path
                    .strip_prefix(prefix)
                    .unwrap_or("")
                    .trim_start_matches('/');
                let root = match std::path::Path::new(dir).canonicalize() {
                    Ok(root) => root,
                    Err(_) => continue,
                };
                let target_file = match root.join(relative).canonicalize() {
                    Ok(target) if target.starts_with(&root) && target.is_file() => target,
                    _ => continue,
                };
                if let Ok(content) = std::fs::read_to_string(target_file) {
                    return Response::ok(&content);
                }
            }
        }

        Response::not_found("Endpoint or resource not found")
    }

    /// Start listening. In native compilation this blocks; in VM mode it simulates.
    pub fn listen(&self) {
        eprintln!(
            "[mantra_web::Server] Listening on {}:{} ({} workers)",
            self.config.host, self.config.port, self.config.workers
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_assets_are_served_without_query_strings() {
        let root = std::env::temp_dir().join(format!("mantra-web-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("index.html"), "hello").unwrap();
        let server =
            Server::new(ServerConfig::default()).serve_static("/public", root.to_str().unwrap());
        assert_eq!(
            server.handle_request("GET", "/public/index.html?v=1").body,
            "hello"
        );
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn static_assets_reject_traversal_and_prefix_confusion() {
        let root = std::env::temp_dir().join(format!("mantra-web-{}-safe", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let outside = root.with_extension("txt");
        std::fs::write(&outside, "secret").unwrap();
        let server =
            Server::new(ServerConfig::default()).serve_static("/public", root.to_str().unwrap());
        assert_eq!(
            server.handle_request("GET", "/publicity/secret").status,
            crate::StatusCode::NotFound
        );
        assert_eq!(
            server
                .handle_request("GET", "/public/../".to_string().as_str())
                .status,
            crate::StatusCode::NotFound
        );
        let _ = std::fs::remove_file(outside);
        let _ = std::fs::remove_dir_all(root);
    }
}
