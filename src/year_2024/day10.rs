use std::{collections::{HashMap, HashSet}, iter};

use crate::parsing::read_lines;

type Elevation = i8;
type Coords = (usize, usize);
type Map = Vec<Vec<Elevation>>;
type Adjacency = HashSet<(Coords, Coords)>;


fn make_map(input: &Vec<String>) -> Map {
    let mut result = Map::new();
    for line in input {
        result.push(
            line.chars().map(|c| c.to_string().parse::<i8>().unwrap()).collect());
    }
    result
}

fn make_adjacency(map: &Map) -> Adjacency {
    let mut raw_adjacency: Adjacency = Adjacency::new();
    let h = map.len();
    let w = map[0].len();
    for y in 0..h {
        for x in 0..w {
            if x > 0 { raw_adjacency.insert(((x, y), (x-1, y))); }
            if y > 0 { raw_adjacency.insert(((x, y), (x, y-1))); }
            if x < w-1 { raw_adjacency.insert(((x, y), (x+1, y))); }
            if y < h-1 { raw_adjacency.insert(((x, y), (x, y+1))); }
        }
    }
    let raw_adjacency = raw_adjacency;  // Drop mutability.
    let mut result = HashSet::new();
    for from_to in raw_adjacency {
        let ((from_x, from_y), (to_x, to_y)) = from_to;
        if map[from_y][from_x] == map[to_y][to_x] - 1 {
            result.insert(from_to);
        }
    }
    result
}

fn make_successors(adjacency: &Adjacency) -> HashMap<Coords, HashSet<Coords>> {
    let mut result: HashMap<Coords, HashSet<Coords>> = HashMap::new();
    for (from, to) in adjacency {
        result.entry(*from).or_default().insert(*to);
    }
    result
}

fn all_of(map: &Map, elevation: Elevation) -> HashSet<Coords> {
    let mut result = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, found_elevation) in row.iter().enumerate() {
            if *found_elevation == elevation {
                result.insert((x, y));
            }
        }
    }
    result
}

fn reachable(map: &Map, start: &Coords) -> HashSet<Coords> {
    let adjacency = make_adjacency(map);
    let successors = make_successors(&adjacency);
    // Simple breadth-first search of the adjacency graph.
    let mut worklist: Vec<Coords> = vec![*start];
    let mut visited: HashSet<Coords> = HashSet::new();
    while let Some(from) = worklist.pop() {
        if successors.contains_key(&from) {
            for to in successors[&from].iter() {
                worklist.push(*to);
            }
        }
        visited.insert(from);
    }
    visited
}

fn paths(successors: &HashMap<Coords, HashSet<Coords>>,
         start: &Coords,
         ends: &HashSet<Coords>) -> Vec<Vec<Coords>> {
    if ends.contains(start) {
        return vec![Vec::new()];
    }
    let mut result = Vec::new();
    let empty = HashSet::new();
    for successor in successors.get(start).unwrap_or(&empty).iter() {
        for suffix in paths(successors, successor, ends) {
            result.push(iter::once(start).chain(suffix.iter()).copied().collect());
        }
    }
    println!("Found paths {:?}", result);
    result
}

fn all_paths(map: &Vec<Vec<i8>>) -> Vec<Vec<Coords>> {
    let starts = all_of(map, 0);
    let ends = all_of(map, 9);
    let adjacency = make_adjacency(map);
    let successors = make_successors(&adjacency);
    let mut result = Vec::new();
    for start in starts {
        result.extend(paths(&successors, &start, &ends));
    }
    result
}

pub fn day10(source: Option<String>) -> i64 {
    let lines = read_lines(source);
    let map = make_map(&lines);
    let starts = all_of(&map, 0);
    let ends = all_of(&map, 9);
    let mut score: i64 = 0;
    for s in starts {
        let reachable_ends: HashSet<Coords> =
            reachable(&map, &s).intersection(&ends).copied().collect();
        score += i64::try_from(reachable_ends.len()).unwrap();
    }
    score
}

pub fn day10b(source: Option<String>) -> i64 {
    let lines = read_lines(source);
    let map = make_map(&lines);
    let paths = all_paths(&map);
    i64::try_from(paths.len()).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day10(Some("data/day10_example.txt".to_string())), 36);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day10(Some("inputs/day10_test.txt".to_string())), 489);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(day10b(Some("data/day10_example.txt".to_string())), 81);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_b() {
        assert_eq!(day10b(Some("inputs/day10_test.txt".to_string())), 1086);
    }
}
