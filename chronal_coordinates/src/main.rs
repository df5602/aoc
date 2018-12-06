extern crate util;

use std::cmp::{max, min};
use std::collections::HashMap;
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

    let input: Vec<Point> = match FileReader::read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let (largest_area, point_with_largest_area) = find_largest_area(&input);

    println!(
        "Largest area: {} (Point: ({},{}))",
        largest_area, point_with_largest_area.0, point_with_largest_area.1
    );

    let size_of_region = find_region(&input, 10000);
    println!("Size  of region: {}", size_of_region);
}

fn find_largest_area(points: &[Point]) -> (usize, (usize, usize)) {
    let (max_width, max_height) = points
        .iter()
        .fold((0, 0), |m, p| (max(m.0, p.x), max(m.1, p.y)));

    let mut map = HashMap::new();
    for x in 0..=max_width {
        for y in 0..=max_height {
            let p = Point::new(x, y);
            let mut shortest_dist = usize::max_value();
            let mut closest_point = None;
            for (dist, q) in points.iter().map(|q| (p.manhattan_distance_to(&q), q)) {
                if dist < shortest_dist {
                    shortest_dist = dist;
                    closest_point = Some((*q).clone());
                } else if dist == shortest_dist {
                    closest_point = None;
                }
            }

            if let Some(q) = closest_point {
                let counter = map.entry((q.x, q.y)).or_insert(0usize);

                if p.x == 0 || p.x == max_width || p.y == 0 || p.y == max_height {
                    *counter = usize::max_value();
                } else {
                    *counter = (*counter).saturating_add(1);
                }
            }
        }
    }

    let mut point_with_largest_area = (0, 0);
    let mut largest_area = 0;
    for (&k, &v) in map.iter() {
        if v < usize::max_value() && v > largest_area {
            point_with_largest_area = k;
            largest_area = v;
        }
    }

    (largest_area, point_with_largest_area)
}

fn find_region(points: &[Point], distance_threshold: usize) -> usize {
    let (max_width, max_height) = points
        .iter()
        .fold((0, 0), |m, p| (max(m.0, p.x), max(m.1, p.y)));

    let mut size_of_region = 0;
    for x in 0..=max_width {
        for y in 0..=max_height {
            let p = Point::new(x, y);
            let mut sum_of_distances = 0;
            for dist in points.iter().map(|q| p.manhattan_distance_to(&q)) {
                sum_of_distances += dist;
            }

            if sum_of_distances < distance_threshold {
                size_of_region += 1;
            }
        }
    }

    size_of_region
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let substrings: Vec<_> = s
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim())
            .collect();

        if substrings.len() != 2 {
            panic!("input does not match format");
        }

        Ok(Self {
            x: substrings[0].parse()?,
            y: substrings[1].parse()?,
        })
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn manhattan_distance_to(&self, other: &Point) -> usize {
        max(self.x, other.x) - min(self.x, other.x) + max(self.y, other.y) - min(self.y, other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let p = Point::new(0, 0);
        let q = Point::new(5, 2);
        assert_eq!(7, p.manhattan_distance_to(&q));
    }

    #[test]
    fn test_largest_area() {
        let points = vec![
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9),
        ];

        let (largest_area, point_with_largest_area) = find_largest_area(&points);
        assert_eq!(17, largest_area);
        assert_eq!((5, 5), point_with_largest_area);
    }

    #[test]
    fn test_find_region() {
        let points = vec![
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9),
        ];
        assert_eq!(16, find_region(&points, 32));
    }
}
