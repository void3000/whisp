pub fn print_welcome() {
    let version = env!("CARGO_PKG_VERSION");

    println!("Whisp {} | Interactive interpreter", version);
    println!("Type \"help\", \"copyright\", \"credits\" or \"license\" for more information.");
}
