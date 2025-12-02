use std::collections::{HashMap, HashSet};

use crate::parsing::read_lines;

type Channel = char;
type CoordScalar = i32;
fn as_coord_scalar(x: usize) -> CoordScalar {
    x.try_into().unwrap()
}
type Coords = (CoordScalar, CoordScalar);

fn find_all_antennas(lines: Vec<String>) -> Vec<(Channel, Coords)> {
    let mut result = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                result.push((c, (as_coord_scalar(x), as_coord_scalar(y))));
            }
        }
    }
    result
}

fn find_antenna_pairs(antennas: Vec<(Channel, Coords)>) -> Vec<(Channel, Coords, Coords)> {
    let mut channel_coords: HashMap<char, Vec<Coords>> = HashMap::new();
    let mut result = Vec::new();
    for (channel, coords) in antennas {
        let coords_list = channel_coords.entry(channel).or_default();
        for existing_coord in coords_list.iter() {
            result.push((channel, *existing_coord, coords));
        }
        coords_list.push(coords);
    }
    result
}

fn find_antinode_pairs(antennas: Vec<(Channel, Coords, Coords)>) -> HashSet<Coords> {
    let mut result = HashSet::new();
    for (c_, (x1, y1), (x2, y2)) in antennas {
        //         A -- B
        //     *             *
        //  A + (A - B)   B + (B - A)
        println!(
            "Channel {} produced an antinode at {},{}",
            c_,
            2 * x1 - x2,
            2 * y1 - y2
        );
        println!(
            "Channel {} produced an antinode at {},{}",
            c_,
            2 * x2 - x1,
            2 * y2 - y1
        );
        result.insert((2 * x1 - x2, 2 * y1 - y2));
        result.insert((2 * x2 - x1, 2 * y2 - y1));
    }
    result
}

fn find_antinode_rays(
    antennas: Vec<(Channel, Coords, Coords)>,
    w: CoordScalar,
    h: CoordScalar,
) -> HashSet<Coords> {
    let mut result = HashSet::new();
    for (_c, (x1, y1), (x2, y2)) in antennas {
        for i in 0.. {
            let new_x = x1 + i * (x1 - x2);
            let new_y = y1 + i * (y1 - y2);
            if new_x < 0 || new_x >= w || new_y < 0 || new_y >= h {
                break;
            }
            result.insert((new_x, new_y));
        }
        for i in 0.. {
            let new_x = x1 + -i * (x1 - x2);
            let new_y = y1 + -i * (y1 - y2);
            if new_x < 0 || new_x >= w || new_y < 0 || new_y >= h {
                break;
            }
            result.insert((new_x, new_y));
        }
    }
    result
}

pub fn day8(source: Option<String>) -> i64 {
    let lines = read_lines(source);
    let h: CoordScalar = as_coord_scalar(lines.len());
    let w: CoordScalar = as_coord_scalar(lines[0].len());
    let antennas = find_all_antennas(lines);
    let pairs = find_antenna_pairs(antennas);
    let linear_antinodes: HashSet<Coords> = find_antinode_pairs(pairs);
    let filtered_result: HashSet<Coords> = linear_antinodes
        .iter()
        .filter(|c| c.0 >= 0 && c.0 < w && c.1 >= 0 && c.1 < h)
        .copied()
        .collect();
    filtered_result.len().try_into().unwrap()
}

pub fn day8b(source: Option<String>) -> i64 {
    let lines = read_lines(source);
    let h: CoordScalar = as_coord_scalar(lines.len());
    let w: CoordScalar = as_coord_scalar(lines[0].len());
    let antennas = find_all_antennas(lines);
    let pairs = find_antenna_pairs(antennas);
    let linear_antinodes: HashSet<Coords> = find_antinode_rays(pairs, w, h);
    let filtered_result: HashSet<Coords> = linear_antinodes
        .iter()
        .filter(|c| c.0 >= 0 && c.0 < w && c.1 >= 0 && c.1 < h)
        .copied()
        .collect();
    filtered_result.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day8(Some("data/day8_example.txt".to_string())), 14);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day8(Some("inputs/day8_test.txt".to_string())), 344);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(day8b(Some("data/day8_example.txt".to_string())), 34);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_b() {
        assert_eq!(day8b(Some("inputs/day8_test.txt".to_string())), 1182);
    }
}
