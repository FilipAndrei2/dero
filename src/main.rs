use std::env;

mod interpreter;
use interpreter::Interpreter;

fn main() {
    let interpreter: Interpreter = Interpreter::new();
    interpreter.run();
}
