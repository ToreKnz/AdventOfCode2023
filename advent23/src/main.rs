use std::collections::{HashSet, HashMap};

const TEST_INPUT: bool = false;
type InputType<'a> = Vec<Vec<char>>;
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

    // let input2 = process_input_part_two(input_str.clone());
    let res2 = part_two(&input);
    println!("Solution part two: {:?}",res2);
}

fn process_input(input: &str) -> InputType {
    input.lines().filter(|line| !line.is_empty()).map(|line| line.chars().collect()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn max_path(mut seen: HashSet<Position>, grid: &Vec<Vec<char>>, mut start: Position) -> u32 {
    if start == (Position {x: grid.len() as i32 - 1, y: grid[0].len() as i32 - 2}) {
        return 0;
    }
    let mut val = 1;
    seen.insert(start);
    loop{
        let mut new_possible_positions: Vec<Position> = Vec::new();
        let possible_offsets: &[(i32,i32)] = match grid[start.x as usize][start.y as usize] {
            '>' => &[(0,1)],
            '<' => &[(0,-1)],
            'v' => &[(1,0)],
            '^' => &[(-1,0)],
            _ => &[(-1,0),(0,-1),(1,0),(0,1)],
        };
        for (dx, dy) in possible_offsets {
            let new_x = start.x + dx;
            let new_y = start.y + dy;
            if new_x >= 0 && new_x < grid.len() as i32 && new_y >= 0 && new_y < grid[0].len() as i32 && !seen.contains(&Position {x: new_x, y: new_y}) {
                match grid[new_x as usize][new_y as usize] {
                    '#' => {},
                    _ => {new_possible_positions.push(Position {x: new_x, y: new_y})},
                }
            }
        }
        if new_possible_positions.is_empty() {
            return 0;
        } else if new_possible_positions.len() > 1 {
            return val + new_possible_positions.iter().map(|new_position| max_path(seen.clone(), grid, *new_position)).max().unwrap();
        }
        else {
            start = new_possible_positions[0];
            if start == (Position {x: grid.len() as i32 - 1, y: grid[0].len() as i32 - 2}) {
                return val;
            }
            val += 1;
            seen.insert(start);
        }
    }
}

fn get_transitions(grid: &Vec<Vec<char>>) -> HashMap<Position, Vec<(Position, u32)>> {
    let mut seen = HashSet::new();
    let mut map: HashMap<Position, Vec<(Position, u32)>> = HashMap::new();
    seen.insert(Position{x: 0, y: 1});
    let mut key_positions = vec![Position{x: 0, y: 1}];
    while let Some(pos) = key_positions.pop() {
        for (dx, dy) in [(-1,0),(0,-1),(1,0),(0,1)] {
            let new_x = pos.x + dx;
            let new_y = pos.y + dy;
            if new_x >= 0 && new_x < grid.len() as i32 && new_y >= 0 && new_y < grid[0].len() as i32 && grid[new_x as usize][new_y as usize] != '#' {
                let mut t = HashSet::new();
                t.insert(pos);
                if let (Some(key_pos), val) = find_key_position(t, grid, Position { x: new_x, y: new_y }) {
                    let entry = map.entry(pos).or_insert(Vec::new());
                    entry.push((key_pos, val));
                    if !seen.contains(&key_pos) {
                        seen.insert(key_pos);
                        key_positions.push(key_pos);
                    }
                }
            }
        }
        seen.insert(pos);
    }
    map
}

fn find_key_position(mut seen: HashSet<Position>, grid: &Vec<Vec<char>>, mut start: Position) -> (Option<Position>, u32) {
    let mut val = 1;
    seen.insert(start);
    loop{
        let mut new_possible_positions: Vec<Position> = Vec::new();
        for (dx, dy) in [(-1,0),(0,-1),(1,0),(0,1)] {
            let new_x = start.x + dx;
            let new_y = start.y + dy;
            if new_x >= 0 && new_x < grid.len() as i32 && new_y >= 0 && new_y < grid[0].len() as i32 && !seen.contains(&Position {x: new_x, y: new_y}) {
                match grid[new_x as usize][new_y as usize] {
                    '#' => {},
                    _ => {new_possible_positions.push(Position {x: new_x, y: new_y})},
                }
            }
        }
        if new_possible_positions.is_empty() {
            return (None, 0)
        } else if new_possible_positions.len() > 1 {
            return (Some(start), val);
        }
        else {
            start = new_possible_positions[0];
            if start == (Position {x: grid.len() as i32 - 1, y: grid[0].len() as i32 - 2}) || start == (Position {x: 0, y: 1})
            {
                return (Some(start), val + 1);
            }
            val += 1;
            seen.insert(start);
        }
    }
}

fn max_path_by_map(map: &HashMap<Position,Vec<(Position, u32)>>, seen: &mut HashSet<Position>, grid: &Vec<Vec<char>>, start: Position) -> u32 {
    let mut seen_copy = seen.clone();
    if start == (Position {x: grid.len() as i32 - 1, y: grid[0].len() as i32 - 2}) {
        return 0;
    }
    seen_copy.insert(start);
    let mut new_possible_positions: Vec<(Position, u32)> = Vec::new();
    for (Position{x: new_x, y:new_y}, dist) in map.get(&start).unwrap_or(&Vec::new()) {
        if *new_x >= 0 && *new_x < grid.len() as i32 && *new_y >= 0 && *new_y < grid[0].len() as i32 && !seen_copy.contains(&Position {x: *new_x, y: *new_y}) {
            match grid[*new_x as usize][*new_y as usize] {
                '#' => {},
                _ => {new_possible_positions.push((Position {x: *new_x, y: *new_y}, *dist))},
            }
        }
    }
    if new_possible_positions.is_empty() {
        0
    } else {
        let v = new_possible_positions.iter().map(|(new_position, dist)| dist + max_path_by_map(map, &mut seen_copy, grid, *new_position)).max().unwrap();
        v
    }
}

fn part_one(lines: &InputType) -> ResultType {
    let seen = HashSet::new();
    max_path(seen, lines, Position {x: 0, y: 1})
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let map = get_transitions(lines);
    let mut seen = HashSet::new();
    max_path_by_map(&map, &mut seen, lines, Position { x: 0, y: 1 })
}
