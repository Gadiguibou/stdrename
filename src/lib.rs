use std::error::Error;
use std::fs;

pub struct Config {
    current_dir: String,
    naming_convention: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let current_dir = args[0].clone();
        let naming_convention = args[1].clone();

        Ok(Config { current_dir, naming_convention })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
