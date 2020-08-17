//! # stdrename
//! `stdrename` is a small command line utility to rename all
//! files in a folder according to a specified naming convention
//! (camelCase, snake_case, kebab-case, etc.). 
//! 
//! See <https://github.com/Gadiguibou/stdrename> for the full documentation.

use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::*;

use clap::{App, Arg, ArgGroup};

use ignore::WalkBuilder;

use inflector::Inflector;

pub struct Config {
    target_dir: PathBuf,
    naming_convention: String,
    recursive: bool,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let matches = App::new("stdrename")
        .version("v1.0.0")
        .author("Gabriel Lacroix <lacroixgabriel@gmail.com>")
        .about("This small utility is designed to rename all files in a folder according to a specified naming convention (camelCase, snake_case, kebab-case, etc.).")
        .arg(
            Arg::with_name("TARGET")
                .help("Specifies a different target directory")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("recursive")
                .help("Makes renaming recursive, renaming files in subfolders as well")
                .short("r")
                .long("recursive"),
        )
        .arg(
            Arg::with_name("camelCase")
                .help("Uses the camelCase naming convention")
                .short("c")
                .long("camel"),
        )
        .arg(
            Arg::with_name("kebab-case")
                .help("Uses the kebab-case naming convention")
                .short("k")
                .long("kebab"),
        )
        .arg(
            Arg::with_name("PascalCase")
                .help("Uses the PascalCase naming convention")
                .short("p")
                .long("pascal"),
        )
        .arg(
            Arg::with_name("SCREAMING_SNAKE_CASE")
                .help("Uses the SCREAMING_SNAKE_CASE naming convention")
                .long("screaming"),
        )
        .arg(
            Arg::with_name("Sentence case")
                .help("Uses the Sentence case naming convention")
                .short("S")
                .long("sentence"),
        )
        .arg(
            Arg::with_name("snake_case")
                .help("Uses the snake_case naming convention")
                .short("s")
                .long("snake"),
        )
        .arg(
            Arg::with_name("Title Case")
                .help("Uses the Title Case naming convention")
                .short("T")
                .long("title"),
        )
        .arg(
            Arg::with_name("Train-Case")
                .help("Uses the Train-Case naming convention")
                .short("t")
                .long("train"),
        )
        .group(
            ArgGroup::with_name("convention")
                .required(true)
                .args(&["camelCase","kebab-case","PascalCase","SCREAMING_SNAKE_CASE","Sentence case","snake_case","Title Case","Train-Case"]),
        )
        .get_matches();

        let target_dir = match matches.value_of("TARGET") {
            Some(dir) => PathBuf::from(dir),
            None => env::current_dir()?,
        };

        let naming_convention = {
            let (camel, kebab, pascal, screaming, sentence, snake, title, train) = (
                matches.is_present("camelCase"),
                matches.is_present("kebab-case"),
                matches.is_present("PascalCase"),
                matches.is_present("SCREAMING_SNAKE_CASE"),
                matches.is_present("Sentence case"),
                matches.is_present("snake_case"),
                matches.is_present("Title Case"),
                matches.is_present("Train-Case"),
            );
    
            if camel {
                "camelCase"
            } else if kebab {
                "kebab-case"
            } else if pascal {
                "PascalCase"
            } else if screaming {
                "SCREAMING_SNAKE_CASE"
            } else if sentence {
                "Sentence_case"
            } else if snake {
                "snake_case"
            } else if title {
                "Title_Case"
            } else if train {
                "Train-Case"
            } else {
                unreachable!()
            }
        }.to_owned();

        let recursive = matches.is_present("recursive");
         
        Ok(Config {
            target_dir,
            naming_convention,
            recursive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for entry in WalkBuilder::new(&config.target_dir)
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
