const TEST_INPUT: bool = false;
type InputType<'a> = Vec<(Vec<Condition>, Vec<u64>)>;
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

    let input2 = process_input_part_two(input_str.clone());
    let res2 = part_two(&input2);
    println!("Solution part two: {:?}", res2);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

use std::collections::HashMap;

use Condition::*;
impl Condition {
    fn from_char(c: char) -> Option<Condition> {
        match c {
            '.' => Some(Operational),
            '#' => Some(Damaged),
            '?' => Some(Unknown),
            _ => None,
        }
    }
}

fn process_input(input: &str) -> InputType {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split_vec = line.split(' ').collect::<Vec<_>>();
            let condition_vec = split_vec[0]
                .chars()
                .map(|c| Condition::from_char(c).unwrap())
                .collect::<Vec<_>>();
            let num_vec = split_vec[1]
                .split(',')
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (condition_vec, num_vec)
        })
        .collect::<Vec<_>>()
}

fn process_input_part_two(input: &str) -> InputType2 {
    process_input(input)
        .iter()
        .map(|(cond_vec, num_vec)| {
            let mut cond_vec_extended = cond_vec.clone();
            cond_vec_extended.extend([Unknown]);
            cond_vec_extended = cond_vec_extended.repeat(5);
            cond_vec_extended.remove(cond_vec_extended.len() - 1);
            (cond_vec_extended, num_vec.repeat(5))
        })
        .collect::<Vec<_>>()
}

fn count_consistent(
    line: &Vec<Condition>,
    num_vec: Vec<u64>,
    pos: u64,
    curr_chain_len: u64,
    cache: &mut HashMap<(Vec<u64>, u64, u64), u64>,
) -> u64 {
    let mut curr_chain_len_new = curr_chain_len;
    let mut num_vec_new = num_vec.clone();
    if let Some(&val) = cache.get(&(num_vec.clone(), pos, curr_chain_len)) {
        return val;
    }

    let mut total_count = 0;
    for i in pos as usize..line.len() {
        match line[i] {
            Operational => {
                if curr_chain_len_new != 0 {
                    if let Some(&j) = num_vec_new.first() {
                        if j == curr_chain_len_new {
                            num_vec_new = num_vec_new[1..].to_vec();
                        } else {
                            cache.insert((num_vec, pos, curr_chain_len), 0);
                            return 0;
                        }
                    } else {
                        cache.insert((num_vec, pos, curr_chain_len), 0);
                        return 0;
                    }
                }
                curr_chain_len_new = 0
            }
            Damaged => {
                curr_chain_len_new += 1;
            }
            Unknown => {
                let mut op_num_vec = num_vec_new.clone();
                if let Some(&j) = num_vec_new.first() {
                    if curr_chain_len_new == 0 {
                        total_count += count_consistent(line, op_num_vec, i as u64 + 1, 0, cache);
                    } else if j == curr_chain_len_new {
                        op_num_vec = op_num_vec[1..].to_vec();
                        total_count += count_consistent(line, op_num_vec, i as u64 + 1, 0, cache);
                    }

                    if curr_chain_len_new < j {
                        total_count += count_consistent(
                            line,
                            num_vec_new.clone(),
                            i as u64 + 1,
                            curr_chain_len_new + 1,
                            cache,
                        );
                    }
                    cache.insert((num_vec, pos, curr_chain_len), total_count);
                    return total_count;
                } else if curr_chain_len_new == 0 {
                    total_count += count_consistent(line, op_num_vec, i as u64 + 1, 0, cache);
                } else {
                    cache.insert((num_vec, pos, curr_chain_len), 0);
                    return 0;
                }
            }
        }
    }
    if curr_chain_len_new == 0 {
        if num_vec_new.is_empty() {
            cache.insert((num_vec, pos, curr_chain_len), 1);
            1
        } else {
            cache.insert((num_vec, pos, curr_chain_len), 0);
            0
        }
    } else if let Some(&j) = num_vec_new.first() {
        if j == curr_chain_len_new && num_vec_new.len() == 1 {
            cache.insert((num_vec, pos, curr_chain_len), 1);
            1
        } else {
            cache.insert((num_vec, pos, curr_chain_len), 0);
            0
        }
    } else {
        cache.insert((num_vec, pos, curr_chain_len), 0);
        0
    }
}

fn part_one(lines: &InputType) -> ResultType {
    let mut sum = 0;
    for (conditions, nums) in lines {
        sum += count_consistent(conditions, nums.clone(), 0, 0, &mut HashMap::new());
    }
    sum
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut sum = 0;
    for (conditions, nums) in lines {
        sum += count_consistent(conditions, nums.clone(), 0, 0, &mut HashMap::new());
    }
    sum
}
