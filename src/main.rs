mod lexer;

use crate::lexer::Lexer;

fn main() {
    let source_code = "1234";
    let mut lexer: Lexer = Lexer::new(&source_code);
    println!("{:?}", lexer.next_token());
}
