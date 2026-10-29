use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct PipelineProfiler {
    pub lexing: Duration,
    pub parsing: Duration,
    pub typechecking: Duration,
    pub ir_lowering: Duration,
    pub execution_or_codegen: Duration,
}

impl PipelineProfiler {
    pub fn print_report(&self) {
        println!("==================================================");
        println!("           MANTRA COMPILATION PROFILE            ");
        println!("==================================================");
        println!("  Lexer Phase       : {:>8.2?}", self.lexing);
        println!("  Parser Phase      : {:>8.2?}", self.parsing);
        println!("  Type Checker      : {:>8.2?}", self.typechecking);
        println!("  IR Lowering       : {:>8.2?}", self.ir_lowering);
        println!("  Backend / Runtime : {:>8.2?}", self.execution_or_codegen);
        let total = self.lexing + self.parsing + self.typechecking + self.ir_lowering + self.execution_or_codegen;
        println!("--------------------------------------------------");
        println!("  Total Pipeline    : {:>8.2?}", total);
        println!("==================================================");
    }
}

