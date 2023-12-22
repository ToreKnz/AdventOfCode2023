use core::panic;
use std::collections::BTreeSet;

const TEST_INPUT: bool = false;
type InputType<'a> = Vec<Vec<u32>>;
type InputType2<'a> = InputType<'a>;
type ResultType = u32;
type ResultType2 = u64;

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
    println!("Solution part two: {:?}",res2);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct PositionAdvanced {
    x: i32,
    y: i32,
    x_offset: i32,
    y_offset: i32,
}

fn process_input(input: &str) -> InputType {
    input.lines().filter(|line| !line.is_empty()).map(|line| line.chars().map(|c| match c {
        '.' => 0,
        '#' => 1,
        'S' => 2,
        _ => panic!("rip")
    }).collect()).collect()
}

fn position_is_valid(pos: &Position, grid: &Vec<Vec<u32>>) -> bool {
    pos.x >= 0 && pos.x < grid.len() as i32  && pos.y >= 0 && pos.y < grid[0].len() as i32 && grid[pos.x as usize][pos.y as usize] == 0
}

fn position_is_valid_new(pos: &PositionAdvanced, grid: &[Vec<u32>]) -> bool {
    grid[pos.x as usize][pos.y as usize] == 0
}

fn modulus(x: i32,y: i32) -> i32 {
    let rem = x % y;
    if rem < 0 {
        rem + y
    } else {
        rem
    }
}

fn part_one(lines: &InputType) -> ResultType {
    let mut start_position = Position {x:0, y:0};
    let mut lines = lines.clone();
    'outer: for x in 0..lines.len() {
        for y in 0..lines[0].len() {
            if lines[x][y] == 2 {
                start_position = Position {x: x as i32, y: y as i32};
                lines[x][y] = 0;
                break 'outer;
            }
        }
    }
    let mut possible_positions: BTreeSet<Position> = BTreeSet::new();
    possible_positions.insert(start_position);
    for _ in 0..64 {
        let mut new_possible_positions: BTreeSet<Position> = BTreeSet::new();
        for &position in &possible_positions {
            for (dx, dy) in [(1,0),(0,1),(-1,0),(0,-1)] {
                let new_position = Position {x: position.x + dx, y: position.y + dy};
                if position_is_valid(&new_position, &lines) {
                    new_possible_positions.insert(new_position);
                }
            }
        }
        possible_positions = new_possible_positions;
    }
    possible_positions.len() as u32
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut start_position = PositionAdvanced {x:0, y:0, x_offset:0, y_offset: 0};
    let mut lines = lines.clone();
    let mut possible_positions: BTreeSet<PositionAdvanced> = BTreeSet::new();
    for x in 0..lines.len() {
        for y in 0..lines[0].len() {
            if lines[x][y] == 2 {
                start_position = PositionAdvanced {x: x as i32, y: y as i32, x_offset: 0 ,y_offset: 0};
                lines[x][y] = 0;
            }
        }
    }
    let mut a: i64 = 0;
    let mut b: i64 = 0;
    let mut c: i64 = 0;
    possible_positions.insert(start_position);
    for i in 1..10_000 {
        let mut new_possible_positions: BTreeSet<PositionAdvanced> = BTreeSet::new();
        for position in possible_positions {
            for (dx, dy) in [(1,0),(0,1),(-1,0),(0,-1)] {
                let new_position = PositionAdvanced {x: modulus(position.x + dx, lines.len() as i32), y: modulus(position.y + dy, lines[0].len() as i32),
                x_offset: position.x_offset + match position.x + dx {
                    x if x < 0 => {
                        -1
                    }
                    x if x >= lines.len() as i32 => {1},
                    _ => 0,
                }, y_offset: position.y_offset + match position.y + dy {
                    y if y < 0 => {-1},
                    y if y >= lines[0].len() as i32 => {1},
                    _ => 0,
                }};
                if position_is_valid_new(&new_position, &lines) {
                    new_possible_positions.insert(new_position);
                }
            }
        }
        possible_positions = new_possible_positions;
        if i == lines.len() / 2 {
            c = possible_positions.len() as i64;
        }
        else if i == lines.len() / 2 + lines.len() {
            b = possible_positions.len() as i64;
        }
        else if i == lines.len() / 2 + 2* lines.len() {
            a = possible_positions.len() as i64;
            break;
        }
    }
    
    let val = (26_501_365 - lines.len()/2) / lines.len();
    (c as f64
        +(-3.0 * (c as f64/2.0) + 2.0*b as f64 - (a as f64/2.0)) * val as f64
        +(c as f64/2.0 - b as f64 + a as f64/2.0) * val as f64 * val as f64) as u64
}
