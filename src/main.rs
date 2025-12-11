mod compiler;
use compiler::Compiler;

fn main() {
    let compiler= Compiler::new();
    let code: String = "println(\"Hello, world!\");".to_string();
    compiler.compile(code);
}