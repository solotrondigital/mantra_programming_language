//! # mantra_graphql
//!
//! A native GraphQL runtime engine and schema executor for the Mantra programming language.
//!
//! Features:
//! - Complete GraphQL query, mutation, and subscription execution.
//! - Typed schema builder with SDL generation (`to_sdl`).
//! - Standard scalars (`Int`, `Float`, `String`, `Boolean`, `ID`) and custom scalar extension (`DateTime`, `JSON`, `UUID`).
//! - Lexer and recursive descent AST parser.
//! - Document validation and fragment spread expansion.
//! - Spec-compliant introspection (`__schema`, `__type`, `__typename`).
//! - Standard GraphQL JSON response serialization.

pub mod ast;
pub mod executor;
pub mod introspection;
pub mod lexer;
pub mod parser;
pub mod resolver;
pub mod response;
pub mod scalars;
pub mod schema;
pub mod types;
pub mod validation;

pub use executor::GraphQLEngine;
pub use parser::Parser;
pub use resolver::{ResolveContext, ResolverRegistry};
pub use response::{GraphQLError, GraphQLResponse};
pub use scalars::{coerce_scalar, CustomScalar, DateTimeScalar, JsonScalar, UuidScalar};
pub use schema::{GraphQLSchema, ObjectType, SchemaBuilder, TypeDefinition};
pub use types::{ArgumentDefinition, FieldDefinition, GraphQLType, GraphQLValue, InputFieldDefinition};

