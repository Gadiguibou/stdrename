use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::*;

use inflector::Inflector;

pub struct Config {
    current_dir: PathBuf,
    naming_convention: String,
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

        Ok(Config {
            current_dir,
            naming_convention,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(&config.current_dir)?;

    for entry in entries {
        let entry = entry?;

        let path = entry.path();

        if path.is_file() {
            let file_name = path
                .file_name()
                .unwrap()
                .to_str()
                .ok_or("failed to parse file name into valid Unicode")?;

            if file_name.starts_with('.') {
                continue;
            };

            let new_name = change_naming_convention(&path, &config.naming_convention)?;
            let new_path = config.current_dir.join(new_name);

            fs::rename(&path, &new_path)?;
        }
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
