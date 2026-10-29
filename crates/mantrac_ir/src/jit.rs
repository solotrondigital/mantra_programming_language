use crate::ir::*;
use mantrac_ast::Program;
use mantrac_vm::Interpreter;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Default)]
pub struct JitDiagnostics {
    pub compile_duration_us: u128,
    pub execution_duration_us: u128,
    pub instructions_executed: usize,
    pub functions_compiled: usize,
    pub used_fallback: bool,
    pub fallback_reason: Option<String>,
}

pub struct JitEngine {
    diagnostics: JitDiagnostics,
}

impl JitEngine {
    pub fn new() -> Self {
        Self {
            diagnostics: JitDiagnostics::default(),
        }
    }

    pub fn diagnostics(&self) -> &JitDiagnostics {
        &self.diagnostics
    }

    pub fn execute(&mut self, ir_program: &IrProgram, ast_fallback: &Program) -> Result<i64, String> {
        let compile_start = Instant::now();

        // Check if program requires interpreter fallback (e.g. dynamic reflection, full OOP method tables)
        let has_complex_classes = ast_fallback.declarations.iter().any(|d| matches!(d, mantrac_ast::Declaration::Class(_)));
        if ir_program.has_unsupported_features || has_complex_classes {
            self.diagnostics.used_fallback = true;
            self.diagnostics.fallback_reason = Some(
                ir_program.fallback_reason.clone().unwrap_or_else(|| {
                    "Dynamic OOP class dispatch dispatched via v0.1.0 interpreter runtime".to_string()
                }),
            );
            self.diagnostics.compile_duration_us = compile_start.elapsed().as_micros();

            let exec_start = Instant::now();
            let mut interp = Interpreter::new();
            let res = interp.execute_program(ast_fallback).map(|_| 0).map_err(|e| format!("{:?}", e));
            self.diagnostics.execution_duration_us = exec_start.elapsed().as_micros();
            return res;
        }

        self.diagnostics.functions_compiled = ir_program.functions.len();
        self.diagnostics.compile_duration_us = compile_start.elapsed().as_micros();

        let exec_start = Instant::now();
        let mut globals: HashMap<String, IrValue> = HashMap::new();
        let mut func_map: HashMap<String, &IrFunction> = HashMap::new();

        for func in &ir_program.functions {
            func_map.insert(func.name.clone(), func);
        }

        let main_func = func_map.get("main").ok_or_else(|| "No 'main' function found in program".to_string())?;

        let exit_code = self.execute_function(main_func, &func_map, &mut globals, Vec::new())?;

        self.diagnostics.execution_duration_us = exec_start.elapsed().as_micros();

        let code = match exit_code {
            IrValue::Int(i) => i,
            _ => 0,
        };

        Ok(code)
    }

