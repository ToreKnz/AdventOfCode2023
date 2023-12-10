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
        .split('\n')
        .map(|line| line.chars().collect())
        .collect()
}

fn part_one(lines: &InputType) -> ResultType {
    let mut sum = 0;
    let adjacent: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    for (line_idx, line) in lines.iter().enumerate() {
        let mut num_start = None;
        let mut num_end = None;
        let mut num = 0;
        for (idx, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                if num_start.is_none() {
                    num_start = Some(idx);
                }
                num *= 10;
                num += c.to_digit(10).unwrap();
                num_end = Some(idx);
            } else if num_start.is_some() {
                let mut count = false;
                for i in num_start.unwrap()..=num_end.unwrap() {
                    count = count
                        || adjacent
                            .iter()
                            .map(|(dx, dy)| {
                                let x_idx = line_idx as i32 + dx;
                                let y_idx = i as i32 + dy;
                                if x_idx >= 0 && y_idx >= 0 {
                                    return lines
                                        .get(x_idx as usize)
                                        .and_then(|line| line.get(y_idx as usize));
                                }
                                None
                            })
                            .filter(|t| t.is_some())
                            .any(|x| is_symbol(*x.unwrap()))
                }
                if count {
                    sum += num;
                }
                num_start = None;
                num_end = None;
                num = 0;
            }
        }
        if let Some(start) = num_start {
            let mut count = false;
            for i in start..=num_end.unwrap() {
                count = count
                    || adjacent
                        .iter()
                        .map(|(dx, dy)| {
                            let x_idx = line_idx as i32 + dx;
                            let y_idx = i as i32 + dy;
                            if x_idx >= 0 && y_idx >= 0 {
                                return lines
                                    .get(x_idx as usize)
                                    .and_then(|line| line.get(y_idx as usize));
                            }
                            None
                        })
                        .filter(|t| t.is_some())
                        .any(|x| is_symbol(*x.unwrap()))
            }
            if count {
                sum += num;
            }
        }
    }
    sum
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut sum = 0;
    for (line_idx, line) in lines.iter().enumerate() {
        for (char_idx, c) in line.iter().enumerate() {
            if *c == '*' {
                let neighbor_nums = neighbor_nums(lines, (line_idx, char_idx));
                if neighbor_nums.len() == 2 {
                    sum += neighbor_nums.iter().product::<u32>();
                }
            }
        }
    }

    sum
}

fn is_symbol(c: char) -> bool {
    if c.is_ascii_digit() || c == '.' {
        return false;
    }
    true
}

fn neighbor_nums(grid: &[Vec<char>], gear_pos: (usize, usize)) -> Vec<u32> {
    let adjacent: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut neighbor_nums = HashSet::new();
    for (dx, dy) in adjacent {
        if gear_pos.0 as i32 + dx >= 0 && gear_pos.1 as i32 + dy >= 0 {
            let x_idx = (gear_pos.0 as i32 + dx) as usize;
            let y_idx = (gear_pos.1 as i32 + dy) as usize;
            if let Some(x) = grid.get(x_idx).and_then(|line| line.get(y_idx)) {
                if x.is_ascii_digit() {
                    neighbor_nums.insert(find_num(grid, (x_idx, y_idx)));
                }
            }
        }
    }
    neighbor_nums.iter().map(|(idx, _)| *idx).collect()
}

fn find_num(grid: &[Vec<char>], digit_pos: (usize, usize)) -> (u32, (usize, usize)) {
    let mut start_pos = digit_pos.1;
    let mut end_pos = digit_pos.1;
    let mut i = start_pos;
    while i < grid[0].len() && grid[digit_pos.0][i].is_ascii_digit() {
        end_pos = i;
        i += 1;
    }
    let mut i: i32 = start_pos as i32;
    while i >= 0 && grid[digit_pos.0][i as usize].is_ascii_digit() {
        start_pos = i as usize;
        i -= 1;
    }
    (
        grid[digit_pos.0][start_pos..=end_pos]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap(),
        (digit_pos.0, start_pos),
    )
}
