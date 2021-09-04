use crate::lexer::create_lexer;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::ast::{Value, Expression};
use crate::ast::scope::Scope;
use crate::ast::e_plus::PlusExpression;

mod lexer;
mod ast;

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
    match create_lexer(input) {
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
    let expr = PlusExpression {
        left: Box::new(PlusExpression {
            left: Box::new(Value::Null),
            right: Box::new(Value::Integer(5)),
        }),
        right: Box::new(Value::Integer(1)),
    };
    match expr.evaluate(&mut scope) {
        Ok(value) => {
            println!("{}", value)
        },
        Err(err) => {
            println!("{}", err)
        }
    }

}