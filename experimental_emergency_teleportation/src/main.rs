extern crate regex;
extern crate util;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::str::FromStr;

//use rand::Rng;
use regex::Regex;

use util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: Vec<Nanobot> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let strongest_nanobot = find_strongest_nanobot(&input);
    println!("Strongest nanobot: {:?}", strongest_nanobot);

    let number_in_range = number_of_nanobots_in_range(strongest_nanobot, &input);
    let mean_dist = mean_distance(strongest_nanobot.position, &input);
    println!(
        "Number of nanobots in range: {} (mean distance: {})",
        number_in_range, mean_dist
    );

    /* Brute force procedure:
     * 1) Run random search on whole volume (e.g. x, y, z in -100000000..100000000)
     * 2) Reduce search volume based on most promising candidates
     * 3) Repeat step 2, until no new candidates are found in a while
     * 4) Starting with best candidate, evaluate neighbors in very small bounding box:
     *    Maximize points in range and then minimize distance to origin
     * 5) Once stable, cross fingers and hope result is the accepted answer..
     */

    /* try to maximize points in range and minimize distance to origin */
    let mut maximum_points_in_range = usize::min_value();
    let mut minimum_distance = usize::max_value();
    let origin = Point3D { x: 0, y: 0, z: 0 };
    // best point found with random search (see below): Point3D { x: 21137940, y: 40831087, z: 57450666 }, points in range: 928, distance: 119419693
    let mut current = Point3D {
        x: 21_137_940,
        y: 40_831_087,
        z: 57_450_666,
    };
    let mut minimum_point = current;

    loop {
        for x in current.x - 3..current.x + 3 {
            for y in current.y - 3..current.y + 3 {
                for z in current.z - 3..current.z + 3 {
                    let point = Point3D { x, y, z };
                    let points_in_range = number_of_nanobots_in_range_from_point(point, &input);
                    if points_in_range >= maximum_points_in_range {
                        maximum_points_in_range = points_in_range;
                        let distance = origin.manhattan_distance_to(point);
                        if distance < minimum_distance {
                            minimum_distance = distance;
                            minimum_point = point;
                        }
                    }
                }
            }
        }
        println!(
            "Minimum point: {:?}, points in range: {}, distance: {}",
            minimum_point, maximum_points_in_range, minimum_distance
        );
        current = minimum_point;
    }

    /* Random search: randomly search points within bounding box */
    // let mut rng = rand::thread_rng();
    // let mut maximum_points_in_range = usize::min_value();
    // let mut minimum_distance = usize::max_value();
    // let origin = Point3D { x: 0, y: 0, z: 0 };
    // loop {
    //     /*
    //         x = 21000000..23000000
    //         y = 41000000..43000000
    //         z = 55000000..57000000
    //     */
    //     let rand_x = rng.gen_range(21300000, 22300000);
    //     let rand_y = rng.gen_range(41000000, 42000000);
    //     let rand_z = rng.gen_range(57000000, 58000000);
    //     let point = Point3D {
    //         x: rand_x,
    //         y: rand_y,
    //         z: rand_z,
    //     };
    //     let points_in_range = number_of_nanobots_in_range_from_point(point, &input);
    //     if points_in_range >= maximum_points_in_range {
    //         if points_in_range > maximum_points_in_range {
    //             minimum_distance = usize::max_value();
    //         }
    //         maximum_points_in_range = points_in_range;
    //         let distance = origin.manhattan_distance_to(point);
    //         if distance < minimum_distance {
    //             minimum_distance = distance;
    //             println!(
    //                 "Point: {:?} => points in range: {}, distance: {}",
    //                 point, points_in_range, distance
    //             );
    //         }
    //     }
    // }
}

fn find_strongest_nanobot(nanobots: &[Nanobot]) -> Nanobot {
    let mut strongest = Nanobot {
        position: Point3D { x: 0, y: 0, z: 0 },
        signal_radius: usize::min_value(),
    };

    for nanobot in nanobots {
        if nanobot.signal_radius > strongest.signal_radius {
            strongest = *nanobot;
        }
    }

    strongest
}

fn mean_distance(point: Point3D, bots: &[Nanobot]) -> usize {
    let sum: usize = bots
        .iter()
        .map(|bot| point.manhattan_distance_to(bot.position))
        .sum();
    sum / bots.len()
}

fn number_of_nanobots_in_range(nanobot: Nanobot, others: &[Nanobot]) -> usize {
    let mut number_in_range = 0;
    for other_bot in others {
        if nanobot.position.manhattan_distance_to(other_bot.position) <= nanobot.signal_radius {
            number_in_range += 1;
        }
    }

    number_in_range
}

fn number_of_nanobots_in_range_from_point(point: Point3D, bots: &[Nanobot]) -> usize {
    let mut number_in_range = 0;
    for bot in bots {
        if point.manhattan_distance_to(bot.position) <= bot.signal_radius {
            number_in_range += 1;
        }
    }

    number_in_range
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point3D {
    x: isize,
    y: isize,
    z: isize,
}

impl Point3D {
    fn manhattan_distance_to(self, other: Point3D) -> usize {
        ((other.x - self.x).abs() + (other.y - self.y).abs() + (other.z - self.z).abs()) as usize
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Nanobot {
    position: Point3D,
    signal_radius: usize,
}

impl FromStr for Nanobot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref regex: Regex =
                Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$").unwrap();
        }
        let captures = match regex.captures(s) {
            Some(captures) => captures,
            None => {
                return Err("input does not match expected format".to_string());
            }
        };
        let mut values = [0isize; 4];
        for (i, val) in values.iter_mut().enumerate() {
            *val = match captures.get(i + 1) {
                Some(capture) => capture
                    .as_str()
                    .parse::<isize>()
                    .map_err(|e| format!("cannot parse number: {}", e))?,
                None => {
                    return Err("input does not match expected format".to_string());
                }
            };
        }
        if captures.get(5).is_some() {
            return Err("input does not match expected format".to_string());
        }
        Ok(Nanobot {
            position: Point3D {
                x: values[0],
                y: values[1],
                z: values[2],
            },
            signal_radius: values[3] as usize,
        })
    }
}
