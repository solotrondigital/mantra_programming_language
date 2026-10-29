//! Model — Active Record base for Mantra ORM entities.

use std::collections::HashMap;
use std::fmt;

/// Represents a single typed field value.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
    Bytes(Vec<u8>),
    Null,
}

impl fmt::Display for FieldValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldValue::Int(i) => write!(f, "{}", i),
            FieldValue::Float(v) => write!(f, "{}", v),
            FieldValue::Text(s) => write!(f, "{}", s),
            FieldValue::Bool(b) => write!(f, "{}", b),
            FieldValue::Bytes(b) => write!(f, "<{} bytes>", b.len()),
            FieldValue::Null => write!(f, "NULL"),
        }
    }
}

/// A single row representing column name to FieldValue mappings.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Row {
    pub columns: HashMap<String, FieldValue>,
}

impl Row {
    pub fn new() -> Self {
        Row {
            columns: HashMap::new(),
        }
    }

    pub fn insert(&mut self, col: &str, val: FieldValue) {
        self.columns.insert(col.to_string(), val);
    }

    pub fn get(&self, col: &str) -> Option<&FieldValue> {
        self.columns.get(col)
    }

    pub fn get_text(&self, col: &str) -> Option<&str> {
        match self.columns.get(col) {
            Some(FieldValue::Text(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn get_int(&self, col: &str) -> Option<i64> {
        match self.columns.get(col) {
            Some(FieldValue::Int(i)) => Some(*i),
            _ => None,
        }
    }

    pub fn get_float(&self, col: &str) -> Option<f64> {
        match self.columns.get(col) {
            Some(FieldValue::Float(f)) => Some(*f),
            _ => None,
        }
    }

    pub fn get_bool(&self, col: &str) -> Option<bool> {
        match self.columns.get(col) {
            Some(FieldValue::Bool(b)) => Some(*b),
            _ => None,
        }
    }
}

/// Supported column types for schema definitions.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Integer,
    Float,
    Text,
    Boolean,
    Timestamp,
}

/// Metadata for a model field.
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub primary_key: bool,
    pub nullable: bool,
}

impl Field {
    pub fn new(name: &str, field_type: FieldType) -> Self {
        Field { name: name.to_string(), field_type, primary_key: false, nullable: true }
    }
    pub fn primary(mut self) -> Self { self.primary_key = true; self }
    pub fn not_null(mut self) -> Self { self.nullable = false; self }
}

/// Active Record base trait — implemented by generated Mantra model classes.
pub trait Model: Sized {
    /// Returns the database table name for this model.
    fn table_name() -> &'static str;
    /// Returns the field definitions for schema inference.
    fn fields() -> Vec<Field>;
    /// Serialises the model instance to a JSON string.
    fn to_json(&self) -> String;
    /// Persists this instance (INSERT or UPDATE).
    fn save(&self, adapter: &dyn super::adapters::DatabaseAdapter) -> bool;
}
