use super::fs_service::FilesystemService;
use super::validator::ValidatedOptions;
use std::path::Path;

pub struct RestTemplateGenerator;

impl RestTemplateGenerator {
    pub fn generate(root: &Path, opts: &ValidatedOptions) -> Result<(), String> {
        let name = &opts.project_name;

        // 1. Directory Structure
        FilesystemService::create_dir_all(&root.join("src/config"))?;
        FilesystemService::create_dir_all(&root.join("src/routes"))?;
        FilesystemService::create_dir_all(&root.join("src/controllers"))?;
        FilesystemService::create_dir_all(&root.join("src/models"))?;
        FilesystemService::create_dir_all(&root.join("src/services"))?;
        FilesystemService::create_dir_all(&root.join("src/repositories"))?;
        FilesystemService::create_dir_all(&root.join("src/middleware"))?;
        FilesystemService::create_dir_all(&root.join("src/validators"))?;
        FilesystemService::create_dir_all(&root.join("src/utils"))?;
        FilesystemService::create_dir_all(&root.join("tests/unit"))?;
        FilesystemService::create_dir_all(&root.join("tests/integration"))?;
        FilesystemService::create_dir_all(&root.join("docs"))?;

        // 2. src/main.mantra
        let main_src = format!(
            r#"# {name} — REST API Server Entry Point

class ServerApplication:
    def __init__(self, host: String, port: Int):
        self.host = host
        self.port = port
        self.is_running = False

    def start(self) -> Int:
        print("Starting {name} REST API server on " + self.host + ":" + str(self.port) + "...")
        self.is_running = True
        print("✓ Routes mounted: /health, /api/users")
        print("✓ Middleware active: LoggerMiddleware, AuthMiddleware")
        print("Server is listening for HTTP requests.")
        return 0

def main() -> Int:
    let app = ServerApplication("127.0.0.1", 8080)
    return app.start()
"#
        );
        FilesystemService::write_file(&root.join("src/main.mantra"), &main_src)?;

        // 3. src/config/app_config.mantra
        let config_src = r#"# Application Configuration

class AppConfig:
    def __init__(self):
        self.env = "development"
        self.host = "127.0.0.1"
        self.port = 8080
        self.log_level = "info"

    def is_production(self) -> Bool:
        return self.env == "production"
"#;
        FilesystemService::write_file(&root.join("src/config/app_config.mantra"), config_src)?;

        // 4. src/routes/api_routes.mantra
        let routes_src = r#"# API Route Definitions

class ApiRouter:
    def __init__(self):
        self.route_count = 0

    def register_routes(self) -> Int:
        # GET /health
        self.route_count = self.route_count + 1
        # GET /api/users
        self.route_count = self.route_count + 1
        # POST /api/users
        self.route_count = self.route_count + 1
        return self.route_count
"#;
        FilesystemService::write_file(&root.join("src/routes/api_routes.mantra"), routes_src)?;

        // 5. src/controllers/health_controller.mantra
        let health_ctrl_src = r#"# Health Check Controller

class HealthController:
    def check(self) -> String:
        return "{\"status\":\"UP\",\"service\":\"REST\",\"timestamp\":\"2026-10-29T00:00:00Z\"}"
"#;
        FilesystemService::write_file(
            &root.join("src/controllers/health_controller.mantra"),
            health_ctrl_src,
        )?;

        // 6. src/controllers/user_controller.mantra
        let user_ctrl_src = r#"# User REST Controller

class UserController:
    def __init__(self):
        self.resource_name = "users"

    def get_users(self) -> String:
        return "{\"users\":[{\"id\":\"usr_1\",\"name\":\"Alice\"}]}"

    def create_user(self, name: String) -> String:
        return "{\"success\":true,\"user\":{\"id\":\"usr_new\",\"name\":\"" + name + "\"}}"
"#;
        FilesystemService::write_file(
            &root.join("src/controllers/user_controller.mantra"),
            user_ctrl_src,
        )?;

        // 7. src/models/user.mantra
        let model_src = r#"# User Domain Model Entity

class UserModel:
    def __init__(self, id: String, name: String, email: String):
        self.id = id
        self.name = name
        self.email = email
        self.is_active = True

    def to_json(self) -> String:
        return "{\"id\":\"" + self.id + "\",\"name\":\"" + self.name + "\",\"email\":\"" + self.email + "\"}"
"#;
        FilesystemService::write_file(&root.join("src/models/user.mantra"), model_src)?;

        // 8. src/services/user_service.mantra
        let service_src = r#"# User Business Service

class UserService:
    def __init__(self):
        self.total_users = 1

    def find_by_id(self, id: String) -> String:
        return "User found: " + id

    def validate_and_create(self, name: String, email: String) -> Bool:
        if len(name) == 0:
            return False
        self.total_users = self.total_users + 1
        return True
"#;
        FilesystemService::write_file(&root.join("src/services/user_service.mantra"), service_src)?;

        // 9. src/repositories/user_repository.mantra
        let repo_src = r#"# User Persistence Repository

class UserRepository:
    def __init__(self):
        self.record_count = 0

    def save(self, user_id: String) -> Bool:
        self.record_count = self.record_count + 1
        return True

    def find(self, user_id: String) -> Bool:
        return True
"#;
        FilesystemService::write_file(
            &root.join("src/repositories/user_repository.mantra"),
            repo_src,
        )?;

        // 10. src/middleware/auth_middleware.mantra
        let auth_mid_src = r#"# Authentication Middleware

class AuthMiddleware:
    def __init__(self):
        self.header_key = "Authorization"

    def authenticate(self, token: String) -> Bool:
        return len(token) > 0
"#;
        FilesystemService::write_file(
            &root.join("src/middleware/auth_middleware.mantra"),
            auth_mid_src,
        )?;

        // 11. src/middleware/logger_middleware.mantra
        let logger_mid_src = r#"# Request Logging Middleware

class LoggerMiddleware:
    def log_request(self, method: String, path: String) -> String:
        return "[HTTP] " + method + " " + path
"#;
        FilesystemService::write_file(
            &root.join("src/middleware/logger_middleware.mantra"),
            logger_mid_src,
        )?;

        // 12. src/validators/user_validator.mantra
        let validator_src = r#"# User Input Validator

class UserValidator:
    def validate_create_input(self, name: String, email: String) -> Bool:
        let valid_name: Bool = len(name) > 0
        let valid_email: Bool = len(email) > 3
        return valid_name and valid_email
"#;
        FilesystemService::write_file(
            &root.join("src/validators/user_validator.mantra"),
            validator_src,
        )?;

        // 13. src/utils/response_formatter.mantra
        let formatter_src = r#"# JSON Response Envelope Formatter

class ResponseFormatter:
    def success(self, data_json: String) -> String:
        return "{\"success\":true,\"data\":" + data_json + ",\"error\":null}"

    def error(self, code: Int, message: String) -> String:
        return "{\"success\":false,\"data\":null,\"error\":{\"code\":" + str(code) + ",\"message\":\"" + message + "\"}}"
"#;
        FilesystemService::write_file(
            &root.join("src/utils/response_formatter.mantra"),
            formatter_src,
        )?;

        // 14. tests/unit/user_service_test.mantra
        let unit_test_src = format!(
            r#"# {name} User Service Unit Test

def test_user_creation() -> Int:
    let is_valid: Bool = True
    assert(is_valid == True)
    return 0

def main() -> Int:
    print("Running UserService unit test...")
    test_user_creation()
    print("✓ UserService tests passed.")
    return 0
"#
        );
        FilesystemService::write_file(
            &root.join("tests/unit/user_service_test.mantra"),
            &unit_test_src,
        )?;

        // 15. tests/integration/health_api_test.mantra
        let int_test_src = format!(
            r#"# {name} Health Endpoint Integration Test

def test_health_check() -> Int:
    let status_up: Bool = True
    assert(status_up == True)
    return 0

def main() -> Int:
    print("Running Health API integration test...")
    test_health_check()
    print("✓ Health API tests passed.")
    return 0
"#
        );
        FilesystemService::write_file(
            &root.join("tests/integration/health_api_test.mantra"),
            &int_test_src,
        )?;

        // 16. docs/api_spec.md
        let doc_src = format!(
            r#"# {name} REST API Specification

## Base URL
`http://127.0.0.1:8080`

## Endpoints

### 1. Health Check
* **Method:** `GET`
* **Path:** `/health`
* **Response (200 OK):**
```json
{{
  "status": "UP",
  "service": "REST",
  "timestamp": "2026-10-29T00:00:00Z"
}}
```

### 2. List Users
* **Method:** `GET`
* **Path:** `/api/users`
* **Response (200 OK):**
```json
{{
  "success": true,
  "data": [
    {{ "id": "usr_1", "name": "Alice", "email": "alice@example.com" }}
  ]
}}
```

### 3. Create User
* **Method:** `POST`
* **Path:** `/api/users`
* **Request Body:**
```json
{{
  "name": "Bob",
  "email": "bob@example.com"
}}
```
* **Response (201 Created):**
```json
{{
  "success": true,
  "data": {{
    "id": "usr_2",
    "name": "Bob",
    "email": "bob@example.com"
  }}
}}
```
"#
        );
        FilesystemService::write_file(&root.join("docs/api_spec.md"), &doc_src)?;

        Ok(())
    }
}
