use super::{AdapterKind, Cursor, CursorDirection, DatabaseAdapter, ProcedureParam, ProcedureResult};
use crate::model::{FieldValue, Row};
use std::collections::HashMap;

/// Oracle Database Adapter (PL/SQL, Stored Procedures, Ref Cursors, Materialized Views).
pub struct OracleAdapter {
    pub connection_string: String,
    pub connected: bool,
}

impl OracleAdapter {
    pub fn new(connection_string: &str) -> Self {
        OracleAdapter {
            connection_string: connection_string.to_string(),
            connected: false,
        }
    }
}

impl DatabaseAdapter for OracleAdapter {
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
            return Err("Oracle adapter: Not connected to database".to_string());
        }
        Ok(vec![format!("Oracle execute: {}", sql)])
    }

    fn query(&self, sql: &str) -> Result<Vec<Row>, String> {
        if !self.connected {
            return Err("Oracle adapter: Not connected to database".to_string());
        }
        let mut row = Row::new();
        row.insert("STATEMENT", FieldValue::Text(sql.to_string()));
        row.insert("DB", FieldValue::Text("Oracle".to_string()));
        Ok(vec![row])
    }

    fn driver_name(&self) -> &str {
        "oracle"
    }

    fn kind(&self) -> AdapterKind {
        AdapterKind::Oracle
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn execute_procedure(&self, name: &str, params: &[ProcedureParam]) -> Result<ProcedureResult, String> {
        if !self.connected {
            return Err("Oracle adapter: Not connected to database".to_string());
        }
        let mut output_params = HashMap::new();
        let mut plsql_args = Vec::new();

        for p in params {
            match p.direction {
                super::ParamDirection::In => {
                    plsql_args.push(format!(":{} => {}", p.name, p.value));
                }
                super::ParamDirection::Out => {
                    plsql_args.push(format!(":{} => OUT", p.name));
                    output_params.insert(p.name.clone(), FieldValue::Text("ORACLE_OUT_VAL".to_string()));
                }
                super::ParamDirection::InOut => {
                    plsql_args.push(format!(":{} => IN OUT", p.name));
                    output_params.insert(p.name.clone(), p.value.clone());
                }
            }
        }

        let mut row = Row::new();
        row.insert("PROCEDURE", FieldValue::Text(name.to_string()));
        row.insert("PLSQL", FieldValue::Text(format!("BEGIN {}({}); END;", name, plsql_args.join(", "))));

        Ok(ProcedureResult {
            status: 0,
            output_params,
            result_sets: vec![vec![row]],
            rows_affected: 1,
        })
    }

    fn open_cursor(&self, name: &str, query: &str) -> Result<Cursor, String> {
        if !self.connected {
            return Err("Oracle adapter: Not connected to database".to_string());
        }
        let mut row = Row::new();
        row.insert("CURSOR_NAME", FieldValue::Text(name.to_string()));
        row.insert("QUERY", FieldValue::Text(query.to_string()));
        Ok(Cursor::new(name, query, vec![row], CursorDirection::ForwardOnly))
    }

    fn close_cursor(&self, _name: &str) -> Result<(), String> {
        if !self.connected {
            return Err("Oracle adapter: Not connected to database".to_string());
        }
        Ok(())
    }

    fn create_view(&self, name: &str, query: &str, materialized: bool) -> Result<String, String> {
        if !self.connected {
            return Err("Oracle adapter: Not connected to database".to_string());
        }
        let sql = if materialized {
            format!("CREATE MATERIALIZED VIEW {} BUILD IMMEDIATE REFRESH COMPLETE AS {}", name, query)
        } else {
            format!("CREATE OR REPLACE VIEW {} AS {}", name, query)
        };
        Ok(sql)
    }

    fn drop_view(&self, name: &str) -> Result<String, String> {
        if !self.connected {
            return Err("Oracle adapter: Not connected to database".to_string());
        }
        Ok(format!("DROP VIEW {}", name))
    }

    fn query_view(&self, name: &str, filter: Option<&str>) -> Result<Vec<Row>, String> {
        if !self.connected {
            return Err("Oracle adapter: Not connected to database".to_string());
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
            return Err("Oracle adapter: Not connected to database".to_string());
        }
        Ok(format!("BEGIN DBMS_MVIEW.REFRESH('{}'); END;", name))
    }

    fn execute_scalar_function(&self, name: &str, args: &[FieldValue]) -> Result<FieldValue, String> {
        if !self.connected {
            return Err("Oracle adapter: Not connected to database".to_string());
        }
        let args_str = args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
        Ok(FieldValue::Text(format!("ORACLE_FN_RESULT({}({}))", name, args_str)))
    }

    fn execute_table_function(&self, name: &str, args: &[FieldValue]) -> Result<Vec<Row>, String> {
        if !self.connected {
            return Err("Oracle adapter: Not connected to database".to_string());
        }
        let mut row = Row::new();
        row.insert("FUNCTION", FieldValue::Text(name.to_string()));
        row.insert("NUM_ARGS", FieldValue::Int(args.len() as i64));
        Ok(vec![row])
    }
}
