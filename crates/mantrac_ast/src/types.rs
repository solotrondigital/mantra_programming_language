use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Byte,
    Unit,
    None,
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Custom(String, Vec<Type>),
    TypeVar(String),
    Any,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Bool"),
            Type::String => write!(f, "String"),
            Type::Byte => write!(f, "Byte"),
            Type::Unit => write!(f, "Unit"),
            Type::None => write!(f, "None"),
            Type::Option(inner) => write!(f, "Option<{}>", inner),
            Type::Result(ok, err) => write!(f, "Result<{}, {}>", ok, err),
            Type::List(elem) => write!(f, "List<{}>", elem),
            Type::Map(k, v) => write!(f, "Map<{}, {}>", k, v),
            Type::Custom(name, args) => {
                if args.is_empty() {
                    write!(f, "{}", name)
                } else {
                    let formatted_args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                    write!(f, "{}<{}>", name, formatted_args.join(", "))
                }
            }
            Type::TypeVar(tv) => write!(f, "{}", tv),
            Type::Any => write!(f, "Any"),
        }
    }
}

