pub enum LexiToken {

}

pub trait Lexer {
    fn lexically_analyze(&self, code: String) -> Vec<LexiToken>;
}