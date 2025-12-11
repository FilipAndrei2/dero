use crate::compiler::lexer::LexiToken;

pub enum AstNode {

}

pub struct Ast {
    root: AstNode
}

pub trait Parser {
    fn semantically_analyze(&self, tokens: Vec<LexiToken>) -> Ast;
}