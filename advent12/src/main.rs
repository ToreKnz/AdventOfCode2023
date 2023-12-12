use rustc_hash::FxHashMap;

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

#[derive(Debug, Clone, Copy)]
struct LineIter<'a> {
    line: &'a Vec<Condition>,
    pos: usize,
}

impl<'a> LineIter<'a> {
    fn get(&self) -> Option<&Condition> {
        self.line.get(self.pos)
    }

    fn next(&self) -> Self {
        LineIter {
            line: self.line,
            pos: self.pos + 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct NumIter<'a> {
    nums: &'a Vec<u64>,
    pos: usize,
}

impl<'a> NumIter<'a> {
    fn matches_count(&self, chain: u64) -> bool {
        chain == self.get()
    }

    fn get(&self) -> u64 {
        *self.nums.get(self.pos).unwrap_or(&0)
    }

    fn updated_pos(&self, chain: u64) -> Self {
        NumIter {
            nums: self.nums,
            pos: if chain == 0 { self.pos } else { self.pos + 1 },
        }
    }
}

fn count_consistent(
    line: LineIter,
    nums: NumIter,
    curr_chain_len: u64,
    cache: &mut FxHashMap<(usize, usize, u64), u64>,
) -> u64 {
    if let Some(&val) = cache.get(&(line.pos, nums.pos, curr_chain_len)) {
        return val;
    }
    if nums.nums[nums.pos..].iter().sum::<u64>() > line.line[line.pos..].iter().count() as u64 + curr_chain_len
    {
        cache.insert((line.pos, nums.pos, curr_chain_len), 0);
        return 0;
    }
    match line.get() {
        Some(Operational) => {
            if nums.matches_count(curr_chain_len) || curr_chain_len == 0 {
                let res = count_consistent(line.next(), nums.updated_pos(curr_chain_len), 0, cache);
                cache.insert((line.pos, nums.pos, curr_chain_len), res);
                res
            } else {
                cache.insert((line.pos, nums.pos, curr_chain_len), 0);
                0
            }
        }
        Some(Damaged) => {
            let res = count_consistent(line.next(), nums, curr_chain_len + 1, cache);
            cache.insert((line.pos, nums.pos, curr_chain_len), res);
            res
        }
        Some(Unknown) => {
            if nums.matches_count(curr_chain_len) {
                let res = count_consistent(line.next(), nums.updated_pos(curr_chain_len), 0, cache);
                cache.insert((line.pos, nums.pos, curr_chain_len), res);
                res
            } else if nums.get() < curr_chain_len {
                cache.insert((line.pos, nums.pos, curr_chain_len), 0);
                0
            } else {
                if curr_chain_len == 0 {
                    let res = count_consistent(line.next(), nums, 0, cache)
                        + count_consistent(line.next(), nums, curr_chain_len + 1, cache);
                    cache.insert((line.pos, nums.pos, curr_chain_len), res);
                    res
                } else {
                    let res = count_consistent(line.next(), nums, curr_chain_len + 1, cache);
                    cache.insert((line.pos, nums.pos, curr_chain_len), res);
                    res
                }
            }
        }
        None => {
            if !nums.matches_count(curr_chain_len) || nums.nums.len() as isize - nums.pos as isize > 1 {
                cache.insert((line.pos, nums.pos, curr_chain_len), 0);
                0
            } else {
                cache.insert((line.pos, nums.pos, curr_chain_len), 1);
                1
            }
        }
    }
}

fn part_one(lines: &InputType) -> ResultType {
    let mut sum = 0;
    for (conditions, nums) in lines {
        sum += count_consistent(
            LineIter {
                line: conditions,
                pos: 0,
            },
            NumIter { nums, pos: 0 },
            0,
            &mut FxHashMap::default(),
        );
    }
    sum
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut sum = 0;
    for (conditions, nums) in lines {
        sum += count_consistent(
            LineIter {
                line: conditions,
                pos: 0,
            },
            NumIter { nums, pos: 0 },
            0,
            &mut FxHashMap::default(),
        );
    }
    sum
}
