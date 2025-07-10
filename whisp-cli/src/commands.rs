use whisp::whisp::Whisp;

pub fn dispatch_command(whisp: &mut Whisp, command: &str) -> bool {
    match command {
        "help"      => handle_help(),
        "copyright" => handle_copyright(),
        "license"   => handle_license(),
        "credits"   => handle_credits(),
        "exit" | "quit" => {
            return false;
        }
        other => {
            match whisp.parse(other) {
                Ok(ast) => 
                    match whisp.eval(&ast) {
                        Ok(val) => println!("{}", val.to_string()),
                        Err(err) => eprintln!("error: {}", err),
                },
                Err(e) => eprintln!("Parse error: {}", e)
            }
        }
    }

    true
}

fn handle_help() {
    println!("Whisp Help:");
    println!("- Type \"exit\" or press Ctrl+D to quit.");
    println!("- Use \"copyright\", \"credits\", or \"license\" for more info.");
}

fn handle_copyright() {
    println!("Whisp is Copyright (C) 2025 by Void/3000.");
    println!("Licensed under MIT.");
}

pub fn handle_license() {
    println!("License Information:");
    println!("Whisp is distributed under the MIT License.");
    println!("You are free to use, modify, and distribute this software with proper attribution.");
    println!("See the LICENSE file for full details.");
}

fn handle_credits() {
    println!("Whisp was developed by the Whisp Team.");
}

fn handle_exit() {
    println!("Exiting Whisp...");
}
