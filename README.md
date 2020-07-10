# stdrename

This small utility is designed to rename all files in a folder according to a specified naming convention (camelCase, snake_case, kebab-case, etc.).

It currently takes the following naming conventions as arguments:

| Arguments              | Examples                   |
| ---------------------- | -------------------------- |
| `camelCase`            | "camelCase.txt"            |
| `kebab-case`           | "kebab-case.txt"           |
| `PascalCase`           | "PascalCase.txt"           |
| `SCREAMING_SNAKE_CASE` | "SCREAMING_SNAKE_CASE.txt" |
| `Sentence_case`        | "Sentence case.txt"        |
| `snake_case`           | "snake_case.txt"           |
| `Title_Case`           | "Title Case.txt"           |
| `Train-Case`           | "Train-Case.txt"           |

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

## Usage

On Linux you may need to make the shared library file executable with:

```bash
chmod +x stdrename
```

Then, just type the path to the file:

```bash
./stdrename
```

By default, the program will rename all files in the current directory with the default convention (kebab-case).

You can specify a different naming convention with the first argument you pass to the program e.g.:

```bash
./stdrename snake_case
```

You can also specify another folder to parse with a second argument e.g.:

```bash
./stdrename camelCase ~/Pictures
```
