const TEST_INPUT: bool = false;
type InputType<'a> = Vec<Vec<i64>>;
type InputType2<'a> = InputType<'a>;
type ResultType = i64;
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

fn process_input(input: &str) -> InputType {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}


fn part_one(lines: &InputType) -> ResultType {
    let mut sum = 0;
    for line in lines {
        sum += extrapolate_value(line);
    }
    sum
}

fn extrapolate_value(line: &Vec<i64>) -> i64 {
    let mut lines: Vec<Vec<i64>> = vec![line.to_vec()];
    while !is_zero_vec(&lines.last().unwrap()) {
        lines.push(vec![]);
        for i in 0..lines[lines.len()-2].len() - 1 {
            let val1 = lines[lines.len()-2][i];
            let val2 = lines[lines.len()-2][i + 1];
            let last_idx = lines.len() - 1;
            lines[last_idx].push(val2 - val1);
        }
    }
    lines.reverse();
    let mut extrapolated = 0;
    for line in lines.iter().skip(1) {
        extrapolated = *line.last().unwrap() + extrapolated;
    }
    extrapolated
}


fn extrapolate_value_rev(line: &Vec<i64>) -> i64 {
    let mut lines: Vec<Vec<i64>> = vec![line.to_vec()];
    while !is_zero_vec(&lines.last().unwrap()) {
        lines.push(vec![]);
        for i in 0..lines[lines.len()-2].len() - 1 {
            let val1 = lines[lines.len()-2][i];
            let val2 = lines[lines.len()-2][i + 1];
            let last_idx = lines.len() - 1;
            lines[last_idx].push(val2 - val1);
        }
    }
    lines.reverse();
    let mut extrapolated = 0;
    for line in lines.iter().skip(1) {
        extrapolated = *line.first().unwrap() - extrapolated;
    }
    extrapolated
}

fn is_zero_vec(vec: &Vec<i64>) -> bool {
    vec.iter().all(|val| *val == 0)
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut sum = 0;
    for line in lines {
        sum += extrapolate_value_rev(line);
    }
    sum
}
