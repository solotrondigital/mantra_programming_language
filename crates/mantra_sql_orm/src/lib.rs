//! mantra_sql_orm — Native ORM runtime for the Mantra programming language.
//! Provides Model, QueryBuilder, DatabaseAdapter, Migration, and database-specific adapters.
//! Used by the Mantra standard library package `mantra.sql.orm`.

pub mod model;
pub mod query;
pub mod adapters;
pub mod migration;

pub use model::{Model, Field, FieldType, FieldValue};
pub use query::{QueryBuilder, WhereClause, OrderBy};
pub use adapters::{DatabaseAdapter, AdapterKind};
pub use adapters::mysql::MySQLAdapter;
pub use adapters::postgres::PostgreSQLAdapter;
pub use adapters::sqlite::SQLiteAdapter;
pub use migration::{Migration, MigrationRunner, SchemaBuilder, ColumnDef};
