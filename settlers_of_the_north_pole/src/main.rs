use std::collections::HashSet;
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

    let input: Vec<String> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let mut grid = Grid::create(&input);
    println!("{}", grid);

    let num_steps = 10;
    for _ in 0..num_steps {
        grid.step();
    }

    println!("After {} minutes:", num_steps);
    println!("{}", grid);

    let counts = grid.count_all();
    println!(
        "Trees: {}, lumberyards: {} => {}",
        counts.trees,
        counts.lumberyard,
        counts.trees * counts.lumberyard
    );

    let mut set = HashSet::new();

    let num_steps = 1_000_000_000;
    let mut already_seen = 0;
    let mut loop_start = Counts {
        open: 0,
        trees: 0,
        lumberyard: 0,
    };
    let mut loop_start_idx = 0;
    let mut loop_start_found = false;
    let mut loop_entries = Vec::new();

    for i in 10..num_steps {
        grid.step();
        let counts = grid.count_all();

        if !set.insert(counts) {
            if i - already_seen == 1 {
                println!(
                    "[{}] {:?} => {}",
                    i,
                    counts,
                    counts.trees * counts.lumberyard
                );
                if !loop_start_found {
                    loop_start_found = true;
                    loop_start = counts;
                    loop_start_idx = i;
                    loop_entries.push(counts);
                } else {
                    if loop_start == counts {
                        break;
                    }
                    loop_entries.push(counts);
                }
            }
            already_seen = i;
        }
    }

    println!("Loop found: Length: {}", loop_entries.len());
    let resource_value = loop_entries[(num_steps - 1 - loop_start_idx) % loop_entries.len()];
    println!(
        "Resource value after {} minutes: {}",
        num_steps,
        resource_value.trees * resource_value.lumberyard
    );
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Open,
    Trees,
    Lumberyard,
    OutOfBounds,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Open => write!(f, "."),
            Cell::Trees => write!(f, "|"),
            Cell::Lumberyard => write!(f, "#"),
            Cell::OutOfBounds => write!(f, " "),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Counts {
    open: isize,
    trees: isize,
    lumberyard: isize,
}

impl std::ops::AddAssign for Counts {
    fn add_assign(&mut self, other: Counts) {
        *self = Counts {
            open: self.open + other.open,
            trees: self.trees + other.trees,
            lumberyard: self.lumberyard + other.lumberyard,
        };
    }
}

impl std::ops::Sub for Counts {
    type Output = Counts;

    fn sub(self, other: Counts) -> Counts {
        Counts {
            open: self.open.wrapping_sub(other.open),
            trees: self.trees.wrapping_sub(other.trees),
            lumberyard: self.lumberyard.wrapping_sub(other.lumberyard),
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
}

impl Grid {
    fn create(input: &[String]) -> Self {
        let (width, height) = if !input.is_empty() {
            (input[0].len(), input.len())
        } else {
            (0, 0)
        };

        let mut grid = Vec::with_capacity(width * height);
        input.iter().flat_map(|s| s.chars()).for_each(|c| {
            let cell = match c {
                '.' => Cell::Open,
                '|' => Cell::Trees,
                '#' => Cell::Lumberyard,
                _ => panic!("unexpected input!"),
            };
            grid.push(cell);
        });

        Self {
            width,
            height,
            grid,
        }
    }

    fn step(&mut self) {
        let mut new_grid = Vec::with_capacity(self.grid.len());

        let mut x = 0;
        let mut y = 0;
        for cell in self.grid.iter() {
            let counts = self.count_neighbours(x, y);
            let new_cell = match cell {
                Cell::Open if counts.trees >= 3 => Cell::Trees,
                Cell::Trees if counts.lumberyard >= 3 => Cell::Lumberyard,
                Cell::Lumberyard => {
                    if counts.lumberyard >= 1 && counts.trees >= 1 {
                        Cell::Lumberyard
                    } else {
                        Cell::Open
                    }
                }
                cell => *cell,
            };
            new_grid.push(new_cell);

            x += 1;
            if x >= self.width {
                y += 1;
                x = 0;
            }
        }

        self.grid = new_grid;
    }

    fn count_neighbours(&self, x: usize, y: usize) -> Counts {
        let mut counts = Counts {
            open: 0,
            trees: 0,
            lumberyard: 0,
        };

        let y_above = if y > 0 { y - 1 } else { self.height };
        let x_left = if x > 0 { x - 1 } else { self.width };

        counts += self.count(x_left, y_above);
        counts += self.count(x, y_above);
        counts += self.count(x + 1, y_above);
        counts += self.count(x_left, y);
        counts += self.count(x + 1, y);
        counts += self.count(x_left, y + 1);
        counts += self.count(x, y + 1);
        counts += self.count(x + 1, y + 1);

        counts
    }

    fn count(&self, x: usize, y: usize) -> Counts {
        match self.at(x, y) {
            Cell::Open => Counts {
                open: 1,
                trees: 0,
                lumberyard: 0,
            },
            Cell::Trees => Counts {
                open: 0,
                trees: 1,
                lumberyard: 0,
            },
            Cell::Lumberyard => Counts {
                open: 0,
                trees: 0,
                lumberyard: 1,
            },
            Cell::OutOfBounds => Counts {
                open: 0,
                trees: 0,
                lumberyard: 0,
            },
        }
    }

    fn count_all(&self) -> Counts {
        let mut counts = Counts {
            open: 0,
            trees: 0,
            lumberyard: 0,
        };
        for cell in self.grid.iter() {
            match cell {
                Cell::Open => counts.open += 1,
                Cell::Trees => counts.trees += 1,
                Cell::Lumberyard => counts.lumberyard += 1,
                Cell::OutOfBounds => unreachable!(),
            }
        }
        counts
    }

    fn at(&self, x: usize, y: usize) -> Cell {
        if x >= self.width || y >= self.height {
            return Cell::OutOfBounds;
        }
        self.grid[y * self.width + x]
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, c) in self.grid.iter().enumerate() {
            write!(f, "{}", c)?;
            if (i + 1) % self.width == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
