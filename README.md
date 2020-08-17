# stdrename

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

## Installation

Download the released file supported by your OS (shared library for Linux and .exe for Windows).

### Or

Build it yourself using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) with:

```bash
git clone https://github.com/Gadiguibou/stdrename.git
```

and

```bash
cargo build --release
```

The executable file can then be found at `stdrename/target/release/`.

## Usage

On Linux you may need to make the shared library file executable with:

```bash
chmod +x stdrename
```

Then, just type the path to the file with the flag for the naming convention of your choice. For example, this will rename all files in the current directory using the kebab-case naming convention.

```bash
./stdrename -k
```

The full list of flags that can be used:

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
./stdrename -k -r ./new-dir/
```

### Ignoring files and subdirectories

You may add a `.ignore` file with patterns to ignore in the file's directory and its subdirectories.

This file may use any of the glob patterns that can be used in a `.gitignore` file. It is functionally the same, just with a different name e.g.:

Adding the following line in a new .ignore file in the same directory as stdrename will ignore all files with the extension `.py` and all files in the subdirectory `./target/`.

```ignore
# ./.ignore
# ignore all files ending with .py
# ignore all files in /target and its subdirectories

*.py
/target
```

You may even add a second .ignore file in a subdirectory e.g.:

```ignore
# ./subdir1/.ignore
# match .py files despite previous instructions
# ignore .txt files in this directory and all subdirectories

!*.py
*.txt
```

All files in that directory and all sub directories will then reinclude .py files and ignore .txt files.
