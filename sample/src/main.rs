const TEST_INPUT: bool = true;
type InputType<'a> = Vec<u32>;
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
    // let res2 = part_two(&input);
    // println!("Solution part two: {:?}",res2);
}

fn process_input(input: &str) -> InputType {
    // input.split('\n').collect()
    
    // input.lines().filter(|line| !line.is_empty()).map(|line| line.parse().unwrap()).collect()
    todo!()
}

// fn process_input_part_two(input: &str) -> InputType2 {
//     todo!()
// }

fn part_one(lines: &InputType) -> ResultType {
    todo!()
}

fn part_two(lines: &InputType2) -> ResultType2 {
    todo!()
}
