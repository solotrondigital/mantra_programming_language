//! Database adapter trait, types, and vendor drivers.

pub mod generic;
pub mod mongodb;
pub mod mssql;
pub mod mysql;
pub mod oracle;
pub mod postgres;
pub mod sqlite;

use crate::model::{FieldValue, Row};
use std::collections::HashMap;

/// Identifies which driver is in use.
#[derive(Debug, Clone, PartialEq)]
pub enum AdapterKind {
    MySQL,
    PostgreSQL,
    SQLite,
    Oracle,
    MSSQL,
    MongoDB,
    Generic(String),
}

/// Parameter direction for stored procedures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamDirection {
    In,
    Out,
    InOut,
}

/// Parameter definition for stored procedure execution.
#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureParam {
    pub name: String,
    pub value: FieldValue,
    pub direction: ParamDirection,
    pub data_type: String,
}

impl ProcedureParam {
    pub fn input(name: &str, value: FieldValue) -> Self {
        ProcedureParam {
            name: name.to_string(),
            value,
            direction: ParamDirection::In,
            data_type: "VARCHAR".to_string(),
        }
    }

    pub fn output(name: &str, data_type: &str) -> Self {
        ProcedureParam {
            name: name.to_string(),
            value: FieldValue::Null,
            direction: ParamDirection::Out,
            data_type: data_type.to_string(),
        }
    }

    pub fn inout(name: &str, value: FieldValue, data_type: &str) -> Self {
        ProcedureParam {
            name: name.to_string(),
            value,
            direction: ParamDirection::InOut,
            data_type: data_type.to_string(),
        }
    }
}

/// Execution outcome of a stored procedure.
#[derive(Debug, Clone, Default)]
pub struct ProcedureResult {
    pub status: i32,
    pub output_params: HashMap<String, FieldValue>,
    pub result_sets: Vec<Vec<Row>>,
    pub rows_affected: u64,
}

/// Direction for database cursor traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorDirection {
    ForwardOnly,
    Scrollable,
}

/// Active database cursor for streaming or paginated row retrieval.
#[derive(Debug, Clone)]
pub struct Cursor {
    pub name: String,
    pub query: String,
    pub is_open: bool,
    pub position: usize,
    pub rows: Vec<Row>,
    pub direction: CursorDirection,
}

impl Cursor {
    pub fn new(name: &str, query: &str, rows: Vec<Row>, direction: CursorDirection) -> Self {
        Cursor {
            name: name.to_string(),
            query: query.to_string(),
            is_open: true,
            position: 0,
            rows,
            direction,
        }
    }

    pub fn fetch_next(&mut self) -> Result<Option<Row>, String> {
        if !self.is_open {
            return Err(format!("Cursor '{}' is closed", self.name));
        }
        if self.position < self.rows.len() {
            let row = self.rows[self.position].clone();
            self.position += 1;
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }

    pub fn fetch_many(&mut self, count: usize) -> Result<Vec<Row>, String> {
        if !self.is_open {
            return Err(format!("Cursor '{}' is closed", self.name));
        }
        let end = (self.position + count).min(self.rows.len());
        let slice = self.rows[self.position..end].to_vec();
        self.position = end;
        Ok(slice)
    }

    pub fn close(&mut self) -> Result<(), String> {
        self.is_open = false;
        Ok(())
    }
}

/// Universal database driver interface — implement to support any backend.
pub trait DatabaseAdapter: Send + Sync {
    fn connect(&mut self) -> bool;
    fn disconnect(&mut self) -> bool;
    fn execute(&self, sql: &str) -> Result<Vec<String>, String>;
    fn query(&self, sql: &str) -> Result<Vec<Row>, String>;
    fn driver_name(&self) -> &str;
    fn kind(&self) -> AdapterKind;
    fn is_connected(&self) -> bool;

    // ── Stored Procedures ───────────────────────────────────────────────────
    fn execute_procedure(&self, name: &str, params: &[ProcedureParam]) -> Result<ProcedureResult, String>;

    // ── Cursors ─────────────────────────────────────────────────────────────
    fn open_cursor(&self, name: &str, query: &str) -> Result<Cursor, String>;
    fn close_cursor(&self, name: &str) -> Result<(), String>;

    // ── Views ───────────────────────────────────────────────────────────────
    fn create_view(&self, name: &str, query: &str, materialized: bool) -> Result<String, String>;
    fn drop_view(&self, name: &str) -> Result<String, String>;
    fn query_view(&self, name: &str, filter: Option<&str>) -> Result<Vec<Row>, String>;
    fn refresh_materialized_view(&self, name: &str) -> Result<String, String>;

    // ── Database Functions ──────────────────────────────────────────────────
    fn execute_scalar_function(&self, name: &str, args: &[FieldValue]) -> Result<FieldValue, String>;
    fn execute_table_function(&self, name: &str, args: &[FieldValue]) -> Result<Vec<Row>, String>;
}
