use rustyline::error::ReadlineError;
use rustyline::{ 
    DefaultEditor, 
    Result 
};
use whisp::whisp::Whisp;

mod commands;
mod welcome;

use crate::commands::dispatch_command;
use crate::welcome::print_welcome;

/// Disclaimer: THIS IS TEMPOERARY IMPLEMENTATION OF RPEL.
fn main() -> Result<()> {
    let mut editor = DefaultEditor::new()?;
    let mut buffer = String::new();
    let mut whisp = Whisp::new();

    print_welcome();

    loop {
        let readline = read_input_line(&mut editor, &buffer);
        match readline {
            Ok(line) => {
                let line = line.trim_end();

                buffer.push_str(line);
                buffer.push('\n');

                if is_input_complete(&buffer) {
                    let input = buffer.trim();
                    if !input.is_empty() {
                         if !dispatch_command(&mut whisp, &input) {
                            break;
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

fn read_input_line(editor: &mut DefaultEditor, buffer: &str) -> Result<String> {
    let prompt = if buffer.is_empty() { 
        ">>> " 
    } else { 
        "...  " 
    };

    editor.readline(prompt)
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
