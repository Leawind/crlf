use clap::Parser;
use glob::glob;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use strum::Display;

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

#[derive(clap::ValueEnum, Clone, Debug, Display)]
enum LineEnding {
    CRLF,
    CR,
    LF,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    // Collect all files matching the patterns
    let mut files = vec![];
    for pattern in &args.file_patterns {
        for entry in glob(pattern)? {
            match entry {
                Ok(path) => {
                    if path.is_file() {
                        files.push(path);
                    }
                }
                Err(e) => eprintln!("Glob error: {}", e),
            }
        }
    }

    if files.is_empty() {
        println!("No files found matching the patterns");
        return Ok(());
    }

    println!("Processing {} files...", files.len());

    let results: Vec<_> = files
        .par_iter()
        .map(|file_path| {
            let result = process_file(file_path, &args.format);
            match &result {
                Ok(_) => println!("✓ {}", file_path.display()),
                Err(e) => eprintln!("✗ {} - Error: {}", file_path.display(), e),
            }
            result
        })
        .collect();

    let errors: Vec<_> = results.into_iter().filter_map(|r| r.err()).collect();
    if !errors.is_empty() {
        eprintln!("\n{} files failed to process", errors.len());
        std::process::exit(1);
    }

    println!("All files processed successfully!");
    Ok(())
}

fn process_file(
    file_path: &Path,
    target: &LineEnding,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let content = fs::read_to_string(file_path)?;
    let converted = convert_line_endings(&content, target);
    fs::write(file_path, converted.as_bytes())?;
    Ok(())
}

fn convert_line_endings(content: &str, target: &LineEnding) -> String {
    match target {
        LineEnding::CRLF => content
            .replace("\r\n", "\n")
            .replace("\r", "\n")
            .replace("\n", "\r\n"),
        LineEnding::CR => content.replace("\r\n", "\r").replace("\n", "\r"),
        LineEnding::LF => content.replace("\r\n", "\n").replace("\r", "\n"),
    }
}
