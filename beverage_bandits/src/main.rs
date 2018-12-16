extern crate util;

use std::env;
//use std::io::BufRead;

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

    let mut combat = Combat::create(&input);
    println!("{}", combat);
    loop {
        let result = combat.fight_round();
        if result == CombatState::Finished {
            break;
        }
        println!("After round {}", combat.completed_rounds);
        println!("{}", combat);
        //let mut input_buffer = String::new();
        //let _ = std::io::stdin().lock().read_line(&mut input_buffer);
    }

    let completed_round = combat.completed_rounds;
    let sum_of_hp = combat.calculate_sum_of_hit_points();
    println!(
        "Completed rounds: {}, Sum of hit points: {} => {}",
        completed_round,
        sum_of_hp,
        completed_round * sum_of_hp
    );
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum CombatState {
    Ongoing,
    Finished,
}

struct Combat {
    grid: Grid,
    completed_rounds: usize,
}

impl Combat {
    fn create(input: &[String]) -> Self {
        Self {
            grid: Grid::create(&input),
            completed_rounds: 0,
        }
    }

    fn fight_round(&mut self) -> CombatState {
        let combat_order = self.combat_order();
        let mut dead_units = Vec::new();

        for unit in combat_order {
            let mut unit = unit;

            // Is unit still alive?
            if dead_units.contains(&unit) {
                continue;
            }

            let targets = self.grid.get_targets(&unit);

            // Targets left?
            if targets.is_empty() {
                return CombatState::Finished;
            }

            // Targets reachable?
            if targets.iter().all(|target| match target {
                Target::Blocked(_) => true,
                _ => false,
            }) {
                continue;
            }

            // Move, if necessary
            if !targets.iter().any(|target| match target {
                Target::Adjacent(_) => true,
                _ => false,
            }) {
                let move_candidates: Vec<GridPosition> = targets
                    .iter()
                    .filter_map(|target| match target {
                        Target::Open(pos) => Some(*pos),
                        _ => None,
                    })
                    .collect();
                let shortest_paths =
                    find_shortest_paths(&self.grid, &unit.position, &move_candidates);

                // Move found?
                if shortest_paths.is_empty()
                    || shortest_paths[0].distance == isize::max_value() as usize
                {
                    continue;
                }

                let next_position = shortest_paths[0].initial_step;
                self.grid.move_unit(&unit, &next_position);
                unit.position = next_position;
            }

            // Attack, if possible
            if let Some(victim) = self.select_victim(&unit) {
                if self.grid.attack(&unit, &victim) {
                    dead_units.push(victim);
                }
            }
        }

        self.completed_rounds += 1;
        CombatState::Ongoing
    }

    fn calculate_sum_of_hit_points(&self) -> usize {
        self.grid
            .all_units()
            .iter()
            .map(|unit| unit.hit_points)
            .fold(0, |sum, hp| sum + hp as usize)
    }

    fn combat_order(&self) -> Vec<Unit> {
        self.grid.all_units()
    }

    fn select_victim(&self, unit: &Unit) -> Option<Unit> {
        let mut min_hit_points = isize::max_value();
        let mut victim = None;
        if let Cell::Unit(enemy) = self.grid.at(&unit.position.above()) {
            if enemy.is_enemy_of(&unit) && enemy.hit_points < min_hit_points {
                min_hit_points = enemy.hit_points;
                victim = Some(enemy);
            }
        }
        if let Cell::Unit(enemy) = self.grid.at(&unit.position.left()) {
            if enemy.is_enemy_of(&unit) && enemy.hit_points < min_hit_points {
                min_hit_points = enemy.hit_points;
                victim = Some(enemy);
            }
        }
        if let Cell::Unit(enemy) = self.grid.at(&unit.position.right()) {
            if enemy.is_enemy_of(&unit) && enemy.hit_points < min_hit_points {
                min_hit_points = enemy.hit_points;
                victim = Some(enemy);
            }
        }
        if let Cell::Unit(enemy) = self.grid.at(&unit.position.below()) {
            if enemy.is_enemy_of(&unit) && enemy.hit_points < min_hit_points {
                victim = Some(enemy);
            }
        }

        victim
    }
}

impl std::fmt::Display for Combat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct ShortestPath {
    target: GridPosition,
    distance: usize,
    initial_step: GridPosition,
}

