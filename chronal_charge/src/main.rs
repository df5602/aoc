extern crate util;

use std::env;

use util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: String = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let grid_serial_number = match input.parse::<usize>() {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    println!("Grid serial number: {}", grid_serial_number);

    let width = 300;
    let height = 300;
    let mut grid = vec![0; width * height];

    for x in 1..=width {
        for y in 1..=height {
            grid[to_idx(x, y, width)] = calculate_power_level(x, y, grid_serial_number);
        }
    }

    let mut max_total_power = isize::min_value();
    let mut max_coord = (0, 0);
    for x in 1..=(width - 2) {
        for y in 1..=(height - 2) {
            let total_power = grid[to_idx(x, y, width)..to_idx(x + 3, y, width)]
                .iter()
                .sum::<isize>()
                + grid[to_idx(x, y + 1, width)..to_idx(x + 3, y + 1, width)]
                    .iter()
                    .sum::<isize>()
                + grid[to_idx(x, y + 2, width)..to_idx(x + 3, y + 2, width)]
                    .iter()
                    .sum::<isize>();
            if total_power > max_total_power {
                max_total_power = total_power;
                max_coord = (x, y);
            }
        }
    }

    println!(
        "Total power level at ({},{}) is {}",
        max_coord.0, max_coord.1, max_total_power
    );
}

fn to_idx(x: usize, y: usize, width: usize) -> usize {
    (y - 1) * width + (x - 1)
}

fn calculate_power_level(x: usize, y: usize, grid_serial_number: usize) -> isize {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += grid_serial_number;
    power_level *= rack_id;

    let thousands_above = power_level / 1000 * 1000;
    let hundreds_above = power_level / 100 * 100;

    power_level = (hundreds_above - thousands_above) / 100;

    (power_level as isize) - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        let position = (3, 5);
        let grid_serial_number = 8;
        assert_eq!(
            4,
            calculate_power_level(position.0, position.1, grid_serial_number)
        );

        let position = (122, 79);
        let grid_serial_number = 57;
        assert_eq!(
            -5,
            calculate_power_level(position.0, position.1, grid_serial_number)
        );

        let position = (217, 196);
        let grid_serial_number = 39;
        assert_eq!(
            0,
            calculate_power_level(position.0, position.1, grid_serial_number)
        );

        let position = (101, 153);
        let grid_serial_number = 71;
        assert_eq!(
            4,
            calculate_power_level(position.0, position.1, grid_serial_number)
        );
    }
}
