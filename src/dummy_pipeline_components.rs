use crate::compiler::lexer::*;
use crate::compiler::parser::*;
use crate::compiler::backend_converter::*;

pub struct DummyLexer;
impl Lexer for DummyLexer {
    fn lexically_analyze(&self, code: String) -> Vec<LexiToken> {
        println!("Dummy lexer: lexically analyze");
        vec![]
    }
}

pub struct DummyParser;
impl Parser for DummyParser {
    fn semantically_analyze(&self, tokens: Vec<LexiToken>) -> Ast {
        println!("Dummy parser: semantically analyze");
        return Ast {root:AstNode::Dummy}
    }
}

pub struct DummyBackendAst;
impl BackendAst for DummyBackendAst {
    fn get_desired_output(&self) -> BackendDesiredOutput {
        todo!()
    }
}

pub struct DummyBeAstConverter;
impl BackendAstConverter for DummyBeAstConverter {
    fn convert_ast(&self, ast: Ast) -> impl BackendAst {
        println!("Dummy backend converter: convert to backend ast");
        DummyBackendAst { }
    }
}

pub struct DummyBeOutputGenerator;
impl BackendOutputGenerator for DummyBeOutputGenerator {
    fn generate_output(&self, b_ast: Box<dyn BackendAst + '_>) -> String {
        println!("Dummy output generator: generating the output");
        return "Pipeline complete!".to_string();
    }
}
