use std::collections::{HashMap, HashSet};

const TEST_INPUT: bool = false;
type InputType<'a> = Vec<Vec<u8>>;
type InputType2<'a> = InputType<'a>;
type ResultType = u32;
type ResultType2 = ResultType;

fn main() {
    let input_str = if TEST_INPUT {
        include_str!("test_input.txt")
    } else {
        include_str!("input.txt")
    };
    let input = process_input(input_str.clone());

    let res = part_one(&input);
    println!("Solution part one: {:?}", res);

    let res2 = part_two(&input);
    println!("Solution part two: {:?}", res2);
}

fn process_input(input: &str) -> InputType {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|num| num.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
enum Direction {
    Top,
    Left,
    Right,
    Bottom,
}
use Direction::*;
impl Direction {
    fn adjacent(self) -> &'static [Direction] {
        match self {
            Top => &[Top, Left, Right],
            Left => &[Left, Bottom, Top],
            Right => &[Right, Top, Bottom],
            Bottom => &[Bottom, Left, Right],
        }
    }

    fn values() -> &'static [Direction] {
        &[Top, Left, Right, Bottom]
    }
}

fn pos_in_dir(pos: Position, dir: Direction) -> Position {
    match dir {
        Top => Position {
            x: pos.x - 1,
            y: pos.y,
        },
        Left => Position {
            x: pos.x,
            y: pos.y - 1,
        },
        Right => Position {
            x: pos.x,
            y: pos.y + 1,
        },
        Bottom => Position {
            x: pos.x + 1,
            y: pos.y,
        },
    }
}

fn get_new_positions(
    old_pos: Position,
    dir: Direction,
    amt: u32,
    grid: &Vec<Vec<u8>>,
) -> Vec<(Position, Direction, u32)> {
    let mut res = Vec::new();
    for &d in dir.adjacent() {
        if d != dir {
            let new_pos = pos_in_dir(old_pos, d);
            if pos_is_valid(new_pos, grid) {
                res.push((new_pos, d, 1));
            }
        } else if amt < 3 {
            let new_pos = pos_in_dir(old_pos, d);
            if pos_is_valid(new_pos, grid) {
                res.push((new_pos, d, amt + 1));
            }
        }
    }
    res
}

fn get_new_positions_v2(
    old_pos: Position,
    dir: Direction,
    amt: u32,
    grid: &Vec<Vec<u8>>,
) -> Vec<(Position, Direction, u32)> {
    let mut res = Vec::new();
    if amt < 4 {
        let new_pos = pos_in_dir(old_pos, dir);
        if pos_is_valid(new_pos, grid) {
            res.push((new_pos, dir, amt + 1));
        }
    } else {
        for &d in dir.adjacent() {
            if d != dir {
                let new_pos = pos_in_dir(old_pos, d);
                if pos_is_valid(new_pos, grid) {
                    res.push((new_pos, d, 1));
                }
            } else if amt < 10 {
                let new_pos = pos_in_dir(old_pos, d);
                if pos_is_valid(new_pos, grid) {
                    res.push((new_pos, d, amt + 1));
                }
            }
        }
    }
    res
}

