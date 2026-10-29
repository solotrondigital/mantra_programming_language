//! Database adapter trait and factory.

pub mod mysql;
pub mod postgres;
pub mod sqlite;

/// Identifies which driver is in use.
#[derive(Debug, Clone, PartialEq)]
pub enum AdapterKind {
    MySQL,
    PostgreSQL,
    SQLite,
}

/// Universal database driver interface — implement to support any backend.
pub trait DatabaseAdapter: Send + Sync {
    fn connect(&mut self) -> bool;
    fn disconnect(&mut self) -> bool;
    fn execute(&self, sql: &str) -> Result<Vec<String>, String>;
    fn driver_name(&self) -> &str;
    fn kind(&self) -> AdapterKind;
    fn is_connected(&self) -> bool;
}
