use super::{AdapterKind, Cursor, CursorDirection, DatabaseAdapter, ProcedureParam, ProcedureResult};
use crate::model::{FieldValue, Row};
use std::collections::HashMap;

/// MongoDB Document Database Adapter.
pub struct MongoDBAdapter {
    pub uri: String,
    pub database: String,
    pub connected: bool,
}

impl MongoDBAdapter {
    pub fn new(uri: &str, database: &str) -> Self {
        MongoDBAdapter {
            uri: uri.to_string(),
            database: database.to_string(),
            connected: false,
        }
    }
}

impl DatabaseAdapter for MongoDBAdapter {
    fn connect(&mut self) -> bool {
        self.connected = true;
        true
    }

    fn disconnect(&mut self) -> bool {
        self.connected = false;
        true
    }

    fn execute(&self, command: &str) -> Result<Vec<String>, String> {
        if !self.connected {
            return Err("MongoDB adapter: Not connected to database".to_string());
        }
        Ok(vec![format!("MongoDB execute on {}: {}", self.database, command)])
    }

    fn query(&self, command: &str) -> Result<Vec<Row>, String> {
        if !self.connected {
            return Err("MongoDB adapter: Not connected to database".to_string());
        }
        let mut row = Row::new();
        row.insert("_id", FieldValue::Text("60c72b2f9b1d8b001c8e4d2a".to_string()));
        row.insert("command", FieldValue::Text(command.to_string()));
        row.insert("db", FieldValue::Text(self.database.clone()));
        Ok(vec![row])
    }

    fn driver_name(&self) -> &str {
        "mongodb"
    }

    fn kind(&self) -> AdapterKind {
        AdapterKind::MongoDB
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn execute_procedure(&self, name: &str, params: &[ProcedureParam]) -> Result<ProcedureResult, String> {
        if !self.connected {
            return Err("MongoDB adapter: Not connected to database".to_string());
        }
        let mut output_params = HashMap::new();
        let args_repr = params.iter().map(|p| format!("{}: {}", p.name, p.value)).collect::<Vec<_>>().join(", ");

        let mut row = Row::new();
        row.insert("system_function", FieldValue::Text(name.to_string()));
        row.insert("args", FieldValue::Text(args_repr));
        output_params.insert("status".to_string(), FieldValue::Text("executed".to_string()));

        Ok(ProcedureResult {
            status: 1,
            output_params,
            result_sets: vec![vec![row]],
            rows_affected: 1,
        })
    }

    fn open_cursor(&self, name: &str, query: &str) -> Result<Cursor, String> {
        if !self.connected {
            return Err("MongoDB adapter: Not connected to database".to_string());
        }
        let mut doc = Row::new();
        doc.insert("_id", FieldValue::Text("doc_cursor_1".to_string()));
        doc.insert("filter", FieldValue::Text(query.to_string()));
        Ok(Cursor::new(name, query, vec![doc], CursorDirection::ForwardOnly))
    }

    fn close_cursor(&self, _name: &str) -> Result<(), String> {
        if !self.connected {
            return Err("MongoDB adapter: Not connected to database".to_string());
        }
        Ok(())
    }

    fn create_view(&self, name: &str, pipeline_or_query: &str, _materialized: bool) -> Result<String, String> {
        if !self.connected {
            return Err("MongoDB adapter: Not connected to database".to_string());
        }
        Ok(format!("db.createView('{}', 'collection', {})", name, pipeline_or_query))
    }

    fn drop_view(&self, name: &str) -> Result<String, String> {
        if !self.connected {
            return Err("MongoDB adapter: Not connected to database".to_string());
        }
        Ok(format!("db.{}.drop()", name))
    }

    fn query_view(&self, name: &str, filter: Option<&str>) -> Result<Vec<Row>, String> {
        if !self.connected {
            return Err("MongoDB adapter: Not connected to database".to_string());
        }
        let mut doc = Row::new();
        doc.insert("view", FieldValue::Text(name.to_string()));
        if let Some(f) = filter {
            doc.insert("filter", FieldValue::Text(f.to_string()));
        }
        Ok(vec![doc])
    }

    fn refresh_materialized_view(&self, name: &str) -> Result<String, String> {
        if !self.connected {
            return Err("MongoDB adapter: Not connected to database".to_string());
        }
        // In MongoDB on-demand materialized views are written via $merge or $out aggregation
        Ok(format!("db.{}.aggregate([{{ $match: {{}} }}, {{ $merge: {{ into: '{}_mat' }} }}])", name, name))
    }

    fn execute_scalar_function(&self, name: &str, args: &[FieldValue]) -> Result<FieldValue, String> {
        if !self.connected {
            return Err("MongoDB adapter: Not connected to database".to_string());
        }
        let args_str = args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
        Ok(FieldValue::Text(format!("db.eval('{}({})')", name, args_str)))
    }

    fn execute_table_function(&self, name: &str, args: &[FieldValue]) -> Result<Vec<Row>, String> {
        if !self.connected {
            return Err("MongoDB adapter: Not connected to database".to_string());
        }
        let mut row = Row::new();
        row.insert("pipeline_func", FieldValue::Text(name.to_string()));
        row.insert("args_len", FieldValue::Int(args.len() as i64));
        Ok(vec![row])
    }
}
