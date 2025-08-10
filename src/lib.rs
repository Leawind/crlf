#[cfg(test)]
mod test;

use std::fs;
use std::path::Path;
use strum::Display;

#[derive(clap::ValueEnum, Clone, Debug, Display, Copy)]
pub enum LineEnding {
    CRLF,
    CR,
    LF,
}

pub trait CRLF {
    fn crlf(&self, line_ending: LineEnding) -> String;
}

impl CRLF for &str {
    fn crlf(&self, line_ending: LineEnding) -> String {
        match line_ending {
            LineEnding::CRLF => self
                .replace("\r\n", "\n")
                .replace("\r", "\n")
                .replace("\n", "\r\n"),
            LineEnding::CR => self.replace("\r\n", "\r").replace("\n", "\r"),
            LineEnding::LF => self.replace("\r\n", "\n").replace("\r", "\n"),
        }
    }
}

impl CRLF for String {
    fn crlf(&self, line_ending: LineEnding) -> String {
        self.as_str().crlf(line_ending)
    }
}

pub fn process_file(file_path: &Path, line_ending: LineEnding) -> Result<(), std::io::Error> {
    fs::write(
        file_path,
        fs::read_to_string(file_path)?.crlf(line_ending).as_bytes(),
    )
}
