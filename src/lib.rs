#![warn(clippy::all, rust_2018_idioms)]

mod app;

pub use app::Into1TextFileApp;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn list_files(dir_path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                println!("File: {}", path.display());
                files.push(path);
            } else if path.is_dir() {
                list_files(&path)?;
            }
        }
    }
    Ok(files)
}

/// Processes a list of files, concatenating their contents with headers.
/// Ignores files that don't contain valid UTF-8 text.
///
/// # Arguments
///
/// * `file_paths` - A vector of PathBuf representing the files to process
///
/// # Returns
///
/// * `io::Result<String>` - The concatenated contents of all valid files
pub fn process_files(file_paths: Vec<PathBuf>) -> io::Result<String> {
    let mut combined_content = String::new();

    for path in file_paths {
        match process_single_file(&path) {
            Ok(Some(content)) => combined_content.push_str(&content),
            Ok(None) => println!("Skipping file {:?}: Not valid UTF-8", path),
            Err(e) => println!("Error processing file {:?}: {}", path, e),
        }
    }

    Ok(combined_content)
}

/// Processes a single file, returning its content if it's valid UTF-8.
fn process_single_file(path: &PathBuf) -> io::Result<Option<String>> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown");

    let mut content = String::new();
    let mut reader = BufReader::new(File::open(path)?);

    // Read the file content
    reader.read_to_string(&mut content)?;

    // If we've successfully read the content, it's valid UTF-8
    let separator = "=".repeat(file_name.len());
    let header = format!("{}\n{}\n{}\n", separator, file_name, separator);

    Ok(Some(format!("{}{}\n", header, content)))
}
