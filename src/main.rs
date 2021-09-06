use crate::lexer::{Lexer};
use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::ast::{Value, Expression};
use crate::ast::scope::Scope;
use crate::ast::e_plus::PlusExpression;

mod lexer;
mod ast;
mod parser;

enum ReplMode {
    Normal,
    Lexus,
}

fn prompt(repl_mode: &ReplMode) -> &str {
    match repl_mode {
        ReplMode::Normal => "bogus> ",
        ReplMode::Lexus => "bogus [lex]> ",
    }
}

fn main() {
    let mut repl_mode = ReplMode::Lexus;
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(prompt(&repl_mode));
        match readline {
            Ok(line) => {
                match line {
                    _ if line.is_empty() => (),
                    _ if line.starts_with(":normal") => {
                        repl_mode = ReplMode::Normal
                    }
                    _ if line.starts_with(":lexus") => {
                        repl_mode = ReplMode::Lexus
                    }
                    _ => {
                        rl.add_history_entry(line.as_str());
                        match repl_mode {
                            ReplMode::Lexus => {
                                lex_input(&line);
                            }
                            ReplMode::Normal => {
                                dummy_eval();
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn lex_input(input: &str) {
    match Lexer::new(input) {
        Ok(mut lexer) => {
            println!("[");
            while lexer.has_next() {
                if let Some(token) = lexer.next() {
                    println!("\t{},", token)
                }
            }
            println!("]");
        }
        Err(error) => {
            println!("{}", error)
        }
    }
}

fn dummy_eval() {
    let mut scope = Scope::new();
    let expr = PlusExpression::new(
        Box::new(PlusExpression::new(
            Box::new(Value::Null),
            Box::new(Value::Integer(5)),
        )),
        Box::new(Value::Integer(1)),
    );
    match expr.evaluate(&mut scope) {
        Ok(value) => {
            println!("{}", value)
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}