use crate::compiler::parser::Ast;

#[repr(u8)]
pub enum BackendDesiredOutput {
    C = 0
}

pub trait BackendAst {
    fn get_desired_output() -> BackendDesiredOutput;
}

pub trait BackendAstConverter {
    fn convert_ast(&self, ast: Ast) -> impl BackendAst;
}

pub trait BackendOutputGenerator {
    fn generate_output(&self, b_ast: BackendAst) -> String;
}