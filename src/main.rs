use std::process;
use stdrename::Config;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = stdrename::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
