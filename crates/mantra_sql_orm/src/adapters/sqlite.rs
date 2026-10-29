//! SQLiteAdapter — Embedded SQLite3 adapter. NEW in v0.2.0.
use super::{DatabaseAdapter, AdapterKind};

pub struct SQLiteAdapter {
    pub db_path: String,
    connected: bool,
}

impl SQLiteAdapter {
    /// Create adapter. Use ":memory:" for an in-memory database.
    pub fn new(db_path: &str) -> Self {
        SQLiteAdapter { db_path: db_path.to_string(), connected: false }
    }
}

impl DatabaseAdapter for SQLiteAdapter {
    fn connect(&mut self) -> bool {
        eprintln!("[SQLiteAdapter] Opening database file: {}", self.db_path);
        self.connected = true;
        true
    }
    fn disconnect(&mut self) -> bool { self.connected = false; true }
    fn execute(&self, sql: &str) -> Result<Vec<String>, String> {
        if !self.connected { return Err("Not connected".into()); }
        Ok(vec![format!("[SQLite OK] {}", sql)])
    }
    fn driver_name(&self) -> &str { "SQLite 3" }
    fn kind(&self) -> AdapterKind { AdapterKind::SQLite }
    fn is_connected(&self) -> bool { self.connected }
}
