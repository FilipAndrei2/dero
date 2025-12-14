mod lexer;

use crate::lexer::{Lexer, LexiToken};

fn main() {
    let source_code = "32.4 65 1 2 3 4 67.67 21";
    let mut lexer = Lexer::new(&source_code);
    let mut tk = lexer.next_token();
    while !matches!(tk, Some(LexiToken::Eof)) {
        println!("{:?}", tk);
        tk = lexer.next_token();
    }
}
