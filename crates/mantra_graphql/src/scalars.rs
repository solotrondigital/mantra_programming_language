use crate::types::GraphQLValue;

pub trait CustomScalar: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> Option<&str>;
    fn parse_value(&self, value: &GraphQLValue) -> Result<GraphQLValue, String>;
    fn serialize(&self, value: &GraphQLValue) -> Result<GraphQLValue, String>;
}

pub struct DateTimeScalar;
impl CustomScalar for DateTimeScalar {
    fn name(&self) -> &str {
        "DateTime"
    }
    fn description(&self) -> Option<&str> {
        Some("ISO-8601 compliant UTC date-time string")
    }
    fn parse_value(&self, value: &GraphQLValue) -> Result<GraphQLValue, String> {
        match value {
            GraphQLValue::String(s) => {
                // Validate basic ISO-8601 format e.g. "2026-10-29T21:00:00Z"
                if s.contains('T') || s.contains('-') {
                    Ok(GraphQLValue::String(s.clone()))
                } else {
                    Err(format!("Invalid DateTime format: '{}'", s))
                }
            }
            _ => Err("DateTime must be an ISO-8601 string".to_string()),
        }
    }
    fn serialize(&self, value: &GraphQLValue) -> Result<GraphQLValue, String> {
        self.parse_value(value)
    }
}

pub struct JsonScalar;
impl CustomScalar for JsonScalar {
    fn name(&self) -> &str {
        "JSON"
    }
    fn description(&self) -> Option<&str> {
        Some("Arbitrary dynamic JSON payload scalar")
    }
    fn parse_value(&self, value: &GraphQLValue) -> Result<GraphQLValue, String> {
        Ok(value.clone())
    }
    fn serialize(&self, value: &GraphQLValue) -> Result<GraphQLValue, String> {
        Ok(value.clone())
    }
}

pub struct UuidScalar;
impl CustomScalar for UuidScalar {
    fn name(&self) -> &str {
        "UUID"
    }
    fn description(&self) -> Option<&str> {
        Some("Universally Unique Identifier (UUID) string")
    }
    fn parse_value(&self, value: &GraphQLValue) -> Result<GraphQLValue, String> {
        match value {
            GraphQLValue::String(s) => {
                let clean = s.trim();
                if clean.len() == 36 && clean.chars().filter(|c| *c == '-').count() == 4 {
                    Ok(GraphQLValue::String(clean.to_string()))
                } else {
                    Err(format!("Invalid UUID string: '{}'", s))
                }
            }
            _ => Err("UUID must be a 36-character hyphenated hex string".to_string()),
        }
    }
    fn serialize(&self, value: &GraphQLValue) -> Result<GraphQLValue, String> {
        self.parse_value(value)
    }
}

pub fn is_built_in_scalar(name: &str) -> bool {
    matches!(name, "Int" | "Float" | "String" | "Boolean" | "ID")
}

pub fn coerce_scalar(name: &str, value: &GraphQLValue) -> Result<GraphQLValue, String> {
    match name {
        "Int" => match value {
            GraphQLValue::Int(i) => Ok(GraphQLValue::Int(*i)),
            _ => Err(format!("Expected Int, got {:?}", value)),
        },
        "Float" => match value {
            GraphQLValue::Float(f) => Ok(GraphQLValue::Float(*f)),
            GraphQLValue::Int(i) => Ok(GraphQLValue::Float(*i as f64)),
            _ => Err(format!("Expected Float, got {:?}", value)),
        },
        "String" => match value {
            GraphQLValue::String(s) => Ok(GraphQLValue::String(s.clone())),
            _ => Err(format!("Expected String, got {:?}", value)),
        },
        "Boolean" => match value {
            GraphQLValue::Boolean(b) => Ok(GraphQLValue::Boolean(*b)),
            _ => Err(format!("Expected Boolean, got {:?}", value)),
        },
        "ID" => match value {
            GraphQLValue::String(s) => Ok(GraphQLValue::String(s.clone())),
            GraphQLValue::Int(i) => Ok(GraphQLValue::String(i.to_string())),
            _ => Err(format!("Expected ID, got {:?}", value)),
        },
        "DateTime" => DateTimeScalar.parse_value(value),
        "JSON" => JsonScalar.parse_value(value),
        "UUID" => UuidScalar.parse_value(value),
        _ => Ok(value.clone()),
    }
}

