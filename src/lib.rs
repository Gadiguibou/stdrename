use std::error::Error;
use std::fs;
use std::path::Path;

use inflector::Inflector;

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

        Ok(Config {
            current_dir,
            naming_convention,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn change_naming_convention(
    path_to_file: &Path,
    new_naming_convention: &str,
) -> Result<String, &'static str> {

    let file_stem = match path_to_file.file_stem() {
        Some(s_opt) => match s_opt.to_str() {
            Some(s) => s,
            None => return Err("couldn't convert file stem to valid Unicode"),
        },
        None => "",
    };

    let file_extension = match path_to_file.extension() {
        Some(extension_opt) => match extension_opt.to_str() {
            Some(extension) => extension,
            None => return Err("couldn't convert file extension to valid Unicode"),
        },
        None => "",
    };

    match new_naming_convention {
        "snake_case" => Ok([file_stem.to_snake_case(), file_extension.to_string()].join(".")),
        _ => Err("naming convention not found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doesnt_rename_files_with_no_breaks() {
        assert_eq!(
            Ok("file.txt".to_string()),
            change_naming_convention(Path::new("file.txt"), "snake_case")
        );
    }

    #[test]
    fn renames_files_to_snake_case() {
        assert_eq!(
            Ok("i_like_to_fly.txt".to_string()),
            change_naming_convention(Path::new("iLikeToFly.txt"), "snake_case")
        )
    }
}
