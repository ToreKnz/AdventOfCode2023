const TEST_INPUT: bool = false;
type InputType<'a> = Vec<(char, u32, String)>;
type InputType2<'a> = InputType<'a>;
type ResultType = u64;
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
    println!("Solution part two: {:?}",res2);
}

fn color_to_num(color: &str) -> u64 {
    let color_num = color[2..color.len()-2].chars().rev().collect::<Vec<_>>();
    let mut sum: u64 = 0;
    let mut value: u64 = 1;
    for c in color_num {
        let num = c.to_digit(16).unwrap();
        sum += num as u64 * value;
        value *= 16;
    }
    sum
} 

fn process_input(input: &str) -> InputType {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut splits = line.split(' ');
            let dir = splits.next().unwrap().parse::<char>().unwrap();
            let amt = splits.next().unwrap().parse::<u32>().unwrap();
            let color = splits.next().unwrap().to_string();
            (dir, amt, color)
        })
        .collect()
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn cross(pos1: Position, pos2: Position) -> i64 {
        pos1.x * pos2.y - pos2.x * pos1.y
    }
}

// fn process_input_part_two(input: &str) -> InputType2 {
//     todo!()
// }

fn part_one(lines: &InputType) -> ResultType {
    let mut amts = 0;
    let mut curr = Position { x: 0, y: 0 };
    let mut positions = Vec::new();
    for (dir, amt, _) in lines {
        amts += amt;
        curr = Position {
            x: {
                curr.x
                    + match dir {
                        'U' => -1,
                        'D' => 1,
                        _ => 0,
                    } * *amt as i64
            },
            y: {
                curr.y
                    + match dir {
                        'R' => 1,
                        'L' => -1,
                        _ => 0,
                    } * *amt as i64
            },
        };
        positions.push(curr);
    }
    let mut res = 0;
    for window in positions.windows(2) {
        let first = window[0];
        let second = window[1];
        res += Position::cross(first, second);
    }
    (res.abs() / 2) as u64 + amts as u64 / 2 + 1
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut amts = 0;
    let mut curr = Position { x: 0, y: 0 };
    let mut positions = Vec::new();
    for (_, _, color) in lines {
        let dir = match color.chars().nth(color.len() -2 ).unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => unreachable!(),
        };
        let amt = color_to_num(color);
        amts += amt;
        curr = Position {
            x: {
                curr.x
                    + match dir {
                        'U' => -1,
                        'D' => 1,
                        _ => 0,
                    } * amt as i64
            },
            y: {
                curr.y
                    + match dir {
                        'R' => 1,
                        'L' => -1,
                        _ => 0,
                    } * amt as i64
            },
        };
        positions.push(curr);
    }
    let mut res: i64 = 0;
    for window in positions.windows(2) {
        let first = window[0];
        let second = window[1];
        res += Position::cross(first, second);
    }
    (res.abs() / 2) as u64 + amts / 2 + 1
}
