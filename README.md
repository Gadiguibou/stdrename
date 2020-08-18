# stdrename

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FGadiguibou%2Fstdrename.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FGadiguibou%2Fstdrename?ref=badge_shield)

This small utility is designed to rename all files in a folder according to a specified naming convention (camelCase, snake_case, kebab-case, etc.).

It currently supports the following naming conventions:

- camelCase
- kebab-case
- PascalCase
- SCREAMING_SNAKE_CASE
- Sentence case
- snake_case
- Title Case
- Train-Case

![screenshot-of-stdrename](https://user-images.githubusercontent.com/34945306/90450368-8e582600-e0b7-11ea-8fe8-628c07005c3b.png)

## Installation

### Manual

Download the [latest released file](https://github.com/Gadiguibou/stdrename/releases) supported by your OS (shared library for Linux and .exe for Windows) and add it to your `$PATH`.

On Linux you may need to make the shared library file executable with:

```bash
chmod +x stdrename
```

### Cargo

Install [`stdrename`](https://crates.io/crates/stdrename) using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) with:

```bash
cargo install stdrename
```

The executable file can then be found in `$HOME/.cargo/bin/stdrename`.

To make sure `$HOME/.cargo/bin` is in your `$PATH`  use:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Usage

You must specify the naming convention you want to use with the appropriate flag. For example, this will rename all files in the current directory using the kebab-case naming convention.

```bash
stdrename -k
```

Here is the full list of flags that can be used:

| Short | Long          | Example                    |
| ----- | ------------- | -------------------------- |
| `-c`  | `--camel`     | `camelCase.txt`            |
| `-k`  | `--kebab`     | `kebab-case.txt`           |
| `-p`  | `--pascal`    | `PascalCase.txt`           |
|       | `--screaming` | `SCREAMING_SNAKE_CASE.txt` |
| `-S`  | `--sentence`  | `Sentence case.txt`        |
| `-s`  | `--snake`     | `snake_case.txt`           |
| `-T`  | `--title`     | `Title Case.txt`           |
| `-t`  | `--train`     | `Train-Case.txt`           |

### Specifying a different folder to parse

You can also specify a different folder to parse with a second argument e.g.:

```bash
./stdrename -k ~/Pictures
```

### Renaming files in subfolders as well

To rename recursively, use the flag `-r` e.g.:

```bash
./stdrename -k -r ~/Pictures
```

### Ignoring files and subdirectories

By default, patterns in `.gitignore` files, global gitignore files and git exclude files will be ignored as well if the directory is a git repository.

You may also add a `.ignore` file with patterns to ignore in the file's directory and its subdirectories.

This file may use any of the glob patterns that can be used in a `.gitignore` file. It is functionally the same, just with a different name e.g.:

Adding the following line in a new .ignore file in the same directory as stdrename will ignore all files with the extension `.py` and all files in the subdirectory `./target/` when renaming.

```ignore
# ./.ignore
# ignore all files ending with .py
# ignore all files in /target and its subdirectories

*.py
/target
```

You may even add a second `.ignore` file in a subdirectory e.g.:

```ignore
# ./subdir1/.ignore
# match .py files despite previous instructions
# ignore .txt files in this directory and all subdirectories

!*.py
*.txt
```

All files in that directory and all sub directories will then reinclude .py files and ignore .txt files.

## License

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FGadiguibou%2Fstdrename.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FGadiguibou%2Fstdrename?ref=badge_large)