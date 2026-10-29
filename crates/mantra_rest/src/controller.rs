//! RestController — Base abstract controller for REST resources.
use crate::api_response::ApiResponse;
use mantra_web::request::Request;

/// Base trait for REST resource controllers.
/// Implement only the methods you need — defaults return 405 Method Not Allowed.
pub trait RestController {
    fn resource_name(&self) -> &str;

    fn get_all(&self, _req: &Request) -> ApiResponse {
        ApiResponse::method_not_allowed("GET_ALL not implemented")
    }
    fn get_by_id(&self, _req: &Request, _id: &str) -> ApiResponse {
        ApiResponse::method_not_allowed("GET_BY_ID not implemented")
    }
    fn create(&self, _req: &Request) -> ApiResponse {
        ApiResponse::method_not_allowed("CREATE not implemented")
    }
    fn update(&self, _req: &Request, _id: &str) -> ApiResponse {
        ApiResponse::method_not_allowed("UPDATE not implemented")
    }
    fn delete(&self, _req: &Request, _id: &str) -> ApiResponse {
        ApiResponse::method_not_allowed("DELETE not implemented")
    }
}

/// Convenience: full CRUD controller — provides default 200 OK stubs.
pub trait CrudController: RestController {
    fn mount_prefix(&self) -> String {
        format!("/api/v1/{}", self.resource_name())
    }
}
