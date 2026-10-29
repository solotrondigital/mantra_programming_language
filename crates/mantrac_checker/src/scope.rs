use mantrac_ast::Type;
use mantrac_lexer::span::Span;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VarSymbol {
    pub name: String,
    pub ty: Type,
    pub is_mut: bool,
    pub span: Span,
}

#[derive(Debug, Clone, Default)]
pub struct Scope {
    variables: HashMap<String, VarSymbol>,
}

pub struct ScopeStack {
    scopes: Vec<Scope>,
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack {
            scopes: vec![Scope::default()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn insert(&mut self, name: String, ty: Type, is_mut: bool, span: Span) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.variables.insert(
                name.clone(),
                VarSymbol {
                    name,
                    ty,
                    is_mut,
                    span,
                },
            );
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&VarSymbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.variables.get(name) {
                return Some(var);
            }
        }
        None
    }
}

