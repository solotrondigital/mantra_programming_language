//! mantra_rest — Native REST framework runtime for the Mantra programming language.
//! Provides RestController, JsonSerializer, RouteHandler, and ApiResponse.
//! Used by the Mantra standard library package `mantra.rest`.

pub mod controller;
pub mod json;
pub mod route;
pub mod api_response;

pub use controller::{RestController, CrudController};
pub use json::{JsonSerializer, JsonValue, JsonObject};
pub use route::{RouteHandler, RouteRegistry, HandlerFn};
pub use api_response::{ApiResponse, ApiError, Pagination};
