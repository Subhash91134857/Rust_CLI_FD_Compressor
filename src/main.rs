// Declare the 'Compression' module, which contains the compression logic
mod compression;

// Import necessary modules and types
use clap::{Arg, Command};
use std::fs;
use std::path::Path; // For handling file paths

// Import the 'compress_file_or_directory' function and 'Result' type from the 'compression' module

use crate::compression::compress_file_or_directory;
use crate::compression::error::Result;

fn main() -> Result<()> {
    // Define the command-line interface using 'clap'
    let matches = Command::new("File Compressor")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Compresses files and directories")
        .arg(
            Arg::new("source")
                .short('s')
                .long("source")
                .value_name("SOURCE")
                .help("Sets the source file or directory to compress")
                .required(true),
        )
        .arg(
            Arg::new("destination")
                .short('d')
                .long("destination")
                .value_name("DESTINATION")
                .help("Sets the destination path for the compressed file")
                .required(true),
        )
        .get_matches();
    // Get the source path from the parsed arguments.
    let src = Path::new(matches.get_one::<String>("source").unwrap());

    // Get the destination path from the parsed arguments.
    let dst = Path::new(matches.get_one::<String>("destination").unwrap());

    // Checking permission for source and destination paths
    println!("Checking permissions for source and destination paths...");

    if let Err(e) = fs::metadata(src) {
        eprintln!("Error accessing source path {}: {}", src.display(), e);
        return Err(e.into());
    }
    if let Err(e) = fs::metadata(dst.parent().unwrap_or_else(|| Path::new("."))) {
        eprintln!("Error accessing destination path {}: {}", dst.display(), e);
        return Err(e.into());
    }

    // Compress the file or directory.
    compress_file_or_directory(src, dst)?;

    // Print a success message.
    println!(
        "Compression successful: {} -> {}",
        src.display(),
        dst.display()
    );

    Ok(())
}
