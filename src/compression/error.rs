use std::io::Error as IoError;

use thiserror::Error; // Importing the "thiserror" crate for defining custom error types.

// Define an enum to represent different types of error that can occur during compression.
#[derive(Error, Debug)]
pub enum CompressionError {
    // Represents an IO error, using `std::io::Error`.
    #[error("IO error occurred: {0}")]
    Io(#[from] IoError),

    // Represents an error related to tar operations, using `tar::Error`.
    #[error("Tar error occurred: {0}")]
    Tar(String),
}

// Define a type alias for `Result` that uses our custom `CompressionError`.
pub type Result<T> = std::result::Result<T, CompressionError>;