fn find_shortest_paths(
    grid: &Grid,
    position: &GridPosition,
    targets: &[GridPosition],
) -> Vec<ShortestPath> {
    let mut shortest_paths = Vec::new();
    for target in targets {
        // Initialize search grid
        let mut search_grid: Vec<isize> = Vec::with_capacity(grid.width * grid.height);
        for cell in grid.grid.iter() {
            match cell {
                Cell::Open => search_grid.push(0),
                Cell::Unit(_) | Cell::Wall => search_grid.push(isize::max_value()),
            }
        }

        // Make self position non-blocked
        search_grid[position.y * grid.width + position.x] = 0;

        // Search shortest path
        let (dist, _, step) =
            find_shortest_paths_internal(&mut search_grid, &position, target, grid.width, 0);
        assert!(dist > 0);
        shortest_paths.push(ShortestPath {
            target: *target,
            distance: dist as usize,
            initial_step: step,
        });
    }

    shortest_paths.sort_by_key(|path| path.distance);
    shortest_paths
}

fn find_shortest_paths_internal(
    mut search_grid: &mut Vec<isize>,
    position: &GridPosition,
    target: &GridPosition,
    width: usize,
    depth: usize,
) -> (isize, GridPosition, GridPosition) {
    let value = search_grid[position.y * width + position.x];

    // Abort if already visited (but not yet a result) or blocked
    if value < 0 || value == isize::max_value() {
        return (value, *position, *position);
    }

    // Abort if target found
    if position == target {
        return (1, *position, *position);
    }

    // Abort if we have already evaluated this cell
    if value > 0 {
        return (value + 1, *position, *position);
    }

    search_grid[position.y * width + position.x] = -1;
    let mut min = (isize::max_value(), *position, *position);
    let above = find_shortest_paths_internal(
        &mut search_grid,
        &position.above(),
        target,
        width,
        depth + 1,
    );
    if above.0 > 0 && above.0 < min.0 {
        min = above;
    }
    if depth == 0 {
        clean_visited_cells(&mut search_grid);
    }
    let left =
        find_shortest_paths_internal(&mut search_grid, &position.left(), target, width, depth + 1);
    if left.0 > 0 && left.0 < min.0 {
        min = left;
    }
    if depth == 0 {
        clean_visited_cells(&mut search_grid);
    }
    let right = find_shortest_paths_internal(
        &mut search_grid,
        &position.right(),
        target,
        width,
        depth + 1,
    );
    if right.0 > 0 && right.0 < min.0 {
        min = right;
    }
    if depth == 0 {
        clean_visited_cells(&mut search_grid);
    }
    let below = find_shortest_paths_internal(
        &mut search_grid,
        &position.below(),
        target,
        width,
        depth + 1,
    );
    if below.0 > 0 && below.0 < min.0 {
        min = below;
    }

    min.2 = min.1;
    min.1 = *position;

    if min.0 < isize::max_value() {
        search_grid[position.y * width + position.x] = min.0;
        clean_neighbour_cells(&mut search_grid, &position, width)
    }

    if min.0 < isize::max_value() {
        min.0 += 1;
    }
    min
}

fn clean_visited_cells(search_grid: &mut Vec<isize>) {
    search_grid
        .iter_mut()
        .filter(|c| **c < isize::max_value())
        .for_each(|c| *c = 0);
}

