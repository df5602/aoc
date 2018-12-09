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

use regex::Regex;

/// Generic trait to read from file and into a destination of type `T`.
pub trait FromFile<T> {
    /// The error type
    type Error;

    /// Takes a file path and tries to read the file content into a destination of type `T`.
    fn read_from_file<P: AsRef<Path>>(&self, path: P) -> Result<T, Self::Error>;
}

#[derive(Debug)]
/// Generic error type that is returned by `FileReader` if it fails to read the input from file.
pub enum Error<E> {
    /// Returned if the specified file cannot be opened or read (e.g. invalid UTF-8).
    IoError(std::io::Error),
    /// Returned if the input cannot be parsed into the specified data type.
    ParseError(E),
    /// Returned if the input doesn't correspond to the expected format.
    FormatError(String),
}

impl<E: std::fmt::Display> std::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "{}", e),
            Error::ParseError(e) => write!(f, "{}", e),
            Error::FormatError(s) => write!(f, "{}", s),
        }
    }
}

impl<E> From<std::io::Error> for Error<E> {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

/// Read input from file.
pub struct FileReader {
    item_separator: char, // TODO: make more generic over type of pattern
    parse_regex: Option<Regex>,
}

#[allow(clippy::new_without_default_derive)]
impl FileReader {
    // TODO: investigate builder pattern...
    pub fn new() -> Self {
        Self {
            item_separator: ',',
            parse_regex: None,
        }
    }

    pub fn separator(self, separator: char) -> Self {
        Self {
            item_separator: separator,
            ..self
        }
    }

    pub fn parse(self, regex: Regex) -> Self {
        Self {
            parse_regex: Some(regex),
            ..self
        }
    }
}

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
    fn read_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<T>, Self::Error> {
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
    fn read_from_file<P: AsRef<Path>>(&self, path: P) -> Result<String, Self::Error> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}

/// Read input into tuple of type `(T, T)`. By default it is assumed that the items are separated by comma.
impl<T> FromFile<(T, T)> for FileReader
where
    T: std::str::FromStr,
{
    type Error = Error<<T as std::str::FromStr>::Err>;

    fn read_from_file<P: AsRef<Path>>(&self, path: P) -> Result<(T, T), Self::Error> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;

        match self.parse_regex {
            Some(ref regex) => {
                let mut captures = match regex.captures(&buffer) {
                    Some(captures) => captures,
                    None => {
                        return Err(Error::FormatError(
                            "input does not match expected format (0)".to_string(),
                        ));
                    }
                };

                let first = match captures.get(1) {
                    Some(capture) => capture.as_str().parse::<T>().map_err(Error::ParseError)?,
                    None => {
                        return Err(Error::FormatError(
                            "input does not match expected format (1)".to_string(),
                        ));
                    }
                };
                let second = match captures.get(2) {
                    Some(capture) => capture.as_str().parse::<T>().map_err(Error::ParseError)?,
                    None => {
                        return Err(Error::FormatError(
                            "input does not match expected format (2)".to_string(),
                        ));
                    }
                };

                if captures.get(3).is_some() {
                    return Err(Error::FormatError(
                        "input does not match expected format (3)".to_string(),
                    ));
                }

                Ok((first, second))
            }
            None => {
                let mut iter = buffer
                    .split(self.item_separator)
                    .map(|s| s.trim())
                    .map(|s| s.parse::<T>().map_err(Error::ParseError));

                let first = match iter.next() {
                    Some(item) => item?,
                    None => return Err(Error::FormatError("expected 2 items, got 0".to_string())),
                };
                let second = match iter.next() {
                    Some(item) => item?,
                    None => return Err(Error::FormatError("expected 2 items, got 1".to_string())),
                };

                if iter.next().is_some() {
                    return Err(Error::FormatError("expected 2 items, got more".to_string()));
                }

                Ok((first, second))
            }
        }
    }
}
