use std::process;
use stdrename::Config;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match stdrename::run(config) {
        Err(e) => {
            eprintln!("Application error: {}", e);

            process::exit(1);
        }

        Ok(()) => (),
    }
}
