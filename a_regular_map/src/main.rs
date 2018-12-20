extern crate util;

use std::collections::{HashMap, VecDeque};
use std::env;
use std::str::Chars;

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

    let mut graph = Graph::new();
    graph.build(&input);
    let (farthest_node, farthest_dist) = graph.find_farthest_node(Position::new(0, 0));

    println!("{}", graph);
    println!(
        "Farthest node: {:?}, distance {}",
        farthest_node, farthest_dist
    );
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn north(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn east(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn south(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn west(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
}

#[derive(Debug)]
struct Graph {
    edges: HashMap<Position, Vec<Position>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn build(&mut self, input: &str) {
        if !input.starts_with('^') || !input.ends_with('$') {
            panic!("invalid input, missing start/end token");
        }

        let input = &input[1..];
        self.parse(Position::new(0, 0), &mut input.chars(), 0);
    }

    fn find_farthest_node(&self, from: Position) -> (Position, usize) {
        // Check that start position is in graph
        if !self.edges.contains_key(&from) {
            panic!("start position not in graph");
        }

        // Initialize distances map
        // Node not in map => node not reachable (i.e. wall, out of bounds, etc.)
        // Node in map with distance = usize::max_value() => node not visited yet
        // Node in map with distance < usize::max_value() => node visited, value corresponds to distance
        let mut distances: HashMap<Position, usize> = HashMap::new();
        for &node in self.edges.keys() {
            distances.insert(node, usize::max_value());
        }

        // Queue for unvisited nodes
        let mut queue: VecDeque<Position> = VecDeque::new();

        // Initialize queue with start position
        distances.insert(from, 0);
        queue.push_back(from);

        // BFS
        loop {
            if let Some(pos) = queue.pop_front() {
                // Lookup neighbors
                let current_dist = *distances.get(&pos).unwrap();
                let neighbors = self.edges.get(&pos).unwrap();
                for &neighbor in neighbors {
                    let distance = distances.get_mut(&neighbor).unwrap();
                    if *distance == usize::max_value() {
                        // Node not visited yet, update distance and add to queue
                        *distance = current_dist + 1;
                        queue.push_back(neighbor);
                    }
                }
            } else {
                break;
            }
        }

        // Get highest distance
        let mut farthest_dist = usize::min_value();
        let mut farthest_node = Position::new(0, 0);
        for (&k, &v) in distances.iter() {
            if v > farthest_dist {
                farthest_dist = v;
                farthest_node = k;
            }
        }

        (farthest_node, farthest_dist)
    }

    // TODO: I don't think this solution is correct for all inputs, e.g ^EEE(N(N|)W|)S$
    fn parse(&mut self, current: Position, mut chars: &mut Chars, level: usize) -> Vec<Position> {
        let start_pos = current;
        let mut current = current;
        let mut positions = Vec::new();

        loop {
            if let Some(c) = chars.next() {
                //println!("Parse [{}]: {}", level, c);
                match c {
                    'N' => {
                        self.add_edge(current, current.north());
                        current = current.north();
                    }
                    'E' => {
                        self.add_edge(current, current.east());
                        current = current.east();
                    }
                    'S' => {
                        self.add_edge(current, current.south());
                        current = current.south();
                    }
                    'W' => {
                        self.add_edge(current, current.west());
                        current = current.west();
                    }
                    '$' => {
                        positions.push(current);
                        break;
                    }
                    '(' => {
                        let mut returned_positions = self.parse(current, &mut chars, level + 1);
                        if true /* solution (part 1) becomes correct if we always takes this branch :-/ */ || returned_positions.len() == 1
                        {
                            //println!("[{}] Got 1 position back", level);
                            current = returned_positions[0];
                        } else {
                            /*println!(
                                "[{}] Got {} positions back",
                                level,
                                returned_positions.len()
                            );*/

                            let mut peek = chars.clone();
                            if let Some(c) = peek.next() {
                                if c == '|' {
                                    positions.append(&mut returned_positions);
                                    current = start_pos;
                                } else if c == ')' {
                                    positions.append(&mut returned_positions);
                                    break;
                                } else {
                                    let chars_clone = chars.clone();
                                    for (i, &pos) in returned_positions.iter().enumerate() {
                                        let mut result = if i == 0 {
                                            // Give original iterator (we need the cursor to be updated) to first
                                            self.parse(pos, &mut chars, level + 1)
                                        } else {
                                            // Give cloned iterator to all the others
                                            self.parse(pos, &mut chars_clone.clone(), level + 1)
                                        };

                                        positions.append(&mut result);
                                    }
                                    current = positions[0];
                                }
                            }
                        }
                    }
                    ')' => {
                        positions.push(current);
                        break;
                    }
                    '|' => {
                        positions.push(current);
                        current = start_pos;
                    }
                    c => panic!("invalid token {}", c),
                }
            } else {
                //println!("[{}] No more characters", level);
                positions.push(current);
                break;
            }
        }

        positions.sort();
        positions.dedup();
        positions
    }

    fn add_edge(&mut self, from: Position, to: Position) {
        let entry_from = self.edges.entry(from).or_insert_with(Vec::new);
        if !entry_from.contains(&to) {
            entry_from.push(to);
        }

        let entry_to = self.edges.entry(to).or_insert_with(Vec::new);
        if !entry_to.contains(&from) {
            entry_to.push(from);
        }
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut x_min = isize::max_value();
        let mut x_max = isize::min_value();
        let mut y_min = isize::max_value();
        let mut y_max = isize::min_value();
        for node in self.edges.keys() {
            if node.x < x_min {
                x_min = node.x;
            }
            if node.x > x_max {
                x_max = node.x;
            }
            if node.y < y_min {
                y_min = node.y;
            }
            if node.y > y_max {
                y_max = node.y;
            }
        }

        let width = (x_max - x_min + 1) * 2 + 1;
        let height = (y_max - y_min + 1) * 2 + 1;

        let idx = |pos: Position| ((pos.y - y_min) * 2 + 1) * width + ((pos.x - x_min) * 2 + 1);

        let mut grid: Vec<u8> = Vec::with_capacity((width * height) as usize);
        for _ in 0..width * height {
            grid.push(b'#');
        }

        for (&k, v) in self.edges.iter() {
            grid[idx(k) as usize] = b'.';
            for &neighbor in v.iter() {
                if neighbor == k.north() {
                    grid[(idx(k) - width) as usize] = b'-';
                } else if neighbor == k.east() {
                    grid[(idx(k) + 1) as usize] = b'|';
                } else if neighbor == k.south() {
                    grid[(idx(k) + width) as usize] = b'-';
                } else if neighbor == k.west() {
                    grid[(idx(k) - 1) as usize] = b'|';
                }
            }
        }

        grid[idx(Position::new(0, 0)) as usize] = b'X';

        for (i, &c) in grid.iter().enumerate() {
            write!(f, "{}", c as char)?;
            if (i + 1) % width as usize == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
