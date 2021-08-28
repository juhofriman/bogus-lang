use crate::lexer::create_lexer;

mod lexer;

fn main() {
    let source = "let seppo = 1234;\nlet kalle = \"jorma heert\"";
    let mut lexer = create_lexer(source);

    println!("Lexing source `{}`", source);
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
    }
}