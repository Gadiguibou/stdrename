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
use std::io;
use std::path::*;
use std::time::Instant;

use clap::{App, AppSettings, Arg, ArgGroup};

use ignore::WalkBuilder;

use inflector::Inflector;

pub struct Config {
    target_dir: PathBuf,
    naming_convention: String,
    recursive: bool,
    include_dir: bool,
    quiet: bool,
    text: bool,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let matches = App::new("stdrename")
        .version("v1.3.0")
        .author("Gabriel Lacroix <lacroixgabriel@gmail.com>")
        .about("This small utility is designed to rename all files in a folder according to a specified naming convention (camelCase, snake_case, kebab-case, etc.).")
        .usage("stdrename [FLAGS] <convention> [TARGET]")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("TARGET")
                .help("Specifies a different target directory")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("camelCase")
                .help("Uses the camelCase naming convention")
                .short('c')
                .long("camel"),
        )
        .arg(
            Arg::with_name("kebab-case")
                .help("Uses the kebab-case naming convention")
                .short('k')
                .long("kebab"),
        )
        .arg(
            Arg::with_name("lower case")
                .help("Returns the lowercase equivalent of the input name")
                .short('l')
                .long("lower")
        )
        .arg(
            Arg::with_name("PascalCase")
                .help("Uses the PascalCase naming convention")
                .short('p')
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
                .short('S')
                .long("sentence"),
        )
        .arg(
            Arg::with_name("snake_case")
                .help("Uses the snake_case naming convention")
                .short('s')
                .long("snake"),
        )
        .arg(
            Arg::with_name("Title Case")
                .help("Uses the Title Case naming convention")
                .short('T')
                .long("title"),
        )
        .arg(
            Arg::with_name("Train-Case")
                .help("Uses the Train-Case naming convention")
                .short('t')
                .long("train"),
        )
        .group(
            ArgGroup::with_name("convention")
                .required(true)
                .args(&["camelCase","kebab-case","lower case","PascalCase","SCREAMING_SNAKE_CASE","Sentence case","snake_case","Title Case","Train-Case"]),
        )
        .arg(
            Arg::with_name("recursive")
                .help("Makes renaming recursive, renaming files in subfolders as well")
                .short('r')
                .long("recursive"),
        )
        .arg(
            Arg::with_name("directories")
                .help("Renames directories as well")
                .short('D')
                .long("dir")
        )
        .arg(
            Arg::with_name("text")
                .help("Reads lines from stdin and translates them to the given convention in stdout until the first empty line")
                .long("text")
        )
        .arg(
            Arg::with_name("quiet")
                .help("Suppress output")
                .short('q')
                .long("quiet")
        )
        .after_help("Full documentation available here: https://github.com/Gadiguibou/stdrename")
        .get_matches();

        let target_dir = match matches.value_of("TARGET") {
            Some(dir) => PathBuf::from(dir),
            None => env::current_dir()?,
        };

        let naming_convention = {
            if matches.is_present("camelCase") {
                "camelCase"
            } else if matches.is_present("kebab-case") {
                "kebab-case"
            } else if matches.is_present("lower case") {
                "lower case"
            } else if matches.is_present("PascalCase") {
                "PascalCase"
            } else if matches.is_present("SCREAMING_SNAKE_CASE") {
                "SCREAMING_SNAKE_CASE"
            } else if matches.is_present("Sentence case") {
                "Sentence case"
            } else if matches.is_present("snake_case") {
                "snake_case"
            } else if matches.is_present("Title Case") {
                "Title Case"
            } else if matches.is_present("Train-Case") {
                "Train-Case"
            } else {
                unreachable!()
            }
        }
        .to_owned();

        let recursive = matches.is_present("recursive");
        let include_dir = matches.is_present("directories");
        let quiet = matches.is_present("quiet");
        let text = matches.is_present("text");

        Ok(Config {
            target_dir,
            naming_convention,
            recursive,
            include_dir,
            quiet,
            text,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    let mut files_renamed: u64 = 0;

    // If the text flag is specified, read from stdin and translate to stdout instead of renaming files
    if config.text {
        let stdin = io::stdin();

        loop {
            let mut input = String::new();

            let len = stdin.read_line(&mut input)?;

            if len == 0 || input.trim().is_empty() {
                let running_time: f32 = start_time.elapsed().as_micros() as f32 / 1_000_000f32;

                if !config.quiet {
                    println!(
                        "{} names translated in {} s. See you next time!\n(^ _ ^)/",
                        files_renamed, running_time
                    )
                };

                return Ok(());
            } else {
                let translation = change_naming_convention(
                    &PathBuf::from(input.trim()),
                    &config.naming_convention,
                )?;
                println!("{}", translation);
                files_renamed += 1;
            }
        }
    }

    let mut walk_builder = WalkBuilder::new(&config.target_dir);

    walk_builder
        .max_depth(if !config.recursive { Some(1) } else { None })
        .require_git(true);

    // Parse different locations for a global config file depending on OS family
    // On unix systems (MacOS or Linux), searches for ~/.config/stdrename/ignore
    // On windows systems searches for %USERPROFILE%\AppData\Local\stdrename\ignore
    // Outputs errors to stderr if there's one but doesn't stop program execution
    #[cfg(unix)]
    if let Some(home_path) = env::var_os("HOME") {
        let config_location = format!("{}/.config/stdrename/ignore", home_path.to_string_lossy());
        if PathBuf::from(&config_location).is_file() {
            if let Some(e) = walk_builder.add_ignore(Path::new(&config_location)) {
                eprintln!("Error parsing global config file: {}", e);
            }
        }
    }
    #[cfg(windows)]
    if let Some(user_profile) = env::var_os("USERPROFILE") {
        let config_location = format!(
            "{}\\AppData\\Local\\stdrename\\ignore",
            user_profile.to_string_lossy()
        );
        if PathBuf::from(&config_location).is_file() {
            if let Some(e) = walk_builder.add_ignore(Path::new(&config_location)) {
                eprintln!("Error parsing global config file: {}", e);
            }
        }
    }

    for entry in walk_builder.build() {
        let entry = entry?;

        let path = entry.path();

        // Skips any entry that isn't a file if the "-D" flag is not specified.
        // Always skips the target directory to prevent changing paths that the program will try to access.
        // (and because it would be quite unexpected as well)
        if !config.include_dir && !path.is_file() || path.eq(&config.target_dir) {
            continue;
        }

        let new_name = change_naming_convention(&path, &config.naming_convention)?;
        let new_path = path
            .parent()
            .ok_or("can't find path parent")?
            .join(new_name);
        if path != new_path {
            fs::rename(&path, &new_path)?;
            files_renamed += 1;
        }
    }
    let running_time: f32 = start_time.elapsed().as_micros() as f32 / 1_000_000f32;

    if !config.quiet {
        println!(
            "{} files renamed in {} s. See you next time!\n(^ _ ^)/",
            files_renamed, running_time
        )
    };

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
        "lower case" => file_stem.to_lowercase(),
        "PascalCase" => file_stem.to_pascal_case(),
        "SCREAMING_SNAKE_CASE" => file_stem.to_screaming_snake_case(),
        "Sentence case" => file_stem.to_sentence_case(),
        "snake_case" => file_stem.to_snake_case(),
        "Title Case" => file_stem.to_title_case(),
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
