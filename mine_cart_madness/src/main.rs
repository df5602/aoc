use std::collections::HashMap;
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

    let mut grid = Grid::create(&input);
    run_simulation(&mut grid, false);

    let mut grid = Grid::create(&input);
    run_simulation(&mut grid, true);
}

fn run_simulation(grid: &mut Grid, remove_collisions: bool) {
    loop {
        let outcome = grid.move_carts(remove_collisions);
        match outcome {
            Outcome::Running => continue,
            Outcome::Collision(collision) => {
                println!("Collision at ({},{})", collision.0, collision.1);
                break;
            }
            Outcome::LastCartStanding(position) => {
                println!("Last cart standing ({},{})", position.0, position.1);
                break;
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Outcome {
    Running,
    Collision((usize, usize)),
    LastCartStanding((usize, usize)),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // left curve = '\', i.e. "turn left" is whatever is appropriate here...
    fn turn_on_left_curve(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    // right curve = '/', i.e. "turn right" is whatever is appropriate here...
    fn turn_on_right_curve(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    // turn left from the point of view of the cart
    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    // turn right from the point of view of the cart
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next_turn(self) -> Turn {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cart {
    id: usize,
    x: usize,
    y: usize,
    direction: Direction,
    last_turn: Turn,
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
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
    shadowed_cells: HashMap<(usize, usize), Cell>,
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
        let mut shadowed_cells = HashMap::new();
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
                        last_turn: Turn::Right,
                    };
                    shadowed_cells.insert((x, y), Cell::VerticalTrack);
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
                        last_turn: Turn::Right,
                    };
                    shadowed_cells.insert((x, y), Cell::VerticalTrack);
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
                        last_turn: Turn::Right,
                    };
                    shadowed_cells.insert((x, y), Cell::HorizontalTrack);
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
                        last_turn: Turn::Right,
                    };
                    shadowed_cells.insert((x, y), Cell::HorizontalTrack);
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
            shadowed_cells,
            carts,
        }
    }

    fn move_carts(&mut self, remove_collisions: bool) -> Outcome {
        // Take snapshot of current positions and sort
        let mut carts = self.carts.clone();
        carts.sort_unstable_by(|a, b| {
            if a.y == b.y {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        });

        // Dead carts
        let mut dead_carts = Vec::new();

        // Move all carts
        for cart in carts {
            // Make sure current cart is still running
            if dead_carts.iter().any(|&c: &Cart| c.id == cart.id) {
                continue;
            }

            if let Some(collision) = self.move_cart(cart) {
                if !remove_collisions {
                    return Outcome::Collision((collision.0.x, collision.0.y));
                } else {
                    // Remove carts from list of carts
                    self.carts
                        .retain(|cart| cart.id != collision.0.id && cart.id != collision.1.id);

                    // Also remove carts from carts snapshot
                    // (We can't modify carts at this point since we're iterating over it, so store "dead" carts in list)
                    dead_carts.push(collision.0);
                    dead_carts.push(collision.1);

                    // Update map
                    let shadowed = self.retrieve_shadowed(collision.0.x, collision.0.y);
                    self.set_at(collision.0.x, collision.0.y, shadowed);
                }
            }
        }

        if self.carts.len() == 1 {
            let cart = self.carts[0];
            Outcome::LastCartStanding((cart.x, cart.y))
        } else {
            assert!(!self.carts.is_empty());
            Outcome::Running
        }
    }

    fn move_cart(&mut self, cart: Cart) -> Option<(Cart, Cart)> {
        let next_track = self.next_track(cart);

        let mut collision = None;

        match (self.at(next_track.0, next_track.1), cart.direction) {
            (Cell::VerticalTrack, Direction::Up)
            | (Cell::VerticalTrack, Direction::Down)
            | (Cell::HorizontalTrack, Direction::Left)
            | (Cell::HorizontalTrack, Direction::Right) => {
                // Normal move up/down
                let cart = self.make_move(cart, next_track, cart.direction, None, true);
                self.set_at(cart.x, cart.y, Cell::Cart(cart));
            }
            (Cell::LeftCurve, _) => {
                // Turn on left curve
                let cart = self.make_move(
                    cart,
                    next_track,
                    cart.direction.turn_on_left_curve(),
                    None,
                    true,
                );
                self.set_at(cart.x, cart.y, Cell::Cart(cart));
            }
            (Cell::RightCurve, _) => {
                // Turn on right curve
                let cart = self.make_move(
                    cart,
                    next_track,
                    cart.direction.turn_on_right_curve(),
                    None,
                    true,
                );
                self.set_at(cart.x, cart.y, Cell::Cart(cart));
            }
            (Cell::Intersection, _) => {
                let next_turn = cart.last_turn.next_turn();
                let next_direction = match next_turn {
                    Turn::Left => cart.direction.turn_left(),
                    Turn::Straight => cart.direction,
                    Turn::Right => cart.direction.turn_right(),
                };
                let cart = self.make_move(cart, next_track, next_direction, Some(next_turn), true);
                self.set_at(cart.x, cart.y, Cell::Cart(cart));
            }
            (Cell::Cart(other_cart), _) => {
                // Collision
                let cart = self.make_move(cart, next_track, cart.direction, None, false);
                self.set_at(cart.x, cart.y, Cell::Collision(cart, other_cart));
                collision = Some((cart, other_cart));
            }
            _ => panic!("illegal move"),
        }

        collision
    }

    fn make_move(
        &mut self,
        cart: Cart,
        next: (usize, usize),
        dir: Direction,
        turn: Option<Turn>,
        shadow: bool,
    ) -> Cart {
        let shadowed = self.retrieve_shadowed(cart.x, cart.y);
        if shadow {
            self.store_shadowed(next.0, next.1);
        }
        self.set_at(cart.x, cart.y, shadowed);
        let stored_cart = self.retrieve_cart(cart);
        stored_cart.x = next.0;
        stored_cart.y = next.1;
        stored_cart.direction = dir;
        if let Some(turn) = turn {
            stored_cart.last_turn = turn;
        }
        *stored_cart
    }

    fn next_track(&self, cart: Cart) -> (usize, usize) {
        match cart.direction {
            Direction::Up => (cart.x, cart.y - 1),
            Direction::Down => (cart.x, cart.y + 1),
            Direction::Left => (cart.x - 1, cart.y),
            Direction::Right => (cart.x + 1, cart.y),
        }
    }

    fn at(&self, x: usize, y: usize) -> Cell {
        if x >= self.width || y >= self.height {
            panic!("out of bounds");
        }
        self.grid[y * self.width + x]
    }

    fn set_at(&mut self, x: usize, y: usize, value: Cell) {
        if x >= self.width || y >= self.height {
            panic!("out of bounds");
        }
        self.grid[y * self.width + x] = value;
    }

    fn retrieve_shadowed(&mut self, x: usize, y: usize) -> Cell {
        if x >= self.width || y >= self.height {
            panic!("out of bounds");
        }
        self.shadowed_cells.remove(&(x, y)).unwrap()
    }

    fn store_shadowed(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            panic!("out of bounds");
        }
        let to_shadow = self.at(x, y);
        self.shadowed_cells.insert((x, y), to_shadow);
    }

    fn retrieve_cart(&mut self, cart: Cart) -> &mut Cart {
        self.carts.iter_mut().find(|&&mut c| c == cart).unwrap()
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

        for cart in self.carts.iter() {
            writeln!(f, "{:?}", cart)?;
        }
        Ok(())
    }
}