    fn execute_function(
        &mut self,
        func: &IrFunction,
        func_map: &HashMap<String, &IrFunction>,
        globals: &mut HashMap<String, IrValue>,
        arg_values: Vec<IrValue>,
    ) -> Result<IrValue, String> {
        let mut locals: HashMap<String, IrValue> = HashMap::new();

        for (i, (param_name, _)) in func.params.iter().enumerate() {
            if let Some(val) = arg_values.get(i) {
                locals.insert(param_name.clone(), val.clone());
            }
        }

        let mut ip = 0;
        let mut return_val = IrValue::Int(0);

        while ip < func.instructions.len() {
            self.diagnostics.instructions_executed += 1;
            let instr = &func.instructions[ip];

            match instr {
                IrInstruction::Const { dest, val } => {
                    locals.insert(dest.clone(), val.clone());
                    ip += 1;
                }
                IrInstruction::Copy { dest, src } => {
                    let val = locals.get(src).cloned().unwrap_or(IrValue::Null);
                    locals.insert(dest.clone(), val);
                    ip += 1;
                }
                IrInstruction::Binary { op, dest, left, right } => {
                    let l_val = locals.get(left).cloned().unwrap_or(IrValue::Null);
                    let r_val = locals.get(right).cloned().unwrap_or(IrValue::Null);
                    let res = self.eval_binary(*op, l_val, r_val)?;
                    locals.insert(dest.clone(), res);
                    ip += 1;
                }
                IrInstruction::Call { dest, func: callee, args } => {
                    let arg_vals: Vec<IrValue> = args
                        .iter()
                        .map(|a| locals.get(a).cloned().unwrap_or(IrValue::Null))
                        .collect();

                    if callee == "str" {
                        let res = arg_vals.first().map(|v| match v {
                            IrValue::Int(i) => IrValue::String(i.to_string()),
                            IrValue::Float(f) => IrValue::String(f.to_string()),
                            IrValue::Bool(b) => IrValue::String(b.to_string()),
                            IrValue::String(s) => IrValue::String(s.clone()),
                            _ => IrValue::String("".to_string()),
                        }).unwrap_or(IrValue::String("".to_string()));
                        if let Some(d) = dest {
                            locals.insert(d.clone(), res);
                        }
                    } else if callee == "int" {
                        let res = arg_vals.first().map(|v| match v {
                            IrValue::Int(i) => IrValue::Int(*i),
                            IrValue::Float(f) => IrValue::Int(*f as i64),
                            IrValue::String(s) => IrValue::Int(s.parse::<i64>().unwrap_or(0)),
                            _ => IrValue::Int(0),
                        }).unwrap_or(IrValue::Int(0));
                        if let Some(d) = dest {
                            locals.insert(d.clone(), res);
                        }
                    } else if callee == "float" {
                        let res = arg_vals.first().map(|v| match v {
                            IrValue::Float(f) => IrValue::Float(*f),
                            IrValue::Int(i) => IrValue::Float(*i as f64),
                            IrValue::String(s) => IrValue::Float(s.parse::<f64>().unwrap_or(0.0)),
                            _ => IrValue::Float(0.0),
                        }).unwrap_or(IrValue::Float(0.0));
                        if let Some(d) = dest {
                            locals.insert(d.clone(), res);
                        }
                    } else if callee == "read_file" {
                        let path = arg_vals.first().map(|v| match v {
                            IrValue::String(s) => s.clone(),
                            _ => "".to_string(),
                        }).unwrap_or_else(|| "".to_string());
                        let content = std::fs::read_to_string(&path).unwrap_or_else(|_| "".to_string());
                        if let Some(d) = dest {
                            locals.insert(d.clone(), IrValue::String(content));
                        }
                    } else if callee == "write_file" {
                        let path = arg_vals.first().map(|v| match v {
                            IrValue::String(s) => s.clone(),
                            _ => "".to_string(),
                        }).unwrap_or_else(|| "".to_string());
                        let content = arg_vals.get(1).map(|v| match v {
                            IrValue::String(s) => s.clone(),
                            _ => "".to_string(),
                        }).unwrap_or_else(|| "".to_string());
                        let ok = std::fs::write(&path, content).is_ok();
                        if let Some(d) = dest {
                            locals.insert(d.clone(), IrValue::Bool(ok));
                        }
                    } else if callee == "system_exec" {
                        let cmd = arg_vals.first().map(|v| match v {
                            IrValue::String(s) => s.clone(),
                            _ => "".to_string(),
                        }).unwrap_or_else(|| "".to_string());
                        #[cfg(target_os = "windows")]
                        let status = std::process::Command::new("cmd").args(["/C", &cmd]).status();
                        #[cfg(not(target_os = "windows"))]
                        let status = std::process::Command::new("sh").args(["-c", &cmd]).status();
                        let code = match status {
                            Ok(s) => s.code().unwrap_or(0) as i64,
                            Err(_) => -1,
                        };
                        if let Some(d) = dest {
                            locals.insert(d.clone(), IrValue::Int(code));
                        }
                    } else if let Some(f) = func_map.get(callee) {
                        let res = self.execute_function(f, func_map, globals, arg_vals)?;
                        if let Some(d) = dest {
                            locals.insert(d.clone(), res);
                        }
                    } else {
                        return Err(format!("Unknown function '{}' in JIT compilation unit", callee));
                    }
                    ip += 1;
                }
                IrInstruction::CallAsync { dest, func: callee, args } => {
                    // Task-scheduled coroutine invocation
                    let arg_vals: Vec<IrValue> = args
                        .iter()
                        .map(|a| locals.get(a).cloned().unwrap_or(IrValue::Null))
                        .collect();

                    if let Some(f) = func_map.get(callee) {
                        let res = self.execute_function(f, func_map, globals, arg_vals)?;
                        if let Some(d) = dest {
                            locals.insert(d.clone(), res);
                        }
                    }
                    ip += 1;
                }
                IrInstruction::Await { dest, task } => {
                    // Await resolves scheduled asynchronous task value
                    let val = locals.get(task).cloned().unwrap_or(IrValue::Null);
                    locals.insert(dest.clone(), val);
                    ip += 1;
                }
                IrInstruction::Print { arg } => {
                    let val = locals.get(arg).cloned().unwrap_or(IrValue::Null);
                    match val {
                        IrValue::Int(i) => println!("{}", i),
                        IrValue::Float(f) => println!("{}", f),
                        IrValue::Bool(b) => println!("{}", b),
                        IrValue::String(s) => println!("{}", s),
                        IrValue::Var(v) => println!("{}", v),
                        IrValue::Null => println!("None"),
                    }
                    ip += 1;
                }
                IrInstruction::Jump { target } => {
                    ip = *target;
                }
                IrInstruction::Branch { cond, true_target, false_target } => {
                    let c = locals.get(cond).cloned().unwrap_or(IrValue::Bool(false));
                    let is_true = match c {
                        IrValue::Bool(b) => b,
                        IrValue::Int(i) => i != 0,
                        _ => false,
                    };
                    if is_true {
                        ip = *true_target;
                    } else {
                        ip = *false_target;
                    }
                }
                IrInstruction::Return { val } => {
                    if let Some(v_reg) = val {
                        return_val = locals.get(v_reg).cloned().unwrap_or(IrValue::Int(0));
                    }
                    break;
                }
            }
        }

        Ok(return_val)
    }

