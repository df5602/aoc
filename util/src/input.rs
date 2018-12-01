use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub trait Input {
    type Destination;

    fn read_from_file<P: AsRef<Path>>(path: P) -> Self::Destination;
}

impl<T: std::str::FromStr> Input for Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Destination = Vec<T>;

    fn read_from_file<P: AsRef<Path>>(path: P) -> Self::Destination {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| line.unwrap().parse::<T>().unwrap())
            .collect()
    }
}
