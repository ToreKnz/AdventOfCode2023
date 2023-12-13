const TEST_INPUT: bool = false;
type InputType<'a> = Vec<Vec<Vec<char>>>;
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
    println!("Solution part two: {:?}", res2);
}

fn process_input(input: &str) -> InputType {
    input
        .split("\n\n")
        .map(|pat| {
            pat.trim_end()
                .to_string()
                .split('\n')
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn vertical_reflection(pat: &Vec<Vec<char>>) -> (bool, u32) {
    for window in (0..pat[0].len()).collect::<Vec<_>>().windows(2) {
        let i = window[0];
        let j = window[1];
        let mirr = (0..std::cmp::min(i + 1, pat[0].len() - j))
            .all(|offset| columns_match(pat, i - offset, j + offset));
        if mirr {
            return (true, i as u32 + 1);
        }
    }
    (false, 0)
}

fn columns_match(pat: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    for row in pat {
        if row[i] != row[j] {
            return false;
        }
    }
    true
}

fn horizontal_reflection(pat: &Vec<Vec<char>>) -> (bool, u32) {
    for window in (0..pat.len()).collect::<Vec<_>>().windows(2) {
        let i = window[0];
        let j = window[1];
        let mirr = (0..std::cmp::min(i + 1, pat.len() - j))
            .all(|offset| rows_match(pat, i - offset, j + offset));
        if mirr {
            return (true, i as u32 + 1);
        }
    }
    (false, 0)
}

fn rows_match(pat: &[Vec<char>], i: usize, j: usize) -> bool {
    for k in 0..pat[0].len() {
        if pat[i][k] != pat[j][k] {
            return false;
        }
    }
    true
}

fn vertical_reflection_smudged(pat: &Vec<Vec<char>>) -> (bool, u32) {
    'outer: for window in (0..pat[0].len()).collect::<Vec<_>>().windows(2) {
        let i = window[0];
        let j = window[1];
        let mut matching = true;
        let mut smudge = 0;
        for k in 0..std::cmp::min(i + 1, pat[0].len() - j) {
            let (mirr, smudge_count) = columns_match_smudged(pat, i - k, j + k);
            matching &= mirr;
            smudge += smudge_count;
            if !matching || smudge > 1 {
                continue 'outer;
            }
        }
        if smudge == 0 {
            continue 'outer;
        }
        return (true, i as u32 + 1);
    }
    (false, 0)
}

fn columns_match_smudged(pat: &Vec<Vec<char>>, i: usize, j: usize) -> (bool, u32) {
    let mut smudge_count = 0;
    for row in pat {
        if row[i] != row[j] {
            if smudge_count == 1 {
                return (false, 2);
            } else {
                smudge_count += 1;
            }
        }
    }
    (true, smudge_count)
}

fn horizontal_reflection_smudged(pat: &Vec<Vec<char>>) -> (bool, u32) {
    'outer: for window in (0..pat.len()).collect::<Vec<_>>().windows(2) {
        let i = window[0];
        let j = window[1];
        let mut matching = true;
        let mut smudge = 0;
        for k in 0..std::cmp::min(i + 1, pat.len() - j) {
            let (mirr, smudge_count) = rows_match_smudged(pat, i - k, j + k);
            matching &= mirr;
            smudge += smudge_count;
            if !matching || smudge > 1 {
                continue 'outer;
            }
        }
        if smudge == 0 {
            continue 'outer;
        }
        return (true, i as u32 + 1);
    }
    (false, 0)
}

fn rows_match_smudged(pat: &[Vec<char>], i: usize, j: usize) -> (bool, u32) {
    let mut smudge_count = 0;
    for k in 0..pat[0].len() {
        if pat[i][k] != pat[j][k] {
            if smudge_count == 1 {
                return (false, 2);
            } else {
                smudge_count += 1;
            }
        }
    }
    (true, smudge_count)
}

fn part_one(lines: &InputType) -> ResultType {
    let mut sum = 0;
    for line in lines {
        let (mirr, idx) = vertical_reflection(line);
        if mirr {
            sum += idx;
        } else {
            let (_, idx) = horizontal_reflection(line);
            sum += 100 * idx;
        }
    }
    sum
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut sum = 0;
    for line in lines {
        let (mirr, idx) = vertical_reflection_smudged(line);
        if mirr {
            sum += idx;
        } else {
            let (_, idx) = horizontal_reflection_smudged(line);
            sum += 100 * idx;
        }
    }
    sum
}
