use std::collections::HashMap;
use std::fmt;

/// Supported GraphQL type references in schemas and documents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphQLType {
    Named(String),
    NonNull(Box<GraphQLType>),
    List(Box<GraphQLType>),
}

impl GraphQLType {
    pub fn named(name: &str) -> Self {
        GraphQLType::Named(name.to_string())
    }

    pub fn non_null(inner: GraphQLType) -> Self {
        GraphQLType::NonNull(Box::new(inner))
    }

    pub fn list(inner: GraphQLType) -> Self {
        GraphQLType::List(Box::new(inner))
    }

    pub fn unwrap_name(&self) -> &str {
        match self {
            GraphQLType::Named(n) => n.as_str(),
            GraphQLType::NonNull(inner) => inner.unwrap_name(),
            GraphQLType::List(inner) => inner.unwrap_name(),
        }
    }
}

impl fmt::Display for GraphQLType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphQLType::Named(n) => write!(f, "{}", n),
            GraphQLType::NonNull(inner) => write!(f, "{}!", inner),
            GraphQLType::List(inner) => write!(f, "[{}]", inner),
        }
    }
}

/// Dynamic value representation for GraphQL execution, arguments, and responses.
#[derive(Debug, Clone, PartialEq)]
pub enum GraphQLValue {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Enum(String),
    List(Vec<GraphQLValue>),
    Object(HashMap<String, GraphQLValue>),
}

impl GraphQLValue {
    pub fn to_json(&self) -> String {
        match self {
            GraphQLValue::Int(i) => i.to_string(),
            GraphQLValue::Float(f) => {
                if f.fract() == 0.0 {
                    format!("{:.1}", f)
                } else {
                    f.to_string()
                }
            }
            GraphQLValue::String(s) => format!("\"{}\"", escape_json(s)),
            GraphQLValue::Boolean(b) => b.to_string(),
            GraphQLValue::Null => "null".to_string(),
            GraphQLValue::Enum(e) => format!("\"{}\"", escape_json(e)),
            GraphQLValue::List(list) => {
                let items: Vec<String> = list.iter().map(|item| item.to_json()).collect();
                format!("[{}]", items.join(","))
            }
            GraphQLValue::Object(map) => {
                let mut entries: Vec<(&String, &GraphQLValue)> = map.iter().collect();
                entries.sort_by_key(|(k, _)| *k);
                let kvs: Vec<String> = entries
                    .into_iter()
                    .map(|(k, v)| format!("\"{}\":{}", escape_json(k), v.to_json()))
                    .collect();
                format!("{{{}}}", kvs.join(","))
            }
        }
    }
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Definition of an argument on a field or directive.
#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentDefinition {
    pub name: String,
    pub description: Option<String>,
    pub arg_type: GraphQLType,
    pub default_value: Option<GraphQLValue>,
}

/// Field definition on an Object or Interface type.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDefinition {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Vec<ArgumentDefinition>,
    pub field_type: GraphQLType,
    pub deprecation_reason: Option<String>,
}

impl FieldDefinition {
    pub fn new(name: &str, field_type: GraphQLType) -> Self {
        FieldDefinition {
            name: name.to_string(),
            description: None,
            arguments: Vec::new(),
            field_type,
            deprecation_reason: None,
        }
    }

    pub fn with_arg(mut self, name: &str, arg_type: GraphQLType) -> Self {
        self.arguments.push(ArgumentDefinition {
            name: name.to_string(),
            description: None,
            arg_type,
            default_value: None,
        });
        self
    }
}

/// Input field definition for an InputObject type.
#[derive(Debug, Clone, PartialEq)]
pub struct InputFieldDefinition {
    pub name: String,
    pub description: Option<String>,
    pub field_type: GraphQLType,
    pub default_value: Option<GraphQLValue>,
}
