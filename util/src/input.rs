use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub trait FromFile<T> {
    type Error;

    fn read_from_file<P: AsRef<Path>>(path: P) -> Result<T, Self::Error>;
}

#[derive(Debug)]
pub enum Error<E> {
    IoError(std::io::Error),
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

pub struct FileReader;

impl<T> FromFile<Vec<T>> for FileReader
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Error = Error<<T as std::str::FromStr>::Err>;

    fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<T>, Self::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| line?.trim().parse().map_err(Error::ParseError))
            .collect()
    }
}
