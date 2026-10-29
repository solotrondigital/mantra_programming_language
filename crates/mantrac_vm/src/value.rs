use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ObjectInstance {
    pub class_name: String,
    pub fields: HashMap<String, Value>,
}

impl ObjectInstance {
    pub fn new(class_name: String) -> Self {
        ObjectInstance {
            class_name,
            fields: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    None,
    Unit,
    List(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<HashMap<String, Value>>>),
    Object(Rc<RefCell<ObjectInstance>>),
    BuiltinFunction(String),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::None | Value::Unit => false,
            Value::List(l) => !l.borrow().is_empty(),
            Value::Map(m) => !m.borrow().is_empty(),
            Value::Object(_) | Value::BuiltinFunction(_) => true,
        }
    }

    pub fn to_string_repr(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => {
                if f.fract() == 0.0 {
                    format!("{:.1}", f)
                } else {
                    f.to_string()
                }
            }
            Value::Bool(b) => if *b { "True".to_string() } else { "False".to_string() },
            Value::String(s) => s.clone(),
            Value::None => "None".to_string(),
            Value::Unit => "None".to_string(),
            Value::List(items) => {
                let els: Vec<String> = items.borrow().iter().map(|v| v.to_string_repr()).collect();
                format!("[{}]", els.join(", "))
            }
            Value::Map(map) => {
                let entries: Vec<String> = map
                    .borrow()
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.to_string_repr()))
                    .collect();
                format!("{{{}}}", entries.join(", "))
            }
            Value::Object(obj) => {
                let borrowed = obj.borrow();
                format!("<{} instance>", borrowed.class_name)
            }
            Value::BuiltinFunction(name) => format!("<built-in function {}>", name),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Int(a), Value::Float(b)) => (*a as f64 - b).abs() < f64::EPSILON,
            (Value::Float(a), Value::Int(b)) => (a - *b as f64).abs() < f64::EPSILON,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::None, Value::None) => true,
            (Value::Unit, Value::Unit) => true,
            (Value::List(a), Value::List(b)) => *a.borrow() == *b.borrow(),
            (Value::Object(a), Value::Object(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

