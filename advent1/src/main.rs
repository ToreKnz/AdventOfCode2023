const TEST_INPUT: bool = false;
type InputType<'a> = Vec<Vec<u32>>;
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

    let input2 = process_input_part_two(input_str.clone());
    let res2 = part_two(&input2);
    println!("Solution part two: {:?}", res2);
}

fn process_input(input: &str) -> InputType {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .filter(|character| character.is_digit(10))
                .map(|character| character.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn process_input_part_two(input: &str) -> InputType2 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut num_vec: Vec<u32> = Vec::new();
            for i in 0..line.len() {
                if line[i..].starts_with("1") || line[i..].starts_with("one") {
                    num_vec.push(1);
                } else if line[i..].starts_with("2") || line[i..].starts_with("two") {
                    num_vec.push(2);
                } else if line[i..].starts_with("3") || line[i..].starts_with("three") {
                    num_vec.push(3);
                } else if line[i..].starts_with("4") || line[i..].starts_with("four") {
                    num_vec.push(4);
                } else if line[i..].starts_with("5") || line[i..].starts_with("five") {
                    num_vec.push(5);
                } else if line[i..].starts_with("6") || line[i..].starts_with("six") {
                    num_vec.push(6);
                } else if line[i..].starts_with("7") || line[i..].starts_with("seven") {
                    num_vec.push(7);
                } else if line[i..].starts_with("8") || line[i..].starts_with("eight") {
                    num_vec.push(8);
                } else if line[i..].starts_with("9") || line[i..].starts_with("nine") {
                    num_vec.push(9);
                }
            }
            num_vec
        })
        .collect()
}

fn part_one(lines: &InputType) -> ResultType {
    lines
        .iter()
        .map(|line| line.first().unwrap() * 10 + line.last().unwrap())
        .sum()
}

fn part_two(lines: &InputType2) -> ResultType2 {
    lines
        .iter()
        .map(|line| line.first().unwrap() * 10 + line.last().unwrap())
        .sum()
}
