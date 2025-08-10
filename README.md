# CRLF

[![Crates.io Version](https://img.shields.io/crates/v/crlf)](https://crates.io/crates/crlf)

A simple command line tool to convert line endings in files to `CRLF`, `LF` or `CR`.

## Installation

- Install from crates.io:
  ```bash
  cargo install crlf
  ```
- Or install from source:
  ```bash
  cargo install --path .
  ```
- Uninstall:
  ```bash
  cargo uninstall crlf
  ```

## Usage

```txt
crlf <crlf|lf|cr> <FILE_PATTERNS>...
```

### Example

Command:

```bash
crlf lf ./src/**/*.rs
```

Output:

```txt
Processing 1 files...
✓ src\main.rs
All files processed successfully!
```
