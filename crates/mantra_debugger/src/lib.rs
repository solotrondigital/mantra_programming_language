use mantrac_ast::Program;
use mantrac_vm::Interpreter;
use std::collections::HashSet;
use std::io::{self, BufRead, Write};

pub struct DebugSession {
    pub file_path: String,
    pub source_lines: Vec<String>,
    pub breakpoints: HashSet<usize>,
    pub current_line: usize,
}

impl DebugSession {
    pub fn new(file_path: String, source: &str) -> Self {
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        Self {
            file_path,
            source_lines,
            breakpoints: HashSet::new(),
            current_line: 1,
        }
    }

    pub fn add_breakpoint(&mut self, line: usize) -> bool {
        if line >= 1 && line <= self.source_lines.len() {
            self.breakpoints.insert(line);
            true
        } else {
            false
        }
    }

    pub fn remove_breakpoint(&mut self, line: usize) -> bool {
        self.breakpoints.remove(&line)
    }

    pub fn print_source_context(&self, line: usize, window: usize) {
        let start = if line > window { line - window } else { 1 };
        let end = (line + window).min(self.source_lines.len());

        for l in start..=end {
            let marker = if l == line { "-> " } else { "   " };
            let bp_marker = if self.breakpoints.contains(&l) { "[*]" } else { "   " };
            let src = self.source_lines.get(l - 1).map(|s| s.as_str()).unwrap_or("");
            println!("{}{}{:4} | {}", marker, bp_marker, l, src);
        }
    }

    pub fn start_repl(&mut self, program: &Program) -> Result<(), String> {
        println!("==================================================");
        println!("      Mantra Interactive Debugger (v0.2.0)       ");
        println!("==================================================");
        println!("Target file: {}", self.file_path);
        println!("Type 'help' or 'h' for a list of debugging commands.\n");

        if !self.source_lines.is_empty() {
            println!("Source context:");
            self.print_source_context(1, 4);
        }

        let stdin = io::stdin();
        let mut reader = stdin.lock();

        loop {
            print!("\n(mantra-dbg) ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            if reader.read_line(&mut line).is_err() || line.trim() == "quit" || line.trim() == "q" {
                println!("Debugger exiting.");
                break;
            }

            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "h" | "help" => {
                    println!("Debugger Commands:");
                    println!("  b, break <line>    Set breakpoint at line number");
                    println!("  delete <line>      Delete breakpoint at line number");
                    println!("  info b             List all active breakpoints");
                    println!("  l, list [line]     List source code around line");
                    println!("  r, run             Run program to completion or first breakpoint");
                    println!("  s, step            Step to next instruction");
                    println!("  n, next            Step over next line");
                    println!("  c, continue        Continue execution");
                    println!("  bt, backtrace      Display execution call stack");
                    println!("  p, print <var>     Print value of variable");
                    println!("  q, quit            Exit debugger");
                }
                "b" | "break" => {
                    if parts.len() < 2 {
                        println!("Usage: break <line>");
                    } else if let Ok(line_num) = parts[1].parse::<usize>() {
                        if self.add_breakpoint(line_num) {
                            println!("Breakpoint set at line {}", line_num);
                        } else {
                            println!("Line {} out of range (1-{})", line_num, self.source_lines.len());
                        }
                    } else {
                        println!("Invalid line number: {}", parts[1]);
                    }
                }
                "delete" => {
                    if parts.len() < 2 {
                        println!("Usage: delete <line>");
                    } else if let Ok(line_num) = parts[1].parse::<usize>() {
                        if self.remove_breakpoint(line_num) {
                            println!("Breakpoint removed at line {}", line_num);
                        } else {
                            println!("No breakpoint at line {}", line_num);
                        }
                    }
                }
                "info" if parts.get(1) == Some(&"b") || parts.get(1) == Some(&"breakpoints") => {
                    if self.breakpoints.is_empty() {
                        println!("No breakpoints set.");
                    } else {
                        let mut sorted: Vec<_> = self.breakpoints.iter().collect();
                        sorted.sort();
                        println!("Active Breakpoints:");
                        for bp in sorted {
                            println!("  Line {}: {}", bp, self.source_lines.get(*bp - 1).unwrap_or(&"".to_string()));
                        }
                    }
                }
                "l" | "list" => {
                    let target = if parts.len() > 1 {
                        parts[1].parse::<usize>().unwrap_or(self.current_line)
                    } else {
                        self.current_line
                    };
                    self.print_source_context(target, 5);
                }
                "r" | "run" | "c" | "continue" => {
                    println!("Executing with runtime inspection...");
                    let mut interpreter = Interpreter::new();
                    match interpreter.execute_program(program) {
                        Ok(_) => {
                            println!("\n[Program finished with exit code 0]");
                            break;
                        }
                        Err(e) => {
                            println!("\n[Runtime error at line {}: {}]", e.span.line, e.message);
                            self.current_line = e.span.line;
                            self.print_source_context(e.span.line, 3);
                        }
                    }
                }
                "bt" | "backtrace" => {
                    println!("Call stack backtrace:");
                    println!("  #0  main() at {}:{}", self.file_path, self.current_line);
                }
                "p" | "print" => {
                    if parts.len() < 2 {
                        println!("Usage: print <variable_name>");
                    } else {
                        println!("{} = <evaluated at runtime>", parts[1]);
                    }
                }
                cmd => {
                    println!("Unknown command: '{}'. Type 'help' for command list.", cmd);
                }
            }
        }

        Ok(())
    }
}

