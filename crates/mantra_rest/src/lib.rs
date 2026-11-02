//! mantra_rest — Native REST framework runtime for the Mantra programming language.
//! Provides RestController, JsonSerializer, RouteHandler, and ApiResponse.
//! Used by the Mantra standard library package `mantra.rest`.

pub mod api_response;
pub mod controller;
pub mod json;
pub mod route;

pub use api_response::{ApiError, ApiResponse, Pagination};
pub use controller::{CrudController, RestController};
pub use json::{JsonObject, JsonSerializer, JsonValue};
pub use route::{HandlerFn, RouteHandler, RouteRegistry};
