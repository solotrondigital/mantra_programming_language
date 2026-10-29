//! mantra_web — Native web framework runtime for the Mantra programming language.
//! Provides Server, Router, Request, Response, Middleware, and Template.
//! Used by the Mantra standard library package `mantra.web`.

pub mod server;
pub mod router;
pub mod request;
pub mod response;
pub mod middleware;
pub mod template;

pub use server::{Server, ServerConfig};
pub use router::{Router, Route, HttpMethod};
pub use request::{Request, RequestBody, QueryParams};
pub use response::{Response, ResponseBuilder, ContentType, StatusCode};
pub use middleware::{Middleware, MiddlewareChain, LoggingMiddleware, CorsMiddleware};
pub use template::{Template, TemplateEngine};
