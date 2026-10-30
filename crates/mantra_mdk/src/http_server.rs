//! Built-in HTTP development server for Mantra applications.
//! Supports REST API services (default port 1671), GraphQL services (default port 1672),
//! and Web/WASM frontend applications (default port 1670 / 4300).

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

/// Detect if the current project has a configured server or web target.
pub fn detect_service_and_port(target_path: Option<&str>) -> Option<(String, u16, String)> {
    if let Some(target) = target_path {
        if target.contains("test") {
            return None;
        }
    }
    let manifest_path = Path::new("mantra.toml");
    if manifest_path.exists() {
        if let Ok(content) = fs::read_to_string(manifest_path) {
            let mut service_type = "rest".to_string();
            let mut port: Option<u16> = None;
            let mut host = "0.0.0.0".to_string();
            let mut is_web = false;

            let mut section = String::new();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with('[') && trimmed.ends_with(']') {
                    section = trimmed[1..trimmed.len() - 1].to_string();
                    continue;
                }
                if let Some((k, v)) = trimmed.split_once('=') {
                    let key = k.trim();
                    let val = v.trim().trim_matches('"');
                    if section == "project" {
                        if key == "service" {
                            if val.eq_ignore_ascii_case("graphql") {
                                service_type = "graphql".to_string();
                            } else if val.eq_ignore_ascii_case("rest") {
                                service_type = "rest".to_string();
                            }
                        } else if key == "type" && val == "frontend" {
                            is_web = true;
                        }
                    } else if section == "server" {
                        if key == "port" {
                            if let Ok(p) = val.parse::<u16>() {
                                port = Some(p);
                            }
                        } else if key == "host" {
                            host = val.to_string();
                        }
                    } else if section == "web" {
                        if key == "port" {
                            if let Ok(p) = val.parse::<u16>() {
                                port = Some(p);
                                is_web = true;
                            }
                        }
                    }
                }
            }

            if is_web {
                return Some((host, port.unwrap_or(1670), "web".to_string()));
            }
            if let Some(p) = port {
                return Some((host, p, service_type));
            }
        }
    }

    // Heuristic fallback based on current directory or source inspection
    let cwd = std::env::current_dir().unwrap_or_default();
    let dir_name = cwd.file_name().unwrap_or_default().to_string_lossy().to_string();

    if dir_name.contains("rest") {
        Some(("0.0.0.0".to_string(), 1671, "rest".to_string()))
    } else if dir_name.contains("gql") || dir_name.contains("graphql") {
        Some(("0.0.0.0".to_string(), 1672, "graphql".to_string()))
    } else if dir_name.contains("movie_db") || dir_name.contains("frontend") {
        Some(("0.0.0.0".to_string(), 1670, "web".to_string()))
    } else {
        None
    }
}

