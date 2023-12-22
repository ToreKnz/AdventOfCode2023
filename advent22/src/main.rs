use std::collections::HashMap;

const TEST_INPUT: bool = false;
type InputType<'a> = Vec<Brick>;
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
    let res2 = part_two(&input);
    println!("Solution part two: {:?}",res2);
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Brick {
    xrange: Range,
    yrange: Range,
    zrange: Range,
}

impl Brick {
    fn lowest(&self) -> u32 {
        self.zrange.lowest()
    }

    fn lower(&self) -> Brick {
        Brick {
            xrange: self.xrange,
            yrange: self.yrange,
            zrange: self.zrange.clone().lower(),
        }
    }

    fn higher(&self) -> Brick {
        Brick {
            xrange: self.xrange,
            yrange: self.yrange,
            zrange: self.zrange.clone().higher(),
        }
    }

    fn overlap(&self, other: &Brick) -> bool {
        self.xrange.overlap(&other.xrange)
            && self.yrange.overlap(&other.yrange)
            && self.zrange.overlap(&other.zrange)
    }
}

fn fall_down(bricks: &mut [Brick], index: usize) {
    let mut brick = bricks[index].clone();
    while brick.lowest() > 1 {
        brick = brick.lower();
        for (i, other) in bricks.iter().enumerate() {
            if i != index && brick.overlap(other) {
                return;
            }
        }
        bricks[index] = brick.clone();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Range {
    Value(u32),
    Span(u32, u32),
}

use Range::*;
impl Range {
    fn lowest(&self) -> u32 {
        match self {
            Value(num) => *num,
            Span(num, _) => *num,
        }
    }

    fn lower(&mut self) -> Range {
        match self {
            Value(num) => Value(*num - 1),
            Span(min, max) => Span(*min - 1, *max - 1),
        }
    }

    fn higher(&mut self) -> Range {
        match self {
            Value(num) => Value(*num + 1),
            Span(min, max) => Span(*min + 1, *max + 1),
        }
    }

    fn overlap(&self, other: &Range) -> bool {
        match (self, other) {
            (Value(num1), Value(num2)) => num1 == num2,
            (Value(num1), Span(min, max)) | (Span(min, max), Value(num1)) => {
                min <= num1 && num1 <= max
            }
            (Span(min1, max1), Span(min2, max2)) => min1 <= max2 && min2 <= max1,
        }
    }
}

fn process_input(input: &str) -> InputType {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let splits = line.split_once('~').unwrap();
            let mut first_nums = splits.0.split(',').map(|num| num.parse::<u32>().unwrap());
            let x1 = first_nums.next().unwrap();
            let y1 = first_nums.next().unwrap();
            let z1 = first_nums.next().unwrap();
            let mut second_nums = splits.1.split(',').map(|num| num.parse::<u32>().unwrap());
            let x2 = second_nums.next().unwrap();
            let y2 = second_nums.next().unwrap();
            let z2 = second_nums.next().unwrap();
            let xrange = if x1 == x2 {
                Value(x1)
            } else {
                Span(std::cmp::min(x1, x2), std::cmp::max(x1, x2))
            };
            let yrange = if y1 == y2 {
                Value(y1)
            } else {
                Span(std::cmp::min(y1, y2), std::cmp::max(y1, y2))
            };
            let zrange = if z1 == z2 {
                Value(z1)
            } else {
                Span(std::cmp::min(z1, z2), std::cmp::max(z1, z2))
            };
            Brick {
                xrange,
                yrange,
                zrange,
            }
        })
        .collect()
}

fn part_one(lines: &InputType) -> ResultType {
    let mut bricks = lines.clone();

    // sort by z coordinate
    bricks.sort_unstable_by_key(|a| a.lowest());

    // let bricks fall down one by one
    for i in 0..bricks.len() {
        fall_down(&mut bricks, i);
    }

    // calculate support
    let mut bricks_supporting: HashMap<Brick, Vec<Brick>> = HashMap::new();
    let mut bricks_supported: HashMap<Brick, Vec<Brick>> = HashMap::new();
    for index in 0..bricks.len() {
        let brick = bricks[index].clone();
        let lower = brick.lower();
        let higher = brick.higher();
        for (i, other_brick) in bricks.iter().enumerate() {
            if i != index {
                if lower.overlap(other_brick) {
                    let entry = bricks_supported.entry(brick.clone()).or_insert(Vec::new());
                    entry.push(other_brick.clone());
                }
                if higher.overlap(other_brick) {
                    let entry = bricks_supporting.entry(brick.clone()).or_insert(Vec::new());
                    entry.push(other_brick.clone());
                }
            }
        }
    }

    // count removable bricks
    bricks
        .iter()
        .map(|brick| {
            bricks_supporting.get(brick).unwrap_or(&Vec::new()).is_empty()
                || bricks_supporting
                    .get(brick)
                    .unwrap()
                    .iter()
                    .all(|b| bricks_supported.get(b).unwrap().len() > 1)
        })
        .filter(|x| *x)
        .count() as u32
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut bricks = lines.clone();

    // sort by z coordinate
    bricks.sort_unstable_by_key(|a| a.lowest());

    // let bricks fall down one by one
    for i in 0..bricks.len() {
        fall_down(&mut bricks, i);
    }

    // calculate support
    let mut bricks_supporting: HashMap<Brick, Vec<Brick>> = HashMap::new();
    let mut bricks_supported: HashMap<Brick, Vec<Brick>> = HashMap::new();
    for index in 0..bricks.len() {
        let brick = bricks[index].clone();
        let lower = brick.lower();
        let higher = brick.higher();
        for (i, other_brick) in bricks.iter().enumerate() {
            if i != index {
                if lower.overlap(other_brick) {
                    let entry = bricks_supported.entry(brick.clone()).or_insert(Vec::new());
                    entry.push(other_brick.clone());
                }
                if higher.overlap(other_brick) {
                    let entry = bricks_supporting.entry(brick.clone()).or_insert(Vec::new());
                    entry.push(other_brick.clone());
                }
            }
        }
    }

    let mut sum = 0;
    for brick in bricks {
        let mut bricks_supported_clone = bricks_supported.clone();
        let mut bricks_to_remove = vec![brick.clone()];
        while let Some(brick) = bricks_to_remove.pop() {
            for (update_brick, supported) in &mut bricks_supported_clone {
                if !supported.is_empty() {
                    supported.retain(|b| brick != *b);
                    if supported.is_empty() {
                        bricks_to_remove.push(update_brick.clone());
                        sum += 1;
                    }
                }
            }
        }
    }
    sum
}
