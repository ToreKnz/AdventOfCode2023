const TEST_INPUT: bool = false;
type InputType<'a> = (Vec<u64>, Vec<Vec<(u64, u64, u64)>>);
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
    let mut lines = input.split("\n");
    let seed_line = lines.next().unwrap();
    let seed_line = seed_line.replace("seeds: ", "");
    let seeds = seed_line
        .split(" ")
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    lines.next();
    let lines: Vec<String> = lines.map(|line| line.to_string()).collect::<Vec<_>>();
    let splits = lines
        .split(|line| line.is_empty())
        .map(|block| block.to_vec())
        .collect::<Vec<_>>();
    (
        seeds,
        splits
            .iter()
            .map(|block| {
                let block = block.iter().skip(1).collect::<Vec<_>>();
                block
                    .iter()
                    .map(|line| {
                        let nums = line
                            .split(" ")
                            .map(|num| num.parse::<u64>().unwrap())
                            .collect::<Vec<_>>();
                        (nums[0], nums[1], nums[2])
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
    )
}

fn part_one(lines: &InputType) -> ResultType {
    let (seeds, blocks) = lines;
    let mut mappings = vec![];
    for block in blocks {
        let mut ranges_map = vec![];
        for (dest, source, amt) in block {
            ranges_map.push(((*source..source + amt), (*dest..dest + amt)));
        }
        mappings.push(ranges_map);
    }
    let mut locations = vec![];
    for seed in seeds {
        let mut current_val = *seed;
        for mapping in &mappings {
            'map: for (sources, dests) in mapping {
                if sources.contains(&current_val) {
                    let offset = current_val - sources.start;
                    current_val = dests.start + offset;
                    break 'map;
                }
            }
        }
        locations.push(current_val);
    }
    *locations.iter().min().unwrap()
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let (seeds, blocks) = lines;
    let mut blocks = blocks.clone();
    let seeds = seeds.chunks(2).map(|chunk| (chunk[0]..chunk[0] + chunk[1])).collect::<Vec<_>>();
    blocks.reverse();
    let mut mappings = vec![];
    for block in blocks {
        let mut ranges_map = vec![];
        for (source, dest, amt) in block {
            ranges_map.push(((source..source + amt), (dest..dest + amt)));
        }
        mappings.push(ranges_map);
    }
    for i in 0.. {
        let mut current_val = i;
        for mapping in &mappings {
            'map: for (sources, dests) in mapping {
                if sources.contains(&current_val) {
                    let offset = current_val - sources.start;
                    current_val = dests.start + offset;
                    break 'map;
                }
            }
        }
        for seed_range in &seeds {
            if seed_range.contains(&current_val) {
                return i;
            }
        }
    }
    unreachable!()
}
