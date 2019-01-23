use std::collections::HashMap;
use std::env;

use chrono::naive::NaiveDateTime;
use chrono::Timelike;

use util::input::{FileReader, FromFile};

use adhoc_derive::FromStr;

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let mut records: Vec<Record> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    records.sort_unstable_by_key(|r| r.timestamp);
    let records = records;

    let sleep_distributions = create_distributions(&records);

    let (guard_most_asleep, asleep_time) = find_guard_most_minutes_asleep(&sleep_distributions);
    let minute_asleep_most = sleep_distributions[&guard_most_asleep].minute_most_asleep();

    println!(
        "Guard most asleep: {} => was asleep for {} minutes (most at minute {}) => Result: {}",
        guard_most_asleep,
        asleep_time,
        minute_asleep_most,
        guard_most_asleep * minute_asleep_most
    );

    let (guard_most_asleep_at_same_minute, minute_asleep_most) =
        find_guard_most_asleep_at_same_minute(&sleep_distributions);

    println!(
        "Guard most asleep at single minute: {} @ minute {} => Result: {}",
        guard_most_asleep_at_same_minute,
        minute_asleep_most,
        guard_most_asleep_at_same_minute * minute_asleep_most
    );
}

fn create_distributions(records: &[Record]) -> HashMap<usize, SleepDistribution> {
    let mut distributions: HashMap<usize, SleepDistribution> = HashMap::new();

    let mut current_guard_id = 0;
    let mut fall_asleep_time = 0;
    for record in records.iter() {
        match record.entry {
            Entry::ShiftBegin(id) => current_guard_id = id,
            Entry::FallAsleep => fall_asleep_time = record.timestamp.time().minute(),
            Entry::WakeUp => {
                let wake_up = record.timestamp.time().minute();
                distributions
                    .entry(current_guard_id)
                    .or_insert_with(SleepDistribution::new)
                    .increment_asleep(fall_asleep_time as usize, wake_up as usize);
            }
        }
    }

    distributions
}

fn find_guard_most_minutes_asleep(
    sleep_distributions: &HashMap<usize, SleepDistribution>,
) -> (usize, u32) {
    sleep_distributions
        .iter()
        .map(|(&guard, v)| (guard, v.sum()))
        .max_by_key(|(_, v)| *v)
        .unwrap_or_default()
}

fn find_guard_most_asleep_at_same_minute(
    sleep_distributions: &HashMap<usize, SleepDistribution>,
) -> (usize, usize) {
    sleep_distributions
        .iter()
        .map(|(&guard, v)| {
            let minute_most_asleep = v.minute_most_asleep();
            let max_minute = v.at(minute_most_asleep);
            (guard, (minute_most_asleep, max_minute))
        })
        .max_by_key(|(_, (_, max_minute))| *max_minute)
        .map(|(guard, (minute_most_asleep, _))| (guard, minute_most_asleep))
        .unwrap_or_default()
}

struct SleepDistribution {
    minutes: [u32; 60],
}

impl SleepDistribution {
    fn new() -> Self {
        Self { minutes: [0; 60] }
    }

    fn at(&self, minute: usize) -> u32 {
        assert!(minute < 60);
        self.minutes[minute]
    }

    fn increment_asleep(&mut self, from: usize, until: usize) {
        assert!(from <= until);
        assert!(until < 60);

        for min in from..until {
            self.minutes[min] += 1;
        }
    }

    fn sum(&self) -> u32 {
        self.minutes.iter().sum()
    }

    fn minute_most_asleep(&self) -> usize {
        self.minutes
            .iter()
            .enumerate()
            .max_by_key(|(_, amount)| *amount)
            .map(|(minute, _)| minute)
            .unwrap_or_default()
    }
}

#[derive(Debug, FromStr)]
enum Entry {
    #[adhoc(regex = r"^Guard #(?P<0>\d+) begins shift$")]
    ShiftBegin(usize),
    #[adhoc(regex = r"^falls asleep$")]
    FallAsleep,
    #[adhoc(regex = r"^wakes up$")]
    WakeUp,
}

#[derive(Debug, FromStr)]
#[adhoc(regex = r"^\[(?P<timestamp>.+)\] (?P<entry>.+)$")]
struct Record {
    #[adhoc(
        construct_with = r#"NaiveDateTime::parse_from_str(timestamp: &str, "%Y-%m-%d %H:%M")?"#
    )]
    timestamp: NaiveDateTime,
    entry: Entry,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut records: Vec<Record> = FileReader::new().read_from_file("input.txt").unwrap();
        records.sort_unstable_by_key(|r| r.timestamp);
        let sleep_distributions = create_distributions(&records);
        let (guard_most_asleep, _) = find_guard_most_minutes_asleep(&sleep_distributions);
        let minute_asleep_most = sleep_distributions[&guard_most_asleep].minute_most_asleep();
        assert_eq!(36898, guard_most_asleep * minute_asleep_most);
    }

    #[test]
    fn test_part2() {
        let mut records: Vec<Record> = FileReader::new().read_from_file("input.txt").unwrap();
        records.sort_unstable_by_key(|r| r.timestamp);
        let sleep_distributions = create_distributions(&records);
        let (guard_most_asleep_at_same_minute, minute_asleep_most) =
            find_guard_most_asleep_at_same_minute(&sleep_distributions);
        assert_eq!(80711, guard_most_asleep_at_same_minute * minute_asleep_most);
    }
}
