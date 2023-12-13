use std::collections::HashMap;

const TEST_INPUT: bool = false;
type InputType<'a> = (&'a str, HashMap<String, (String,String)>);
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

fn process_input(input: &str) -> InputType {
    let mut lines = input.lines().filter(|line| !line.is_empty());
    let instructions = lines.next().unwrap();
    let mut node_map = HashMap::new();
    for line in lines {
        node_map.insert(line[0..3].to_string(), (line[7..10].to_string(), line[12..15].to_string()));
    }
    (instructions, node_map)
}

fn part_one(lines: &InputType) -> ResultType {
    let (instructions, nodes) = lines;
    let mut steps = 0;
    let mut current = "AAA".to_string();
    for instruction in instructions.chars().cycle() {
        match instruction {
            'L' => {
                current = nodes.get(&current).unwrap().0.clone();
            },
            _ => {
                current = nodes.get(&current).unwrap().1.clone();
            }
        }
        steps += 1;
        if current == *"ZZZ".to_string() {
            return steps;
        }
    }
    unreachable!()
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let (instructions, nodes) = lines;
    let mut current = vec![];
    for node in nodes.keys() {
        if node.ends_with('A') {
            current.push(node.clone());
        }
    }
    let mut z_positions = vec![];
    for node in current {
        let mut node_copy = node;
        let mut steps = 0;
        'instructions: for instruction in instructions.chars().cycle() {
            match instruction {
                'L' => {
                    node_copy = nodes.get(&node_copy).unwrap().0.clone();
                },
                _ => {
                    node_copy = nodes.get(&node_copy).unwrap().1.clone();
                }
            }
            steps += 1;
            if node_copy.ends_with('Z') {
                z_positions.push(steps);
                break 'instructions;
            }
        }
    }
    z_positions.iter().fold(1, |x, &y| lcm(x, y))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}