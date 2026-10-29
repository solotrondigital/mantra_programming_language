use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrType {
    Int,
    Float,
    Bool,
    String,
    Void,
    Custom(String),
}

impl fmt::Display for IrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrType::Int => write!(f, "i64"),
            IrType::Float => write!(f, "f64"),
            IrType::Bool => write!(f, "bool"),
            IrType::String => write!(f, "str"),
            IrType::Void => write!(f, "void"),
            IrType::Custom(name) => write!(f, "ptr<{}>", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Var(String),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrInstruction {
    Const { dest: String, val: IrValue },
    Copy { dest: String, src: String },
    Binary { op: IrBinaryOp, dest: String, left: String, right: String },
    Call { dest: Option<String>, func: String, args: Vec<String> },
    CallAsync { dest: Option<String>, func: String, args: Vec<String> },
    Await { dest: String, task: String },
    Print { arg: String },
    Jump { target: usize },
    Branch { cond: String, true_target: usize, false_target: usize },
    Return { val: Option<String> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<(String, IrType)>,
    pub return_type: IrType,
    pub is_async: bool,
    pub instructions: Vec<IrInstruction>,
    pub locals: Vec<(String, IrType)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrProgram {
    pub functions: Vec<IrFunction>,
    pub has_unsupported_features: bool,
    pub fallback_reason: Option<String>,
}

impl IrProgram {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            has_unsupported_features: false,
            fallback_reason: None,
        }
    }
}
