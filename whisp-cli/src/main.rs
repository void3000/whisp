use rustyline::error::ReadlineError;
use rustyline::{ 
    DefaultEditor, 
    Result 
};
use whisp::whisp::Whisp;

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
    let mut editor = DefaultEditor::new()?;
    let mut buffer = String::new();
    let mut whisp = Whisp::new();

    print_welcome();

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
                        match whisp.parse(input) {
                            Ok(ast) => {
                                match whisp.eval(&ast) {
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
