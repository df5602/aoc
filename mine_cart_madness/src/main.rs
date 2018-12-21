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

    // TODO: only necessary because FileReader trims by default...
    let input: Vec<String> = input.lines().map(|s| s.parse().unwrap()).collect();

    let grid = Grid::create(&input);
    println!("{}", grid);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cart {
    id: usize,
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    VerticalTrack,
    HorizontalTrack,
    RightCurve,
    LeftCurve,
    Intersection,
    Cart(Cart),
    Collision(Cart, Cart),
    OutOfBounds,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::VerticalTrack => write!(f, "|"),
            Cell::HorizontalTrack => write!(f, "-"),
            Cell::RightCurve => write!(f, "/"),
            Cell::LeftCurve => write!(f, "\\"),
            Cell::Intersection => write!(f, "+"),
            Cell::Cart(cart) => match cart.direction {
                Direction::Up => write!(f, "^"),
                Direction::Down => write!(f, "v"),
                Direction::Left => write!(f, "<"),
                Direction::Right => write!(f, ">"),
            },
            Cell::Collision(_, _) => write!(f, "x"),
            Cell::OutOfBounds => write!(f, "@"),
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
    carts: Vec<Cart>,
}

impl Grid {
    fn create(input: &[String]) -> Self {
        let (width, height) = if !input.is_empty() {
            (input[0].len(), input.len())
        } else {
            (0, 0)
        };

        let mut cart_id = 0;
        let mut x = 0;
        let mut y = 0;

        let mut grid = Vec::with_capacity(width * height);
        let mut carts = Vec::new();

        input.iter().flat_map(|s| s.chars()).for_each(|c| {
            let cell = match c {
                ' ' => Cell::Empty,
                '|' => Cell::VerticalTrack,
                '-' => Cell::HorizontalTrack,
                '/' => Cell::RightCurve,
                '\\' => Cell::LeftCurve,
                '+' => Cell::Intersection,
                '^' => {
                    cart_id += 1;
                    let cart = Cart {
                        id: cart_id - 1,
                        x,
                        y,
                        direction: Direction::Up,
                    };
                    carts.push(cart);
                    Cell::Cart(cart)
                }
                'v' => {
                    cart_id += 1;
                    let cart = Cart {
                        id: cart_id - 1,
                        x,
                        y,
                        direction: Direction::Down,
                    };
                    carts.push(cart);
                    Cell::Cart(cart)
                }
                '<' => {
                    cart_id += 1;
                    let cart = Cart {
                        id: cart_id - 1,
                        x,
                        y,
                        direction: Direction::Left,
                    };
                    carts.push(cart);
                    Cell::Cart(cart)
                }
                '>' => {
                    cart_id += 1;
                    let cart = Cart {
                        id: cart_id - 1,
                        x,
                        y,
                        direction: Direction::Right,
                    };
                    carts.push(cart);
                    Cell::Cart(cart)
                }
                c => panic!("unexpected input: {}!", c),
            };
            grid.push(cell);

            x += 1;
            if x >= width {
                y += 1;
                x = 0;
            }
        });

        Self {
            width,
            height,
            grid,
            carts,
        }
    }

    fn at(&self, x: usize, y: usize) -> Cell {
        if x >= self.width || y >= self.height {
            return Cell::OutOfBounds;
        }
        self.grid[y * self.width + x]
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, c) in self.grid.iter().enumerate() {
            write!(f, "{}", c)?;
            if (i + 1) % self.width == 0 {
                writeln!(f)?;
            }
        }

        for cart in self.carts.iter() {
            writeln!(f, "{:?}", cart)?;
        }
        Ok(())
    }
}
