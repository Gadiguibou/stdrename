use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::*;

use ignore::WalkBuilder;

use inflector::Inflector;

pub struct Config {
    current_dir: PathBuf,
    naming_convention: String,
    recursive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, Box<dyn Error>> {
        args.next();

        let naming_convention = match args.next() {
            Some(arg) => arg,
            None => "kebab-case".to_owned(),
        };

        let current_dir = match args.next() {
            Some(arg) => PathBuf::from(arg),
            None => env::current_dir()?,
        };

        let recursive = match args.next() {
            Some(arg) => arg == "-r",
            None => false,
        };

        Ok(Config {
            current_dir,
            naming_convention,
            recursive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for entry in WalkBuilder::new(&config.current_dir)
        .max_depth(if !config.recursive { Some(1) } else { None })
        .git_exclude(false)
        .git_global(false)
        .git_ignore(false)
        .build()
    {
        let entry = entry?;

        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let new_name = change_naming_convention(&path, &config.naming_convention)?;
        let new_path = path
            .parent()
            .ok_or("can't find path parent")?
            .join(new_name);

        fs::rename(&path, &new_path)?;
    }
    Ok(())
}

pub fn change_naming_convention(
    path_to_file: &Path,
    new_naming_convention: &str,
) -> Result<String, Box<dyn Error>> {
    let file_stem = path_to_file
        .file_stem()
        .unwrap_or_else(|| OsStr::new(""))
        .to_str()
        .ok_or_else(|| {
            format!(
                "couldn't convert file stem of {:?} to valid Unicode",
                path_to_file
            )
        })?;

    let file_extension = path_to_file
        .extension()
        .unwrap_or_else(|| OsStr::new(""))
        .to_str()
        .ok_or_else(|| {
            format!(
                "couldn't convert file extension of {:?} to valid Unicode",
                path_to_file
            )
        })?;

    let file_stem = match new_naming_convention {
        "camelCase" => file_stem.to_camel_case(),
        "kebab-case" => file_stem.to_kebab_case(),
        "PascalCase" => file_stem.to_pascal_case(),
        "SCREAMING_SNAKE_CASE" => file_stem.to_screaming_snake_case(),
        "Sentence_case" => file_stem.to_sentence_case(),
        "snake_case" => file_stem.to_snake_case(),
        "Title_Case" => file_stem.to_title_case(),
        "Train-Case" => file_stem.to_train_case(),
        _ => return Err(From::from("naming convention not found")),
    };

    if file_stem.is_empty() {
        Ok(format!(".{}", file_extension))
    } else if file_extension.is_empty() {
        Ok(file_stem)
    } else {
        Ok(format!("{}.{}", file_stem, file_extension))
    }
}
