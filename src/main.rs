use crate::lexer::{Lexer};
use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::ast::scope::Scope;
use crate::parser::Parser;

mod lexer;
mod ast;
mod parser;

enum ReplMode {
    Normal,
    Lexus,
    Ast,
}

fn prompt(repl_mode: &ReplMode) -> &str {
    match repl_mode {
        ReplMode::Normal => "bogus> ",
        ReplMode::Lexus => "bogus [lex]> ",
        ReplMode::Ast => "bogus [ast]> ",
    }
}

fn main() {
    let mut repl_mode = ReplMode::Normal;
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
                    _ if line.starts_with(":ast") => {
                        repl_mode = ReplMode::Ast
                    }
                    _ => {
                        rl.add_history_entry(line.as_str());
                        match repl_mode {
                            ReplMode::Lexus => {
                                lex_input(&line);
                            }
                            ReplMode::Ast => {
                                ast_input(&line);
                            }
                            ReplMode::Normal => {
                                eval(&line);
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

fn ast_input(input: &str) {

    match Lexer::new(input) {
        Ok(mut lexer) => {
            let mut parser = Parser::new(&mut lexer);
            match parser.parse() {
                Ok(things) => {
                    for thing in things {
                        thing.visualize(1);
                    }
                },
                Err(parse_error) => println!("{}", parse_error)
            }
        }
        Err(lexing_error) => println!("{}", lexing_error)
    }
}

fn eval(input: &str) {

    match Lexer::new(input) {
        Ok(mut lexer) => {
            let mut parser = Parser::new(&mut lexer);
            match parser.parse() {
                Ok(things) => {
                    let mut scope = Scope::new();
                    for thing in things {
                        match thing.evaluate(&mut scope) {
                            Ok(res) => {
                                println!("{}", res);
                            },
                            Err(eval_error) => {
                                println!("{}", eval_error);
                            }
                        }

                    }
                },
                Err(parse_error) => println!("{}", parse_error)
            }
        }
        Err(lexing_error) => println!("{}", lexing_error)
    }
}