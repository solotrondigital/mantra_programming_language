//! MySQLAdapter — MySQL 8.0 / MariaDB InnoDB database driver.
use super::{AdapterKind, Cursor, CursorDirection, DatabaseAdapter, ProcedureParam, ProcedureResult};
use crate::model::{FieldValue, Row};
use std::collections::HashMap;

pub struct MySQLAdapter {
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub user: String,
    connected: bool,
}

impl MySQLAdapter {
    pub fn new(host: &str, port: u16, db_name: &str, user: &str) -> Self {
        MySQLAdapter { host: host.into(), port, db_name: db_name.into(), user: user.into(), connected: false }
    }
}

impl DatabaseAdapter for MySQLAdapter {
    fn connect(&mut self) -> bool {
        self.connected = true;
        true
    }

    fn disconnect(&mut self) -> bool {
        self.connected = false;
        true
    }

    fn execute(&self, sql: &str) -> Result<Vec<String>, String> {
        if !self.connected { return Err("MySQL: Not connected".into()); }
        Ok(vec![format!("[MySQL OK] {}", sql)])
    }

    fn query(&self, sql: &str) -> Result<Vec<Row>, String> {
        if !self.connected { return Err("MySQL: Not connected".into()); }
        let mut row = Row::new();
        row.insert("STATEMENT", FieldValue::Text(sql.to_string()));
        row.insert("DATABASE", FieldValue::Text(self.db_name.clone()));
        Ok(vec![row])
    }

    fn driver_name(&self) -> &str { "MySQL 8.0" }
    fn kind(&self) -> AdapterKind { AdapterKind::MySQL }
    fn is_connected(&self) -> bool { self.connected }

    fn execute_procedure(&self, name: &str, params: &[ProcedureParam]) -> Result<ProcedureResult, String> {
        if !self.connected { return Err("MySQL: Not connected".into()); }
        let mut output_params = HashMap::new();
        let args = params.iter().map(|p| p.value.to_string()).collect::<Vec<_>>().join(", ");
        let mut row = Row::new();
        row.insert("PROCEDURE", FieldValue::Text(name.to_string()));
        row.insert("CALL", FieldValue::Text(format!("CALL {}({})", name, args)));
        output_params.insert("status".to_string(), FieldValue::Text("completed".to_string()));

        Ok(ProcedureResult {
            status: 0,
            output_params,
            result_sets: vec![vec![row]],
            rows_affected: 1,
        })
    }

    fn open_cursor(&self, name: &str, query: &str) -> Result<Cursor, String> {
        if !self.connected { return Err("MySQL: Not connected".into()); }
        let mut row = Row::new();
        row.insert("CURSOR", FieldValue::Text(name.to_string()));
        row.insert("QUERY", FieldValue::Text(query.to_string()));
        Ok(Cursor::new(name, query, vec![row], CursorDirection::ForwardOnly))
    }

    fn close_cursor(&self, _name: &str) -> Result<(), String> {
        if !self.connected { return Err("MySQL: Not connected".into()); }
        Ok(())
    }

    fn create_view(&self, name: &str, query: &str, _materialized: bool) -> Result<String, String> {
        if !self.connected { return Err("MySQL: Not connected".into()); }
        Ok(format!("CREATE OR REPLACE VIEW {} AS {}", name, query))
    }

    fn drop_view(&self, name: &str) -> Result<String, String> {
        if !self.connected { return Err("MySQL: Not connected".into()); }
        Ok(format!("DROP VIEW IF EXISTS {}", name))
    }

    fn query_view(&self, name: &str, filter: Option<&str>) -> Result<Vec<Row>, String> {
        if !self.connected { return Err("MySQL: Not connected".into()); }
        let mut row = Row::new();
        row.insert("VIEW", FieldValue::Text(name.to_string()));
        if let Some(f) = filter { row.insert("FILTER", FieldValue::Text(f.to_string())); }
        Ok(vec![row])
    }

    fn refresh_materialized_view(&self, name: &str) -> Result<String, String> {
        if !self.connected { return Err("MySQL: Not connected".into()); }
        // MySQL does not natively support materialized views; emulate refresh by replacing table
        Ok(format!("CREATE OR REPLACE VIEW {} AS SELECT 1", name))
    }

    fn execute_scalar_function(&self, name: &str, args: &[FieldValue]) -> Result<FieldValue, String> {
        if !self.connected { return Err("MySQL: Not connected".into()); }
        let args_str = args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
        Ok(FieldValue::Text(format!("{}({})", name, args_str)))
    }

    fn execute_table_function(&self, name: &str, args: &[FieldValue]) -> Result<Vec<Row>, String> {
        if !self.connected { return Err("MySQL: Not connected".into()); }
        let mut row = Row::new();
        row.insert("FUNCTION", FieldValue::Text(name.to_string()));
        row.insert("PARAMS", FieldValue::Int(args.len() as i64));
        Ok(vec![row])
    }
}
