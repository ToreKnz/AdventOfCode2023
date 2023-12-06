const TEST_INPUT: bool = false;
type InputType<'a> = Vec<Vec<u64>>;
type InputType2<'a> = Vec<u64>;
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
    println!("Solution part two: {:?}",res2);
}

fn process_input(input: &str) -> InputType {
    input
        .lines()
        .map(|line| line.split(":").last().unwrap())
        .map(|line| {
            line.split(" ")
                .filter(|line| !line.is_empty())
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn process_input_part_two(input: &str) -> InputType2 {
    input
        .lines()
        .map(|line| line.split(":").last().unwrap())
        .map(|line| line.replace(" ", ""))
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn part_one(lines: &InputType) -> ResultType {
    let mut possibilities = 1;
    for i in 0..lines[0].len() {
        let mut new_possibilities = 0;
        let time = lines[0][i];
        let distance = lines[1][i];
        for millis in 1..time {
            let remaining_time = time - millis;
            let travelled_distance = remaining_time * millis;
            if travelled_distance > distance {
                new_possibilities += 1;
            }
        }
        possibilities *= new_possibilities;
    }
    possibilities
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let time = lines[0];
    let distance = lines[1];
    let mut possibilities = 0;
    for millis in 1..time {
        let remaining_time = time - millis;
        let travelled_distance = remaining_time * millis;
        if travelled_distance > distance {
            possibilities += 1;
        }
    }
    possibilities
}
