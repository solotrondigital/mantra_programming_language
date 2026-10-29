//! mantra_sql_orm — Native ORM runtime for the Mantra programming language.
//! Provides Model, QueryBuilder, DatabaseAdapter, Migration, and database-specific adapters
//! for Oracle, MSSQL, MongoDB, PostgreSQL, MySQL, SQLite, and Generic SQL databases.
//! Also supports Stored Procedures, Cursors, Views, and Functions.

pub mod adapters;
pub mod migration;
pub mod model;
pub mod query;

pub use adapters::generic::GenericDbAdapter;
pub use adapters::mongodb::MongoDBAdapter;
pub use adapters::mssql::MSSQLAdapter;
pub use adapters::mysql::MySQLAdapter;
pub use adapters::oracle::OracleAdapter;
pub use adapters::postgres::PostgreSQLAdapter;
pub use adapters::sqlite::SQLiteAdapter;
pub use adapters::{
    AdapterKind, Cursor, CursorDirection, DatabaseAdapter, ParamDirection, ProcedureParam, ProcedureResult,
};
pub use migration::{ColumnDef, Migration, MigrationRunner, SchemaBuilder};
pub use model::{Field, FieldType, FieldValue, Model, Row};
pub use query::{OrderBy, QueryBuilder, WhereClause};
