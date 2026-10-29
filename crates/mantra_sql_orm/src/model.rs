//! Model — Active Record base for Mantra ORM entities.

/// Represents a single typed field value.
#[derive(Debug, Clone)]
pub enum FieldValue {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
    Null,
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
