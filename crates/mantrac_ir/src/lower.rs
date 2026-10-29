use crate::ir::*;
use mantrac_ast::*;

pub struct AstLowering {
    temp_counter: usize,
}

impl AstLowering {
    pub fn new() -> Self {
        Self { temp_counter: 0 }
    }

    fn new_temp(&mut self) -> String {
        self.temp_counter += 1;
        format!("_t{}", self.temp_counter)
    }

    pub fn lower_program(&mut self, program: &Program) -> IrProgram {
        let mut ir_prog = IrProgram::new();

        for decl in &program.declarations {
            match decl {
                Declaration::Function(func) => {
                    let ir_func = self.lower_function(func);
                    ir_prog.functions.push(ir_func);
                }
                Declaration::Class(_) => {
                    // OOP classes use hybrid object table dispatch or interpreter fallback in JIT
                    ir_prog.has_unsupported_features = false;
                }
                _ => {}
            }
        }

        ir_prog
    }

    fn lower_type(&self, ty: &Option<Type>) -> IrType {
        match ty {
            Some(Type::Int) => IrType::Int,
            Some(Type::Float) => IrType::Float,
            Some(Type::Bool) => IrType::Bool,
            Some(Type::String) => IrType::String,
            Some(Type::Unit) | None => IrType::Void,
            Some(Type::Custom(name, _)) => IrType::Custom(name.clone()),
            _ => IrType::Void,
        }
    }

    fn lower_function(&mut self, func: &FunctionDecl) -> IrFunction {
        let mut instructions = Vec::new();
        let mut locals = Vec::new();

        let params: Vec<(String, IrType)> = func
            .params
            .iter()
            .map(|p| (p.name.clone(), self.lower_type(&p.ty)))
            .collect();

        for stmt in &func.body {
            self.lower_stmt(stmt, &mut instructions, &mut locals);
        }

        IrFunction {
            name: func.name.clone(),
            params,
            return_type: self.lower_type(&func.return_type),
            is_async: func.is_async,
            instructions,
            locals,
        }
    }

    fn lower_stmt(&mut self, stmt: &Stmt, instrs: &mut Vec<IrInstruction>, locals: &mut Vec<(String, IrType)>) {
        match stmt {
            Stmt::Let { name, ty, init, .. } | Stmt::Mut { name, ty, init, .. } => {
                let init_reg = self.lower_expr(init, instrs);
                instrs.push(IrInstruction::Copy {
                    dest: name.clone(),
                    src: init_reg,
                });
                locals.push((name.clone(), self.lower_type(ty)));
            }
            Stmt::Assign { target, value, .. } => {
                let val_reg = self.lower_expr(value, instrs);
                if let Expr::Ident(ref name, _) = target {
                    instrs.push(IrInstruction::Copy {
                        dest: name.clone(),
                        src: val_reg,
                    });
                }
            }
            Stmt::Return { value, .. } => {
                let reg = value.as_ref().map(|v| self.lower_expr(v, instrs));
                instrs.push(IrInstruction::Return { val: reg });
            }
            Stmt::Expr(expr) => {
                self.lower_expr(expr, instrs);
            }
            Stmt::While { condition, body, .. } => {
                let loop_start = instrs.len();
                let cond_reg = self.lower_expr(condition, instrs);
                let branch_idx = instrs.len();
                // placeholder branch
                instrs.push(IrInstruction::Branch {
                    cond: cond_reg,
                    true_target: branch_idx + 1,
                    false_target: 0, // patched later
                });

                for s in body {
                    self.lower_stmt(s, instrs, locals);
                }
                instrs.push(IrInstruction::Jump { target: loop_start });

                let loop_end = instrs.len();
                if let IrInstruction::Branch { ref mut false_target, .. } = instrs[branch_idx] {
                    *false_target = loop_end;
                }
            }
            Stmt::If { condition, then_branch, elif_branches, else_branch, .. } => {
                let cond_reg = self.lower_expr(condition, instrs);
                let branch_idx = instrs.len();
                instrs.push(IrInstruction::Branch {
                    cond: cond_reg,
                    true_target: branch_idx + 1,
                    false_target: 0,
                });

                for s in then_branch {
                    self.lower_stmt(s, instrs, locals);
                }

                let jump_out_idx = instrs.len();
                instrs.push(IrInstruction::Jump { target: 0 });

                let false_target = instrs.len();
                if let IrInstruction::Branch { false_target: ref mut ft, .. } = instrs[branch_idx] {
                    *ft = false_target;
                }

                for (_, elif_body) in elif_branches {
                    for s in elif_body {
                        self.lower_stmt(s, instrs, locals);
                    }
                }

                if let Some(eb) = else_branch {
                    for s in eb {
                        self.lower_stmt(s, instrs, locals);
                    }
                }

                let end_idx = instrs.len();
                if let IrInstruction::Jump { ref mut target } = instrs[jump_out_idx] {
                    *target = end_idx;
                }
            }
            _ => {}
        }
    }

