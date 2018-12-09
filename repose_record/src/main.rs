extern crate chrono;
extern crate util;

use std::collections::HashMap;
use std::env;
use std::str::FromStr;

use chrono::naive::NaiveDateTime;
use chrono::Timelike;

use util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let mut input: Vec<Record> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    input.sort_unstable_by_key(|r| r.timestamp);
    let input = input;

    let (
        guard_most_asleep,
        asleep_time,
        minute_asleep_most,
        guard_most_asleep_2,
        minute_asleep_most_2,
    ) = part1_2(&input);

    println!(
        "Guard most asleep: {} => was asleep for {} minutes (most at minute {}) => Result: {}",
        guard_most_asleep,
        asleep_time,
        minute_asleep_most,
        guard_most_asleep * minute_asleep_most
    );

    println!(
        "Guard most asleep at single minute: {} @ minute {} => Result: {}",
        guard_most_asleep_2,
        minute_asleep_most_2,
        guard_most_asleep_2 * minute_asleep_most_2
    );
}

fn part1_2(input: &[Record]) -> (usize, usize, usize, usize, usize) {
    let mut map_minutes: HashMap<usize, [usize; 60]> = HashMap::new();
    let mut map_totals: HashMap<usize, usize> = HashMap::new();

    let mut current_guard_id = 0;
    let mut fall_asleep_time = 0;
    for record in input.iter() {
        match record.entry {
            Entry::ShiftBegin(id) => {
                if id == 0 {
                    panic!("Guard ID is 0!");
                } else {
                    current_guard_id = id;
                }
            }
            Entry::FallAsleep => fall_asleep_time = record.timestamp.time().minute(),
            Entry::WakeUp => {
                let wake_up = record.timestamp.time().minute();
                let minutes = map_minutes.entry(current_guard_id).or_insert([0; 60]);
                for min in fall_asleep_time..wake_up {
                    (*minutes)[min as usize] += 1;
                }
                *map_totals.entry(current_guard_id).or_insert(0) +=
                    (wake_up - fall_asleep_time) as usize;
            }
        }
    }

    let mut guard_most_asleep = 0;
    let mut asleep_time = 0;
    for (id, total) in map_totals.iter() {
        if *total > asleep_time {
            guard_most_asleep = *id;
            asleep_time = *total;
        }
    }

    let mut minute_asleep_most = 0;
    {
        let minutes = map_minutes.entry(guard_most_asleep).or_insert([0; 60]);

        let mut max_minute = 0;
        for (min, tot) in minutes.iter().enumerate() {
            if *tot > max_minute {
                max_minute = *tot;
                minute_asleep_most = min;
            }
        }
    }

    let mut guard_most_asleep_2 = 0;
    let mut minute_asleep_most_2 = 0;
    let mut max_minute = 0;
    for (id, minutes) in map_minutes.iter() {
        for (min, tot) in minutes.iter().enumerate() {
            if *tot > max_minute {
                max_minute = *tot;
                minute_asleep_most_2 = min;
                guard_most_asleep_2 = *id;
            }
        }
    }

    (
        guard_most_asleep,
        asleep_time,
        minute_asleep_most,
        guard_most_asleep_2,
        minute_asleep_most_2,
    )
}

#[derive(Debug)]
enum Entry {
    ShiftBegin(usize),
    FallAsleep,
    WakeUp,
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("Guard") {
            match s
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.trim())
                .filter(|s| s.starts_with('#'))
                .map(|s| s[1..].parse().map_err(|e| format!("{}", e)))
                .nth(0)
            {
                Some(id) => Ok(Entry::ShiftBegin(id?)),
                None => Err("could not find guard id".to_string()),
            }
        } else if s.contains("asleep") {
            Ok(Entry::FallAsleep)
        } else if s.contains("wakes") {
            Ok(Entry::WakeUp)
        } else {
            Err("invalid entry".to_string())
        }
    }
}

#[derive(Debug)]
struct Record {
    timestamp: NaiveDateTime,
    entry: Entry,
}

#[derive(Debug)]
enum RecordParseError {
    ParseTimeError(chrono::format::ParseError),
    ParseError(String),
}

impl std::fmt::Display for RecordParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RecordParseError::ParseTimeError(e) => write!(f, "Error parsing timestamp: {}", e),
            RecordParseError::ParseError(s) => write!(f, "Error parsing record: {}", s),
        }
    }
}

impl From<chrono::format::ParseError> for RecordParseError {
    fn from(error: chrono::format::ParseError) -> Self {
        RecordParseError::ParseTimeError(error)
    }
}

impl FromStr for Record {
    type Err = RecordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let substrings: Vec<_> = s
            .split(|c| c == '[' || c == ']')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim())
            .collect();
        if substrings.len() != 2 {
            return Err(RecordParseError::ParseError(String::from(
                "input does not match format",
            )));
        }
        Ok(Self {
            timestamp: NaiveDateTime::parse_from_str(substrings[0], "%Y-%m-%d %H:%M")?,
            entry: substrings[1]
                .parse()
                .map_err(RecordParseError::ParseError)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let inputs = vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep", // [5,25)
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep", // [30, 55)
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep", // [24, 29)
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ];

        let mut input: Vec<Record> = Vec::new();
        for line in inputs {
            input.push(line.parse().unwrap());
        }

        assert_eq!((10, 50, 24, 99, 45), part1_2(&input));
    }
}
