use crate::compiler::lexer::LexiToken;

pub enum AstNode {
    Dummy
}

pub struct Ast {
    pub root: AstNode
}

pub trait Parser {
    fn semantically_analyze(&self, tokens: Vec<LexiToken>) -> Ast;
}

