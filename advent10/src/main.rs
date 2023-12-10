use std::collections::HashSet;

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

    let res2 = part_two(&input);
    println!("Solution part two: {:?}", res2);
}

fn process_input(input: &str) -> InputType {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Top,
    Left,
    Right,
    Bottom,
}
use Direction::*;
impl Direction {
    fn values() -> [Direction; 4] {
        [Top, Left, Right, Bottom]
    }

    fn to_offset(self) -> Position {
        match self {
            Top => Position { x: -1, y: 0 },
            Left => Position { x: 0, y: -1 },
            Right => Position { x: 0, y: 1 },
            Bottom => Position { x: 1, y: 0 },
        }
    }

    fn apply_offset(self, pos: Position) -> Position {
        let (x, y) = (pos.x, pos.y);
        let offset = self.to_offset();
        let (dx, dy) = (offset.x, offset.y);
        Position {
            x: x + dx,
            y: y + dy,
        }
    }

    fn invert(self) -> Self {
        match self {
            Top => Bottom,
            Bottom => Top,
            Left => Right,
            Right => Left,
        }
    }

    fn relative_left(self) -> Self {
        match self {
            Top => Left,
            Left => Bottom,
            Bottom => Right,
            Right => Top,
        }
    }

    fn relative_right(self) -> Self {
        self.relative_left().invert()
    }

    fn get_step_dir(pos1: Position, pos2: Position) -> Self {
        let x = pos2.x - pos1.x;
        let y = pos2.y - pos1.y;
        match (x, y) {
            (-1, 0) => Top,
            (0, -1) => Left,
            (0, 1) => Right,
            (1, 0) => Bottom,
            _ => panic!("no direction found"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn pos_in_bounds(data: &Vec<Vec<char>>, pos: Position) -> bool {
    pos.x >= 0 && pos.x < data.len() as i32 && pos.y >= 0 && pos.y < data[0].len() as i32
}

fn pipe_is_connected(pipe: char, dir: Direction) -> bool {
    match pipe {
        '|' => [Top, Bottom].contains(&dir),
        '-' => [Left, Right].contains(&dir),
        'L' => [Top, Right].contains(&dir),
        'J' => [Top, Left].contains(&dir),
        '7' => [Left, Bottom].contains(&dir),
        'F' => [Bottom, Right].contains(&dir),
        'S' => true,
        _ => false,
    }
}

fn part_one(lines: &InputType) -> ResultType {
    let mut start_position = Position { x: 0, y: 0 };
    'search: for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            if *c == 'S' {
                start_position.x = x as i32;
                start_position.y = y as i32;
                break 'search;
            }
        }
    }
    let mut changed = true;
    let mut connected = HashSet::new();
    let mut newly_connected = HashSet::new();
    connected.insert(start_position);
    newly_connected.insert(start_position);
    let mut steps: i32 = -1;
    while changed {
        steps += 1;
        changed = false;
        let mut temp_neighbours: Vec<Position> = vec![];
        for &pos in &newly_connected {
            for dir in Direction::values() {
                let new_pos = dir.apply_offset(pos);
                if pos_in_bounds(lines, new_pos)
                    && !connected.contains(&new_pos)
                    && pipe_is_connected(lines[pos.x as usize][pos.y as usize], dir)
                    && pipe_is_connected(
                        lines[new_pos.x as usize][new_pos.y as usize],
                        dir.invert(),
                    )
                {
                    temp_neighbours.push(new_pos);
                    connected.insert(new_pos);
                    changed = true;
                }
            }
        }
        newly_connected = HashSet::new();
        for n in temp_neighbours {
            newly_connected.insert(n);
        }
    }
    steps as u32
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut start_position = Position { x: 0, y: 0 };
    'search: for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            if *c == 'S' {
                start_position.x = x as i32;
                start_position.y = y as i32;
                break 'search;
            }
        }
    }
    let mut changed = true;
    let mut connected = HashSet::new();
    let mut newly_connected = HashSet::new();
    connected.insert(start_position);
    newly_connected.insert(start_position);
    while changed {
        changed = false;
        let mut temp_neighbours: Vec<Position> = vec![];
        for &pos in &newly_connected {
            for dir in Direction::values() {
                let new_pos = dir.apply_offset(pos);
                if pos_in_bounds(lines, new_pos)
                    && !connected.contains(&new_pos)
                    && pipe_is_connected(lines[pos.x as usize][pos.y as usize], dir)
                    && pipe_is_connected(
                        lines[new_pos.x as usize][new_pos.y as usize],
                        dir.invert(),
                    )
                {
                    temp_neighbours.push(new_pos);
                    connected.insert(new_pos);
                    changed = true;
                }
            }
        }
        newly_connected = HashSet::new();
        for n in temp_neighbours {
            newly_connected.insert(n);
        }
    }

    let mut directly_connected = vec![];
    for dir in Direction::values() {
        if connected.contains(&dir.apply_offset(start_position)) {
            directly_connected.push(dir.apply_offset(start_position));
        }
    }

    // find the inner side
    let mut traverse = (start_position, directly_connected[0]);
    let mut inner_dir: Option<Direction> = None;
    while inner_dir.is_none() {
        let step_dir = Direction::get_step_dir(traverse.0, traverse.1);
        let left_dir = step_dir.relative_left();
        let left_pos = left_dir.apply_offset(traverse.0);
        let right_dir = step_dir.relative_right();
        let right_pos = right_dir.apply_offset(traverse.0);
        if pos_in_bounds(lines, left_pos)
            && !connected.contains(&left_pos)
            && can_reach_border(&connected, lines, left_pos)
        {
            inner_dir = Some(Right);
        }
        if pos_in_bounds(lines, right_pos)
            && !connected.contains(&right_pos)
            && can_reach_border(&connected, lines, right_pos)
        {
            inner_dir = Some(Left);
        }
        traverse = (
            traverse.1,
            direct_pipe_neighbour(traverse.1, &connected, traverse.0, lines),
        );
    }

    // collect the positions in the inner side
    let mut inner_positions = HashSet::new();
    let Some(inner_dir) = inner_dir else {
        unreachable!()
    };
    let mut traverse = (start_position, directly_connected[0]);
    let mut started = false;
    while !started || traverse.0 != start_position {
        started = true;
        let step_dir = Direction::get_step_dir(traverse.0, traverse.1);
        let dir = match inner_dir {
            Left => step_dir.relative_left(),
            Right => step_dir.relative_right(),
            _ => {
                unreachable!()
            }
        };
        let new_pos = dir.apply_offset(traverse.0);
        if pos_in_bounds(lines, new_pos) && !connected.contains(&new_pos) {
            inner_positions.insert(new_pos);
        }
        let new_pos2 = dir.apply_offset(traverse.1);
        if pos_in_bounds(lines, new_pos2) && !connected.contains(&new_pos2) {
            inner_positions.insert(new_pos2);
        }
        traverse = (
            traverse.1,
            direct_pipe_neighbour(traverse.1, &connected, traverse.0, lines),
        );
    }

    let mut newly_connected = HashSet::new();
    for &pos in &inner_positions {
        newly_connected.insert(pos);
    }

    let mut changed = true;
    while changed {
        changed = false;
        let mut temp_neighbours: Vec<Position> = vec![];
        for &pos in &newly_connected {
            for dir in Direction::values() {
                let new_pos = dir.apply_offset(pos);
                if pos_in_bounds(lines, new_pos)
                    && !connected.contains(&new_pos)
                    && !inner_positions.contains(&new_pos)
                {
                    temp_neighbours.push(new_pos);
                    inner_positions.insert(new_pos);
                    changed = true;
                }
            }
        }
        newly_connected = HashSet::new();
        for n in temp_neighbours {
            newly_connected.insert(n);
        }
    }
    inner_positions.len() as u32
}

