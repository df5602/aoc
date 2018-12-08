//! Helper module that allows to read input from a file and into a user-specified destination.
//!
//! For most implementations, the input is expected to consist of a list of values of the same type separated by newlines.
//!
//! # Examples
//! ```no_run
//! use util::input::{FileReader, FromFile};
//!
//! let strings: Vec<String> = FileReader::read_from_file("string_input.txt").unwrap();
//! let doubles: Vec<f64> = FileReader::read_from_file("double_input.txt").unwrap();
//! ```

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

/// Generic trait to read from file and into a destination of type `T`.
pub trait FromFile<T> {
    /// The error type
    type Error;

    /// Takes a file path and tries to read the file content into a destination of type `T`.
    fn read_from_file<P: AsRef<Path>>(path: P) -> Result<T, Self::Error>;
}

#[derive(Debug)]
/// Generic error type that is returned by `FileReader` if it fails to read the input from file.
pub enum Error<E> {
    /// Returned if the specified file cannot be opened or read (e.g. invalid UTF-8).
    IoError(std::io::Error),
    /// Returned if the input cannot be parsed into the specified data type.
    ParseError(E),
}

impl<E: std::fmt::Display> std::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "{}", e),
            Error::ParseError(e) => write!(f, "{}", e),
        }
    }
}

impl<E> From<std::io::Error> for Error<E> {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

/// Read input from file.
pub struct FileReader;

/// Read input into a `Vec<T>`. Input is assumed to be a list of values that can be parsed into `T`
/// that are separated by newlines.
impl<T> FromFile<Vec<T>> for FileReader
where
    T: std::str::FromStr,
{
    type Error = Error<<T as std::str::FromStr>::Err>;

    /// Takes a file path and tries to read the file content into a destination of type `T`.
    ///
    /// # Failures
    /// Returns an error if the specified file cannot be opened or contains invalid UTF-8.
    /// Also returns an error if the file contents cannot be parsed into values of type `T`.
    fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<T>, Self::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| line?.trim().parse().map_err(Error::ParseError))
            .collect()
    }
}

/// Read input into a `String`.
impl FromFile<String> for FileReader {
    type Error = std::io::Error;

    /// Takes a file path and tries to read the file content into a `String`.
    ///
    /// # Failures
    /// Returns an error if the specified file cannot be opened or contains invalid UTF-8.
    fn read_from_file<P: AsRef<Path>>(path: P) -> Result<String, Self::Error> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}