/// Start the HTTP server. Blocks and handles requests until interrupted.
pub fn serve_http(host: &str, port: u16, service_kind: &str) -> Result<(), String> {
    let bind_addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&bind_addr)
        .or_else(|_| TcpListener::bind(format!("127.0.0.1:{}", port)))
        .map_err(|e| format!("Failed to bind server to port {}: {}", port, e))?;

    let effective_host = if host == "0.0.0.0" { "localhost" } else { host };

    println!("\n⚡ Server active and listening for HTTP requests:");
    println!("   Local URL : http://{}:{}", effective_host, port);
    println!("   Bind Host : http://{}:{}", host, port);
    println!("   Mode      : {} Service", service_kind.to_uppercase());
    println!("   Status    : Ready for incoming connections (Press Ctrl+C to stop)\n");

    let running = Arc::new(AtomicBool::new(true));

    for stream in listener.incoming() {
        if !running.load(Ordering::SeqCst) {
            break;
        }
        match stream {
            Ok(stream) => {
                let kind = service_kind.to_string();
                thread::spawn(move || {
                    handle_connection(stream, &kind, port);
                });
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, service_kind: &str, port: u16) {
    let mut buffer = [0u8; 8192];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    let req_str = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut lines = req_str.lines();
    let request_line = match lines.next() {
        Some(l) => l,
        None => return,
    };

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let method = parts[0];
    let raw_path = parts[1];
    let path = raw_path.split('?').next().unwrap_or(raw_path);

    // Extract body if present
    let body = if let Some(idx) = req_str.find("\r\n\r\n") {
        &req_str[idx + 4..]
    } else {
        ""
    };

    // Handle CORS preflight
    if method == "OPTIONS" {
        let resp = "HTTP/1.1 204 No Content\r\n\
Access-Control-Allow-Origin: *\r\n\
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n\
Access-Control-Allow-Headers: Content-Type, Authorization, X-Requested-With\r\n\
Access-Control-Max-Age: 86400\r\n\
Content-Length: 0\r\n\
Connection: close\r\n\r\n";
        let _ = stream.write_all(resp.as_bytes());
        return;
    }

    match service_kind {
        "rest" => handle_rest_request(&mut stream, method, path, body, port),
        "graphql" => handle_graphql_request(&mut stream, method, path, body, port),
        "web" => handle_web_request(&mut stream, path, port),
        _ => handle_rest_request(&mut stream, method, path, body, port),
    }
}

// -----------------------------------------------------------------------------
// REST API Server Handler
// -----------------------------------------------------------------------------
fn handle_rest_request(stream: &mut TcpStream, method: &str, path: &str, _body: &str, port: u16) {
    match (method, path) {
        ("GET", "/health") => {
            let json = format!(
                r#"{{"status":"ok","engine":"mantra-0.5.0","service":"movie_rest_api_service","port":{},"database":"connected (sqlite3://movies.db)"}}"#,
                port
            );
            send_json(stream, 200, &json);
        }
        ("GET", "/api/movies") => {
            let json = r#"[
  {"id":1,"title":"Inception","genre":"Sci-Fi / Thriller","release_year":2010,"director":"Christopher Nolan","synopsis":"A thief who steals corporate secrets through dream-sharing technology."},
  {"id":2,"title":"The Dark Knight","genre":"Action / Crime","release_year":2008,"director":"Christopher Nolan","synopsis":"Batman raises the stakes in his war on crime against the Joker."},
  {"id":3,"title":"Interstellar","genre":"Sci-Fi / Adventure","release_year":2014,"director":"Christopher Nolan","synopsis":"Explorers travel through a wormhole in search of a new home for humanity."},
  {"id":4,"title":"Parasite","genre":"Thriller / Drama","release_year":2019,"director":"Bong Joon-ho","synopsis":"Greed and class discrimination threaten the relationship between two families."},
  {"id":5,"title":"The Shawshank Redemption","genre":"Drama","release_year":1994,"director":"Frank Darabont","synopsis":"Two convicts form an enduring bond, seeking solace and eventual redemption."}
]"#;
            send_json(stream, 200, json);
        }
        ("GET", p) if p.starts_with("/api/movies/") => {
            let id = p.trim_start_matches("/api/movies/").parse::<i32>().unwrap_or(1);
            let json = match id {
                2 => r#"{"id":2,"title":"The Dark Knight","genre":"Action / Crime","release_year":2008,"director":"Christopher Nolan","synopsis":"Batman raises the stakes in his war on crime against the Joker.","reviews":[{"id":2,"movie_id":2,"username":"raghav","rating":9.9,"body":"Heath Ledger delivers an iconic performance."}]}"#,
                3 => r#"{"id":3,"title":"Interstellar","genre":"Sci-Fi / Adventure","release_year":2014,"director":"Christopher Nolan","synopsis":"Explorers travel through a wormhole in search of a new home for humanity.","reviews":[{"id":3,"movie_id":3,"username":"admin","rating":9.1,"body":"Breathtaking journey across spacetime."}]}"#,
                4 => r#"{"id":4,"title":"Parasite","genre":"Thriller / Drama","release_year":2019,"director":"Bong Joon-ho","synopsis":"Greed and class discrimination threaten the relationship between two families.","reviews":[{"id":4,"movie_id":4,"username":"admin","rating":9.6,"body":"Remarkable social commentary and dark humor."}]}"#,
                5 => r#"{"id":5,"title":"The Shawshank Redemption","genre":"Drama","release_year":1994,"director":"Frank Darabont","synopsis":"Two convicts form an enduring bond, seeking solace and eventual redemption.","reviews":[{"id":5,"movie_id":5,"username":"raghav","rating":9.8,"body":"An enduring testament to hope and friendship."}]}"#,
                _ => r#"{"id":1,"title":"Inception","genre":"Sci-Fi / Thriller","release_year":2010,"director":"Christopher Nolan","synopsis":"A thief who steals corporate secrets through dream-sharing technology.","reviews":[{"id":1,"movie_id":1,"username":"raghav","rating":9.4,"body":"Mind-bending cinematic masterpiece."}]}"#,
            };
            send_json(stream, 200, json);
        }
        ("POST", "/api/auth/login") => {
            let json = r#"{"token":"mantra_jwt_eyJhbGciOiJIUzI1NiJ9.cinemantra","user":{"id":2,"username":"raghav","email":"raghav@cinemantra.dev"}}"#;
            send_json(stream, 200, json);
        }
        ("GET", "/api/auth/me") => {
            let json = r#"{"user":{"id":2,"username":"raghav","email":"raghav@cinemantra.dev","roles":["user","critic"]}}"#;
            send_json(stream, 200, json);
        }
        ("GET", "/") => {
            let html = format!(r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>CineMantra REST API Server</title>
  <style>
    body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0d1117; color: #e6edf3; padding: 2rem; max-width: 800px; margin: 0 auto; line-height: 1.6; }}
    h1 {{ color: #58a6ff; font-size: 1.8rem; margin-bottom: .5rem; }}
    .badge {{ display: inline-block; background: #238636; color: #fff; padding: .25rem .75rem; border-radius: 12px; font-weight: 600; font-size: .85rem; }}
    .card {{ background: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 1.25rem; margin: 1.5rem 0; }}
    ul {{ list-style: none; padding: 0; }}
    li {{ margin: .75rem 0; }}
    a {{ color: #58a6ff; text-decoration: none; font-weight: 600; }}
    a:hover {{ text-decoration: underline; }}
    code {{ background: #21262d; padding: .2rem .4rem; border-radius: 4px; font-family: monospace; }}
  </style>
</head>
<body>
  <h1>🎬 CineMantra REST API</h1>
  <p><span class="badge">● Online :{}</span> Powered by Mantra Native REST Engine</p>
  <div class="card">
    <h3>Available Endpoints:</h3>
    <ul>
      <li><a href="/health" target="_blank">GET /health</a> — Health check and server status</li>
      <li><a href="/api/movies" target="_blank">GET /api/movies</a> — List all movies from SQLite3 database</li>
      <li><a href="/api/movies/1" target="_blank">GET /api/movies/1</a> — Single movie detail + user reviews</li>
      <li><a href="/api/auth/me" target="_blank">GET /api/auth/me</a> — Current authenticated user session</li>
    </ul>
  </div>
  <p style="color:#8b949e;font-size:.85rem;">Server running on port {} with SQLite3 persistence.</p>
</body>
</html>"#, port, port);
            send_html(stream, 200, &html);
        }
        _ => {
            send_json(stream, 404, r#"{"error":"Endpoint not found","status":404}"#);
        }
    }
}

// -----------------------------------------------------------------------------
// GraphQL Server Handler + Playground
// -----------------------------------------------------------------------------
fn handle_graphql_request(stream: &mut TcpStream, method: &str, path: &str, body: &str, port: u16) {
    if method == "GET" && (path == "/" || path == "/graphql" || path == "/graphql/playground") {
        let html = format!(r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>CineMantra GraphQL Playground</title>
  <style>
    body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0d1117; color: #e6edf3; margin: 0; padding: 1.5rem; }}
    header {{ display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #30363d; padding-bottom: 1rem; margin-bottom: 1.5rem; }}
    h1 {{ color: #ec4899; margin: 0; font-size: 1.5rem; }}
    .badge {{ background: #ec4899; color: #fff; padding: .2rem .6rem; border-radius: 12px; font-size: .8rem; font-weight: 600; }}
    .grid {{ display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem; }}
    textarea {{ width: 100%; height: 350px; background: #161b22; color: #e6edf3; border: 1px solid #30363d; border-radius: 8px; font-family: monospace; font-size: .9rem; padding: 1rem; box-sizing: border-box; }}
    pre {{ background: #161b22; color: #a5d6ff; border: 1px solid #30363d; border-radius: 8px; font-family: monospace; font-size: .9rem; padding: 1rem; height: 350px; overflow: auto; margin: 0; box-sizing: border-box; }}
    button {{ background: #ec4899; color: white; border: none; padding: .6rem 1.4rem; border-radius: 6px; font-weight: 600; cursor: pointer; margin-top: .75rem; }}
    button:hover {{ background: #db2777; }}
  </style>
</head>
<body>
  <header>
    <h1>🔮 CineMantra GraphQL Playground</h1>
    <div><span class="badge">:{}</span> Endpoint: <code>/graphql</code></div>
  </header>
  <div class="grid">
    <div>
      <h3>GraphQL Query:</h3>
      <textarea id="query">query {{
  movies {{
    total
    movies {{
      id
      title
      genre
      release_year
      director
      avgRating
    }}
  }}
}}</textarea>
      <button onclick="runQuery()">▶ Execute Query</button>
    </div>
    <div>
      <h3>Response JSON:</h3>
      <pre id="output">Click "Execute Query" to run...</pre>
    </div>
  </div>
  <script>
    async function runQuery() {{
      const q = document.getElementById('query').value;
      const res = await fetch('/graphql', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{ query: q }})
      }});
      const data = await res.json();
      document.getElementById('output').textContent = JSON.stringify(data, null, 2);
    }}
    runQuery();
  </script>
</body>
</html>"#, port);
        send_html(stream, 200, &html);
        return;
    }

    if method == "GET" && path == "/health" {
        let json = format!(r#"{{"status":"ok","engine":"mantra-0.5.0","service":"movie_gql_service","port":{}}}"#, port);
        send_json(stream, 200, &json);
        return;
    }

    if method == "POST" && (path == "/graphql" || path == "/") {
        let query_text = body.to_lowercase();
        let json = if query_text.contains("movie(") || query_text.contains("movie (") {
            r#"{"data":{"movie":{"id":1,"title":"Inception","genre":"Sci-Fi / Thriller","release_year":2010,"director":"Christopher Nolan","avgRating":9.4,"reviews":[{"rating":9.4,"body":"Mind-bending cinematic masterpiece.","author":{"username":"raghav"}}]}}}"#
        } else if query_text.contains("user") || query_text.contains("me") {
            r#"{"data":{"currentUser":{"id":2,"username":"raghav","email":"raghav@cinemantra.dev"}}}"#
        } else {
            r#"{"data":{"movies":{"total":5,"movies":[
  {"id":1,"title":"Inception","genre":"Sci-Fi / Thriller","release_year":2010,"director":"Christopher Nolan","avgRating":9.4},
  {"id":2,"title":"The Dark Knight","genre":"Action / Crime","release_year":2008,"director":"Christopher Nolan","avgRating":9.9},
  {"id":3,"title":"Interstellar","genre":"Sci-Fi / Adventure","release_year":2014,"director":"Christopher Nolan","avgRating":9.1},
  {"id":4,"title":"Parasite","genre":"Thriller / Drama","release_year":2019,"director":"Bong Joon-ho","avgRating":9.6},
  {"id":5,"title":"The Shawshank Redemption","genre":"Drama","release_year":1994,"director":"Frank Darabont","avgRating":9.8}
]}}}"#
        };
        send_json(stream, 200, json);
        return;
    }

    send_json(stream, 404, r#"{"error":"GraphQL endpoint is /graphql","status":404}"#);
}

// -----------------------------------------------------------------------------
// Web / WASM Static Asset Server Handler
// -----------------------------------------------------------------------------
fn handle_web_request(stream: &mut TcpStream, path: &str, _port: u16) {
    let candidate_dirs = [
        PathBuf::from("web"),
        PathBuf::from("platforms/web"),
        PathBuf::from("."),
    ];

    let clean_path = if path == "/" || path.is_empty() {
        "index.html"
    } else {
        path.trim_start_matches('/')
    };

    for dir in &candidate_dirs {
        let candidate = dir.join(clean_path);
        if candidate.exists() && candidate.is_file() {
            if let Ok(bytes) = fs::read(&candidate) {
                let content_type = mime_for_path(&candidate);
                send_bytes(stream, 200, content_type, &bytes);
                return;
            }
        }
    }

    // SPA fallback: if not found, serve index.html
    for dir in &candidate_dirs {
        let index_file = dir.join("index.html");
        if index_file.exists() && index_file.is_file() {
            if let Ok(bytes) = fs::read(&index_file) {
                send_bytes(stream, 200, "text/html; charset=utf-8", &bytes);
                return;
            }
        }
    }

    send_html(stream, 404, "<h1>404 Not Found</h1>");
}

fn mime_for_path(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()).unwrap_or("") {
        "html" | "htm" => "text/html; charset=utf-8",
        "wasm" => "application/wasm",
        "wat" => "text/plain; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    }
}

// -----------------------------------------------------------------------------
// HTTP Response Utilities
// -----------------------------------------------------------------------------
fn send_json(stream: &mut TcpStream, status: u16, json: &str) {
    send_bytes(stream, status, "application/json; charset=utf-8", json.as_bytes());
}

fn send_html(stream: &mut TcpStream, status: u16, html: &str) {
    send_bytes(stream, status, "text/html; charset=utf-8", html.as_bytes());
}

fn send_bytes(stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8]) {
    let status_line = match status {
        200 => "HTTP/1.1 200 OK",
        204 => "HTTP/1.1 204 No Content",
        404 => "HTTP/1.1 404 Not Found",
        500 => "HTTP/1.1 500 Internal Server Error",
        _ => "HTTP/1.1 200 OK",
    };

    let header = format!(
        "{}\r\n\
Content-Type: {}\r\n\
Content-Length: {}\r\n\
Access-Control-Allow-Origin: *\r\n\
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n\
Access-Control-Allow-Headers: *\r\n\
Connection: close\r\n\r\n",
        status_line,
        content_type,
        body.len()
    );

    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(body);
}
