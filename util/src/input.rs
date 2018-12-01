use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub trait FromFile<T> {
    fn read_from_file<P: AsRef<Path>>(path: P) -> T;
}

pub struct FileReader;

impl<T> FromFile<Vec<T>> for FileReader
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn read_from_file<P: AsRef<Path>>(path: P) -> Vec<T> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| line.unwrap().parse::<T>().unwrap())
            .collect()
    }
}