fn clean_neighbour_cells(search_grid: &mut Vec<isize>, position: &GridPosition, width: usize) {
    if search_grid[(position.y - 1) * width + position.x] < 0 {
        search_grid[(position.y - 1) * width + position.x] = 0;
    }
    if search_grid[position.y * width + position.x - 1] < 0 {
        search_grid[position.y * width + position.x - 1] = 0;
    }
    if search_grid[position.y * width + position.x + 1] < 0 {
        search_grid[position.y * width + position.x + 1] = 0;
    }
    if search_grid[(position.y + 1) * width + position.x] < 0 {
        search_grid[(position.y + 1) * width + position.x] = 0;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Unit {
    kind: UnitType,
    position: GridPosition,
    hit_points: isize,
    attack_power: usize,
}

impl Unit {
    fn new(kind: UnitType, x: usize, y: usize, hit_points: isize, attack_power: usize) -> Self {
        Self {
            kind,
            position: GridPosition { x, y },
            hit_points,
            attack_power,
        }
    }

    #[allow(dead_code)]
    fn is_goblin(&self) -> bool {
        match self.kind {
            UnitType::Elf => false,
            UnitType::Goblin => true,
        }
    }

    fn is_elf(&self) -> bool {
        match self.kind {
            UnitType::Elf => true,
            UnitType::Goblin => false,
        }
    }

    fn is_adjacent_to(&self, other: &Unit) -> bool {
        self.position.is_adjacent_to(&other.position)
    }

    fn is_enemy_of(&self, other: &Unit) -> bool {
        self.kind != other.kind
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Open,
    Wall,
    Unit(Unit),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Open => write!(f, "."),
            Cell::Wall => write!(f, "#"),
            Cell::Unit(unit) => {
                if unit.is_elf() {
                    write!(f, "E")
                } else {
                    write!(f, "G")
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Target {
    Adjacent(Unit),
    Open(GridPosition),
    Blocked(Unit),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct GridPosition {
    x: usize,
    y: usize,
}

impl GridPosition {
    fn is_adjacent_to(&self, other: &GridPosition) -> bool {
        (other.y == self.y && (other.x + 1 == self.x || self.x + 1 == other.x))
            || (other.x == self.x && (other.y + 1 == self.y || self.y + 1 == other.y))
    }

    fn above(&self) -> GridPosition {
        GridPosition {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn below(&self) -> GridPosition {
        GridPosition {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> GridPosition {
        GridPosition {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> GridPosition {
        GridPosition {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl Ord for GridPosition {
    fn cmp(&self, other: &GridPosition) -> std::cmp::Ordering {
        if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl PartialOrd for GridPosition {
    fn partial_cmp(&self, other: &GridPosition) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
        let mut x = 0;
        let mut y = 0;
        input.iter().flat_map(|s| s.chars()).for_each(|c| {
            let cell = match c {
                '.' => Cell::Open,
                '#' => Cell::Wall,
                'E' => Cell::Unit(Unit::new(UnitType::Elf, x, y, 200, 3)),
                'G' => Cell::Unit(Unit::new(UnitType::Goblin, x, y, 200, 3)),
                _ => panic!("unexpected input!"),
            };
            grid.push(cell);

            x += 1;
            if x >= width {
                x = 0;
                y += 1;
            }
        });

        Self {
            width,
            height,
            grid,
        }
    }

    fn at(&self, pos: &GridPosition) -> Cell {
        self.grid[pos.y * self.width + pos.x]
    }

    fn move_unit(&mut self, unit: &Unit, position: &GridPosition) {
        let unit =
            if let Cell::Unit(unit) = self.grid[unit.position.y * self.width + unit.position.x] {
                unit
            } else {
                panic!("Original position was not occupied by unit!");
            };

        if let Cell::Open = self.grid[position.y * self.width + position.x] {
            self.grid[position.y * self.width + position.x] = Cell::Unit(Unit::new(
                unit.kind,
                position.x,
                position.y,
                unit.hit_points,
                unit.attack_power,
            ));
            self.grid[unit.position.y * self.width + unit.position.x] = Cell::Open;
        } else {
            panic!("Move to occupied position: {:?} -> {:?}", unit, position);
        }
    }

    fn attack(&mut self, attacker: &Unit, victim: &Unit) -> bool {
        let hit_points = victim.hit_points - attacker.attack_power as isize;
        if hit_points <= 0 {
            self.grid[victim.position.y * self.width + victim.position.x] = Cell::Open;
            true
        } else {
            self.grid[victim.position.y * self.width + victim.position.x] = Cell::Unit(Unit::new(
                victim.kind,
                victim.position.x,
                victim.position.y,
                hit_points,
                victim.attack_power,
            ));
            false
        }
    }

    fn all_units(&self) -> Vec<Unit> {
        self.grid
            .iter()
            .filter_map(|cell| match cell {
                Cell::Unit(unit) => Some(*unit),
                _ => None,
            })
            .collect()
    }

    fn get_targets(&self, unit: &Unit) -> Vec<Target> {
        let mut targets = Vec::new();
        for enemy in self.grid.iter().filter_map(|cell| match cell {
            Cell::Unit(target) => {
                if unit.is_enemy_of(target) {
                    Some(target)
                } else {
                    None
                }
            }
            _ => None,
        }) {
            if unit.is_adjacent_to(&enemy) {
                targets.push(Target::Adjacent(*enemy));
            } else {
                let mut open_spaces = 0;
                if self.at(&enemy.position.above()) == Cell::Open {
                    open_spaces += 1;
                    targets.push(Target::Open(enemy.position.above()));
                }
                if self.at(&enemy.position.left()) == Cell::Open {
                    open_spaces += 1;
                    targets.push(Target::Open(enemy.position.left()));
                }
                if self.at(&enemy.position.right()) == Cell::Open {
                    open_spaces += 1;
                    targets.push(Target::Open(enemy.position.right()));
                }
                if self.at(&enemy.position.below()) == Cell::Open {
                    open_spaces += 1;
                    targets.push(Target::Open(enemy.position.below()));
                }
                if open_spaces == 0 {
                    targets.push(Target::Blocked(*enemy));
                }
            }
        }
        targets.sort_by(|a, b| match (a, b) {
            (Target::Adjacent(a), Target::Adjacent(b)) => a.position.cmp(&b.position),
            (Target::Adjacent(_), _) => std::cmp::Ordering::Less,
            (Target::Open(a), Target::Open(b)) => a.cmp(b),
            (Target::Open(_), Target::Adjacent(_)) => std::cmp::Ordering::Greater,
            (Target::Open(_), Target::Blocked(_)) => std::cmp::Ordering::Less,
            (Target::Blocked(a), Target::Blocked(b)) => a.position.cmp(&b.position),
            (Target::Blocked(_), _) => std::cmp::Ordering::Greater,
        });
        targets
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
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combat_order() {
        let input = vec![
            "#######".to_string(),
            "#.G.E.#".to_string(),
            "#E.G.E#".to_string(),
            "#.G.E.#".to_string(),
            "#######".to_string(),
        ];
        let combat = Combat::create(&input);
        let mut order = combat.combat_order().into_iter();
        assert_eq!(
            Some(Unit::new(UnitType::Goblin, 2, 1, 200, 3)),
            order.next()
        );
        assert_eq!(Some(Unit::new(UnitType::Elf, 4, 1, 200, 3)), order.next());
        assert_eq!(Some(Unit::new(UnitType::Elf, 1, 2, 200, 3)), order.next());
        assert_eq!(
            Some(Unit::new(UnitType::Goblin, 3, 2, 200, 3)),
            order.next()
        );
        assert_eq!(Some(Unit::new(UnitType::Elf, 5, 2, 200, 3)), order.next());
        assert_eq!(
            Some(Unit::new(UnitType::Goblin, 2, 3, 200, 3)),
            order.next()
        );
        assert_eq!(Some(Unit::new(UnitType::Elf, 4, 3, 200, 3)), order.next());
        assert_eq!(None, order.next());
    }

    #[test]
    fn test_shortest_path() {
        /*Targets:      In range:     Reachable:    Nearest:      Chosen:
        #######       #######       #######       #######       #######
        #E..G.#       #E.?G?#       #E.@G.#       #E.!G.#       #E.+G.#
        #...#.#  -->  #.?.#?#  -->  #.@.#.#  -->  #.!.#.#  -->  #...#.#
        #.G.#G#       #?G?#G#       #@G@#G#       #!G.#G#       #.G.#G#
        #######       #######       #######       #######       #######*/

        let input = vec![
            "#######".to_string(),
            "#E..G.#".to_string(),
            "#...#.#".to_string(),
            "#.G.#G#".to_string(),
            "#######".to_string(),
        ];
        let combat = Combat::create(&input);
        let shortest_paths = find_shortest_paths(
            &combat.grid,
            &GridPosition { x: 1, y: 1 },
            &[
                GridPosition { x: 3, y: 1 },
                GridPosition { x: 5, y: 1 },
                GridPosition { x: 2, y: 2 },
                GridPosition { x: 5, y: 2 },
                GridPosition { x: 1, y: 3 },
                GridPosition { x: 3, y: 3 },
            ],
        );

        assert_eq!(
            ShortestPath {
                target: GridPosition { x: 3, y: 1 },
                distance: 3,
                initial_step: GridPosition { x: 2, y: 1 },
            },
            shortest_paths[0]
        );
        assert_eq!(
            ShortestPath {
                target: GridPosition { x: 2, y: 2 },
                distance: 3,
                initial_step: GridPosition { x: 2, y: 1 },
            },
            shortest_paths[1]
        );
        assert_eq!(
            ShortestPath {
                target: GridPosition { x: 1, y: 3 },
                distance: 3,
                initial_step: GridPosition { x: 1, y: 2 },
            },
            shortest_paths[2]
        );
        assert_eq!(
            ShortestPath {
                target: GridPosition { x: 3, y: 3 },
                distance: 5,
                initial_step: GridPosition { x: 2, y: 1 },
            },
            shortest_paths[3]
        );
        assert_eq!(
            ShortestPath {
                target: GridPosition { x: 5, y: 1 },
                distance: isize::max_value() as usize,
                initial_step: GridPosition { x: 1, y: 1 },
            },
            shortest_paths[4]
        );
        assert_eq!(
            ShortestPath {
                target: GridPosition { x: 5, y: 2 },
                distance: isize::max_value() as usize,
                initial_step: GridPosition { x: 1, y: 1 },
            },
            shortest_paths[5]
        );
    }
}
