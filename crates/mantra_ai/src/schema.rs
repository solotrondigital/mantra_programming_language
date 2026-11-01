//! Structured Output and Schema Validation for Mantra AI.

use std::collections::HashMap;
use crate::error::{AIError, AIResult};

/// JSON-compatible Value representation for structured AI outputs without external crates.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(map) => map.get(key),
            _ => None,
        }
    }

    /// Basic JSON serializer
    pub fn to_json_string(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|item| item.to_json_string()).collect();
                format!("[{}]", items.join(","))
            }
            JsonValue::Object(obj) => {
                let pairs: Vec<String> = obj.iter().map(|(k, v)| {
                    format!("\"{}\":{}", k, v.to_json_string())
                }).collect();
                format!("{{{}}}", pairs.join(","))
            }
        }
    }
}

/// Primitive schema data types expected from an AI response.
#[derive(Debug, Clone, PartialEq)]
pub enum SchemaType {
    String,
    Number,
    Boolean,
    Array(Box<SchemaType>),
    Object(HashMap<String, (SchemaType, bool)>), // (type, required)
}

/// Schema definition for validating structured responses (User Story AI-005).
#[derive(Debug, Clone, PartialEq)]
pub struct StructuredSchema {
    pub name: String,
    pub root_type: SchemaType,
}

impl StructuredSchema {
    pub fn new(name: impl Into<String>, root: SchemaType) -> Self {
        Self {
            name: name.into(),
            root_type: root,
        }
    }

    /// Validate a JSON value against this schema.
    pub fn validate(&self, val: &JsonValue) -> AIResult<()> {
        Self::check_type(&self.root_type, val, &self.name)
    }

    fn check_type(schema: &SchemaType, val: &JsonValue, path: &str) -> AIResult<()> {
        match (schema, val) {
            (SchemaType::String, JsonValue::String(_)) => Ok(()),
            (SchemaType::Number, JsonValue::Number(_)) => Ok(()),
            (SchemaType::Boolean, JsonValue::Bool(_)) => Ok(()),
            (SchemaType::Array(elem_type), JsonValue::Array(items)) => {
                for (idx, item) in items.iter().enumerate() {
                    let sub_path = format!("{}[{}]", path, idx);
                    Self::check_type(elem_type, item, &sub_path)?;
                }
                Ok(())
            }
            (SchemaType::Object(expected_fields), JsonValue::Object(actual_map)) => {
                for (field_name, (field_type, is_required)) in expected_fields {
                    let sub_path = format!("{}.{}", path, field_name);
                    match actual_map.get(field_name) {
                        Some(actual_val) => {
                            Self::check_type(field_type, actual_val, &sub_path)?;
                        }
                        None => {
                            if *is_required {
                                return Err(AIError::SchemaValidationError {
                                    expected: format!("Field '{}' is required", field_name),
                                    actual: "Missing field".to_string(),
                                    details: format!("Object at '{}' lacks required field '{}'", path, field_name),
                                });
                            }
                        }
                    }
                }
                Ok(())
            }
            (expected, actual) => {
                Err(AIError::SchemaValidationError {
                    expected: format!("{:?}", expected),
                    actual: format!("{:?}", actual),
                    details: format!("Type mismatch at '{}'", path),
                })
            }
        }
    }
}

