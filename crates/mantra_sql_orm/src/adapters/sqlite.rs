//! SQLiteAdapter — Embedded SQLite3 adapter.
use super::{AdapterKind, Cursor, CursorDirection, DatabaseAdapter, ProcedureParam, ProcedureResult};
use crate::model::{FieldValue, Row};
use std::collections::HashMap;

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
        self.connected = true;
        true
    }

    fn disconnect(&mut self) -> bool {
        self.connected = false;
        true
    }

    fn execute(&self, sql: &str) -> Result<Vec<String>, String> {
        if !self.connected { return Err("SQLite: Not connected".into()); }
        Ok(vec![format!("[SQLite OK] {}", sql)])
    }

    fn query(&self, sql: &str) -> Result<Vec<Row>, String> {
        if !self.connected { return Err("SQLite: Not connected".into()); }
        let mut row = Row::new();
        row.insert("STATEMENT", FieldValue::Text(sql.to_string()));
        row.insert("DB_PATH", FieldValue::Text(self.db_path.clone()));
        Ok(vec![row])
    }

    fn driver_name(&self) -> &str { "SQLite 3" }
    fn kind(&self) -> AdapterKind { AdapterKind::SQLite }
    fn is_connected(&self) -> bool { self.connected }

    fn execute_procedure(&self, name: &str, params: &[ProcedureParam]) -> Result<ProcedureResult, String> {
        if !self.connected { return Err("SQLite: Not connected".into()); }
        // SQLite uses user-defined functions or triggers for procedural emulation
        let mut output_params = HashMap::new();
        output_params.insert("info".to_string(), FieldValue::Text(format!("Emulated procedure {}", name)));
        let mut row = Row::new();
        row.insert("PROCEDURE", FieldValue::Text(name.to_string()));
        row.insert("PARAMS_COUNT", FieldValue::Int(params.len() as i64));

        Ok(ProcedureResult {
            status: 0,
            output_params,
            result_sets: vec![vec![row]],
            rows_affected: 1,
        })
    }

    fn open_cursor(&self, name: &str, query: &str) -> Result<Cursor, String> {
        if !self.connected { return Err("SQLite: Not connected".into()); }
        let mut row = Row::new();
        row.insert("CURSOR", FieldValue::Text(name.to_string()));
        row.insert("QUERY", FieldValue::Text(query.to_string()));
        Ok(Cursor::new(name, query, vec![row], CursorDirection::ForwardOnly))
    }

    fn close_cursor(&self, _name: &str) -> Result<(), String> {
        if !self.connected { return Err("SQLite: Not connected".into()); }
        Ok(())
    }

    fn create_view(&self, name: &str, query: &str, _materialized: bool) -> Result<String, String> {
        if !self.connected { return Err("SQLite: Not connected".into()); }
        Ok(format!("CREATE VIEW IF NOT EXISTS {} AS {}", name, query))
    }

    fn drop_view(&self, name: &str) -> Result<String, String> {
        if !self.connected { return Err("SQLite: Not connected".into()); }
        Ok(format!("DROP VIEW IF EXISTS {}", name))
    }

    fn query_view(&self, name: &str, filter: Option<&str>) -> Result<Vec<Row>, String> {
        if !self.connected { return Err("SQLite: Not connected".into()); }
        let mut row = Row::new();
        row.insert("VIEW", FieldValue::Text(name.to_string()));
        if let Some(f) = filter { row.insert("FILTER", FieldValue::Text(f.to_string())); }
        Ok(vec![row])
    }

    fn refresh_materialized_view(&self, name: &str) -> Result<String, String> {
        if !self.connected { return Err("SQLite: Not connected".into()); }
        Ok(format!("-- SQLite materialized view emulation: REFRESH {}", name))
    }

    fn execute_scalar_function(&self, name: &str, args: &[FieldValue]) -> Result<FieldValue, String> {
        if !self.connected { return Err("SQLite: Not connected".into()); }
        let args_str = args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
        Ok(FieldValue::Text(format!("{}({})", name, args_str)))
    }

    fn execute_table_function(&self, name: &str, args: &[FieldValue]) -> Result<Vec<Row>, String> {
        if !self.connected { return Err("SQLite: Not connected".into()); }
        let mut row = Row::new();
        row.insert("FUNCTION", FieldValue::Text(name.to_string()));
        row.insert("PARAMS", FieldValue::Int(args.len() as i64));
        Ok(vec![row])
    }
}
