//! JsonSerializer — JSON value model and serializer.

/// A JSON value.
#[derive(Debug, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(JsonObject),
}

/// An ordered JSON object (key-value map).
#[derive(Debug, Clone, Default)]
pub struct JsonObject {
    pub fields: Vec<(String, JsonValue)>,
}

impl JsonObject {
    pub fn new() -> Self { JsonObject::default() }
    pub fn field(mut self, key: &str, val: JsonValue) -> Self {
        self.fields.push((key.to_string(), val));
        self
    }
    pub fn to_json(&self) -> String {
        let pairs: Vec<String> = self.fields.iter().map(|(k, v)| {
            format!("\"{}\":{}", k, JsonSerializer::serialize(v))
        }).collect();
        format!("{{{}}}", pairs.join(","))
    }
}

/// JSON serializer — converts JsonValue trees to JSON strings.
pub struct JsonSerializer;

impl JsonSerializer {
    pub fn serialize(val: &JsonValue) -> String {
        match val {
            JsonValue::Null         => "null".into(),
            JsonValue::Bool(b)      => b.to_string(),
            JsonValue::Number(n)    => {
                if n.fract() == 0.0 { format!("{}", *n as i64) } else { format!("{}", n) }
            }
            JsonValue::Str(s)       => format!("\"{}\"", s.replace('"', "\\\"")),
            JsonValue::Array(items) => {
                let parts: Vec<String> = items.iter().map(Self::serialize).collect();
                format!("[{}]", parts.join(","))
            }
            JsonValue::Object(obj)  => obj.to_json(),
        }
    }

    pub fn array(items: Vec<JsonValue>) -> String {
        let parts: Vec<String> = items.iter().map(Self::serialize).collect();
        format!("[{}]", parts.join(","))
    }
}
