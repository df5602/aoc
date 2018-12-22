extern crate util;

use std::collections::VecDeque;
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

    let depth = input[0]
        .split("depth: ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .nth(0)
        .unwrap();

    let mut target = input[1]
        .split("target: ")
        .flat_map(|s| s.split(','))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap());
    let target = (target.next().unwrap(), target.next().unwrap());

    println!("depth = {}", depth);
    println!("target = ({},{})", target.0, target.1);

    let grid = Grid::new(depth, target, (target.0 + 10, target.1 + 10));
    let total_risk_level = grid.total_risk_level(target);
    println!("Total risk level: {}", total_risk_level);

    let map = grid.to_map();
    let shortest_path = map.shortest_path((0, 0), target);
    println!("Shortest path: {} minutes", shortest_path);
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<usize>,
}

impl Grid {
    fn new(depth: usize, target: (usize, usize), dimensions: (usize, usize)) -> Self {
        assert!(target.0 <= dimensions.0);
        assert!(target.1 <= dimensions.1);
        let width = dimensions.0 + 1;
        let height = dimensions.1 + 1;
        let mut grid = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let gi = if (x == 0 && y == 0) || (x == target.0 && y == target.1) {
                    0
                } else if y == 0 {
                    x * 16807
                } else if x == 0 {
                    y * 48271
                } else {
                    let el_left = grid[y * width + x - 1];
                    let el_above = grid[(y - 1) * width + x];
                    el_left * el_above
                };
                let el = (gi + depth) % 20183;
                grid.push(el);
            }
        }

        Self {
            width,
            height,
            grid,
        }
    }

    fn total_risk_level(&self, target: (usize, usize)) -> usize {
        let mut sum = 0;
        for y in 0..=target.1 {
            for x in 0..=target.0 {
                let el = self.grid[y * self.width + x];
                sum += el % 3;
            }
        }
        sum
    }

    fn to_map(&self) -> Map {
        let mut map = Vec::with_capacity(self.width * self.height);
        for cell in self.grid.iter() {
            let region = match cell % 3 {
                0 => Region::Rocky,
                1 => Region::Wet,
                2 => Region::Narrow,
                _ => unreachable!(),
            };
            map.push(region);
        }
        Map {
            width: self.width,
            height: self.height,
            map,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Region {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Equipment {
    Torch,
    ClimbingGear,
    Neither,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct SearchState {
    last_x: usize,
    last_y: usize,
    pos_x: usize,
    pos_y: usize,
    time: usize,
    equipment: Equipment,
}

struct Map {
    width: usize,
    height: usize,
    map: Vec<Region>,
}

impl Map {
    fn shortest_path(&self, start: (usize, usize), target: (usize, usize)) -> usize {
        assert!(start.0 < self.width);
        assert!(start.1 < self.height);
        assert!(target.0 < self.width);
        assert!(target.1 < self.height);

        // Lowest time
        let mut lowest_search_time = usize::max_value();

        // Search grid
        let mut search_grid = Vec::with_capacity(self.width * self.height);
        for _ in 0..self.width * self.height {
            search_grid.push((usize::max_value(), Equipment::Torch));
        }

        // Queue of to be evaluated positions
        let mut queue: VecDeque<SearchState> = VecDeque::new();

        queue.push_back(SearchState {
            last_x: start.0,
            last_y: start.1,
            pos_x: start.0,
            pos_y: start.1,
            time: 0,
            equipment: Equipment::Torch,
        });

        while let Some(state) = queue.pop_front() {
            let search_pos = &mut search_grid[state.pos_y * self.width + state.pos_x];

            // Found target
            if state.pos_x == target.0 && state.pos_y == target.1 {
                let mut search_time = state.time;
                if state.equipment != Equipment::Torch {
                    search_time += 7;
                }
                if search_time < lowest_search_time {
                    lowest_search_time = search_time;
                }
            } else if (state.time >= lowest_search_time)
                || (search_pos.0.saturating_add(7) <= state.time)
                || (search_pos.1 == state.equipment && search_pos.0 <= state.time)
            {
                // Found shorter path to here or target already, no need to continue
            } else {
                let next_states = self.find_all_neighbor_states(state);
                for state in next_states {
                    queue.push_back(state);
                }
            }

            if state.time < search_pos.0 {
                search_pos.0 = state.time;
                search_pos.1 = state.equipment;
            }
        }

        lowest_search_time
    }

    fn find_all_neighbor_states(&self, state: SearchState) -> Vec<SearchState> {
        let mut next_states = Vec::new();

        // Move up
        if state.pos_y > 0 && state.last_y != state.pos_y - 1 {
            let next_state = self.get_next_state(state, state.pos_x, state.pos_y - 1);
            next_states.push(next_state);
        }
        // Move left
        if state.pos_x > 0 && state.last_x != state.pos_x - 1 {
            let next_state = self.get_next_state(state, state.pos_x - 1, state.pos_y);
            next_states.push(next_state);
        }
        // Move right
        if state.pos_x + 1 < self.width && state.last_x != state.pos_x + 1 {
            let next_state = self.get_next_state(state, state.pos_x + 1, state.pos_y);
            next_states.push(next_state);
        }
        // Move down
        if state.pos_y + 1 < self.height && state.last_y != state.pos_y + 1 {
            let next_state = self.get_next_state(state, state.pos_x, state.pos_y + 1);
            next_states.push(next_state);
        }

        next_states
    }

    fn get_next_state(&self, state: SearchState, next_x: usize, next_y: usize) -> SearchState {
        let next_equipment = match (
            self.map[state.pos_y * self.width + state.pos_x],
            self.map[next_y * self.width + next_x],
            state.equipment,
        ) {
            (Region::Rocky, Region::Rocky, equipment) => equipment,
            (Region::Rocky, Region::Wet, _) => Equipment::ClimbingGear,
            (Region::Rocky, Region::Narrow, _) => Equipment::Torch,
            (Region::Wet, Region::Wet, equipment) => equipment,
            (Region::Wet, Region::Rocky, _) => Equipment::ClimbingGear,
            (Region::Wet, Region::Narrow, _) => Equipment::Neither,
            (Region::Narrow, Region::Narrow, equipment) => equipment,
            (Region::Narrow, Region::Wet, _) => Equipment::Neither,
            (Region::Narrow, Region::Rocky, _) => Equipment::Torch,
        };

        let switch_penalty = if next_equipment != state.equipment {
            7
        } else {
            0
        };

        SearchState {
            last_x: state.pos_x,
            last_y: state.pos_y,
            pos_x: next_x,
            pos_y: next_y,
            time: state.time + 1 + switch_penalty,
            equipment: next_equipment,
        }
    }
}
