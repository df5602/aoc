extern crate util;

use std::env;
use std::str::FromStr;

use util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: Vec<Point4D> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let constellations = find_constellations(&input);
    println!("Number of constellations: {}", constellations.len());
}

fn find_constellations(points: &[Point4D]) -> Vec<Constellation> {
    let mut constellations: Vec<Constellation> = Vec::new();

    for &point in points {
        // Check whether part of existing constellations
        let mut part_of_idx: Vec<usize> = Vec::new();

        for (i, constellation) in constellations.iter().enumerate() {
            if constellation.is_part_of(point) {
                part_of_idx.push(i);
            }
        }

        if part_of_idx.is_empty() {
            // If point is not part of any constellation, create new constellation and add point
            constellations.push(Constellation::new(point));
        } else if part_of_idx.len() == 1 {
            // If point is part of exactly one constellation, add to constellation
            constellations[part_of_idx[0]].add_point(point);
        } else {
            // If point is part of multiple constellations, merge constellations and add point
            let mut to_merge: Vec<Constellation> = Vec::new();

            // Hack, to enable working with indices
            for idx in part_of_idx {
                to_merge.push(std::mem::replace(
                    &mut constellations[idx],
                    Constellation::empty(),
                ));
            }
            constellations.retain(|constellation| !constellation.is_empty());

            // One constellation will contain all points in merged constellation
            let mut new_constellation = to_merge.pop().unwrap();

            // Merge all other constellations into new one
            for mut constellation in to_merge.iter_mut() {
                new_constellation.merge(&mut constellation);
            }

            // Add new point to constellation
            new_constellation.add_point(point);
            constellations.push(new_constellation);
        }
    }

    constellations
}

#[derive(Debug, Clone)]
struct Constellation {
    points: Vec<Point4D>,
}

impl Constellation {
    fn new(point: Point4D) -> Self {
        Self {
            points: vec![point],
        }
    }

    fn empty() -> Self {
        Self { points: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    fn is_part_of(&self, point: Point4D) -> bool {
        for p in self.points.iter() {
            if p.manhattan_distance_to(point) <= 3 {
                return true;
            }
        }

        false
    }

    fn add_point(&mut self, point: Point4D) {
        self.points.push(point);
    }

    fn merge(&mut self, other: &mut Constellation) {
        self.points.append(&mut other.points);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point4D {
    x: isize,
    y: isize,
    z: isize,
    t: isize,
}

impl Point4D {
    fn new(x: isize, y: isize, z: isize, t: isize) -> Self {
        Self { x, y, z, t }
    }

    fn manhattan_distance_to(self, other: Point4D) -> usize {
        (self.x - other.x).abs() as usize
            + (self.y - other.y).abs() as usize
            + (self.z - other.z).abs() as usize
            + (self.t - other.t).abs() as usize
    }
}

impl FromStr for Point4D {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut point = Point4D::new(0, 0, 0, 0);
        for (i, coord) in s
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<isize>())
            .enumerate()
        {
            match i {
                0 => point.x = coord?,
                1 => point.y = coord?,
                2 => point.z = coord?,
                3 => point.t = coord?,
                _ => panic!("invalid input"),
            }
        }
        Ok(point)
    }
}
