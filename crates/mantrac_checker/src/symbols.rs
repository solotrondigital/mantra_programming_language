use mantrac_ast::{Type, Visibility};
use mantrac_lexer::span::Span;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FieldSymbol {
    pub name: String,
    pub ty: Type,
    pub visibility: Visibility,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MethodSymbol {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub visibility: Visibility,
    pub is_override: bool,
    pub is_static: bool,
    pub is_abstract: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ClassSymbol {
    pub name: String,
    pub base_class: Option<String>,
    pub interfaces: Vec<String>,
    pub is_abstract: bool,
    pub visibility: Visibility,
    pub fields: HashMap<String, FieldSymbol>,
    pub methods: HashMap<String, MethodSymbol>,
    pub static_fields: HashMap<String, FieldSymbol>,
    pub static_methods: HashMap<String, MethodSymbol>,
    pub constructor: Option<MethodSymbol>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct InterfaceSymbol {
    pub name: String,
    pub methods: HashMap<String, MethodSymbol>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FunctionSymbol {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub visibility: Visibility,
    pub span: Span,
}

#[derive(Debug, Clone, Default)]
pub struct GlobalSymbolTable {
    pub classes: HashMap<String, ClassSymbol>,
    pub interfaces: HashMap<String, InterfaceSymbol>,
    pub functions: HashMap<String, FunctionSymbol>,
}

