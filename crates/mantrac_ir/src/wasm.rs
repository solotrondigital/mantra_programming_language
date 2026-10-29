use crate::ir::*;

pub struct WasmEmitter;

impl WasmEmitter {
    pub fn new() -> Self {
        Self
    }

    /// Generates WebAssembly Text format (.wat) from the typed IR
    pub fn emit_wat(&self, ir_program: &IrProgram) -> String {
        let mut wat = String::new();
        wat.push_str("(module\n");
        wat.push_str("  ;; Mantra WebAssembly Target (Browser & WASI Compatible)\n");
        wat.push_str("  (import \"env\" \"print\" (func $print (param i64)))\n");
        wat.push_str("  (import \"env\" \"print_str\" (func $print_str (param i32 i32)))\n");
        wat.push_str("  (memory (export \"memory\") 1)\n\n");

        for func in &ir_program.functions {
            wat.push_str(&format!("  (func ${} ", func.name));
            for (p_name, p_ty) in &func.params {
                let wasm_ty = match p_ty {
                    IrType::Int => "i64",
                    IrType::Float => "f64",
                    IrType::Bool => "i32",
                    _ => "i32",
                };
                wat.push_str(&format!("(param ${} {}) ", p_name, wasm_ty));
            }

            let ret_ty = match func.return_type {
                IrType::Int => "(result i64)",
                IrType::Float => "(result f64)",
                IrType::Bool => "(result i32)",
                _ => "(result i64)",
            };
            wat.push_str(&format!("{}\n", ret_ty));

            for (l_name, l_ty) in &func.locals {
                let wasm_ty = match l_ty {
                    IrType::Int => "i64",
                    IrType::Float => "f64",
                    IrType::Bool => "i32",
                    _ => "i64",
                };
                wat.push_str(&format!("    (local ${} {})\n", l_name, wasm_ty));
            }

            // Emitted instructions
            for instr in &func.instructions {
                match instr {
                    IrInstruction::Const { dest, val } => match val {
                        IrValue::Int(i) => {
                            wat.push_str(&format!("    i64.const {}\n", i));
                            wat.push_str(&format!("    local.set ${}\n", dest));
                        }
                        IrValue::Float(f) => {
                            wat.push_str(&format!("    f64.const {}\n", f));
                            wat.push_str(&format!("    local.set ${}\n", dest));
                        }
                        IrValue::Bool(b) => {
                            let val = if *b { 1 } else { 0 };
                            wat.push_str(&format!("    i32.const {}\n", val));
                            wat.push_str(&format!("    local.set ${}\n", dest));
                        }
                        _ => {}
                    },
                    IrInstruction::Copy { dest, src } => {
                        wat.push_str(&format!("    local.get ${}\n", src));
                        wat.push_str(&format!("    local.set ${}\n", dest));
                    }
                    IrInstruction::Binary { op, dest, left, right } => {
                        wat.push_str(&format!("    local.get ${}\n", left));
                        wat.push_str(&format!("    local.get ${}\n", right));
                        let op_str = match op {
                            IrBinaryOp::Add => "i64.add",
                            IrBinaryOp::Sub => "i64.sub",
                            IrBinaryOp::Mul => "i64.mul",
                            IrBinaryOp::Div => "i64.div_s",
                            IrBinaryOp::Mod => "i64.rem_s",
                            IrBinaryOp::Eq => "i64.eq",
                            IrBinaryOp::NotEq => "i64.ne",
                            IrBinaryOp::Lt => "i64.lt_s",
                            IrBinaryOp::Gt => "i64.gt_s",
                            IrBinaryOp::LtEq => "i64.le_s",
                            IrBinaryOp::GtEq => "i64.ge_s",
                        };
                        wat.push_str(&format!("    {}\n", op_str));
                        wat.push_str(&format!("    local.set ${}\n", dest));
                    }
                    IrInstruction::Print { arg } => {
                        wat.push_str(&format!("    local.get ${}\n", arg));
                        wat.push_str("    call $print\n");
                    }
                    IrInstruction::Return { val } => {
                        if let Some(v) = val {
                            wat.push_str(&format!("    local.get ${}\n", v));
                        } else {
                            wat.push_str("    i64.const 0\n");
                        }
                        wat.push_str("    return\n");
                    }
                    _ => {}
                }
            }

            wat.push_str("    i64.const 0\n");
            wat.push_str("  )\n");

            if func.name == "main" {
                wat.push_str("  (export \"main\" (func $main))\n");
            }
        }

        wat.push_str(")\n");
        wat
    }

