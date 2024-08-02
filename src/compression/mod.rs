// Declare the 'error' module within the compression module

pub mod error;

// import necessary modules and types
use flate2::write::GzEncoder; // For compressing data using gzip
use flate2::Compression; // For specifying compression levels
use std::fs::File; // For file operation
use std::io; // For IO operations.
use std::path::Path; // For handling file paths.
use tar::Builder; // For creating tar archives

//import custom error types and 'Result' from the 'error' module
use crate::compression::error::{CompressionError, Result};

// Function to compress a file or directory
pub fn compress_file_or_directory(src: &Path, dst: &Path) -> Result<()> {
    // Create a new file to store the compressed data;
    let tar_gz = File::create(dst)?;

    // Create a gzip encoder with the default compression level
    let enc = GzEncoder::new(tar_gz, Compression::default());

    // Create a tar builder to write to the gzip encoder
    let mut tar = Builder::new(enc);

    // Check if the source path is file
    if src.is_file() {
        // Add the file to the tar archive;
        tar.append_path(src)
            .map_err(|e| CompressionError::Tar(e.to_string()))?;
    } else if src.is_dir() {
        // If it's a directory, add all files and subdirectories to the tar archive.
        tar.append_dir_all(src, src)
            .map_err(|e| CompressionError::Tar(e.to_string()))?;
    } else {
        // If the source path is neither a file nor a directory, return an error;
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid source path ").into());
    }

    // FInish writing the tar archive and compress it
    tar.finish()
        .map_err(|e| CompressionError::Tar(e.to_string()))?;
    Ok(())
}