fn direct_pipe_neighbour(
    pos: Position,
    pipes: &HashSet<Position>,
    wrong_neighbour: Position,
    map: &[Vec<char>],
) -> Position {
    for dir in Direction::values() {
        let new_pos = dir.apply_offset(pos);
        if pipes.contains(&new_pos)
            && new_pos != wrong_neighbour
            && pipe_is_connected(map[pos.x as usize][pos.y as usize], dir)
            && pipe_is_connected(map[new_pos.x as usize][new_pos.y as usize], dir.invert())
        {
            return new_pos;
        }
    }
    unreachable!()
}

fn can_reach_border(pipes: &HashSet<Position>, map: &Vec<Vec<char>>, pos: Position) -> bool {
    let mut connected = HashSet::new();
    let mut newly_connected = HashSet::new();
    connected.insert(pos);
    newly_connected.insert(pos);
    let mut changed = true;
    while changed {
        changed = false;
        let mut temp_neighbours: Vec<Position> = vec![];
        for &pos in &newly_connected {
            for dir in Direction::values() {
                let new_pos = dir.apply_offset(pos);
                if pos_in_bounds(map, new_pos)
                    && !connected.contains(&new_pos)
                    && !pipes.contains(&new_pos)
                {
                    temp_neighbours.push(new_pos);
                    connected.insert(new_pos);
                    changed = true;
                }
            }
        }
        newly_connected = HashSet::new();
        for n in temp_neighbours {
            newly_connected.insert(n);
        }
    }
    connected.iter().any(|&pos| {
        pos.x == 0
            || pos.x == map.len() as i32 - 1
            || pos.y == 0
            || pos.y == map[1].len() as i32 - 1
    })
}
