use std::collections::HashSet;

use crate::parsing::read_lines;
use nalgebra::Vector2;

type Scalar = i64;
type Xy = Vector2<Scalar>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BoardContent {
    Crate, BoxLeft, BoxRight, Wall, Robot, Empty
}

impl BoardContent {
    pub fn to_char(&self) -> char {
        match self {
            BoardContent::Crate => 'O',
            BoardContent::BoxLeft => '[',
            BoardContent::BoxRight => ']',
            BoardContent::Wall => '#',
            BoardContent::Robot => '@',
            BoardContent::Empty => '.',
        }
    }

    pub fn from_char(c: char) -> BoardContent {
        match c {
            'O' => Some(BoardContent::Crate),
            '#' => Some(BoardContent::Wall),
            '@' => Some(BoardContent::Robot),
            '.' => Some(BoardContent::Empty),
            _ => None,
        }.unwrap()
    }

    pub fn from_char_wide(c: char) -> Vec<BoardContent> {
        match c {
            'O' => Some(vec!(BoardContent::BoxLeft, BoardContent::BoxRight)),
            '#' => Some(vec!(BoardContent::Wall, BoardContent::Wall)),
            '@' => Some(vec!(BoardContent::Robot, BoardContent::Empty)),
            '.' => Some(vec!(BoardContent::Empty, BoardContent::Empty)),
            _ => None,
        }.unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North, East, South, West
}

impl Direction {
    pub fn to_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }

    pub fn to_offset(&self) -> Xy {
        match self {
            Direction::North => Xy::new(0, -1),
            Direction::East => Xy::new(1, 0),
            Direction::South => Xy::new(0, 1),
            Direction::West => Xy::new(-1, 0),
        }
    }

    pub fn from_char(c: char) -> Direction {
        match c {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            _ => None,
        }.unwrap()
    }
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<BoardContent>>,
    robot_location: Xy,
}

impl Board {
    pub fn at(&self, xy: Xy) -> BoardContent {
        self.board[xy[1] as usize][xy[0]as usize]
    }

    pub fn set_at(&mut self, xy: Xy, c: BoardContent) {
        self.board[xy[1] as usize][xy[0]as usize] = c;
    }

    pub fn to_strings(&self) -> Vec<String> {
        let mut result = Vec::new();
        for row in &self.board {
            result.push(String::from_iter(row.iter().map(|c| c.to_char())));
        }
        result
    }

    pub fn from_strings(strings: Vec<String>, wide: bool) -> Board {
        let mut result = Board{board:Vec::new(), robot_location:Xy::new(-1, -1)};
        for (y, row_string) in strings.iter().enumerate() {
            if wide {
                result.board.push(row_string.chars().map(BoardContent::from_char_wide).flatten().collect());
            } else {
                result.board.push(row_string.chars().map(BoardContent::from_char).collect());
            }
            if let Some(x) = row_string.find(BoardContent::Robot.to_char()) {
                let x = x * if wide {2} else {1};
                result.robot_location = Xy::new(x as Scalar, y as Scalar);
            }
        }
        assert!(result.at(result.robot_location) == BoardContent::Robot);
        result
    }

    pub fn score(&self) -> Scalar {
        let mut result: Scalar = 0;
        for (y, row) in self.board.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let x = x as Scalar;
                let y = y as Scalar;
                if vec![BoardContent::BoxLeft, BoardContent::Crate].contains(c) {
                    result += y * 100 + x;
                }
            }
        }
        result
    }

    pub fn get_move_zone(&self, m: Direction) -> HashSet<Xy> {
        // Repeatedly:
        //  * Identify things that must move (move zone)
        //  * Identify where they must go (frontier)
        //  * For everything in the frontier,
        //     * If it's empty, skip it.
        //     * If it's a wall, give up, nothing moves.
        //     * Add it to the move zone.
        //     * If it's a crate, add its partner too.
        let mut move_frontier = HashSet::<Xy>::new();
        let mut move_zone = HashSet::<Xy>::new();
        let offset = m.to_offset();
        move_frontier.insert(self.robot_location);
        while !move_frontier.is_empty() {
            move_zone.extend(move_frontier.iter());
            let mut new_frontier: HashSet<Xy> = move_zone.iter().map(|xy| xy + offset).collect();
            for &xy in move_frontier.clone().iter() {
                match self.at(xy) {
                    BoardContent::BoxLeft => {
                        new_frontier.insert(xy + Xy::new(1, 0));
                    }
                    BoardContent::BoxRight => {
                        new_frontier.insert(xy + Xy::new(-1, 0));
                    }
                    BoardContent::Wall => {
                        return HashSet::new();
                    }
                    _ => {}
                }
            }
            println!("New frontier is {:?}", new_frontier);
            println!("Old frontier is {:?}", move_frontier);
            move_frontier = new_frontier
                .difference(&move_zone).cloned()
                .filter(|&xy| self.at(xy) != BoardContent::Empty)
                .collect();
            println!("Revised frontier is {:?}", move_frontier);
        }
        move_zone
    }

    fn moved(&self, m: Direction) -> Board {
        let zone = self.get_move_zone(m);
        if zone.is_empty() { return self.clone() }
        let mut new_board = self.clone();
        for &xy in zone.iter() {
            new_board.set_at(xy, BoardContent::Empty);
        }
        for &xy in zone.iter() {
            new_board.set_at(xy + m.to_offset(), self.at(xy));
        }
        new_board.robot_location += m.to_offset();
        new_board
    }

    pub fn apply_move(&mut self, m: Direction) {
        *self = self.moved(m);
    }
}

pub fn day15(source: Option<String>) -> i64 {
    let lines = read_lines(source);
    let board_strings = lines.iter().filter(|l| l.starts_with("#")).cloned().collect();
    let mut board = Board::from_strings(board_strings, false);
    let moves_strings: Vec<String> = lines.iter().filter(|l| l.contains("^")).cloned().collect();
    let moves_string= moves_strings.join("");
    let moves: Vec<Direction> = moves_string.chars().map(Direction::from_char).collect();
    for m in moves {
        println!("{}", board.to_strings().join("\n"));
        println!("{} {:?}", m.to_char(), m.to_offset().transpose());
        board.apply_move(m);
    }
    board.score() as i64
}

pub fn day15b(source: Option<String>) -> i64 {
    let lines = read_lines(source);
    let board_strings = lines.iter().filter(|l| l.starts_with("#")).cloned().collect();
    let mut board = Board::from_strings(board_strings, true);
    let moves_strings: Vec<String> = lines.iter().filter(|l| l.contains("^")).cloned().collect();
    let moves_string= moves_strings.join("");
    let moves: Vec<Direction> = moves_string.chars().map(Direction::from_char).collect();
    for m in moves {
        println!("{}", board.to_strings().join("\n"));
        println!("{} {:?}", m.to_char(), m.to_offset().transpose());
        board.apply_move(m);
    }
    board.score() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_small() {
        assert_eq!(day15(Some("data/day15_example_small.txt".to_string())), 2028);
    }

    #[test]
    fn test_example() {
        assert_eq!(day15(Some("data/day15_example.txt".to_string())), 10092);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day15(Some("inputs/day15_test.txt".to_string())), 1495147);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(day15b(Some("data/day15_example.txt".to_string())), 9021);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_b() {
        assert_eq!(day15b(Some("inputs/day15_test.txt".to_string())), 1524905);
    }

    // B cannot be tested.
}
