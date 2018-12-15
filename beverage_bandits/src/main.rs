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

    let input: Vec<String> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let combat = Combat::create(&input);
    println!("{}", combat);

    combat.fight_round();
}

#[derive(Debug)]
enum CombatState {
    Ongoing,
    Finished,
}

struct Combat {
    grid: Grid,
}

impl Combat {
    fn create(input: &[String]) -> Self {
        Self {
            grid: Grid::create(&input),
        }
    }

    fn fight_round(&self) -> CombatState {
        let combat_order = self.combat_order();

        for unit in combat_order {
            let targets = self.grid.get_targets(&unit);
            if targets.is_empty() {
                // No targets left
                return CombatState::Finished;
            }
        }

        CombatState::Finished
    }

    fn combat_order(&self) -> Vec<Unit> {
        self.grid.all_units()
    }
}

impl std::fmt::Display for Combat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Debug, Clone, PartialEq)]
struct Unit {
    kind: UnitType,
    position: GridPosition,
}

impl Unit {
    fn new(kind: UnitType, x: usize, y: usize) -> Self {
        Self {
            kind,
            position: GridPosition { x, y },
        }
    }

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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug)]
enum Target {
    Adjacent(Unit),
    Open(GridPosition),
    Blocked(Unit),
}

#[derive(Debug, Clone, PartialEq)]
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
                'E' => Cell::Unit(Unit::new(UnitType::Elf, x, y)),
                'G' => Cell::Unit(Unit::new(UnitType::Goblin, x, y)),
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
        self.grid[pos.y * self.width + pos.x].clone()
    }

    fn all_units(&self) -> Vec<Unit> {
        self.grid
            .iter()
            .filter_map(|cell| match cell {
                Cell::Unit(unit) => Some(unit.clone()),
                _ => None,
            })
            .collect()
    }

    fn get_targets(&self, unit: &Unit) -> Vec<Target> {
        let mut targets = Vec::new();
        for enemy in self.grid.iter().filter_map(|cell| match cell {
            Cell::Unit(target) => {
                if unit.is_enemy_of(target) {
                    Some(target.clone())
                } else {
                    None
                }
            }
            _ => None,
        }) {
            if unit.is_adjacent_to(&enemy) {
                targets.push(Target::Adjacent(enemy));
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
                    targets.push(Target::Blocked(enemy));
                }
            }
        }
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
        assert_eq!(Some(Unit::new(UnitType::Goblin, 2, 1)), order.next());
        assert_eq!(Some(Unit::new(UnitType::Elf, 4, 1)), order.next());
        assert_eq!(Some(Unit::new(UnitType::Elf, 1, 2)), order.next());
        assert_eq!(Some(Unit::new(UnitType::Goblin, 3, 2)), order.next());
        assert_eq!(Some(Unit::new(UnitType::Elf, 5, 2)), order.next());
        assert_eq!(Some(Unit::new(UnitType::Goblin, 2, 3)), order.next());
        assert_eq!(Some(Unit::new(UnitType::Elf, 4, 3)), order.next());
        assert_eq!(None, order.next());
    }
}
