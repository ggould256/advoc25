#![allow(dead_code)]

use std::fmt::Debug;
use nalgebra::Vector2;

pub type Scalar = i64;
pub type Xy = Vector2<Scalar>;

pub fn xy_range(ul: Xy, br: Xy) -> Vec<Xy> {
    let mut result = Vec::new();
    for y in ul[1]..br[1] {
        for x in ul[0]..br[0] {
            result.push(Xy::new(x, y));
        }
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub fn to_char(self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }

    pub fn to_offset(self) -> Xy {
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
        }
        .unwrap()
    }

    pub fn cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Board<BoardContent> {
    pub(crate) board: Vec<Vec<BoardContent>>,
}

impl<BoardContent> Debug for Board<BoardContent>
where BoardContent: Copy + TryInto<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            let row_string: String = row
                .iter()
                .map(|c| {
                    let ch: char = (*c).try_into().unwrap_or('?');
                    ch
                })
                .collect();
            writeln!(f, "{}", row_string)?;
        }
        Ok(())
    }
}

impl<BoardContent> Board<BoardContent>
where BoardContent: Copy + Debug + PartialEq,
{
    pub fn new(rows: usize, cols: usize, default_value: BoardContent) -> Self {
        Board {
            board: vec![vec![default_value; cols]; rows],
        }
    }

    pub fn height(&self) -> usize {
        self.board.len()
    }
    pub fn width(&self) -> usize {
        self.board[0].len()
    }
    pub fn dimensions(&self) -> Xy {
        Xy::new(self.width() as Scalar, self.height() as Scalar)
    }

    pub fn all_coords(&self) -> Vec<Xy> {
        let mut result = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                result.push(Xy::new(x as Scalar, y as Scalar));
            }
        }
        result
    }

    pub fn coords_of(&self, content: BoardContent) -> Vec<Xy> {
        let mut result = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.at(Xy::new(x as Scalar, y as Scalar)) == content {
                    result.push(Xy::new(x as Scalar, y as Scalar));
                }
            }
        }
        result
    }

    pub fn at(&self, xy: Xy) -> BoardContent {
        self.board[xy[1] as usize][xy[0] as usize]
    }

    pub fn mut_at(&mut self, xy: Xy) -> &mut BoardContent {
        &mut self.board[xy[1] as usize][xy[0] as usize]
    }

    pub fn maybe_at(&self, xy: Xy) -> Option<BoardContent> {
        if xy[0] < 0
            || xy[0] >= self.width() as Scalar
            || xy[1] < 0
            || xy[1] >= self.height() as Scalar
        {
            None
        } else {
            Some(self.board[xy[1] as usize][xy[0] as usize])
        }
    }

    pub fn set_at(&mut self, xy: Xy, c: BoardContent) {
        self.board[xy[1] as usize][xy[0] as usize] = c;
    }
        
    pub fn maybe_set_at(&mut self, xy: Xy, c: BoardContent) {
        if self.maybe_at(xy).is_some() {
            self.set_at(xy, c);
        }
    }

    pub fn fill_rect(&mut self, ul: Xy, br: Xy, c: BoardContent) {
        for y in ul[1]..br[1] {
            self.board[y as usize][ul[0] as usize..br[0] as usize].fill(c);
        }
    }

    pub fn row(&self, y: usize) -> Vec<BoardContent> {
        self.board[y].clone()
    }

    pub fn update_rect(&mut self, ul: Xy, br: Xy, f: impl Fn(BoardContent) -> BoardContent) {
        for y in ul[1]..br[1] {
            for x in ul[0]..br[0] {
                let xy = Xy::new(x, y);
                let old_value = self.at(xy);
                let new_value = f(old_value);
                self.set_at(xy, new_value);
            }
        }
    }

    pub fn to_strings(&self) -> Vec<String>
    where BoardContent: ToString,
    {
        let mut result = Vec::new();
        for row in &self.board {
            result.push(String::from_iter(row.iter().map(|c| c.to_string())));
        }
        result
    }

    pub fn from_strings(strings: &Vec<String>) -> Board<BoardContent>
    where BoardContent: TryFrom<char, Error: Debug>,
    {
        let mut result = Board { board: Vec::new() };
        for row_string in strings {
            result.board.push(
                row_string
                    .chars()
                    .map(|c| BoardContent::try_from(c).unwrap())
                    .collect(),
            );
        }
        result
    }

    pub fn opt_from_strs(strs: &Vec<&str>) -> Board<Option<BoardContent>>
    where
        BoardContent: TryFrom<char, Error: Debug>,
    {
        let mut result = Board { board: Vec::new() };

        fn char_to_opt<BoardContent>(
            c: char,
        ) -> Option<BoardContent>
        where
            BoardContent: TryFrom<char, Error: Debug>,
        {
            match BoardContent::try_from(c) {
                Ok(value) => Some(value),
                Err(_) => None,
            }
        }

        for row_string in strs {
            result.board.push(
                row_string
                    .chars()
                    .map(|c| char_to_opt::<BoardContent>(c))
                    .collect(),
            );
        }
        result
    }

    pub fn neighbors(&self) -> Vec<BoardContent> {
        let mut result = Vec::new();
        for dir in Direction::ALL.iter() {
            let offset = dir.to_offset();
            if let Some(c) = self.maybe_at(offset) {
                result.push(c);
            }
        }
        result
    }

    pub fn neighbors8(&self, loc: Xy) -> Vec<BoardContent> {
        let mut result = Vec::new();
        let dirs8: Vec<Xy> = vec![
            Xy::new(1, 0),
            Xy::new(1, 1),
            Xy::new(0, 1),
            Xy::new(-1, 1),
            Xy::new(-1, 0),
            Xy::new(-1, -1),
            Xy::new(0, -1),
            Xy::new(1, -1),
        ];
        for &offset in dirs8.iter() {
            if let Some(c) = self.maybe_at(loc + offset) {
                result.push(c);
            }
        }
        result
    }

    pub fn count(&self, target: BoardContent) -> usize {
        let mut result = 0;
        for row in &self.board {
            for &cell in row {
                if cell == target {
                    result += 1;
                }
            }
        }
        result
    }

    /// Scan the board for occurrences of the given "needle" board.
    /// The needle may contain `None` values, which are treated as wildcards
    pub fn scan(&self, needle: &Board<Option<BoardContent>>) -> Board<bool>
    where BoardContent: Eq,
    {
        let new_w = self.width() - needle.width() + 1;
        let new_h = self.height() - needle.height() + 1;
        let mut result = Board {
            board: vec![vec![true; new_w]; new_h],
        };
        for result_xy in result.all_coords() {
            let [result_x, result_y] = result_xy.into();
            for needle_xy in needle.all_coords() {
                let [needle_x, needle_y] = needle_xy.into();
                let haystack_xy = Xy::new(result_x + needle_x, result_y + needle_y);
                let needle_cell = needle.at(needle_xy);
                if let Some(needle_cell_value) = needle_cell {
                    let haystack_cell = self.at(haystack_xy);
                    if haystack_cell != needle_cell_value {
                        result.set_at(result_xy, false);
                        break;
                    }
                }
            }
        }
        result
    }

    /// Replace multiple locations on the board according to the given
    /// replacements board and locations board.
    /// The replacements board may contain `None` values, which are left unchanged.
    /// If multiple replacements overlap, precedence is unspecified.
    pub fn replace_many(&self,
      replacement: &Board<Option<BoardContent>>,
      locs: &Board<bool>)
      -> Board<BoardContent> {
        assert_eq!(self.dimensions(), locs.dimensions() + replacement.dimensions() - Xy::new(1, 1));
        let mut result = self.clone();
        for xy in locs.all_coords() {
            let [x, y] = xy.into();
            if !locs.at(xy) {
                continue;
            }
            for offset in replacement.all_coords() {
                let [offset_x, offset_y] = offset.into();
                let target_xy = Xy::new(
                    x + offset_x,
                    y + offset_y,
                );
                if let Some(new_value) = replacement.at(offset) {
                    result.set_at(target_xy, new_value);
                }
            }
        }
        result
    }

    /// A cellular-automaton-style transformation of the board.
    /// For each location where `template` has a non-None value,
    /// replace that location with the corresponding value from `replacements`.
    pub fn transform(&self,
      template: &Board<Option<BoardContent>>,
      replacement: &Board<Option<BoardContent>>) -> Board<BoardContent>
    where BoardContent: Eq,
    {
        self.replace_many(
            replacement,
            &self.scan(template),
        )
    }

    pub fn iter(&self) -> BoardIterator<BoardContent> {
        BoardIterator {
            board: self,
            current_x: 0,
            current_y: 0,
        }
    }
}

impl std::fmt::Display for Board<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        for row in &self.board {
            for &cell in row {
                result.push(if cell { '#' } else { '.' });
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

pub struct BoardIterator<'a, BoardContent> {
    board: &'a Board<BoardContent>,
    current_x: usize,
    current_y: usize,
}
impl<BoardContent> Iterator for BoardIterator<'_, BoardContent>
where
    BoardContent: Copy + Debug + ToString + PartialEq,
    BoardContent: TryFrom<char, Error: Debug>,
{
    type Item = (Xy, BoardContent);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.board.height() {
            return None;
        }
        let xy = Xy::new(self.current_x as Scalar, self.current_y as Scalar);
        let content = self.board.at(xy);
        self.current_x += 1;
        if self.current_x >= self.board.width() {
            self.current_x = 0;
            self.current_y += 1;
        }
        Some((xy, content))
    }
}

