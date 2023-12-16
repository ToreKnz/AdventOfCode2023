use fxhash::FxHashSet;
const TEST_INPUT: bool = false;
type InputType<'a> = Vec<Vec<Tile>>;
type InputType2<'a> = InputType<'a>;
type ResultType = u32;
type ResultType2 = ResultType;
type MaybeLight = Option<(Position, Direction)>;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

use Direction::*;
impl Direction {
    fn is_vertical(self) -> bool {
        self == Top || self == Bottom
    }

    fn is_horizontal(self) -> bool {
        !self.is_vertical()
    }

    fn pos_in_dir(self, pos: Position) -> Position {
        match self {
            Right => Position {
                x: pos.x,
                y: pos.y + 1,
            },
            Top => Position {
                x: pos.x - 1,
                y: pos.y,
            },
            Left => Position {
                x: pos.x,
                y: pos.y - 1,
            },
            Bottom => Position {
                x: pos.x + 1,
                y: pos.y,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    MirrorSlash,
    MirrorBackSlash,
    ConnectorMinus,
    ConnectorVertical,
}

use Tile::*;
impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Empty,
            '/' => MirrorSlash,
            '\\' => MirrorBackSlash,
            '-' => ConnectorMinus,
            '|' => ConnectorVertical,
            _ => panic!("no tile match found"),
        }
    }

    fn get_new_pos(
        self,
        dir: Direction,
        pos: Position,
        cave: &Vec<Vec<Tile>>,
    ) -> (MaybeLight, MaybeLight) {
        match self {
            Empty => {
                let new_pos = dir.pos_in_dir(pos);
                if new_pos.is_valid(cave) {
                    return (Some((new_pos, dir)), None);
                }
            }
            MirrorSlash => {
                let (new_pos, new_dir) = match dir {
                    Right => (Top.pos_in_dir(pos), Top),
                    Top => (Right.pos_in_dir(pos), Right),
                    Left => (Bottom.pos_in_dir(pos), Bottom),
                    Bottom => (Left.pos_in_dir(pos), Left),
                };
                if new_pos.is_valid(cave) {
                    return (Some((new_pos, new_dir)), None);
                }
            }
            MirrorBackSlash => {
                let (new_pos, new_dir) = match dir {
                    Right => (Bottom.pos_in_dir(pos), Bottom),
                    Top => (Left.pos_in_dir(pos), Left),
                    Left => (Top.pos_in_dir(pos), Top),
                    Bottom => (Right.pos_in_dir(pos), Right),
                };
                if new_pos.is_valid(cave) {
                    return (Some((new_pos, new_dir)), None);
                }
            }
            ConnectorMinus => {
                if dir.is_horizontal() {
                    let new_pos = dir.pos_in_dir(pos);
                    if new_pos.is_valid(cave) {
                        return (Some((new_pos, dir)), None);
                    }
                } else {
                    let new_pos_right = Right.pos_in_dir(pos);
                    let new_pos_left = Left.pos_in_dir(pos);
                    let mut first = None;
                    let mut second = None;
                    if new_pos_right.is_valid(cave) {
                        first = Some((new_pos_right, Right));
                    }
                    if new_pos_left.is_valid(cave) {
                        second = Some((new_pos_left, Left));
                    }
                    return (first, second);
                }
            }
            ConnectorVertical => {
                if dir.is_vertical() {
                    let new_pos = dir.pos_in_dir(pos);
                    if new_pos.is_valid(cave) {
                        return (Some((new_pos, dir)), None);
                    }
                } else {
                    let new_pos_top = Top.pos_in_dir(pos);
                    let new_pos_bot = Bottom.pos_in_dir(pos);
                    let mut first = None;
                    let mut second = None;
                    if new_pos_top.is_valid(cave) {
                        first = Some((new_pos_top, Top));
                    }
                    if new_pos_bot.is_valid(cave) {
                        second = Some((new_pos_bot, Bottom));
                    }
                    return (first, second);
                }
            }
        }
        (None, None)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn is_valid(self, cave: &Vec<Vec<Tile>>) -> bool {
        self.x >= 0 && self.x < cave.len() as i32 && self.y >= 0 && self.y < cave[0].len() as i32
    }
}

fn process_input(input: &str) -> InputType {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect()
}

fn part_one(lines: &InputType) -> ResultType {
    let mut seen: FxHashSet<(Position, Direction)> = FxHashSet::default();
    let start = (Position { x: 0, y: 0 }, Right);
    seen.insert(start);
    let mut current_light = vec![start];
    let mut changed = true;
    while changed {
        changed = false;
        let mut new_light = Vec::new();
        for (pos, dir) in &current_light {
            let new_pos = lines[pos.x as usize][pos.y as usize].get_new_pos(*dir, *pos, lines);
            if let Some(light) = new_pos.0 {
                new_light.push(light);
            }
            if let Some(light) = new_pos.1 {
                new_light.push(light);
            }
        }
        current_light = new_light;
        for light in &current_light {
            changed |= seen.insert(*light);
        }
    }
    let mut energized = FxHashSet::default();
    for (pos, _) in seen {
        energized.insert(pos);
    }
    energized.len() as u32
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut start_positions = Vec::new();
    for x in 0..lines.len() {
        start_positions.push((Position { x: x as i32, y: 0 }, Right));
        start_positions.push((
            Position {
                x: x as i32,
                y: lines[0].len() as i32 - 1,
            },
            Left,
        ));
    }
    for y in 0..lines[0].len() {
        start_positions.push((Position { x: 0, y: y as i32 }, Bottom));
        start_positions.push((
            Position {
                x: lines.len() as i32 - 1,
                y: y as i32,
            },
            Top,
        ));
    }
    let mut res = 0;
    while !start_positions.is_empty() {
        let mut seen: FxHashSet<(Position, Direction)> = FxHashSet::default();
        let start = start_positions.pop().unwrap();
        seen.insert(start);
        let mut current_light = Vec::new();
        current_light.push(start);
        let mut changed = true;
        while changed {
            changed = false;
            let mut new_light = Vec::new();
            for (pos, dir) in &current_light {
                let new_pos = lines[pos.x as usize][pos.y as usize].get_new_pos(*dir, *pos, lines);
                if let Some(light) = new_pos.0 {
                    new_light.push(light);
                }
                if let Some(light) = new_pos.1 {
                    new_light.push(light);
                }
            }
            for light in &new_light {
                let was_not_contained = seen.insert(*light);
                if was_not_contained {
                    changed = true;
                    current_light.push(*light);
                }
            }
        }
        let mut energized = FxHashSet::default();
        for (pos, _) in seen {
            energized.insert(pos);
        }
        res = std::cmp::max(energized.len() as u32, res)
    }
    res
}
