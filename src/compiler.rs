mod lexer;
mod parser;
mod backend_converter;

use lexer::Lexer;
use parser::Parser;

use crate::compiler::backend_converter::{BackendAstConverter, BackendOutputGenerator};

pub struct Compiler <DiLexer: Lexer, DiParser: Parser, DiBackendAstConverter: BackendAstConverter, DiBackendOutputGenerator: BackendOutputGenerator> {
    lexer: DiLexer,
    parser: DiParser,
    backend_ast_converter: DiBackendAstConverter,
    backend_output_generator: DiBackendOutputGenerator
}


// Constructors
impl <DiLexer: Lexer, DiParser: Parser, DiBackendAstConverter: BackendAstConverter, DiBackendOutputGenerator: BackendOutputGenerator> Compiler<DiLexer, DiParser, DiBackendAstConverter, DiBackendOutputGenerator> {
    pub fn new(lexer: DiLexer, parser: DiParser, backend_ast_converter: DiBackendAstConverter, backend_output_generator: DiBackendOutputGenerator) -> Compiler<DiLexer, DiParser, DiBackendAstConverter, DiBackendOutputGenerator> {
        Compiler {
            lexer,
            parser,
            backend_ast_converter,
            backend_output_generator
        }
    }
}

// Object methods
impl <DiLexer: Lexer, DiParser: Parser, DiBackendAstConverter: BackendAstConverter, DiBackendOutputGenerator: BackendOutputGenerator>  Compiler<DiLexer, DiParser, DiBackendAstConverter, DiBackendOutputGenerator> {
    pub fn compile(&self, code: String) -> String {
        let tokens = self.lexer.lexically_analyze(code);
        let ast = self.parser.semantically_analyze(tokens);
        let backend_ast = self.backend_ast_converter.convert_ast(ast); 
        self.backend_output_generator.generate_output(backend_ast)
    }
}