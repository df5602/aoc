use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;

use adhoc_derive::FromStr;

use util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: Vec<Point> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let (max_width, max_height) = calculate_dimensions(&input);

    let (largest_area, point_with_largest_area) = find_largest_area(&input, max_width, max_height);

    println!(
        "Largest area: {} (Point: ({},{}))",
        largest_area, point_with_largest_area.x, point_with_largest_area.y
    );

    let size_of_region = find_region(&input, 10000, max_width, max_height);
    println!("Size  of region: {}", size_of_region);
}

fn find_largest_area(points: &[Point], max_width: usize, max_height: usize) -> (usize, Point) {
    let mut map = HashMap::new();
    for x in 0..=max_width {
        for y in 0..=max_height {
            let p = Point::new(x, y);
            let mut shortest_dist = usize::max_value();
            let mut closest_point = None;
            for (dist, q) in points.iter().map(|q| (p.manhattan_distance_to(&q), q)) {
                if dist < shortest_dist {
                    shortest_dist = dist;
                    closest_point = Some(*q);
                } else if dist == shortest_dist {
                    closest_point = None;
                }
            }

            if let Some(q) = closest_point {
                let counter = map.entry(q).or_insert(0usize);

                if p.x == 0 || p.x == max_width || p.y == 0 || p.y == max_height {
                    *counter = usize::max_value();
                } else {
                    *counter = (*counter).saturating_add(1);
                }
            }
        }
    }

    map.iter()
        .filter(|(_, &area)| area < usize::max_value())
        .max_by_key(|(_, &area)| area)
        .map(|(&point, &area)| (area, point))
        .unwrap_or_else(|| (0, Point::new(0, 0)))
}

fn find_region(
    points: &[Point],
    distance_threshold: usize,
    max_width: usize,
    max_height: usize,
) -> usize {
    let mut size_of_region = 0;
    for x in 0..=max_width {
        for y in 0..=max_height {
            let p = Point::new(x, y);
            let sum_of_distances: usize = points.iter().map(|q| p.manhattan_distance_to(&q)).sum();

            if sum_of_distances < distance_threshold {
                size_of_region += 1;
            }
        }
    }

    size_of_region
}

fn calculate_dimensions(points: &[Point]) -> (usize, usize) {
    points
        .iter()
        .fold((0, 0), |m, p| (max(m.0, p.x), max(m.1, p.y)))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromStr)]
#[adhoc(regex = r"^(?P<x>\d+), (?P<y>\d+)$")]
struct Point {
    x: usize,
    y: usize,
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

        let (max_width, max_height) = calculate_dimensions(&points);
        let (largest_area, point_with_largest_area) =
            find_largest_area(&points, max_width, max_height);
        assert_eq!(17, largest_area);
        assert_eq!(Point::new(5, 5), point_with_largest_area);
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
        let (max_width, max_height) = calculate_dimensions(&points);
        assert_eq!(16, find_region(&points, 32, max_width, max_height));
    }

    #[test]
    fn test_part1() {
        let input: Vec<Point> = FileReader::new().read_from_file("input.txt").unwrap();
        let (max_width, max_height) = calculate_dimensions(&input);
        let (largest_area, _) = find_largest_area(&input, max_width, max_height);
        assert_eq!(3260, largest_area);
    }

    #[test]
    fn test_part2() {
        let input: Vec<Point> = FileReader::new().read_from_file("input.txt").unwrap();
        let (max_width, max_height) = calculate_dimensions(&input);
        let size_of_region = find_region(&input, 10000, max_width, max_height);
        assert_eq!(42535, size_of_region);
    }
}
