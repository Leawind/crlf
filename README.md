# CRLF

A simple command line tool to convert line endings in files to `CRLF`, `LF` or `CR`.

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
