use std::collections::{HashMap, VecDeque};
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

    let mut graph = Graph::new();
    graph.build(&input);

    let (farthest_node, farthest_dist) = graph.find_farthest_node(Position::new(0, 0));
    let distance_at_least_1000 = graph.find_nodes_farther_than(Position::new(0, 0), 1000);

    println!("{}", graph);
    println!(
        "Farthest node: {:?}, distance {}",
        farthest_node, farthest_dist
    );
    println!("Nodes farther than 1000: {}", distance_at_least_1000.len());
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
struct Item<'a> {
    chars: &'a [u8],
    cursor: usize,
    position: Position,
}

impl<'a> Item<'a> {
    fn copy_from(item: &Item<'a>) -> Item<'a> {
        Item { ..*item }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for ch in self.chars {
            print!("{}", *ch as char);
        }
        println!();
        for _ in 0..self.cursor {
            print!(" ");
        }
        println!("^");
    }
}

impl<'a> Eq for Item<'a> {}

impl<'a> PartialEq for Item<'a> {
    fn eq(&self, other: &Item<'_>) -> bool {
        self.cursor == other.cursor && self.position == other.position
    }
}

impl<'a> Ord for Item<'a> {
    fn cmp(&self, other: &Item<'_>) -> std::cmp::Ordering {
        if self.cursor == other.cursor {
            self.position.cmp(&other.position)
        } else {
            self.cursor.cmp(&other.cursor)
        }
    }
}

impl<'a> PartialOrd for Item<'a> {
    fn partial_cmp(&self, other: &Item<'_>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct SortedQueueSet<'a> {
    queue: VecDeque<Item<'a>>,
}

impl<'a> SortedQueueSet<'a> {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.queue.len()
    }

    fn pop_front(&mut self) -> Option<Item<'a>> {
        self.queue.pop_front()
    }

    fn insert(&mut self, item: Item<'a>) {
        if self.is_empty() || *self.queue.back().unwrap() < item {
            self.queue.push_back(item);
        } else {
            for i in 0..self.queue.len() {
                let curr = &self.queue[i];
                if *curr >= item {
                    if *curr == item {
                        break;
                    } else {
                        self.queue.insert(i, item);
                        break;
                    }
                }
            }
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

        self.parse(Position::new(0, 0), input[1..].as_bytes());
    }

    fn find_farthest_node(&self, from: Position) -> (Position, usize) {
        let distances = self.calculate_distances(from);

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

    fn find_nodes_farther_than(&self, from: Position, threshold: usize) -> Vec<(Position, usize)> {
        let distances = self.calculate_distances(from);

        let mut nodes = Vec::new();
        for (&k, &v) in distances.iter() {
            if v >= threshold {
                nodes.push((k, v));
            }
        }

        nodes
    }

    fn calculate_distances(&self, from: Position) -> HashMap<Position, usize> {
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
        while let Some(pos) = queue.pop_front() {
            // Lookup neighbors
            let current_dist = distances[&pos];
            let neighbors = &self.edges[&pos];
            for &neighbor in neighbors {
                let distance = distances.get_mut(&neighbor).unwrap();
                if *distance == usize::max_value() {
                    // Node not visited yet, update distance and add to queue
                    *distance = current_dist + 1;
                    queue.push_back(neighbor);
                }
            }
        }

        distances
    }

    fn parse(&mut self, start_position: Position, chars: &[u8]) {
        let mut items = SortedQueueSet::new();
        items.insert(Item {
            chars,
            cursor: 0,
            position: start_position,
        });

        loop {
            if items.is_empty() {
                break;
            }

            let current_item = items.pop_front().unwrap();
            let new_items = self.process(current_item);
            for item in new_items {
                items.insert(item);
            }
        }
    }

    fn process<'a>(&mut self, item: Item<'a>) -> Vec<Item<'a>> {
        let mut items = Vec::new();

        let c = item.chars[item.cursor];
        match c {
            b'N' => {
                self.add_edge(item.position, item.position.north());
                let mut new_item = Item::copy_from(&item);
                new_item.cursor += 1;
                new_item.position = item.position.north();
                items.push(new_item);
            }
            b'E' => {
                self.add_edge(item.position, item.position.east());
                let mut new_item = Item::copy_from(&item);
                new_item.cursor += 1;
                new_item.position = item.position.east();
                items.push(new_item);
            }
            b'S' => {
                self.add_edge(item.position, item.position.south());
                let mut new_item = Item::copy_from(&item);
                new_item.cursor += 1;
                new_item.position = item.position.south();
                items.push(new_item);
            }
            b'W' => {
                self.add_edge(item.position, item.position.west());
                let mut new_item = Item::copy_from(&item);
                new_item.cursor += 1;
                new_item.position = item.position.west();
                items.push(new_item);
            }
            b'(' => {
                let mut new_item = Item::copy_from(&item);
                new_item.cursor += 1;
                items.push(new_item);

                let mut parens = 1;
                for i in item.cursor + 1..item.chars.len() {
                    match item.chars[i] {
                        b'(' => parens += 1,
                        b')' => {
                            parens -= 1;
                            if parens == 0 {
                                break;
                            }
                        }
                        b'|' if parens == 1 => {
                            let mut new_item = Item::copy_from(&item);
                            new_item.cursor = i + 1;
                            items.push(new_item);
                        }
                        _ => {}
                    }
                }
            }
            b'|' => {
                let mut parens = 1;
                for i in item.cursor + 1..item.chars.len() {
                    match item.chars[i] {
                        b'(' => parens += 1,
                        b')' => {
                            parens -= 1;
                            if parens == 0 {
                                let mut new_item = Item::copy_from(&item);
                                new_item.cursor = i + 1;
                                items.push(new_item);
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                assert_eq!(0, parens);
            }
            b')' => {
                let mut new_item = Item::copy_from(&item);
                new_item.cursor += 1;
                items.push(new_item);
            }
            b'$' => {
                // nothing to do (item will be consumed, but not generate any new ones)
            }
            c => panic!("invalid token {}", c),
        }

        items
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
