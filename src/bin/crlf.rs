use clap::Parser;
use crlf::{LineEnding, process_file};
use glob::glob;
use rayon::prelude::*;

/// Convert line endings in files to `CRLF`, `LF` or `CR`.
#[derive(Parser)]
#[command(author, version, about, long_about)]
struct CliArgs {
    /// The target line ending format to convert to.
    #[arg(value_enum)]
    format: LineEnding,

    /// The file patterns to process.
    #[arg(required = true)]
    file_patterns: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    let mut file_paths = vec![];
    for pattern in &args.file_patterns {
        for entry in glob(pattern)? {
            match entry {
                Ok(path) => {
                    if path.is_file() {
                        file_paths.push(path);
                    }
                }
                Err(e) => eprintln!("Glob error: {}", e),
            }
        }
    }

    if file_paths.is_empty() {
        println!("No files found matching the patterns");
        return Ok(());
    }

    println!("Processing {} files...", file_paths.len());

    let errors_count = file_paths
        .par_iter()
        .map(|file_path| {
            let result = process_file(file_path, args.format);
            match &result {
                Ok(()) => println!("✓ {}", file_path.display()),
                Err(err) => eprintln!("✗ {} - Error: {}", file_path.display(), err),
            }
            result
        })
        .filter(|x| x.is_err())
        .count();

    if errors_count == 0 {
        println!("All files processed successfully!");
        Ok(())
    } else {
        eprintln!("\n{} files failed to process", errors_count);
        std::process::exit(1);
    }
}
