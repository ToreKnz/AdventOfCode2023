use std::collections::HashMap;

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

// fn process_input_part_two(input: &str) -> InputType2 {
//     todo!()
// }

fn tilt_north(col_idx: usize, platform: &mut Vec<Vec<char>>) {
    for row_idx in 0..platform.len() {
        if platform[row_idx][col_idx] == 'O' {
            let mut new_idx = row_idx;
            for i in (0..row_idx).rev() {
                if platform[i][col_idx] == '.' {
                    new_idx = i;
                }
                else {
                    break;
                }
            }
            platform[row_idx][col_idx] = '.';
            platform[new_idx][col_idx] = 'O';
        }
    }
}

fn tilt_south(col_idx: usize, platform: &mut Vec<Vec<char>>) {
    for row_idx in (0..platform.len()).rev() {
        if platform[row_idx][col_idx] == 'O' {
            let mut new_idx = row_idx;
            for i in row_idx + 1..platform.len() {
                if platform[i][col_idx] == '.' {
                    new_idx = i;
                }
                else {
                    break;
                }
            }
            platform[row_idx][col_idx] = '.';
            platform[new_idx][col_idx] = 'O';
        }
    }
}

fn tilt_east(row_idx: usize, platform: &mut Vec<Vec<char>>) {
    for col_idx in (0..platform[0].len()).rev() {
        if platform[row_idx][col_idx] == 'O' {
            let mut new_idx = col_idx;
            for i in col_idx + 1..platform.len() {
                if platform[row_idx][i] == '.' {
                    new_idx = i;
                }
                else {
                    break;
                }
            }
            platform[row_idx][col_idx] = '.';
            platform[row_idx][new_idx] = 'O';
        }
    }
}

fn tilt_west(row_idx: usize, platform: &mut Vec<Vec<char>>) {
    for col_idx in 0..platform[0].len() {
        if platform[row_idx][col_idx] == 'O' {
            let mut new_idx = col_idx;
            for i in (0..col_idx).rev() {
                if platform[row_idx][i] == '.' {
                    new_idx = i;
                }
                else {
                    break;
                }
            }
            platform[row_idx][col_idx] = '.';
            platform[row_idx][new_idx] = 'O';
        }
    }
}

fn tilt_north_all(platform: &mut Vec<Vec<char>>) {
    for col_idx in 0..platform[0].len() {
        tilt_north(col_idx, platform);
    }
}

fn tilt_south_all(platform: &mut Vec<Vec<char>>) {
    for col_idx in 0..platform[0].len() {
        tilt_south(col_idx, platform);
    }
}

fn tilt_east_all(platform: &mut Vec<Vec<char>>) {
    for row_idx in 0..platform.len() {
        tilt_east(row_idx, platform);
    }
}

fn tilt_west_all(platform: &mut Vec<Vec<char>>) {
    for row_idx in 0..platform.len() {
        tilt_west(row_idx, platform);
    }
}

fn cycle(platform: &mut Vec<Vec<char>>) {
    tilt_north_all(platform);
    tilt_west_all(platform);
    tilt_south_all(platform);
    tilt_east_all(platform)
}

fn part_one(lines: &InputType) -> ResultType {
    let mut lines = lines.clone();
    for col_idx in 0..lines[0].len() {
        tilt_north(col_idx, &mut lines);
    }
    let mut sum = 0;
    for (row_idx, val) in (0..lines.len()).zip((1..=lines.len()).rev()) {
        let count = lines[row_idx].iter().filter(|c| **c == 'O').count();
        sum += count * val;
    }
    sum as u32
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut seen: HashMap<Vec<Vec<char>>, u32> = HashMap::new();
    let mut lines = lines.clone();
    let mut new_lines = lines.clone();
    cycle(&mut new_lines);
    let mut cycle_count = 0;
    let mut sum = 0;
    loop {
        seen.insert(lines.clone(), cycle_count);
        lines = new_lines.clone();
        cycle(&mut new_lines);
        cycle_count += 1;
        if seen.contains_key(&lines) {
            let cycle_length = cycle_count - seen.get(&lines).unwrap();
            let offset = cycle_count - cycle_length;
            let rest = (1_000_000_000 - offset) % cycle_length;
            for (key, val) in seen {
                if val == offset + rest {
                    lines = key;
                    break;
                }
            }
            for (row_idx, val) in (0..lines.len()).zip((1..=lines.len()).rev()) {
                let count = lines[row_idx].iter().filter(|c| **c == 'O').count();
                sum += count * val;
            }
            break;
        }
    }
    sum as u32
}
