# CRLF

[![GitHub License](https://img.shields.io/github/license/Leawind/crlf)](https://github.com/Leawind/crlf)
[![Crates.io Version](https://img.shields.io/crates/v/crlf)](https://crates.io/crates/crlf)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Leawind/crlf/test.yml?label=test)](https://github.com/Leawind/crlf/actions/workflows/test.yml)

A simple command line tool to convert line endings in files to `CRLF`, `LF` or `CR`.

## Installation

```bash
cargo install crlf
```

Or install from source code:

```bash
git clone https://github.com/Leawind/crlf.git && cd crlf
cargo install --path .
```

Uninstall:

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
