use super::{AdapterKind, Cursor, CursorDirection, DatabaseAdapter, ProcedureParam, ProcedureResult};
use crate::model::{FieldValue, Row};
use std::collections::HashMap;

/// Microsoft SQL Server (MSSQL / Azure SQL) Database Adapter.
pub struct MSSQLAdapter {
    pub connection_string: String,
    pub connected: bool,
}

impl MSSQLAdapter {
    pub fn new(connection_string: &str) -> Self {
        MSSQLAdapter {
            connection_string: connection_string.to_string(),
            connected: false,
        }
    }
}

impl DatabaseAdapter for MSSQLAdapter {
    fn connect(&mut self) -> bool {
        self.connected = true;
        true
    }

    fn disconnect(&mut self) -> bool {
        self.connected = false;
        true
    }

    fn execute(&self, sql: &str) -> Result<Vec<String>, String> {
        if !self.connected {
            return Err("MSSQL adapter: Not connected to database".to_string());
        }
        Ok(vec![format!("MSSQL execute: {}", sql)])
    }

    fn query(&self, sql: &str) -> Result<Vec<Row>, String> {
        if !self.connected {
            return Err("MSSQL adapter: Not connected to database".to_string());
        }
        let mut row = Row::new();
        row.insert("STATEMENT", FieldValue::Text(sql.to_string()));
        row.insert("DB", FieldValue::Text("MSSQL".to_string()));
        Ok(vec![row])
    }

    fn driver_name(&self) -> &str {
        "mssql"
    }

    fn kind(&self) -> AdapterKind {
        AdapterKind::MSSQL
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn execute_procedure(&self, name: &str, params: &[ProcedureParam]) -> Result<ProcedureResult, String> {
        if !self.connected {
            return Err("MSSQL adapter: Not connected to database".to_string());
        }
        let mut output_params = HashMap::new();
        let mut tsql_args = Vec::new();

        for p in params {
            match p.direction {
                super::ParamDirection::In => {
                    tsql_args.push(format!("@{} = {}", p.name, p.value));
                }
                super::ParamDirection::Out => {
                    tsql_args.push(format!("@{} OUTPUT", p.name));
                    output_params.insert(p.name.clone(), FieldValue::Text("MSSQL_OUT_VAL".to_string()));
                }
                super::ParamDirection::InOut => {
                    tsql_args.push(format!("@{} = {} OUTPUT", p.name, p.value));
                    output_params.insert(p.name.clone(), p.value.clone());
                }
            }
        }

        let mut row = Row::new();
        row.insert("PROCEDURE", FieldValue::Text(name.to_string()));
        row.insert("TSQL", FieldValue::Text(format!("EXEC {} {}", name, tsql_args.join(", "))));

        Ok(ProcedureResult {
            status: 0,
            output_params,
            result_sets: vec![vec![row]],
            rows_affected: 1,
        })
    }

    fn open_cursor(&self, name: &str, query: &str) -> Result<Cursor, String> {
        if !self.connected {
            return Err("MSSQL adapter: Not connected to database".to_string());
        }
        let mut row = Row::new();
        row.insert("CURSOR_NAME", FieldValue::Text(name.to_string()));
        row.insert("QUERY", FieldValue::Text(query.to_string()));
        Ok(Cursor::new(name, query, vec![row], CursorDirection::Scrollable))
    }

    fn close_cursor(&self, _name: &str) -> Result<(), String> {
        if !self.connected {
            return Err("MSSQL adapter: Not connected to database".to_string());
        }
        Ok(())
    }

    fn create_view(&self, name: &str, query: &str, _materialized: bool) -> Result<String, String> {
        if !self.connected {
            return Err("MSSQL adapter: Not connected to database".to_string());
        }
        Ok(format!("CREATE OR ALTER VIEW {} AS {}", name, query))
    }

    fn drop_view(&self, name: &str) -> Result<String, String> {
        if !self.connected {
            return Err("MSSQL adapter: Not connected to database".to_string());
        }
        Ok(format!("DROP VIEW IF EXISTS {}", name))
    }

    fn query_view(&self, name: &str, filter: Option<&str>) -> Result<Vec<Row>, String> {
        if !self.connected {
            return Err("MSSQL adapter: Not connected to database".to_string());
        }
        let mut row = Row::new();
        row.insert("VIEW_NAME", FieldValue::Text(name.to_string()));
        if let Some(f) = filter {
            row.insert("FILTER", FieldValue::Text(f.to_string()));
        }
        Ok(vec![row])
    }

    fn refresh_materialized_view(&self, name: &str) -> Result<String, String> {
        if !self.connected {
            return Err("MSSQL adapter: Not connected to database".to_string());
        }
        Ok(format!("EXEC sp_refreshview '{}'", name))
    }

    fn execute_scalar_function(&self, name: &str, args: &[FieldValue]) -> Result<FieldValue, String> {
        if !self.connected {
            return Err("MSSQL adapter: Not connected to database".to_string());
        }
        let args_str = args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
        Ok(FieldValue::Text(format!("dbo.{}({})", name, args_str)))
    }

    fn execute_table_function(&self, name: &str, args: &[FieldValue]) -> Result<Vec<Row>, String> {
        if !self.connected {
            return Err("MSSQL adapter: Not connected to database".to_string());
        }
        let mut row = Row::new();
        row.insert("TABLE_FUNCTION", FieldValue::Text(name.to_string()));
        row.insert("ARGS_COUNT", FieldValue::Int(args.len() as i64));
        Ok(vec![row])
    }
}