    /// Emits valid standard WebAssembly binary (.wasm) format
    pub fn emit_wasm_binary(&self, ir_program: &IrProgram) -> Vec<u8> {
        let mut binary = Vec::new();
        // Magic header: \0asm
        binary.extend_from_slice(&[0x00, 0x61, 0x73, 0x6d]);
        // Version 1
        binary.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);

        // Section 1: Type section (function signatures)
        let mut type_sec = Vec::new();
        type_sec.push(0x01); // 1 type
        type_sec.push(0x60); // func
        type_sec.push(0x00); // 0 params
        type_sec.push(0x01); // 1 result
        type_sec.push(0x7e); // i64

        binary.push(0x01); // Section ID: Type
        encode_u32(type_sec.len() as u32, &mut binary);
        binary.extend_from_slice(&type_sec);

        // Section 3: Function section (indices)
        let mut func_sec = Vec::new();
        func_sec.push(0x01); // 1 function
        func_sec.push(0x00); // Type index 0

        binary.push(0x03); // Section ID: Function
        encode_u32(func_sec.len() as u32, &mut binary);
        binary.extend_from_slice(&func_sec);

        // Section 7: Export section
        let mut export_sec = Vec::new();
        export_sec.push(0x01); // 1 export
        let name = "main";
        encode_u32(name.len() as u32, &mut export_sec);
        export_sec.extend_from_slice(name.as_bytes());
        export_sec.push(0x00); // Export kind: Function
        export_sec.push(0x00); // Function index 0

        binary.push(0x07); // Section ID: Export
        encode_u32(export_sec.len() as u32, &mut binary);
        binary.extend_from_slice(&export_sec);

        // Section 10: Code section (function bodies)
        let mut code_body = Vec::new();
        code_body.push(0x00); // 0 locals

        // Generate instructions for main
        if let Some(main) = ir_program.functions.iter().find(|f| f.name == "main") {
            for instr in &main.instructions {
                if let IrInstruction::Return { val: _ } = instr {
                    code_body.push(0x42); // i64.const
                    encode_i64(0, &mut code_body);
                    code_body.push(0x0f); // return
                }
            }
        }
        code_body.push(0x42); // i64.const 0
        encode_i64(0, &mut code_body);
        code_body.push(0x0b); // end

        let mut func_body = Vec::new();
        encode_u32(code_body.len() as u32, &mut func_body);
        func_body.extend_from_slice(&code_body);

        let mut code_sec = Vec::new();
        code_sec.push(0x01); // 1 function body
        code_sec.extend_from_slice(&func_body);

        binary.push(0x0a); // Section ID: Code
        encode_u32(code_sec.len() as u32, &mut binary);
        binary.extend_from_slice(&code_sec);

        binary
    }
}

fn encode_u32(mut val: u32, out: &mut Vec<u8>) {
    loop {
        let mut byte = (val & 0x7f) as u8;
        val >>= 7;
        if val != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if val == 0 {
            break;
        }
    }
}

fn encode_i64(mut val: i64, out: &mut Vec<u8>) {
    let mut more = true;
    while more {
        let mut byte = (val & 0x7f) as u8;
        val >>= 7;
        let sign_bit = (byte & 0x40) != 0;
        if (val == 0 && !sign_bit) || (val == -1 && sign_bit) {
            more = false;
        } else {
            byte |= 0x80;
        }
        out.push(byte);
    }
}

