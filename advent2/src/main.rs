use std::cmp::max;

const TEST_INPUT: bool = false;
type InputType<'a> = Vec<(u32, Vec<(u32, u32, u32)>)>;
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
    println!("Solution part two: {:?}",res2);
}

fn process_input(input: &str) -> InputType {
    input.lines().filter(|line| !line.is_empty()).map(|line| {
        let splits = line.split(":");
        let game_vec: Vec<&str> = splits.collect();
        let game_id: u32 = game_vec[0][5..].parse().unwrap();
        let games: Vec<&str> = game_vec[1].split(";").map(|part| part.trim()).collect();
        let games: Vec<(u32, u32, u32)> = games.iter().map(|part| {
            let mut blue = 0;
            let mut red = 0;
            let mut green = 0;
            let counts: Vec<&str> = part.split(",").map(|count| count.trim()).collect();
            let parts: Vec<_> = counts.iter().map(|part| part.split(" ")).collect();
            for split in parts {
                let split: Vec<_> = split.collect();
                let count: u32 = split[0].parse().unwrap();
                let color = split[1];
                match color {
                    "blue" => {blue += count},
                    "red" => {red +=  count},
                    "green" => {green += count}
                    _ => {}, 
                };
            }
            (blue as u32, red as u32, green as u32)
        }).collect();
        (game_id, games)
    }).collect()
}

fn part_one(lines: &InputType) -> ResultType {
    lines.iter().filter(|line| {
        for (b, r, g) in &line.1 {
            if *b > 14 || *r > 12 || *g > 13 {
                return false
            }
        }
        true
        }
    ).map(|part| part.0).sum::<u32>() as u32
}

fn part_two(lines: &InputType2) -> ResultType2 {
    lines.iter().map(|line| {
        let mut b_max = 0;
        let mut r_max = 0;
        let mut g_max = 0;
        for (b, r, g) in &line.1 {
            b_max = max(b_max, *b);
            r_max = max(r_max, *r);
            g_max = max(g_max, *g);
        }
        b_max * r_max * g_max
        }
    ).sum::<u32>() as u32
}
