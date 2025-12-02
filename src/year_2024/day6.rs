use std::collections::HashSet;

use crate::parsing::read_lines;

// Occupancy Grid
type Grid = Vec<Vec<bool>>;

// Coordinates in the grid
type Coords = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Facing {
    North,
    East,
    South,
    West,
}
const FACINGS: [Facing; 4] = [Facing::North, Facing::East, Facing::South, Facing::West];

impl Facing {
    fn repr(&self) -> char {
        match self {
            Facing::North => '^',
            Facing::East => '>',
            Facing::South => 'v',
            Facing::West => '<',
        }
    }

    fn from_char(c: char) -> Option<Facing> {
        FACINGS.into_iter().find(|&f| f.repr() == c)
    }

    fn clockwise(&self) -> Facing {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }

    fn offset(&self) -> (i8, i8) {
        match self {
            Facing::North => (0, -1),
            Facing::East => (1, 0),
            Facing::South => (0, 1),
            Facing::West => (-1, 0),
        }
    }
}

#[derive(Clone, Debug)]
struct GameState {
    position: Coords,
    facing: Facing,
    grid: Grid,
}

impl GameState {
    pub fn h(&self) -> usize {
        self.grid.len()
    }
    pub fn w(&self) -> usize {
        self.grid[0].len()
    }

    #[allow(dead_code)] // Debug visualization.
    pub fn ascii_art(&self) -> String {
        let mut result = String::new();
        for y in 0..self.h() {
            for x in 0..self.w() {
                if self.position == (x, y) {
                    result += &self.facing.repr().to_string();
                } else if self.at((x, y)) {
                    result += "#";
                } else {
                    result += ".";
                }
            }
            result += "\n";
        }
        result
    }

    pub fn at(&self, xy: Coords) -> bool {
        self.grid[xy.1][xy.0]
    }

    fn position_forward(&self) -> Option<Coords> {
        // TODO(ggould) There's got to be a better way!
        let old_pos = self.position;
        let offset = self.facing.offset();
        let new_pos_as_i32: (i32, i32) = (
            i32::try_from(old_pos.0).ok()? + i32::from(offset.0),
            i32::try_from(old_pos.1).ok()? + i32::from(offset.1),
        );
        let new_pos: Coords = (
            usize::try_from(new_pos_as_i32.0).ok()?,
            usize::try_from(new_pos_as_i32.1).ok()?,
        );
        if new_pos.0 >= self.w() || new_pos.1 >= self.h() {
            None
        } else {
            Some(new_pos)
        }
    }

    pub fn advance_game_state(&self) -> Option<GameState> {
        let next_pos = self.position_forward();
        match next_pos {
            None => None, // Left the game board.
            Some(pos) => {
                if self.at(pos) {
                    // Collided; turn right
                    let mut result = self.clone();
                    result.facing = self.facing.clockwise();
                    Some(result)
                } else {
                    Some(GameState {
                        position: pos,
                        facing: self.facing,
                        grid: self.grid.clone(),
                    })
                }
            }
        }
    }

    fn create_grid(lines: &Vec<String>) -> Grid {
        let mut result: Grid = Vec::new();
        for line in lines {
            let grid_line: Vec<bool> = line.chars().map(|c| c == '#').collect();
            result.push(grid_line);
        }
        result
    }

    fn detect_guard(lines: &[String]) -> (Facing, Coords) {
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(f) = Facing::from_char(c) {
                    return (f, (x, y));
                }
            }
        }
        panic!("Did not find guard in room");
    }

    pub fn from_lines(lines: Vec<String>) -> GameState {
        let grid = GameState::create_grid(&lines);
        let (facing, position) = GameState::detect_guard(&lines);
        GameState {
            position,
            facing,
            grid,
        }
    }
}

/// For a given game state, the set of visited facings/coordinates before exit,
/// or None if it iterates without bound.
fn visited_set(start: &GameState) -> Option<HashSet<(Facing, Coords)>> {
    let mut visited: HashSet<(Facing, Coords)> = HashSet::new();
    let mut game = start.clone();
    loop {
        if visited.contains(&(game.facing, game.position)) {
            return None;
        }
        visited.insert((game.facing, game.position));
        match game.advance_game_state() {
            None => {
                break;
            }
            Some(s) => {
                game = s;
            }
        }
    }
    Some(visited)
}

pub fn day6(source: Option<String>) -> i32 {
    let lines = read_lines(source);
    let game = GameState::from_lines(lines);
    let visited = visited_set(&game);
    let visited_xy: HashSet<Coords> =
        HashSet::from_iter(visited.unwrap().iter().map(|(_, xy)| *xy));
    visited_xy.len().try_into().unwrap()
}

pub fn day6b(source: Option<String>) -> i32 {
    let lines = read_lines(source);
    let game = GameState::from_lines(lines);
    let possible_obstacle_locs = visited_set(&game).unwrap();
    let mut looping_obstacle_locs = HashSet::new();
    for (counter, (_, xy)) in possible_obstacle_locs.iter().enumerate() {
        let mut altered_game = game.clone();
        altered_game.grid[xy.1][xy.0] = true;
        if visited_set(&altered_game).is_none() {
            println!(
                "Found loop at {} {}; tested {} of {}",
                xy.0,
                xy.1,
                counter,
                possible_obstacle_locs.len()
            );
            looping_obstacle_locs.insert(xy);
        }
    }
    i32::try_from(looping_obstacle_locs.len()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day6(Some("data/day6_example.txt".to_string())), 41);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day6(Some("inputs/day6_test.txt".to_string())), 4903);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(day6b(Some("data/day6_example.txt".to_string())), 6);
    }

    // This test is not run as it requires a lot of time.
    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_b() {
        assert_eq!(day6b(Some("inputs/day6_test.txt".to_string())), 1911);
    }
}