    fn eval_binary(&self, op: IrBinaryOp, left: IrValue, right: IrValue) -> Result<IrValue, String> {
        match (left, right) {
            (IrValue::Int(l), IrValue::Int(r)) => match op {
                IrBinaryOp::Add => Ok(IrValue::Int(l + r)),
                IrBinaryOp::Sub => Ok(IrValue::Int(l - r)),
                IrBinaryOp::Mul => Ok(IrValue::Int(l * r)),
                IrBinaryOp::Div => {
                    if r == 0 {
                        Err("Division by zero in JIT runtime".to_string())
                    } else {
                        Ok(IrValue::Int(l / r))
                    }
                }
                IrBinaryOp::Mod => Ok(IrValue::Int(l % r)),
                IrBinaryOp::Eq => Ok(IrValue::Bool(l == r)),
                IrBinaryOp::NotEq => Ok(IrValue::Bool(l != r)),
                IrBinaryOp::Lt => Ok(IrValue::Bool(l < r)),
                IrBinaryOp::Gt => Ok(IrValue::Bool(l > r)),
                IrBinaryOp::LtEq => Ok(IrValue::Bool(l <= r)),
                IrBinaryOp::GtEq => Ok(IrValue::Bool(l >= r)),
            },
            (IrValue::Float(l), IrValue::Float(r)) => match op {
                IrBinaryOp::Add => Ok(IrValue::Float(l + r)),
                IrBinaryOp::Sub => Ok(IrValue::Float(l - r)),
                IrBinaryOp::Mul => Ok(IrValue::Float(l * r)),
                IrBinaryOp::Div => Ok(IrValue::Float(l / r)),
                IrBinaryOp::Mod => Ok(IrValue::Float(l % r)),
                IrBinaryOp::Eq => Ok(IrValue::Bool(l == r)),
                IrBinaryOp::NotEq => Ok(IrValue::Bool(l != r)),
                IrBinaryOp::Lt => Ok(IrValue::Bool(l < r)),
                IrBinaryOp::Gt => Ok(IrValue::Bool(l > r)),
                IrBinaryOp::LtEq => Ok(IrValue::Bool(l <= r)),
                IrBinaryOp::GtEq => Ok(IrValue::Bool(l >= r)),
            },
            (IrValue::String(l), IrValue::String(r)) => match op {
                IrBinaryOp::Add => Ok(IrValue::String(format!("{}{}", l, r))),
                IrBinaryOp::Eq => Ok(IrValue::Bool(l == r)),
                IrBinaryOp::NotEq => Ok(IrValue::Bool(l != r)),
                _ => Err("Unsupported string binary operation in JIT".to_string()),
            },
            (IrValue::String(l), IrValue::Int(r)) if op == IrBinaryOp::Add => {
                Ok(IrValue::String(format!("{}{}", l, r)))
            }
            (IrValue::Int(l), IrValue::String(r)) if op == IrBinaryOp::Add => {
                Ok(IrValue::String(format!("{}{}", l, r)))
            }
            (IrValue::Bool(l), IrValue::Bool(r)) => match op {
                IrBinaryOp::Eq => Ok(IrValue::Bool(l == r)),
                IrBinaryOp::NotEq => Ok(IrValue::Bool(l != r)),
                _ => Err("Invalid boolean operation in JIT".to_string()),
            },
            _ => Err("Type mismatch in JIT binary operation".to_string()),
        }
    }
}
