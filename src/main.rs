use crate::lexer::create_lexer;
use rustyline::Editor;
use rustyline::error::ReadlineError;

mod lexer;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("bogus> ");
        match readline {
            Ok(line) => {
                if !line.is_empty() {
                    rl.add_history_entry(line.as_str());
                    lex_input(&line);
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}

fn lex_input(input: &str) {
    let mut lexer = create_lexer(input);
    println!("[");
    while lexer.has_next() {
        if let Some(token) = lexer.next() {
            println!("\t{},", token)
        }
    }
    println!("]");
}