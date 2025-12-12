pub mod compiler;
pub mod dummy_pipeline_components;

use compiler::Compiler;

use crate::dummy_pipeline_components::{DummyBeAstConverter, DummyBeOutputGenerator, DummyLexer, DummyParser};
fn main() {
    let compiler= Compiler::new(DummyLexer, DummyParser, DummyBeAstConverter, DummyBeOutputGenerator);
    let code: String = "println(\"Hello, world!\");".to_string();
    compiler.compile(code);
}