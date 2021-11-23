# stdrename

[![Build Status](https://travis-ci.com/Gadiguibou/stdrename.svg?branch=master)](https://travis-ci.com/Gadiguibou/stdrename)

This small utility is designed to rename all files in a folder according to a specified naming convention (camelCase, snake_case, kebab-case, etc.).

It currently supports the following naming conventions:

- camelCase
- kebab-case
- lower case
- PascalCase
- SCREAMING_SNAKE_CASE
- Sentence case
- snake_case
- Title Case
- Train-Case

![screenshot-of-stdrename](https://user-images.githubusercontent.com/34945306/90803472-c85b3f00-e2e6-11ea-8552-9e14ac306522.png)

## Installation

### Manual

1. Download the [latest binary executable release](https://github.com/Gadiguibou/stdrename/releases) supported by your OS.

    For most Linux distributions, download `stdrename-x86_64-unknown-linux-gnu`.
    
    For Windows, download `stdrename.exe`.
    
    No macOS binary is available yet, but `stdrename` can still be installed using [Cargo](#cargo).

2. Add it to your `PATH`.

    On Linux and MacOS, this is achieved by executing the following command in your terminal after downloading the file.

    ```bash
    chmod +x ~/Downloads/stdrename && sudo mv ~/Downloads/stdrename /usr/local/bin
    ```

    On Windows, you can move the file to a new folder such as "C:\Users\\*YourName*\\bin" and then add the folder to your `PATH` using the instructions [here](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/).

### Cargo

If you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed you can install [`stdrename`](https://crates.io/crates/stdrename) with:

```bash
cargo install stdrename
```

The executable file can then be found in `$HOME/.cargo/bin/stdrename`.

On Linux and MacOS to make sure `$HOME/.cargo/bin` is in your `$PATH`  add the following line at the end of  `.bashrc` (in your home directory):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Usage

You must specify the naming convention you want to use with the appropriate flag. For example, this will rename all files in the current directory using the kebab-case naming convention.

```bash
stdrename -k
```

Here is the full list of naming convention flags that can be used:

| Short | Long          | Example                    |
| ----- | ------------- | -------------------------- |
| `-c`  | `--camel`     | `camelCase.txt`            |
| `-k`  | `--kebab`     | `kebab-case.txt`           |
| `-l`  | `--lower`     | `lower case.txt`           |
| `-p`  | `--pascal`    | `PascalCase.txt`           |
|       | `--screaming` | `SCREAMING_SNAKE_CASE.txt` |
| `-S`  | `--sentence`  | `Sentence case.txt`        |
| `-s`  | `--snake`     | `snake_case.txt`           |
| `-T`  | `--title`     | `Title Case.txt`           |
| `-t`  | `--train`     | `Train-Case.txt`           |

### Specifying a different folder to parse

You can also specify a different folder to parse with a second argument e.g.:

```bash
stdrename -k ~/Pictures
```

### Renaming files in subfolders as well

To rename recursively, use the flag `-r` or `--recursive` e.g.:

```bash
stdrename -kr ~/Pictures
```

### Renaming directories as well

To rename directories as well, use the flag `-D` or `--dir` e.g.:

```bash
stdrename -kD ~/Pictures
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

If you'd like to use global ignore patterns specific to stdrename, you can do so by creating an "`ignore`" (notice this one does not start with a ".") in the following location:

On Windows: `%USERPROFILE%\AppData\Local\stdrename\"`

On Unix based systems (e.g. MacOS or GNU Linux):

`$HOME/.config/stdrename/`

This file follows the same pattern matching principles as other `.gitignore` or `.ignore` files and has a lower precedence than all other sources of ignore rules.

### `--text` mode

Text mode allows for either piping through stdin, e.g.:

```bash
echo 'Hello World' | stdrename --text -k
```

or interactive use, e.g.:

```bash
stdrename --text -s
reallyCreativeProgramName.js
really_creative_program_name.js
PYTHONISTA_BANANA.py
pythonista_banana.py
This is really just a normal sentence
this_is_really_just_a_normal_sentence
```

Either way, the program terminates and stops reading as soon as it encounters an empty line.

To suppress the exit message (if piping stdout to another file for example) add the `-q` or `--quiet` flag.
