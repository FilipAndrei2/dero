mod lexer;

use crate::lexer::{Lexer, LexiToken};

fn main() {
    let source_code = "let variabila = 32 ";
    let mut lexer = Lexer::new(&source_code);
    let mut tk = lexer.next_token();
    while !matches!(tk, Ok(LexiToken::Eof)) {
        println!("{:?}", tk);
        tk = lexer.next_token();
        if tk.is_err() {
            println!("Avem o problema grava");
            break;
        }
    }
}