    fn lower_expr(&mut self, expr: &Expr, instrs: &mut Vec<IrInstruction>) -> String {
        match expr {
            Expr::Literal(Literal::Integer(i), _) => {
                let dest = self.new_temp();
                instrs.push(IrInstruction::Const {
                    dest: dest.clone(),
                    val: IrValue::Int(*i),
                });
                dest
            }
            Expr::Literal(Literal::Float(f), _) => {
                let dest = self.new_temp();
                instrs.push(IrInstruction::Const {
                    dest: dest.clone(),
                    val: IrValue::Float(*f),
                });
                dest
            }
            Expr::Literal(Literal::String(s), _) => {
                let dest = self.new_temp();
                instrs.push(IrInstruction::Const {
                    dest: dest.clone(),
                    val: IrValue::String(s.clone()),
                });
                dest
            }
            Expr::Literal(Literal::Bool(b), _) => {
                let dest = self.new_temp();
                instrs.push(IrInstruction::Const {
                    dest: dest.clone(),
                    val: IrValue::Bool(*b),
                });
                dest
            }
            Expr::Literal(Literal::None, _) => {
                let dest = self.new_temp();
                instrs.push(IrInstruction::Const {
                    dest: dest.clone(),
                    val: IrValue::Null,
                });
                dest
            }
            Expr::Ident(name, _) => name.clone(),
            Expr::Binary { op, left, right, .. } => {
                let l_reg = self.lower_expr(left, instrs);
                let r_reg = self.lower_expr(right, instrs);
                let dest = self.new_temp();
                let ir_op = match op {
                    BinaryOp::Add => IrBinaryOp::Add,
                    BinaryOp::Sub => IrBinaryOp::Sub,
                    BinaryOp::Mul => IrBinaryOp::Mul,
                    BinaryOp::Div => IrBinaryOp::Div,
                    BinaryOp::Mod => IrBinaryOp::Mod,
                    BinaryOp::Eq => IrBinaryOp::Eq,
                    BinaryOp::NotEq => IrBinaryOp::NotEq,
                    BinaryOp::Lt => IrBinaryOp::Lt,
                    BinaryOp::Gt => IrBinaryOp::Gt,
                    BinaryOp::LtEq => IrBinaryOp::LtEq,
                    BinaryOp::GtEq => IrBinaryOp::GtEq,
                    _ => IrBinaryOp::Add,
                };
                instrs.push(IrInstruction::Binary {
                    op: ir_op,
                    dest: dest.clone(),
                    left: l_reg,
                    right: r_reg,
                });
                dest
            }
            Expr::Call { callee, args, .. } => {
                if let Expr::Ident(ref name, _) = **callee {
                    if name == "print" {
                        let arg_reg = if let Some(first) = args.first() {
                            self.lower_expr(first, instrs)
                        } else {
                            let empty = self.new_temp();
                            instrs.push(IrInstruction::Const {
                                dest: empty.clone(),
                                val: IrValue::String("".to_string()),
                            });
                            empty
                        };
                        instrs.push(IrInstruction::Print { arg: arg_reg });
                        let dest = self.new_temp();
                        instrs.push(IrInstruction::Const {
                            dest: dest.clone(),
                            val: IrValue::Int(0),
                        });
                        return dest;
                    }

                    let arg_regs: Vec<String> = args.iter().map(|a| self.lower_expr(a, instrs)).collect();
                    let dest = self.new_temp();
                    instrs.push(IrInstruction::Call {
                        dest: Some(dest.clone()),
                        func: name.clone(),
                        args: arg_regs,
                    });
                    dest
                } else {
                    let dest = self.new_temp();
                    dest
                }
            }
            Expr::Await { value, .. } => {
                let task_reg = self.lower_expr(value, instrs);
                let dest = self.new_temp();
                instrs.push(IrInstruction::Await {
                    dest: dest.clone(),
                    task: task_reg,
                });
                dest
            }
            _ => {
                let dest = self.new_temp();
                dest
            }
        }
    }
}
