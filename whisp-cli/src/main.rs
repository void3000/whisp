use rustyline::error::ReadlineError;
use rustyline::{ DefaultEditor, Result };

use whisp_lexer::lexer::{ Lexer, Collection, TokenIterator };
use whisp_parser::parser::ll_parser::{ LLParser, Parser };
use whisp_runtime::runtime::{ evaluator, interpreter, builtin };
use whisp_parser::symbol::SymbolTable;
use whisp_lexer::token::Token;

use whisp_runtime::environment::Environment;

fn print_welcome() {
    let version = env!("CARGO_PKG_VERSION");

    println!("Whisp {} | Interactive interpreter", version);
    println!("Type \"help\" for more information.");
}

fn is_input_complete(input: &str) -> bool {
    let mut balance = 0;
    for c in input.chars() {
        match c {
            '{' | '(' | '[' => balance += 1,
            '}' | ')' | ']' => {
                if balance == 0 {
                    return true;
                }
                balance -= 1;
            }
            _ => {}
        }
    }
    balance == 0
}


/// Disclaimer: THIS IS TEMPOERARY IMPLEMENTATION OF RPEL.
fn main() -> Result<()> {
    print_welcome();

    let mut editor = DefaultEditor::new()?;
    let mut env = Environment::new();
    let mut symbols = SymbolTable::new();

    builtin::register_builtins(&mut env, &mut symbols);

    let mut interpreter = interpreter::Interpreter::new(&mut env);
    let mut buffer = String::new();

    loop {
        let prompt = if buffer.is_empty() { ">>> " } else { ".. " };
        let readline = editor.readline(prompt);

        match readline {
            Ok(line) => {
                let line = line.trim_end();

                buffer.push_str(line);
                buffer.push('\n');

                if is_input_complete(&buffer) {
                    let input = buffer.trim();
                    if !input.is_empty() {
                        let lexer   = Lexer::new(input);
                        let stream  = lexer.stream();
                        let mut parser = LLParser::new(stream, &mut symbols);
                        match parser.parse() {
                            Ok(ast) => {
                                match evaluator::eval(&mut interpreter, &ast) {
                                    Ok(val) => println!("{}", val.to_string()),
                                    Err(err) => eprintln!("error: {}", err),
                                }
                            }
                            Err(e) => {
                                eprintln!("Parse error: {}", e);
                            }
                        }

                    }
                    buffer.clear();
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                buffer.clear();
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
    Ok(())
}
