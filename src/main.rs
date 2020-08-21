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

        Ok((files_renamed, time_elapsed)) => {
            println!(
                "{} files renamed in {} s. See you next time!\n(^ _ ^)/",
                files_renamed, time_elapsed
            );
        }
    }
}
