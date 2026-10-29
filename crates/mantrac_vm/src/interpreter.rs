use crate::value::{ObjectInstance, Value};
use mantrac_ast::*;
use mantrac_lexer::span::Span;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub span: Span,
}

#[derive(Debug)]
enum ControlFlow {
    None,
    Return(Value),
    Break,
    Continue,
}

pub struct Interpreter {
    pub globals: HashMap<String, Value>,
    pub static_fields: HashMap<String, HashMap<String, Value>>,
    pub classes: HashMap<String, ClassDecl>,
    pub functions: HashMap<String, FunctionDecl>,
    pub current_self: Option<Rc<RefCell<ObjectInstance>>>,
    pub current_class_context: Option<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interp = Interpreter {
            globals: HashMap::new(),
            static_fields: HashMap::new(),
            classes: HashMap::new(),
            functions: HashMap::new(),
            current_self: None,
            current_class_context: None,
        };
        interp.register_builtins();
        interp
    }

    fn register_builtins(&mut self) {
        let builtins = vec!["print", "len", "range", "str", "int", "float", "assert", "read_file", "get_env"];
        for b in builtins {
            self.globals.insert(b.to_string(), Value::BuiltinFunction(b.to_string()));
        }
    }

    pub fn execute_program(&mut self, program: &Program) -> Result<Value, RuntimeError> {
        // Collect class declarations and function declarations
        for decl in &program.declarations {
            match decl {
                Declaration::Class(cls) => {
                    self.classes.insert(cls.name.clone(), cls.clone());
                    let mut s_fields = HashMap::new();
                    for member in &cls.members {
                        if let ClassMember::StaticField(sf) = member {
                            let mut empty_env = HashMap::new();
                            let init_val = self.eval_expr(&sf.default, &mut empty_env)?;
                            s_fields.insert(sf.name.clone(), init_val);
                        }
                    }
                    self.static_fields.insert(cls.name.clone(), s_fields);
                }
                Declaration::Function(func) => {
                    self.functions.insert(func.name.clone(), func.clone());
                }
                _ => {}
            }
        }

        // Execute top-level declarations and statements
        let mut env = HashMap::new();
        for decl in &program.declarations {
            match decl {
                Declaration::Var(v) => {
                    let val = self.eval_expr(&v.init, &mut env)?;
                    env.insert(v.name.clone(), val.clone());
                    self.globals.insert(v.name.clone(), val);
                }
                Declaration::Stmt(stmt) => {
                    let flow = self.execute_stmt(stmt, &mut env)?;
                    if let ControlFlow::Return(val) = flow {
                        return Ok(val);
                    }
                }
                _ => {}
            }
        }

        // If there is a `main()` function, invoke it!
        if let Some(main_func) = self.functions.get("main").cloned() {
            let mut main_env = HashMap::new();
            return self.call_function(&main_func, vec![], &mut main_env);
        }

        Ok(Value::None)
    }

    fn execute_block(&mut self, statements: &[Stmt], env: &mut HashMap<String, Value>) -> Result<ControlFlow, RuntimeError> {
        for stmt in statements {
            let flow = self.execute_stmt(stmt, env)?;
            match flow {
                ControlFlow::None => {}
                ret @ ControlFlow::Return(_) => return Ok(ret),
                brk @ ControlFlow::Break => return Ok(brk),
                cnt @ ControlFlow::Continue => return Ok(cnt),
            }
        }
        Ok(ControlFlow::None)
    }

    fn execute_stmt(&mut self, stmt: &Stmt, env: &mut HashMap<String, Value>) -> Result<ControlFlow, RuntimeError> {
        match stmt {
            Stmt::Let { name, init, .. } | Stmt::Mut { name, init, .. } => {
                let val = self.eval_expr(init, env)?;
                env.insert(name.clone(), val.clone());
                self.globals.insert(name.clone(), val);
                Ok(ControlFlow::None)
            }
            Stmt::Assign { target, value, span } => {
                let val = self.eval_expr(value, env)?;
                match target {
                    Expr::Ident(name, _) => {
                        if env.contains_key(name) {
                            env.insert(name.clone(), val.clone());
                        }
                        self.globals.insert(name.clone(), val);
                    }
                    Expr::FieldAccess { receiver, field, .. } => {
                        let recv_val = self.eval_expr(receiver, env)?;
                        if let Value::Object(obj) = recv_val {
                            obj.borrow_mut().fields.insert(field.clone(), val);
                        } else {
                            return Err(RuntimeError {
                                message: format!("Cannot set field '{}' on non-object value", field),
                                span: *span,
                            });
                        }
                    }
                    Expr::Index { receiver, index, .. } => {
                        let recv_val = self.eval_expr(receiver, env)?;
                        let idx_val = self.eval_expr(index, env)?;
                        match (recv_val, idx_val) {
                            (Value::List(list), Value::Int(i)) => {
                                let mut items = list.borrow_mut();
                                let u_idx = if i < 0 {
                                    items.len() as i64 + i
                                } else {
                                    i
                                } as usize;
                                if u_idx < items.len() {
                                    items[u_idx] = val;
                                } else {
                                    return Err(RuntimeError {
                                        message: "Index out of bounds".to_string(),
                                        span: *span,
                                    });
                                }
                            }
                            (Value::Map(map), Value::String(k)) => {
                                map.borrow_mut().insert(k, val);
                            }
                            _ => {
                                return Err(RuntimeError {
                                    message: "Invalid indexing assignment target".to_string(),
                                    span: *span,
                                });
                            }
                        }
                    }
                    _ => {
                        return Err(RuntimeError {
                            message: "Invalid assignment target".to_string(),
                            span: *span,
                        });
                    }
                }
                Ok(ControlFlow::None)
            }
            Stmt::AugAssign { target, op, value, span } => {
                let cur_val = self.eval_expr(target, env)?;
                let rhs_val = self.eval_expr(value, env)?;
                let new_val = self.eval_binary_op(*op, cur_val, rhs_val, *span)?;
                let assign = Stmt::Assign {
                    target: target.clone(),
                    value: Expr::Literal(
                        match new_val {
                            Value::Int(i) => Literal::Integer(i),
                            Value::Float(f) => Literal::Float(f),
                            Value::String(s) => Literal::String(s),
                            Value::Bool(b) => Literal::Bool(b),
                            _ => Literal::None,
                        },
                        *span,
                    ),
                    span: *span,
                };
                self.execute_stmt(&assign, env)
            }
            Stmt::If { condition, then_branch, elif_branches, else_branch, .. } => {
                let cond_val = self.eval_expr(condition, env)?;
                if cond_val.is_truthy() {
                    return self.execute_block(then_branch, env);
                }

                for (elif_cond, elif_body) in elif_branches {
                    let el_val = self.eval_expr(elif_cond, env)?;
                    if el_val.is_truthy() {
                        return self.execute_block(elif_body, env);
                    }
                }

                if let Some(else_body) = else_branch {
                    return self.execute_block(else_body, env);
                }

                Ok(ControlFlow::None)
            }
            Stmt::While { condition, body, .. } => {
                loop {
                    let cond_val = self.eval_expr(condition, env)?;
                    if !cond_val.is_truthy() {
                        break;
                    }
                    let flow = self.execute_block(body, env);
                    match flow? {
                        ControlFlow::Break => break,
                        ControlFlow::Continue => continue,
                        ret @ ControlFlow::Return(_) => return Ok(ret),
                        ControlFlow::None => {}
                    }
                }
                Ok(ControlFlow::None)
            }
            Stmt::For { var, iterable, body, span } => {
                let iter_val = self.eval_expr(iterable, env)?;
                let items: Vec<Value> = match iter_val {
                    Value::List(l) => l.borrow().clone(),
                    Value::String(s) => s.chars().map(|c| Value::String(c.to_string())).collect(),
                    _ => {
                        return Err(RuntimeError {
                            message: "Value is not iterable".to_string(),
                            span: *span,
                        });
                    }
                };

                for item in items {
                    env.insert(var.clone(), item);
                    let flow = self.execute_block(body, env);
                    match flow? {
                        ControlFlow::Break => break,
                        ControlFlow::Continue => continue,
                        ret @ ControlFlow::Return(_) => return Ok(ret),
                        ControlFlow::None => {}
                    }
                }
                Ok(ControlFlow::None)
            }
            Stmt::Return { value, .. } => {
                let ret_val = if let Some(v) = value {
                    self.eval_expr(v, env)?
                } else {
                    Value::Unit
                };
                Ok(ControlFlow::Return(ret_val))
            }
            Stmt::Break(_) => Ok(ControlFlow::Break),
            Stmt::Continue(_) => Ok(ControlFlow::Continue),
            Stmt::Expr(e) => {
                self.eval_expr(e, env)?;
                Ok(ControlFlow::None)
            }
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr, env: &mut HashMap<String, Value>) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(lit, _) => match lit {
                Literal::Integer(i) => Ok(Value::Int(*i)),
                Literal::Float(f) => Ok(Value::Float(*f)),
                Literal::String(s) => Ok(Value::String(s.clone())),
                Literal::Bool(b) => Ok(Value::Bool(*b)),
                Literal::None => Ok(Value::None),
            },
            Expr::Ident(name, span) => {
                if name == "self" {
                    if let Some(ref s) = self.current_self {
                        return Ok(Value::Object(Rc::clone(s)));
                    }
                }
                if let Some(val) = env.get(name) {
                    return Ok(val.clone());
                }
                if let Some(val) = self.globals.get(name) {
                    return Ok(val.clone());
                }
                if self.classes.contains_key(name) {
                    // Class name as constructor reference
                    return Ok(Value::String(format!("<class {}>", name)));
                }
                if self.functions.contains_key(name) {
                    return Ok(Value::String(format!("<function {}>", name)));
                }
                Err(RuntimeError {
                    message: format!("Variable '{}' not found", name),
                    span: *span,
                })
            }
            Expr::Binary { op, left, right, span } => {
                let l_val = self.eval_expr(left, env)?;
                let r_val = self.eval_expr(right, env)?;
                self.eval_binary_op(*op, l_val, r_val, *span)
            }
            Expr::Unary { op, operand, span } => {
                let val = self.eval_expr(operand, env)?;
                match op {
                    UnaryOp::Neg => match val {
                        Value::Int(i) => Ok(Value::Int(-i)),
                        Value::Float(f) => Ok(Value::Float(-f)),
                        _ => Err(RuntimeError {
                            message: "Cannot negate non-numeric value".to_string(),
                            span: *span,
                        }),
                    },
                    UnaryOp::Not => Ok(Value::Bool(!val.is_truthy())),
                }
            }
            Expr::Call { callee, args, span } => {
                // Check if calling a Class constructor
                if let Expr::Ident(ref name, _) = **callee {
                    if let Some(cls) = self.classes.get(name).cloned() {
                        let mut evaluated_args = Vec::new();
                        for arg in args {
                            evaluated_args.push(self.eval_expr(arg, env)?);
                        }
                        return self.instantiate_class(&cls, evaluated_args);
                    }
                    if let Some(f) = self.functions.get(name).cloned() {
                        let mut evaluated_args = Vec::new();
                        for arg in args {
                            evaluated_args.push(self.eval_expr(arg, env)?);
                        }
                        let mut fn_env = HashMap::new();
                        return self.call_function(&f, evaluated_args, &mut fn_env);
                    }
                }

                // Check static method call or regular function call
                if let Expr::FieldAccess { receiver, field, .. } = &**callee {
                    if let Expr::Ident(ref cls_name, _) = **receiver {
                        if let Some(cls) = self.classes.get(cls_name).cloned() {
                            for member in &cls.members {
                                if let ClassMember::StaticMethod(sm) = member {
                                    if &sm.name == field {
                                        let mut evaluated_args = Vec::new();
                                        for arg in args {
                                            evaluated_args.push(self.eval_expr(arg, env)?);
                                        }
                                        let mut sm_env = HashMap::new();
                                        return self.call_function(sm, evaluated_args, &mut sm_env);
                                    }
                                }
                            }
                        }
                    }
                }

                let callee_val = self.eval_expr(callee, env)?;
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.eval_expr(arg, env)?);
                }

                match callee_val {
                    Value::BuiltinFunction(name) => self.call_builtin(&name, evaluated_args, *span),
                    _ => {
                        // Look for function by name
                        if let Expr::Ident(ref fn_name, _) = **callee {
                            if let Some(f) = self.functions.get(fn_name).cloned() {
                                let mut fn_env = HashMap::new();
                                return self.call_function(&f, evaluated_args, &mut fn_env);
                            }
                        }
                        Err(RuntimeError {
                            message: "Callee is not a function".to_string(),
                            span: *span,
                        })
                    }
                }
            }
            Expr::MethodCall { receiver, method, args, span } => {
                // Check if receiver is a class name for static method call
                if let Expr::Ident(ref cls_name, _) = **receiver {
                    if let Some(cls) = self.classes.get(cls_name).cloned() {
                        for member in &cls.members {
                            if let ClassMember::StaticMethod(sm) = member {
                                if &sm.name == method {
                                    let mut evaluated_args = Vec::new();
                                    for arg in args {
                                        evaluated_args.push(self.eval_expr(arg, env)?);
                                    }
                                    let mut sm_env = HashMap::new();
                                    return self.call_function(sm, evaluated_args, &mut sm_env);
                                }
                            }
                        }
                    }
                }

                let recv_val = self.eval_expr(receiver, env)?;
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.eval_expr(arg, env)?);
                }

                match recv_val {
                    Value::Object(obj) => {
                        let class_name = obj.borrow().class_name.clone();
                        self.call_method(obj, &class_name, method, evaluated_args, *span)
                    }
                    Value::String(s) => match method.as_str() {
                        "to_string" => Ok(Value::String(s)),
                        "len" => Ok(Value::Int(s.len() as i64)),
                        "trim" => Ok(Value::String(s.trim().to_string())),
                        "upper" => Ok(Value::String(s.to_uppercase())),
                        "lower" => Ok(Value::String(s.to_lowercase())),
                        _ => Err(RuntimeError {
                            message: format!("Unknown method '{}' on String", method),
                            span: *span,
                        }),
                    },
                    Value::Int(i) => match method.as_str() {
                        "to_string" => Ok(Value::String(i.to_string())),
                        _ => Err(RuntimeError {
                            message: format!("Unknown method '{}' on Int", method),
                            span: *span,
                        }),
                    },
                    Value::Float(f) => match method.as_str() {
                        "to_string" => Ok(Value::String(f.to_string())),
                        _ => Err(RuntimeError {
                            message: format!("Unknown method '{}' on Float", method),
                            span: *span,
                        }),
                    },
                    Value::List(list) => match method.as_str() {
                        "len" => Ok(Value::Int(list.borrow().len() as i64)),
                        "append" => {
                            if let Some(item) = evaluated_args.into_iter().next() {
                                list.borrow_mut().push(item);
                            }
                            Ok(Value::Unit)
                        }
                        "pop" => {
                            let popped = list.borrow_mut().pop().unwrap_or(Value::None);
                            Ok(popped)
                        }
                        _ => Err(RuntimeError {
                            message: format!("Unknown method '{}' on List", method),
                            span: *span,
                        }),
                    },
                    _ => Err(RuntimeError {
                        message: format!("Cannot call method '{}' on value {}", method, recv_val),
                        span: *span,
                    }),
                }
            }
            Expr::FieldAccess { receiver, field, span } => {
                // Static member access e.g. MathUtils.PI
                if let Expr::Ident(ref cls_name, _) = **receiver {
                    if let Some(s_fields) = self.static_fields.get(cls_name) {
                        if let Some(val) = s_fields.get(field) {
                            return Ok(val.clone());
                        }
                    }
                }

                let recv_val = self.eval_expr(receiver, env)?;
                match recv_val {
                    Value::Object(obj) => {
                        // Check instance fields
                        if let Some(val) = obj.borrow().fields.get(field).cloned() {
                            return Ok(val);
                        }
                        // Check properties
                        let cls_name = obj.borrow().class_name.clone();
                        if let Some(cls) = self.classes.get(&cls_name).cloned() {
                            for member in &cls.members {
                                if let ClassMember::Property(prop) = member {
                                    if &prop.name == field {
                                        return self.call_method(
                                            obj.clone(),
                                            &cls_name,
                                            &prop.getter.name,
                                            vec![],
                                            *span,
                                        );
                                    }
                                }
                            }
                        }
                        Ok(Value::None)
                    }
                    _ => Err(RuntimeError {
                        message: format!("Cannot access field '{}' on non-object", field),
                        span: *span,
                    }),
                }
            }
            Expr::SuperInitCall { args, span } => {
                let obj = self.current_self.clone().ok_or_else(|| RuntimeError {
                    message: "super() used outside of class instance context".to_string(),
                    span: *span,
                })?;
                let curr_cls_name = self.current_class_context.clone().ok_or_else(|| RuntimeError {
                    message: "super() used outside of class context".to_string(),
                    span: *span,
                })?;

                let curr_cls = self.classes.get(&curr_cls_name).cloned().ok_or_else(|| RuntimeError {
                    message: format!("Class '{}' not found", curr_cls_name),
                    span: *span,
                })?;

                let base_name = curr_cls.base_class.ok_or_else(|| RuntimeError {
                    message: format!("Class '{}' has no base class to call super() on", curr_cls_name),
                    span: *span,
                })?;

                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.eval_expr(arg, env)?);
                }

                self.call_method(obj, &base_name, "__init__", evaluated_args, *span)
            }
            Expr::SuperMethodCall { method, args, span } => {
                let obj = self.current_self.clone().ok_or_else(|| RuntimeError {
                    message: "super() used outside of class instance context".to_string(),
                    span: *span,
                })?;
                let curr_cls_name = self.current_class_context.clone().ok_or_else(|| RuntimeError {
                    message: "super() used outside of class context".to_string(),
                    span: *span,
                })?;

                let curr_cls = self.classes.get(&curr_cls_name).cloned().ok_or_else(|| RuntimeError {
                    message: format!("Class '{}' not found", curr_cls_name),
                    span: *span,
                })?;

                let base_name = curr_cls.base_class.ok_or_else(|| RuntimeError {
                    message: format!("Class '{}' has no base class", curr_cls_name),
                    span: *span,
                })?;

                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.eval_expr(arg, env)?);
                }

                self.call_method(obj, &base_name, method, evaluated_args, *span)
            }
            Expr::Index { receiver, index, span } => {
                let recv_val = self.eval_expr(receiver, env)?;
                let idx_val = self.eval_expr(index, env)?;
                match (recv_val, idx_val) {
                    (Value::List(list), Value::Int(i)) => {
                        let items = list.borrow();
                        let u_idx = if i < 0 {
                            items.len() as i64 + i
                        } else {
                            i
                        } as usize;
                        if u_idx < items.len() {
                            Ok(items[u_idx].clone())
                        } else {
                            Err(RuntimeError {
                                message: "List index out of range".to_string(),
                                span: *span,
                            })
                        }
                    }
                    (Value::String(s), Value::Int(i)) => {
                        let chars: Vec<char> = s.chars().collect();
                        let u_idx = if i < 0 {
                            chars.len() as i64 + i
                        } else {
                            i
                        } as usize;
                        if u_idx < chars.len() {
                            Ok(Value::String(chars[u_idx].to_string()))
                        } else {
                            Err(RuntimeError {
                                message: "String index out of range".to_string(),
                                span: *span,
                            })
                        }
                    }
                    (Value::Map(map), Value::String(k)) => {
                        Ok(map.borrow().get(&k).cloned().unwrap_or(Value::None))
                    }
                    _ => Err(RuntimeError {
                        message: "Invalid indexing operation".to_string(),
                        span: *span,
                    }),
                }
            }
            Expr::List(items, _) => {
                let mut evaluated = Vec::new();
                for item in items {
                    evaluated.push(self.eval_expr(item, env)?);
                }
                Ok(Value::List(Rc::new(RefCell::new(evaluated))))
            }
            Expr::Map(entries, _) => {
                let mut map = HashMap::new();
                for (k, v) in entries {
                    let k_val = self.eval_expr(k, env)?;
                    let v_val = self.eval_expr(v, env)?;
                    map.insert(k_val.to_string_repr(), v_val);
                }
                Ok(Value::Map(Rc::new(RefCell::new(map))))
            }
            Expr::Await { value, .. } => {
                self.eval_expr(value, env)
            }
        }
    }

    fn instantiate_class(&mut self, cls: &ClassDecl, args: Vec<Value>) -> Result<Value, RuntimeError> {
        let instance = Rc::new(RefCell::new(ObjectInstance::new(cls.name.clone())));

        // Initialize default fields from class and base classes
        let mut curr_class = Some(cls.clone());
        while let Some(c) = curr_class {
            for member in &c.members {
                if let ClassMember::Field(f) = member {
                    if let Some(ref def) = f.default {
                        let mut empty_env = HashMap::new();
                        let val = self.eval_expr(def, &mut empty_env)?;
                        instance.borrow_mut().fields.insert(f.name.clone(), val);
                    }
                }
            }
            curr_class = c.base_class.and_then(|base_name| self.classes.get(&base_name).cloned());
        }

        // Call __init__ if defined
        let prev_self = self.current_self.take();
        let prev_ctx = self.current_class_context.take();

        self.current_self = Some(Rc::clone(&instance));
        self.current_class_context = Some(cls.name.clone());

        let res = self.call_method(instance.clone(), &cls.name, "__init__", args, cls.span);

        self.current_self = prev_self;
        self.current_class_context = prev_ctx;

        match res {
            Ok(_) | Err(RuntimeError { .. }) => {}
        }

        Ok(Value::Object(instance))
    }

    fn call_method(
        &mut self,
        instance: Rc<RefCell<ObjectInstance>>,
        class_name: &str,
        method_name: &str,
        args: Vec<Value>,
        span: Span,
    ) -> Result<Value, RuntimeError> {
        // Find method in class or ancestors
        let mut curr_name = Some(class_name.to_string());
        let mut target_method: Option<(FunctionDecl, String)> = None;

        while let Some(c_name) = curr_name {
            if let Some(cls) = self.classes.get(&c_name).cloned() {
                for member in &cls.members {
                    match member {
                        ClassMember::Method(m) if m.name == method_name => {
                            target_method = Some((m.clone(), c_name.clone()));
                            break;
                        }
                        ClassMember::Property(p) if p.name == method_name => {
                            target_method = Some((p.getter.clone(), c_name.clone()));
                            break;
                        }
                        _ => {}
                    }
                }
                if target_method.is_some() {
                    break;
                }
                curr_name = cls.base_class.clone();
            } else {
                break;
            }
        }

        if let Some((method, found_in_class)) = target_method {
            let prev_self = self.current_self.take();
            let prev_ctx = self.current_class_context.take();

            self.current_self = Some(Rc::clone(&instance));
            self.current_class_context = Some(found_in_class);

            let mut method_env = HashMap::new();
            method_env.insert("self".to_string(), Value::Object(instance));

            let non_self_params: Vec<&Param> = method.params.iter().filter(|p| p.name != "self").collect();
            for (i, param) in non_self_params.iter().enumerate() {
                if i < args.len() {
                    method_env.insert(param.name.clone(), args[i].clone());
                } else if let Some(ref def) = param.default_val {
                    let mut def_env = HashMap::new();
                    let def_val = self.eval_expr(def, &mut def_env)?;
                    method_env.insert(param.name.clone(), def_val);
                } else {
                    method_env.insert(param.name.clone(), Value::None);
                }
            }

            let result = match self.execute_block(&method.body, &mut method_env)? {
                ControlFlow::Return(v) => Ok(v),
                _ => Ok(Value::Unit),
            };

            self.current_self = prev_self;
            self.current_class_context = prev_ctx;

            result
        } else if method_name == "__init__" {
            // Default no-op constructor if not defined
            Ok(Value::Unit)
        } else {
            Err(RuntimeError {
                message: format!("Method '{}' not found on class '{}'", method_name, class_name),
                span,
            })
        }
    }

    fn call_function(
        &mut self,
        func: &FunctionDecl,
        args: Vec<Value>,
        env: &mut HashMap<String, Value>,
    ) -> Result<Value, RuntimeError> {
        for (i, param) in func.params.iter().enumerate() {
            if i < args.len() {
                env.insert(param.name.clone(), args[i].clone());
            } else if let Some(ref def) = param.default_val {
                let mut def_env = HashMap::new();
                let def_val = self.eval_expr(def, &mut def_env)?;
                env.insert(param.name.clone(), def_val);
            } else {
                env.insert(param.name.clone(), Value::None);
            }
        }

        match self.execute_block(&func.body, env)? {
            ControlFlow::Return(v) => Ok(v),
            _ => Ok(Value::Unit),
        }
    }

    fn call_builtin(&mut self, name: &str, args: Vec<Value>, span: Span) -> Result<Value, RuntimeError> {
        match name {
            "print" => {
                let out: Vec<String> = args.iter().map(|a| a.to_string_repr()).collect();
                println!("{}", out.join(" "));
                Ok(Value::Unit)
            }
            "len" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Value::List(l) => Ok(Value::Int(l.borrow().len() as i64)),
                        Value::String(s) => Ok(Value::Int(s.len() as i64)),
                        Value::Map(m) => Ok(Value::Int(m.borrow().len() as i64)),
                        _ => Err(RuntimeError {
                            message: "len() requires list, map, or string".to_string(),
                            span,
                        }),
                    }
                } else {
                    Err(RuntimeError {
                        message: "len() takes exactly 1 argument".to_string(),
                        span,
                    })
                }
            }
            "range" => {
                let limit = match args.first() {
                    Some(Value::Int(i)) => *i,
                    _ => 0,
                };
                let list: Vec<Value> = (0..limit).map(Value::Int).collect();
                Ok(Value::List(Rc::new(RefCell::new(list))))
            }
            "str" => {
                let s = args.first().map_or("".to_string(), |v| v.to_string_repr());
                Ok(Value::String(s))
            }
            "int" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Value::Int(i) => Ok(Value::Int(*i)),
                        Value::Float(f) => Ok(Value::Int(*f as i64)),
                        Value::String(s) => match s.trim().parse::<i64>() {
                            Ok(i) => Ok(Value::Int(i)),
                            Err(_) => Err(RuntimeError {
                                message: format!("Cannot convert '{}' to Int", s),
                                span,
                            }),
                        },
                        _ => Ok(Value::Int(0)),
                    }
                } else {
                    Ok(Value::Int(0))
                }
            }
            "float" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Value::Float(f) => Ok(Value::Float(*f)),
                        Value::Int(i) => Ok(Value::Float(*i as f64)),
                        Value::String(s) => match s.trim().parse::<f64>() {
                            Ok(f) => Ok(Value::Float(f)),
                            Err(_) => Err(RuntimeError {
                                message: format!("Cannot convert '{}' to Float", s),
                                span,
                            }),
                        },
                        _ => Ok(Value::Float(0.0)),
                    }
                } else {
                    Ok(Value::Float(0.0))
                }
            }
            "assert" => {
                if let Some(arg) = args.first() {
                    if !arg.is_truthy() {
                        return Err(RuntimeError {
                            message: "Assertion failed".to_string(),
                            span,
                        });
                    }
                    Ok(Value::Unit)
                } else {
                    Err(RuntimeError {
                        message: "assert requires a condition".to_string(),
                        span,
                    })
                }
            }
            "read_file" => {
                let path = args.first().map_or("".to_string(), |v| v.to_string_repr());
                match std::fs::read_to_string(&path) {
                    Ok(content) => Ok(Value::String(content)),
                    Err(_) => Ok(Value::String("".to_string())),
                }
            }
            "get_env" => {
                let key = args.get(0).map_or("".to_string(), |v| v.to_string_repr());
                let default_val = args.get(1).map_or("".to_string(), |v| v.to_string_repr());
                match std::env::var(&key) {
                    Ok(val) => Ok(Value::String(val)),
                    Err(_) => Ok(Value::String(default_val)),
                }
            }
            _ => Err(RuntimeError {
                message: format!("Unknown builtin function '{}'", name),
                span,
            }),
        }
    }

    fn eval_binary_op(&self, op: BinaryOp, l: Value, r: Value, span: Span) -> Result<Value, RuntimeError> {
        match op {
            BinaryOp::Add => match (l, r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b.to_string_repr()))),
                (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a.to_string_repr(), b))),
                (Value::List(a), Value::List(b)) => {
                    let mut combined = a.borrow().clone();
                    combined.extend(b.borrow().clone());
                    Ok(Value::List(Rc::new(RefCell::new(combined))))
                }
                _ => Err(RuntimeError {
                    message: "Unsupported types for '+' operator".to_string(),
                    span,
                }),
            },
            BinaryOp::Sub => match (l, r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)),
                _ => Err(RuntimeError {
                    message: "Unsupported types for '-' operator".to_string(),
                    span,
                }),
            },
            BinaryOp::Mul => match (l, r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)),
                (Value::String(s), Value::Int(n)) => Ok(Value::String(s.repeat(n.max(0) as usize))),
                _ => Err(RuntimeError {
                    message: "Unsupported types for '*' operator".to_string(),
                    span,
                }),
            },
            BinaryOp::Div => match (l, r) {
                (Value::Int(a), Value::Int(b)) => {
                    if b == 0 {
                        return Err(RuntimeError {
                            message: "Division by zero".to_string(),
                            span,
                        });
                    }
                    Ok(Value::Int(a / b))
                }
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 / b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / b as f64)),
                _ => Err(RuntimeError {
                    message: "Unsupported types for '/' operator".to_string(),
                    span,
                }),
            },
            BinaryOp::Mod => match (l, r) {
                (Value::Int(a), Value::Int(b)) => {
                    if b == 0 {
                        return Err(RuntimeError {
                            message: "Modulo by zero".to_string(),
                            span,
                        });
                    }
                    Ok(Value::Int(a % b))
                }
                _ => Err(RuntimeError {
                    message: "Modulo requires integer operands".to_string(),
                    span,
                }),
            },
            BinaryOp::Eq => Ok(Value::Bool(l == r)),
            BinaryOp::NotEq => Ok(Value::Bool(l != r)),
            BinaryOp::Lt => match (l, r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a < (b as f64))),
                (Value::String(a), Value::String(b)) => Ok(Value::Bool(a < b)),
                _ => Err(RuntimeError {
                    message: "Unsupported comparison operands".to_string(),
                    span,
                }),
            },
            BinaryOp::LtEq => match (l, r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) <= b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a <= (b as f64))),
                (Value::String(a), Value::String(b)) => Ok(Value::Bool(a <= b)),
                _ => Err(RuntimeError {
                    message: "Unsupported comparison operands".to_string(),
                    span,
                }),
            },
            BinaryOp::Gt => match (l, r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) > b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a > (b as f64))),
                (Value::String(a), Value::String(b)) => Ok(Value::Bool(a > b)),
                _ => Err(RuntimeError {
                    message: "Unsupported comparison operands".to_string(),
                    span,
                }),
            },
            BinaryOp::GtEq => match (l, r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) >= b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a >= (b as f64))),
                (Value::String(a), Value::String(b)) => Ok(Value::Bool(a >= b)),
                _ => Err(RuntimeError {
                    message: "Unsupported comparison operands".to_string(),
                    span,
                }),
            },
            BinaryOp::And => Ok(Value::Bool(l.is_truthy() && r.is_truthy())),
            BinaryOp::Or => Ok(Value::Bool(l.is_truthy() || r.is_truthy())),
        }
    }
}