fn pos_is_valid(pos: Position, grid: &Vec<Vec<u8>>) -> bool {
    0 <= pos.x && pos.x < grid.len() as i32 && 0 <= pos.y && pos.y < grid[0].len() as i32
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn part_one(lines: &InputType) -> ResultType {
    dijkstra_verbessert(lines)
}

fn dijkstra_verbessert(grid: &Vec<Vec<u8>>) -> u32 {
    let mut dist = HashMap::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            for &dir in Direction::values() {
                for val in 1..4 {
                    dist.insert(
                        (
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                            dir,
                            val,
                        ),
                        None::<u32>,
                    );
                }
            }
        }
    }
    let mut seen = HashSet::new();
    let mut nodes = vec![(Position { x: 0, y: 0 }, Right, 0)];
    for node in get_new_positions(Position { x: 0, y: 0 }, Right, 0, grid) {
        nodes.push(node);
        seen.insert(node);
    }
    dist.insert((Position { x: 0, y: 0 }, Right, 0), Some(0));
    while !nodes.is_empty() {
        if nodes.len() % 100 == 0 {
            println!("nodes {}", nodes.len());
        }
        let (&min_dist_node, val) = nodes
            .iter()
            .map(|x| (x, dist.get(x).unwrap().unwrap_or(u32::MAX)))
            .min_by(|&(&_, x2), &(&_, y2)| x2.cmp(&y2))
            .unwrap();
        if val == u32::MAX {
            break;
        }
        let amt = min_dist_node.2;
        for i in amt..4 {
            let rem_idx = nodes
                .iter()
                .enumerate()
                .find(|(_, node)| **node == (min_dist_node.0, min_dist_node.1, i));
            if let Some(node) = rem_idx {
                seen.insert(*(node.1));
                nodes.remove(node.0);
            }
        }
        let (pos, dir, amt) = min_dist_node;
        for node in get_new_positions(pos, dir, amt, grid) {
            if !nodes.contains(&node) && !seen.contains(&node) {
                nodes.push(node);
            }

            let dist_min_node = val + grid[node.0.x as usize][node.0.y as usize] as u32;
            if dist_min_node < dist.get(&node).unwrap().unwrap_or(u32::MAX) {
                dist.insert(node, Some(dist_min_node));
            }
        }
    }
    dist.iter()
        .filter(|(x, _)| {
            x.0 == Position {
                x: grid.len() as i32 - 1,
                y: grid[0].len() as i32 - 1,
            }
        })
        .map(|(_, y)| y.unwrap_or(u32::MAX))
        .min()
        .unwrap()
}

fn part_two(lines: &InputType2) -> ResultType2 {
    dijkstra_verbessert_v2(lines)
}

fn dijkstra_verbessert_v2(grid: &Vec<Vec<u8>>) -> u32 {
    let mut dist = HashMap::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            for &dir in Direction::values() {
                for val in 1..11 {
                    dist.insert(
                        (
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                            dir,
                            val,
                        ),
                        None::<u32>,
                    );
                }
            }
        }
    }
    let mut seen = HashSet::new();
    let mut nodes = vec![
        (Position { x: 0, y: 0 }, Right, 0),
        (Position { x: 0, y: 0 }, Bottom, 0),
    ];
    for node in get_new_positions_v2(Position { x: 0, y: 0 }, Right, 0, grid) {
        nodes.push(node);
        seen.insert(node);
    }
    for node in get_new_positions_v2(Position { x: 0, y: 0 }, Bottom, 0, grid) {
        nodes.push(node);
        seen.insert(node);
    }
    dist.insert((Position { x: 0, y: 0 }, Right, 0), Some(0));
    dist.insert((Position { x: 0, y: 0 }, Bottom, 0), Some(0));
    while !nodes.is_empty() {
        if nodes.len() % 100 == 0 {
            println!("nodes {}", nodes.len());
        }
        let (&min_dist_node, val) = nodes
            .iter()
            .map(|x| (x, dist.get(x).unwrap().unwrap_or(u32::MAX)))
            .min_by(|&(&_, x2), &(&_, y2)| x2.cmp(&y2))
            .unwrap();
        if val == u32::MAX {
            break;
        }
        let amt = min_dist_node.2;
        if amt >= 4 {
            for i in amt..11 {
                let rem_idx = nodes
                    .iter()
                    .enumerate()
                    .find(|(_, node)| **node == (min_dist_node.0, min_dist_node.1, i));
                if let Some(node) = rem_idx {
                    seen.insert(*(node.1));
                    nodes.remove(node.0);
                }
            }
        } else {
            let rem_idx = nodes
                .iter()
                .enumerate()
                .find(|(_, node)| **node == min_dist_node)
                .unwrap()
                .0;
            seen.insert(min_dist_node);
            nodes.remove(rem_idx);
        }
        let (pos, dir, amt) = min_dist_node;
        for node in get_new_positions_v2(pos, dir, amt, grid) {
            if !nodes.contains(&node) && !seen.contains(&node) {
                nodes.push(node);
            }
            let dist_min_node = val + grid[node.0.x as usize][node.0.y as usize] as u32;
            if dist_min_node < dist.get(&node).unwrap().unwrap_or(u32::MAX) {
                dist.insert(node, Some(dist_min_node));
            }
        }
    }
    dist.iter()
        .filter(|(x, _)| {
            x.0 == Position {
                x: grid.len() as i32 - 1,
                y: grid[0].len() as i32 - 1,
            } && x.2 >= 4
        })
        .map(|(_, y)| y.unwrap_or(u32::MAX))
        .min()
        .unwrap()
}
