use crate::diagnostics::Diagnostic;
use crate::scope::ScopeStack;
use crate::symbols::*;
use mantrac_ast::*;
use mantrac_lexer::span::Span;
use std::collections::{HashMap, HashSet};

pub struct TypeChecker {
    pub symbols: GlobalSymbolTable,
    pub diagnostics: Vec<Diagnostic>,
    pub current_class: Option<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut tc = TypeChecker {
            symbols: GlobalSymbolTable::default(),
            diagnostics: Vec::new(),
            current_class: None,
        };
        tc.register_builtins();
        tc
    }

    fn register_builtins(&mut self) {
        // Built-in functions
        self.symbols.functions.insert(
            "print".to_string(),
            FunctionSymbol {
                name: "print".to_string(),
                params: vec![("value".to_string(), Type::Any)],
                return_type: Type::Unit,
                visibility: Visibility::Pub,
                span: Span::default(),
            },
        );
        self.symbols.functions.insert(
            "len".to_string(),
            FunctionSymbol {
                name: "len".to_string(),
                params: vec![("collection".to_string(), Type::Any)],
                return_type: Type::Int,
                visibility: Visibility::Pub,
                span: Span::default(),
            },
        );
        self.symbols.functions.insert(
            "range".to_string(),
            FunctionSymbol {
                name: "range".to_string(),
                params: vec![("n".to_string(), Type::Int)],
                return_type: Type::List(Box::new(Type::Int)),
                visibility: Visibility::Pub,
                span: Span::default(),
            },
        );
        self.symbols.functions.insert(
            "str".to_string(),
            FunctionSymbol {
                name: "str".to_string(),
                params: vec![("val".to_string(), Type::Any)],
                return_type: Type::String,
                visibility: Visibility::Pub,
                span: Span::default(),
            },
        );
        self.symbols.functions.insert(
            "int".to_string(),
            FunctionSymbol {
                name: "int".to_string(),
                params: vec![("val".to_string(), Type::Any)],
                return_type: Type::Int,
                visibility: Visibility::Pub,
                span: Span::default(),
            },
        );
        self.symbols.functions.insert(
            "float".to_string(),
            FunctionSymbol {
                name: "float".to_string(),
                params: vec![("val".to_string(), Type::Any)],
                return_type: Type::Float,
                visibility: Visibility::Pub,
                span: Span::default(),
            },
        );
        self.symbols.functions.insert(
            "assert".to_string(),
            FunctionSymbol {
                name: "assert".to_string(),
                params: vec![("condition".to_string(), Type::Bool)],
                return_type: Type::Unit,
                visibility: Visibility::Pub,
                span: Span::default(),
            },
        );
        self.symbols.functions.insert(
            "read_file".to_string(),
            FunctionSymbol {
                name: "read_file".to_string(),
                params: vec![("path".to_string(), Type::String)],
                return_type: Type::String,
                visibility: Visibility::Pub,
                span: Span::default(),
            },
        );
        self.symbols.functions.insert(
            "write_file".to_string(),
            FunctionSymbol {
                name: "write_file".to_string(),
                params: vec![("path".to_string(), Type::String), ("content".to_string(), Type::String)],
                return_type: Type::Bool,
                visibility: Visibility::Pub,
                span: Span::default(),
            },
        );
        self.symbols.functions.insert(
            "system_exec".to_string(),
            FunctionSymbol {
                name: "system_exec".to_string(),
                params: vec![("cmd".to_string(), Type::String)],
                return_type: Type::Int,
                visibility: Visibility::Pub,
                span: Span::default(),
            },
        );
        self.symbols.functions.insert(
            "get_env".to_string(),
            FunctionSymbol {
                name: "get_env".to_string(),
                params: vec![("key".to_string(), Type::String), ("default_val".to_string(), Type::String)],
                return_type: Type::String,
                visibility: Visibility::Pub,
                span: Span::default(),
            },
        );
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<Diagnostic>> {
        // Phase 1: Collect symbols (Interfaces, Classes, Functions)
        self.collect_symbols(program);

        // Phase 2: Validate class hierarchies (detect cycles, ensure valid bases)
        self.validate_class_hierarchies();

        // Phase 3: Validate interface contracts and method overrides
        self.validate_oop_contracts();

        // Phase 4: Typecheck bodies
        self.typecheck_bodies(program);

        if self.diagnostics.iter().any(|d| d.level == crate::diagnostics::DiagnosticLevel::Error) {
            Err(self.diagnostics.clone())
        } else {
            Ok(())
        }
    }

    fn collect_symbols(&mut self, program: &Program) {
        for decl in &program.declarations {
            match decl {
                Declaration::Interface(iface) => {
                    let mut methods = HashMap::new();
                    for m in &iface.methods {
                        let params = m.params.iter().map(|p| (p.name.clone(), p.ty.clone().unwrap_or(Type::Any))).collect();
                        methods.insert(
                            m.name.clone(),
                            MethodSymbol {
                                name: m.name.clone(),
                                params,
                                return_type: m.return_type.clone(),
                                visibility: Visibility::Pub,
                                is_override: false,
                                is_static: false,
                                is_abstract: true,
                                span: m.span,
                            },
                        );
                    }
                    self.symbols.interfaces.insert(
                        iface.name.clone(),
                        InterfaceSymbol {
                            name: iface.name.clone(),
                            methods,
                            span: iface.span,
                        },
                    );
                }
                Declaration::Class(cls) => {
                    let mut fields = HashMap::new();
                    let mut methods = HashMap::new();
                    let mut static_fields = HashMap::new();
                    let mut static_methods = HashMap::new();
                    let mut constructor = None;

                    for member in &cls.members {
                        match member {
                            ClassMember::Field(f) => {
                                fields.insert(
                                    f.name.clone(),
                                    FieldSymbol {
                                        name: f.name.clone(),
                                        ty: f.ty.clone().unwrap_or(Type::Any),
                                        visibility: f.visibility,
                                        span: f.span,
                                    },
                                );
                            }
                            ClassMember::StaticField(sf) => {
                                static_fields.insert(
                                    sf.name.clone(),
                                    FieldSymbol {
                                        name: sf.name.clone(),
                                        ty: sf.ty.clone().unwrap_or(Type::Any),
                                        visibility: sf.visibility,
                                        span: sf.span,
                                    },
                                );
                            }
                            ClassMember::Method(m) => {
                                let params: Vec<(String, Type)> = m.params.iter().map(|p| (p.name.clone(), p.ty.clone().unwrap_or(Type::Any))).collect();
                                let m_sym = MethodSymbol {
                                    name: m.name.clone(),
                                    params: params.clone(),
                                    return_type: m.return_type.clone().unwrap_or(Type::Unit),
                                    visibility: m.visibility,
                                    is_override: m.is_override,
                                    is_static: false,
                                    is_abstract: m.is_abstract,
                                    span: m.span,
                                };
                                if m.name == "__init__" {
                                    // Scan __init__ body for `self.field = param` assignments
                                    // and register inferred field types from param annotations.
                                    let param_map: std::collections::HashMap<String, Type> =
                                        params.iter().cloned().collect();
                                    for stmt in &m.body {
                                        if let Stmt::Assign { target, value, .. } = stmt {
                                            if let Expr::FieldAccess { receiver, field, .. } = target {
                                                if let Expr::Ident(ref recv_name, _) = **receiver {
                                                    if recv_name == "self" && !fields.contains_key(field.as_str()) {
                                                        // Infer type from rhs ident (param name) or literal
                                                        let inferred = if let Expr::Ident(ref rhs_name, _) = value {
                                                            param_map.get(rhs_name).cloned().unwrap_or(Type::Any)
                                                        } else {
                                                            Type::Any
                                                        };
                                                        let vis = if field.starts_with('_') {
                                                            Visibility::Private
                                                        } else {
                                                            Visibility::Pub
                                                        };
                                                        fields.insert(
                                                            field.clone(),
                                                            FieldSymbol {
                                                                name: field.clone(),
                                                                ty: inferred,
                                                                visibility: vis,
                                                                span: m.span,
                                                            },
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    constructor = Some(m_sym);
                                } else {
                                    methods.insert(m.name.clone(), m_sym);
                                }
                            }
                            ClassMember::StaticMethod(sm) => {
                                let params = sm.params.iter().map(|p| (p.name.clone(), p.ty.clone().unwrap_or(Type::Any))).collect();
                                static_methods.insert(
                                    sm.name.clone(),
                                    MethodSymbol {
                                        name: sm.name.clone(),
                                        params,
                                        return_type: sm.return_type.clone().unwrap_or(Type::Unit),
                                        visibility: sm.visibility,
                                        is_override: false,
                                        is_static: true,
                                        is_abstract: false,
                                        span: sm.span,
                                    },
                                );
                            }
                            ClassMember::Property(p) => {
                                let params = p.getter.params.iter().map(|param| (param.name.clone(), param.ty.clone().unwrap_or(Type::Any))).collect();
                                methods.insert(
                                    p.name.clone(),
                                    MethodSymbol {
                                        name: p.name.clone(),
                                        params,
                                        return_type: p.getter.return_type.clone().unwrap_or(Type::Any),
                                        visibility: p.getter.visibility,
                                        is_override: false,
                                        is_static: false,
                                        is_abstract: false,
                                        span: p.span,
                                    },
                                );
                            }
                        }
                    }

                    self.symbols.classes.insert(
                        cls.name.clone(),
                        ClassSymbol {
                            name: cls.name.clone(),
                            base_class: cls.base_class.clone(),
                            interfaces: cls.interfaces.clone(),
                            is_abstract: cls.is_abstract,
                            visibility: cls.visibility,
                            fields,
                            methods,
                            static_fields,
                            static_methods,
                            constructor,
                            span: cls.span,
                        },
                    );
                }
                Declaration::Function(f) => {
                    let params = f.params.iter().map(|p| (p.name.clone(), p.ty.clone().unwrap_or(Type::Any))).collect();
                    self.symbols.functions.insert(
                        f.name.clone(),
                        FunctionSymbol {
                            name: f.name.clone(),
                            params,
                            return_type: f.return_type.clone().unwrap_or(Type::Unit),
                            visibility: f.visibility,
                            span: f.span,
                        },
                    );
                }
                Declaration::Import { module, alias, span: _ } => {
                    if let Some(al) = alias {
                        self.symbols.imported_symbols.insert(al.clone());
                    }
                    for part in module {
                        if part != "*" {
                            self.symbols.imported_symbols.insert(part.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn validate_class_hierarchies(&mut self) {
        let class_names: Vec<String> = self.symbols.classes.keys().cloned().collect();

        for name in &class_names {
            let mut visited = HashSet::new();
            let mut curr = name.clone();

            while let Some(cls) = self.symbols.classes.get(&curr) {
                if !visited.insert(curr.clone()) {
                    self.diagnostics.push(Diagnostic::error(
                        "E0100",
                        format!("Cyclic inheritance detected involving class '{}'", curr),
                        cls.span,
                    ));
                    break;
                }

                if let Some(ref base) = cls.base_class {
                    if !self.symbols.classes.contains_key(base) {
                        self.diagnostics.push(Diagnostic::error(
                            "E0107",
                            format!("Class '{}' extends undefined base class '{}'", curr, base),
                            cls.span,
                        ));
                        break;
                    }
                    curr = base.clone();
                } else {
                    break;
                }
            }
        }
    }

    fn validate_oop_contracts(&mut self) {
        let classes: Vec<ClassSymbol> = self.symbols.classes.values().cloned().collect();

        for cls in &classes {
            // Check interfaces implementation
            for iface_name in &cls.interfaces {
                if let Some(iface) = self.symbols.interfaces.get(iface_name).cloned() {
                    for (m_name, required_method) in &iface.methods {
                        // Look for method in class or ancestors
                        if let Some(impl_method) = self.lookup_method_in_hierarchy(&cls.name, m_name) {
                            // Check parameter count
                            // Note: class method has 'self' as first param, while interface might or might not
                            let impl_params_len = if impl_method.params.first().map_or(false, |p| p.0 == "self") {
                                impl_method.params.len() - 1
                            } else {
                                impl_method.params.len()
                            };

                            let req_params_len = if required_method.params.first().map_or(false, |p| p.0 == "self") {
                                required_method.params.len() - 1
                            } else {
                                required_method.params.len()
                            };

                            if impl_params_len != req_params_len {
                                self.diagnostics.push(Diagnostic::error(
                                    "E0103",
                                    format!(
                                        "Method '{}' in class '{}' does not match parameter count for interface '{}'",
                                        m_name, cls.name, iface_name
                                    ),
                                    impl_method.span,
                                ));
                            }
                        } else {
                            self.diagnostics.push(Diagnostic::error(
                                "E0103",
                                format!(
                                    "Class '{}' does not implement required interface method '{}.{}'",
                                    cls.name, iface_name, m_name
                                ),
                                cls.span,
                            ));
                        }
                    }
                } else {
                    self.diagnostics.push(Diagnostic::error(
                        "E0108",
                        format!("Class '{}' implements undefined interface '{}'", cls.name, iface_name),
                        cls.span,
                    ));
                }
            }

            // Check abstract methods implementation if class is concrete
            if !cls.is_abstract {
                if let Some(ref base_name) = cls.base_class {
                    let mut curr = Some(base_name.clone());
                    let mut visited = HashSet::new();
                    visited.insert(cls.name.clone());
                    while let Some(b_name) = curr {
                        if !visited.insert(b_name.clone()) {
                            break;
                        }
                        if let Some(base_cls) = self.symbols.classes.get(&b_name).cloned() {
                            for (m_name, m_sym) in &base_cls.methods {
                                if m_sym.is_abstract {
                                    if !cls.methods.contains_key(m_name) {
                                        self.diagnostics.push(Diagnostic::error(
                                            "E0105",
                                            format!(
                                                "Concrete class '{}' must implement inherited abstract method '{}.{}'",
                                                cls.name, b_name, m_name
                                            ),
                                            cls.span,
                                        ));
                                    }
                                }
                            }
                            curr = base_cls.base_class.clone();
                        } else {
                            break;
                        }
                    }
                }
            }

            // Check overrides
            for (m_name, m_sym) in &cls.methods {
                if m_sym.is_override {
                    let in_base = if let Some(ref base_name) = cls.base_class {
                        self.lookup_method_in_hierarchy(base_name, m_name).is_some()
                    } else {
                        false
                    };
                    let in_iface = cls.interfaces.iter().any(|iface_name| {
                        self.symbols.interfaces.get(iface_name).map_or(false, |iface| iface.methods.contains_key(m_name))
                    });
                    let has_base_method = in_base || in_iface;

                    if !has_base_method {
                        self.diagnostics.push(Diagnostic::error(
                            "E0104",
                            format!(
                                "Method '{}' marked 'override' but no matching method was found in base class or interface",
                                m_name
                            ),
                            m_sym.span,
                        ));
                    }
                }
            }
        }
    }

    fn lookup_method_in_hierarchy(&self, class_name: &str, method_name: &str) -> Option<MethodSymbol> {
        let mut curr = Some(class_name.to_string());
        let mut visited = HashSet::new();
        while let Some(name) = curr {
            if !visited.insert(name.clone()) {
                break;
            }
            if let Some(cls) = self.symbols.classes.get(&name) {
                if let Some(m) = cls.methods.get(method_name) {
                    return Some(m.clone());
                }
                curr = cls.base_class.clone();
            } else {
                break;
            }
        }
        None
    }

    fn lookup_field_in_hierarchy(&self, class_name: &str, field_name: &str) -> Option<FieldSymbol> {
        let mut curr = Some(class_name.to_string());
        let mut visited = HashSet::new();
        while let Some(name) = curr {
            if !visited.insert(name.clone()) {
                break;
            }
            if let Some(cls) = self.symbols.classes.get(&name) {
                if let Some(f) = cls.fields.get(field_name) {
                    return Some(f.clone());
                }
                curr = cls.base_class.clone();
            } else {
                break;
            }
        }
        None
    }

    pub fn is_subtype(&self, sub: &Type, sup: &Type) -> bool {
        if sub == sup || sub == &Type::Any || sup == &Type::Any {
            return true;
        }

        match (sub, sup) {
            (Type::Int, Type::Float) => true, // numeric widening
            (Type::Custom(sub_name, _), Type::Custom(sup_name, _)) => {
                if sub_name == sup_name {
                    return true;
                }
                // Check if sub_name inherits from sup_name
                let mut curr = Some(sub_name.clone());
                let mut visited = HashSet::new();
                while let Some(name) = curr {
                    if !visited.insert(name.clone()) {
                        break;
                    }
                    if let Some(cls) = self.symbols.classes.get(&name) {
                        if cls.interfaces.contains(sup_name) {
                            return true;
                        }
                        if let Some(ref base) = cls.base_class {
                            if base == sup_name {
                                return true;
                            }
                            curr = Some(base.clone());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                false
            }
            (Type::List(sub_elem), Type::List(sup_elem)) => self.is_subtype(sub_elem, sup_elem),
            _ => false,
        }
    }

    fn typecheck_bodies(&mut self, program: &Program) {
        let mut scope_stack = ScopeStack::new();

        for decl in &program.declarations {
            match decl {
                Declaration::Var(v) => {
                    let init_ty = self.infer_expr(&v.init, &scope_stack);
                    let declared_ty = v.ty.clone().unwrap_or(init_ty.clone());
                    if !self.is_subtype(&init_ty, &declared_ty) {
                        self.diagnostics.push(Diagnostic::error(
                            "E0109",
                            format!("Type mismatch in variable '{}': expected {}, found {}", v.name, declared_ty, init_ty),
                            v.span,
                        ));
                    }
                    scope_stack.insert(v.name.clone(), declared_ty, v.is_mut, v.span);
                }
                Declaration::Stmt(stmt) => {
                    self.typecheck_stmt(stmt, &mut scope_stack, &Type::Unit);
                }
                Declaration::Function(f) => {
                    let ret_ty = f.return_type.clone().unwrap_or(Type::Unit);
                    scope_stack.enter_scope();
                    for p in &f.params {
                        scope_stack.insert(p.name.clone(), p.ty.clone().unwrap_or(Type::Any), false, p.span);
                    }
                    for stmt in &f.body {
                        self.typecheck_stmt(stmt, &mut scope_stack, &ret_ty);
                    }
                    scope_stack.exit_scope();
                }
                Declaration::Class(cls) => {
                    self.current_class = Some(cls.name.clone());
                    for member in &cls.members {
                        match member {
                            ClassMember::Method(m) => {
                                let ret_ty = m.return_type.clone().unwrap_or(Type::Unit);
                                scope_stack.enter_scope();
                                scope_stack.insert("self".to_string(), Type::Custom(cls.name.clone(), Vec::new()), false, m.span);
                                for p in &m.params {
                                    if p.name != "self" {
                                        scope_stack.insert(p.name.clone(), p.ty.clone().unwrap_or(Type::Any), false, p.span);
                                    }
                                }
                                for stmt in &m.body {
                                    self.typecheck_stmt(stmt, &mut scope_stack, &ret_ty);
                                }
                                scope_stack.exit_scope();
                            }
                            ClassMember::StaticMethod(sm) => {
                                let ret_ty = sm.return_type.clone().unwrap_or(Type::Unit);
                                scope_stack.enter_scope();
                                for p in &sm.params {
                                    scope_stack.insert(p.name.clone(), p.ty.clone().unwrap_or(Type::Any), false, p.span);
                                }
                                for stmt in &sm.body {
                                    self.typecheck_stmt(stmt, &mut scope_stack, &ret_ty);
                                }
                                scope_stack.exit_scope();
                            }
                            _ => {}
                        }
                    }
                    self.current_class = None;
                }
                _ => {}
            }
        }
    }

    fn typecheck_stmt(&mut self, stmt: &Stmt, scopes: &mut ScopeStack, expected_ret: &Type) {
        match stmt {
            Stmt::Let { name, ty, init, span } => {
                let init_ty = self.infer_expr(init, scopes);
                let final_ty = ty.clone().unwrap_or(init_ty.clone());
                if !self.is_subtype(&init_ty, &final_ty) {
                    self.diagnostics.push(Diagnostic::error(
                        "E0109",
                        format!("Type mismatch in variable '{}': expected {}, found {}", name, final_ty, init_ty),
                        *span,
                    ));
                }
                scopes.insert(name.clone(), final_ty, false, *span);
            }
            Stmt::Mut { name, ty, init, span } => {
                let init_ty = self.infer_expr(init, scopes);
                let final_ty = ty.clone().unwrap_or(init_ty.clone());
                if !self.is_subtype(&init_ty, &final_ty) {
                    self.diagnostics.push(Diagnostic::error(
                        "E0109",
                        format!("Type mismatch in mutable variable '{}': expected {}, found {}", name, final_ty, init_ty),
                        *span,
                    ));
                }
                scopes.insert(name.clone(), final_ty, true, *span);
            }
            Stmt::Assign { target, value, span } => {
                let val_ty = self.infer_expr(value, scopes);
                if let Expr::Ident(ref name, _) = target {
                    if let Some(var) = scopes.lookup(name) {
                        if !var.is_mut {
                            self.diagnostics.push(Diagnostic::error(
                                "E0106",
                                format!("Cannot assign to immutable variable '{}'. Declare it with 'mut' to allow mutation.", name),
                                *span,
                            ));
                        }
                        if !self.is_subtype(&val_ty, &var.ty) {
                            self.diagnostics.push(Diagnostic::error(
                                "E0109",
                                format!("Cannot assign value of type {} to variable '{}' of type {}", val_ty, name, var.ty),
                                *span,
                            ));
                        }
                    } else {
                        // Inferred local variable binding
                        scopes.insert(name.clone(), val_ty, true, *span);
                    }
                } else if let Expr::FieldAccess { receiver, field, span: field_span } = target {
                    let recv_ty = self.infer_expr(receiver, scopes);
                    if let Type::Custom(ref cls_name, _) = recv_ty {
                        self.check_member_access(cls_name, field, *field_span);
                    }
                }
            }
            Stmt::AugAssign { target, value, .. } => {
                let _ = self.infer_expr(target, scopes);
                let _ = self.infer_expr(value, scopes);
            }
            Stmt::If { condition, then_branch, elif_branches, else_branch, span } => {
                let cond_ty = self.infer_expr(condition, scopes);
                if !self.is_subtype(&cond_ty, &Type::Bool) {
                    self.diagnostics.push(Diagnostic::error(
                        "E0110",
                        format!("If condition must evaluate to Bool, found {}", cond_ty),
                        *span,
                    ));
                }
                scopes.enter_scope();
                for s in then_branch {
                    self.typecheck_stmt(s, scopes, expected_ret);
                }
                scopes.exit_scope();

                for (elif_cond, elif_body) in elif_branches {
                    let elif_ty = self.infer_expr(elif_cond, scopes);
                    if !self.is_subtype(&elif_ty, &Type::Bool) {
                        self.diagnostics.push(Diagnostic::error(
                            "E0110",
                            format!("Elif condition must evaluate to Bool, found {}", elif_ty),
                            elif_cond.span(),
                        ));
                    }
                    scopes.enter_scope();
                    for s in elif_body {
                        self.typecheck_stmt(s, scopes, expected_ret);
                    }
                    scopes.exit_scope();
                }

                if let Some(ref else_body) = else_branch {
                    scopes.enter_scope();
                    for s in else_body {
                        self.typecheck_stmt(s, scopes, expected_ret);
                    }
                    scopes.exit_scope();
                }
            }
            Stmt::While { condition, body, span } => {
                let cond_ty = self.infer_expr(condition, scopes);
                if !self.is_subtype(&cond_ty, &Type::Bool) {
                    self.diagnostics.push(Diagnostic::error(
                        "E0110",
                        format!("While condition must evaluate to Bool, found {}", cond_ty),
                        *span,
                    ));
                }
                scopes.enter_scope();
                for s in body {
                    self.typecheck_stmt(s, scopes, expected_ret);
                }
                scopes.exit_scope();
            }
            Stmt::For { var, iterable, body, span: _ } => {
                let iter_ty = self.infer_expr(iterable, scopes);
                let elem_ty = match iter_ty {
                    Type::List(elem) => *elem,
                    _ => Type::Int,
                };
                scopes.enter_scope();
                scopes.insert(var.clone(), elem_ty, false, iterable.span());
                for s in body {
                    self.typecheck_stmt(s, scopes, expected_ret);
                }
                scopes.exit_scope();
            }
            Stmt::Return { value, span } => {
                let val_ty = if let Some(ref v) = value {
                    self.infer_expr(v, scopes)
                } else {
                    Type::Unit
                };
                if !self.is_subtype(&val_ty, expected_ret) {
                    self.diagnostics.push(Diagnostic::error(
                        "E0111",
                        format!("Return type mismatch: expected {}, found {}", expected_ret, val_ty),
                        *span,
                    ));
                }
            }
            Stmt::Expr(expr) => {
                let _ = self.infer_expr(expr, scopes);
            }
            Stmt::Break(_) | Stmt::Continue(_) => {}
        }
    }

    fn check_member_access(&mut self, class_name: &str, member_name: &str, span: Span) {
        if let Some(field) = self.lookup_field_in_hierarchy(class_name, member_name) {
            if field.visibility == Visibility::Private {
                let is_inside_declaring_class = self.current_class.as_deref() == Some(class_name);
                if !is_inside_declaring_class {
                    self.diagnostics.push(Diagnostic::error(
                        "E0102",
                        format!("Cannot access private field '{}' of class '{}' from outside", member_name, class_name),
                        span,
                    ));
                }
            }
        }
    }

    pub fn infer_expr(&mut self, expr: &Expr, scopes: &ScopeStack) -> Type {
        match expr {
            Expr::Literal(lit, _) => match lit {
                Literal::Integer(_) => Type::Int,
                Literal::Float(_) => Type::Float,
                Literal::String(_) => Type::String,
                Literal::Bool(_) => Type::Bool,
                Literal::None => Type::None,
            },
            Expr::Ident(name, span) => {
                if let Some(var) = scopes.lookup(name) {
                    var.ty.clone()
                } else if self.symbols.classes.contains_key(name) {
                    Type::Custom(name.clone(), Vec::new())
                } else if self.symbols.imported_symbols.contains(name) {
                    Type::Any
                } else {
                    self.diagnostics.push(Diagnostic::error(
                        "E0112",
                        format!("Undefined variable or identifier '{}'", name),
                        *span,
                    ));
                    Type::Any
                }
            }
            Expr::Binary { op, left, right, span } => {
                let l_ty = self.infer_expr(left, scopes);
                let r_ty = self.infer_expr(right, scopes);
                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        if l_ty == Type::Float || r_ty == Type::Float {
                            Type::Float
                        } else if *op == BinaryOp::Add && (l_ty == Type::String || r_ty == Type::String) {
                            // String + anything (including Any from unresolved exprs) = String
                            Type::String
                        } else {
                            Type::Int
                        }
                    }
                    BinaryOp::Eq | BinaryOp::NotEq | BinaryOp::Lt | BinaryOp::LtEq | BinaryOp::Gt | BinaryOp::GtEq => {
                        Type::Bool
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        // Allow Any (unresolved imported types) to flow through without errors.
                        // Only emit E0113 when both sides are concretely non-boolean.
                        let l_bad = l_ty != Type::Bool && l_ty != Type::Any && l_ty != Type::None;
                        let r_bad = r_ty != Type::Bool && r_ty != Type::Any && r_ty != Type::None;
                        if l_bad || r_bad {
                            self.diagnostics.push(Diagnostic::error(
                                "E0113",
                                "Logical operators 'and' and 'or' require boolean operands",
                                *span,
                            ));
                        }
                        Type::Bool
                    }
                }
            }
            Expr::Unary { op, operand, .. } => {
                let o_ty = self.infer_expr(operand, scopes);
                match op {
                    UnaryOp::Neg => o_ty,
                    UnaryOp::Not => Type::Bool,
                }
            }
            Expr::Call { callee, args, span } => {
                if let Expr::Ident(ref name, _) = **callee {
                    // Check if it's class instantiation
                    if let Some(cls) = self.symbols.classes.get(name).cloned() {
                        if cls.is_abstract {
                            self.diagnostics.push(Diagnostic::error(
                                "E0101",
                                format!("Cannot instantiate abstract class '{}'", name),
                                *span,
                            ));
                        }
                        return Type::Custom(name.clone(), Vec::new());
                    }

                    // Check if function
                    if let Some(f) = self.symbols.functions.get(name).cloned() {
                        return f.return_type.clone();
                    }
                }

                let _ = self.infer_expr(callee, scopes);
                for arg in args {
                    let _ = self.infer_expr(arg, scopes);
                }
                Type::Any
            }
            Expr::MethodCall { receiver, method, args, span } => {
                let recv_ty = self.infer_expr(receiver, scopes);
                for arg in args {
                    let _ = self.infer_expr(arg, scopes);
                }

                if let Type::Custom(ref cls_name, _) = recv_ty {
                    // Check if method exists in class or base class
                    if let Some(m) = self.lookup_method_in_hierarchy(cls_name, method) {
                        if m.visibility == Visibility::Private {
                            let is_inside = self.current_class.as_deref() == Some(cls_name);
                            if !is_inside {
                                self.diagnostics.push(Diagnostic::error(
                                    "E0102",
                                    format!("Cannot call private method '{}' of class '{}' from outside", method, cls_name),
                                    *span,
                                ));
                            }
                        }
                        return m.return_type;
                    }
                    // Check if interface
                    if let Some(iface) = self.symbols.interfaces.get(cls_name) {
                        if let Some(m) = iface.methods.get(method) {
                            return m.return_type.clone();
                        }
                    }
                } else if recv_ty == Type::String {
                    if method == "to_string" || method == "trim" || method == "upper" || method == "lower" {
                        return Type::String;
                    }
                    if method == "len" {
                        return Type::Int;
                    }
                } else if recv_ty == Type::Int || recv_ty == Type::Float {
                    if method == "to_string" {
                        return Type::String;
                    }
                }
                Type::Any
            }
            Expr::FieldAccess { receiver, field, span } => {
                let recv_ty = self.infer_expr(receiver, scopes);
                if let Type::Custom(ref cls_name, _) = recv_ty {
                    self.check_member_access(cls_name, field, *span);
                    if let Some(f) = self.lookup_field_in_hierarchy(cls_name, field) {
                        return f.ty;
                    }
                    if let Some(cls) = self.symbols.classes.get(cls_name) {
                        if let Some(m) = cls.methods.get(field) {
                            // Property getter
                            return m.return_type.clone();
                        }
                    }
                }
                Type::Any
            }
            Expr::SuperInitCall { args, span: _ } => {
                for arg in args {
                    let _ = self.infer_expr(arg, scopes);
                }
                Type::Unit
            }
            Expr::SuperMethodCall { method: _, args, span: _ } => {
                for arg in args {
                    let _ = self.infer_expr(arg, scopes);
                }
                Type::Any
            }
            Expr::Index { receiver, index, span: _ } => {
                let r_ty = self.infer_expr(receiver, scopes);
                let _ = self.infer_expr(index, scopes);
                match r_ty {
                    Type::List(elem) => *elem,
                    Type::Map(_, val) => *val,
                    Type::String => Type::String,
                    _ => Type::Any,
                }
            }
            Expr::List(items, _) => {
                if let Some(first) = items.first() {
                    let elem_ty = self.infer_expr(first, scopes);
                    for item in &items[1..] {
                        let _ = self.infer_expr(item, scopes);
                    }
                    Type::List(Box::new(elem_ty))
                } else {
                    Type::List(Box::new(Type::Any))
                }
            }
            Expr::Map(entries, _) => {
                if let Some((first_k, first_v)) = entries.first() {
                    let k_ty = self.infer_expr(first_k, scopes);
                    let v_ty = self.infer_expr(first_v, scopes);
                    Type::Map(Box::new(k_ty), Box::new(v_ty))
                } else {
                    Type::Map(Box::new(Type::Any), Box::new(Type::Any))
                }
            }
            Expr::Await { value, .. } => {
                let inner_ty = self.infer_expr(value, scopes);
                match inner_ty {
                    Type::Custom(ref name, ref args) if (name == "Task" || name == "Future") && !args.is_empty() => {
                        args[0].clone()
                    }
                    _ => inner_ty,
                }
            }
        }
    }
}
