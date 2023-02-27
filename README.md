# csurename

This small utility is designed to rename all files in a folder so that they
adhere to @csunibo's `kebab-case` naming convention.

![screenshot-of-csurename](https://user-images.githubusercontent.com/34945306/90803472-c85b3f00-e2e6-11ea-8552-9e14ac306522.png)

## Installation

### Manual

1. Download the [latest binary executable release](https://github.com/csunibo/csurename/releases) supported by your OS.

   For most Linux distributions, download `csurename-x86_64-unknown-linux-gnu`.

   For Windows, download `csurename.exe`.

2. Add it to your `PATH`.

   On Linux and MacOS, this is achieved by executing the following command in your terminal after downloading the file.

   ```bash
   chmod +x ~/Downloads/csurename && sudo mv ~/Downloads/csurename /usr/local/bin
   ```

   On Windows, you can move the file to a new folder such as "C:\Users\\_YourName_\\bin" and then add the folder to your `PATH` using the instructions [here](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/).

## Usage

### Specifying a different folder to parse

You can also specify a different folder to parse with a second argument e.g.:

```bash
csurename ~/Pictures
```

### Renaming files in subfolders as well

To rename recursively, use the flag `-r` or `--recursive` e.g.:

```bash
csurename -kr ~/Pictures
```

### Renaming directories as well

To rename directories as well, use the flag `-D` or `--dir` e.g.:

```bash
csurename -D ~/Pictures
```

### Ignoring files and subdirectories

By default, patterns in `.gitignore` files, global gitignore files and git exclude files will be ignored as well if the directory is a git repository.

You may also add a `.ignore` file with patterns to ignore in the file's directory and its subdirectories.

This file may use any of the glob patterns that can be used in a `.gitignore` file. It is functionally the same, just with a different name e.g.:

Adding the following line in a new .ignore file in the same directory as csurename will ignore all files with the extension `.py` and all files in the subdirectory `./target/` when renaming.

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

If you'd like to use global ignore patterns specific to csurename, you can do so by creating an "`ignore`" (notice this one does not start with a ".") in the following location:

On Windows: `%USERPROFILE%\AppData\Local\csurename\"`

On Unix based systems (e.g. MacOS or GNU Linux):

`$HOME/.config/csurename/`

This file follows the same pattern matching principles as other `.gitignore` or `.ignore` files and has a lower precedence than all other sources of ignore rules.

### `--text` mode

Text mode allows for either piping through stdin, e.g.:

```bash
echo 'Hello World' | csurename --text
```

or interactive use, e.g.:

```bash
csurename --text
reallyCreativeProgramName.js
really_creative_program_name.js
PYTHONISTA_BANANA.py
pythonista_banana.py
This is really just a normal sentence
this_is_really_just_a_normal_sentence
```

Either way, the program terminates and stops reading as soon as it encounters an empty line.

To suppress the exit message (if piping stdout to another file for example) add the `-q` or `--quiet` flag.
