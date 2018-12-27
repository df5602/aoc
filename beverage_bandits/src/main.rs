use std::collections::VecDeque;
use std::env;
use std::{thread, time};

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

    let visualize = false;
    let delay = time::Duration::from_millis(100);

    for power in 3.. {
        let mut combat = Combat::create(&input, power);
        if visualize {
            println!("{}", combat);
            thread::sleep(delay);
        }
        loop {
            let result = combat.fight_round();
            if result == CombatState::Finished {
                break;
            }
            if visualize {
                println!("After round {}", combat.completed_rounds);
                println!("{}", combat);
                thread::sleep(delay);
            }
        }

        let completed_round = combat.completed_rounds;
        let killed_elves = combat.killed_elves;
        let sum_of_hp = combat.calculate_sum_of_hit_points();
        println!(
            "Elf attack power: {}, Completed rounds: {}, Killed elves: {}, Sum of hit points: {} => {}",
            power,
            completed_round,
            killed_elves,
            sum_of_hp,
            completed_round * sum_of_hp
        );
        if visualize {
            thread::sleep(delay * 10);
        }

        if killed_elves == 0 {
            break;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum CombatState {
    Ongoing,
    Finished,
}

struct Combat {
    grid: Grid,
    completed_rounds: usize,
    killed_elves: usize,
}

impl Combat {
    fn create(input: &[String], attack_power_elves: usize) -> Self {
        Self {
            grid: Grid::create(&input, attack_power_elves),
            completed_rounds: 0,
            killed_elves: 0,
        }
    }

    fn fight_round(&mut self) -> CombatState {
        let combat_order = self.combat_order();

        for unit in combat_order {
            let mut unit = if let Some(unit) = self.grid.get_unit_by_id(unit.id) {
                unit
            } else {
                // Unit is already dead...
                continue;
            };

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
                    find_shortest_paths(&self.grid, unit.position, &move_candidates);

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
                    // Unit was killed, was it an elf?
                    if victim.is_elf() {
                        self.killed_elves += 1;
                    }
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    position: GridPosition,
    targets: &[GridPosition],
) -> Vec<ShortestPath> {
    let mut shortest_paths = Vec::new();

    // Initialize search grid
    let mut search_grid: Vec<(usize, GridPosition)> = Vec::with_capacity(grid.width * grid.height);
    for cell in grid.grid.iter() {
        match cell {
            Cell::Open => search_grid.push((0, GridPosition { x: 0, y: 0 })),
            Cell::Unit(_) | Cell::Wall => {
                search_grid.push((usize::max_value(), GridPosition { x: 0, y: 0 }))
            }
        }
    }

    // Priority queue of unvisited nodes
    let mut queue = VecDeque::new();
    queue.push_back(position);

    // BFS
    while let Some(pos) = queue.pop_front() {
        let this = search_grid[pos.y * grid.width + pos.x];

        // Above
        let above = &mut search_grid[(pos.y - 1) * grid.width + pos.x];
        if above.0 == 0 {
            queue.push_back(pos.above());
            above.0 = if pos == position { 1 } else { this.0 + 1 };
            above.1 = if pos == position { pos.above() } else { this.1 };
        }

        // Left
        let left = &mut search_grid[pos.y * grid.width + pos.x - 1];
        if left.0 == 0 {
            queue.push_back(pos.left());
            left.0 = if pos == position { 1 } else { this.0 + 1 };
            left.1 = if pos == position { pos.left() } else { this.1 };
        }

        // Right
        let right = &mut search_grid[pos.y * grid.width + pos.x + 1];
        if right.0 == 0 {
            queue.push_back(pos.right());
            right.0 = if pos == position { 1 } else { this.0 + 1 };
            right.1 = if pos == position { pos.right() } else { this.1 };
        }

        // Below
        let below = &mut search_grid[(pos.y + 1) * grid.width + pos.x];
        if below.0 == 0 {
            queue.push_back(pos.below());
            below.0 = if pos == position { 1 } else { this.0 + 1 };
            below.1 = if pos == position { pos.below() } else { this.1 };
        }
    }

    // Summarize results
    for target in targets {
        let t = search_grid[target.y * grid.width + target.x];
        if t.0 > 0 {
            shortest_paths.push(ShortestPath {
                target: *target,
                distance: t.0,
                initial_step: t.1,
            });
        }
    }

    shortest_paths.sort_by_key(|path| path.distance);
    shortest_paths
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Unit {
    id: usize,
    kind: UnitType,
    position: GridPosition,
    hit_points: isize,
    attack_power: usize,
}

impl Unit {
    fn new(
        id: usize,
        kind: UnitType,
        x: usize,
        y: usize,
        hit_points: isize,
        attack_power: usize,
    ) -> Self {
        Self {
            id,
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn create(input: &[String], attack_power_elves: usize) -> Self {
        let (width, height) = if !input.is_empty() {
            (input[0].len(), input.len())
        } else {
            (0, 0)
        };

        let mut grid = Vec::with_capacity(width * height);
        let mut x = 0;
        let mut y = 0;
        let mut unit_id = 0;

        input.iter().flat_map(|s| s.chars()).for_each(|c| {
            let cell = match c {
                '.' => Cell::Open,
                '#' => Cell::Wall,
                'E' => {
                    unit_id += 1;
                    Cell::Unit(Unit::new(
                        unit_id - 1,
                        UnitType::Elf,
                        x,
                        y,
                        200,
                        attack_power_elves,
                    ))
                }
                'G' => {
                    unit_id += 1;
                    Cell::Unit(Unit::new(unit_id - 1, UnitType::Goblin, x, y, 200, 3))
                }
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
                unit.id,
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
                victim.id,
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

    fn get_unit_by_id(&self, id: usize) -> Option<Unit> {
        self.grid
            .iter()
            .filter_map(|cell| match cell {
                Cell::Unit(unit) => Some(*unit),
                _ => None,
            })
            .filter(|unit| unit.id == id)
            .nth(0)
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " 0: ")?;
        let mut y = 0;
        for (i, c) in self.grid.iter().enumerate() {
            write!(f, "{}", c)?;
            if (i + 1) % self.width == 0 {
                y += 1;
                writeln!(f)?;
                if y < self.height {
                    if y < 10 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}: ", y)?;
                }
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
        let combat = Combat::create(&input, 3);
        let mut order = combat.combat_order().into_iter();
        assert_eq!(
            Some(Unit::new(0, UnitType::Goblin, 2, 1, 200, 3)),
            order.next()
        );
        assert_eq!(
            Some(Unit::new(1, UnitType::Elf, 4, 1, 200, 3)),
            order.next()
        );
        assert_eq!(
            Some(Unit::new(2, UnitType::Elf, 1, 2, 200, 3)),
            order.next()
        );
        assert_eq!(
            Some(Unit::new(3, UnitType::Goblin, 3, 2, 200, 3)),
            order.next()
        );
        assert_eq!(
            Some(Unit::new(4, UnitType::Elf, 5, 2, 200, 3)),
            order.next()
        );
        assert_eq!(
            Some(Unit::new(5, UnitType::Goblin, 2, 3, 200, 3)),
            order.next()
        );
        assert_eq!(
            Some(Unit::new(6, UnitType::Elf, 4, 3, 200, 3)),
            order.next()
        );
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
        let combat = Combat::create(&input, 3);
        let shortest_paths = find_shortest_paths(
            &combat.grid,
            GridPosition { x: 1, y: 1 },
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
                distance: 2,
                initial_step: GridPosition { x: 2, y: 1 },
            },
            shortest_paths[0]
        );
        assert_eq!(
            ShortestPath {
                target: GridPosition { x: 2, y: 2 },
                distance: 2,
                initial_step: GridPosition { x: 2, y: 1 },
            },
            shortest_paths[1]
        );
        assert_eq!(
            ShortestPath {
                target: GridPosition { x: 1, y: 3 },
                distance: 2,
                initial_step: GridPosition { x: 1, y: 2 },
            },
            shortest_paths[2]
        );
        assert_eq!(
            ShortestPath {
                target: GridPosition { x: 3, y: 3 },
                distance: 4,
                initial_step: GridPosition { x: 2, y: 1 },
            },
            shortest_paths[3]
        );
    }
}
