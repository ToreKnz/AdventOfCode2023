const TEST_INPUT: bool = false;
type InputType<'a> = Vec<HailStone>;
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

    // let res2 = part_two(&input);
    // println!("Solution part two: {:?}",res2);
}

struct LinearEquation {
    first_row: (f64, f64, f64),
    second_row: (f64, f64, f64),
}

impl LinearEquation {
    fn has_unique_solution(&self) -> bool {
        let det = self.first_row.0 * self.second_row.1 - self.first_row.1 * self.second_row.0;
        !(-0.001..=0.001).contains(&det)
    }

    fn solve(mut self) -> (f64, f64) {
        // swap rows if necessary
        if self.first_row.0 == 0.0 {
            (self.first_row, self.second_row) = (self.second_row, self.first_row);
        }

        // normalize first row
        let factor = 1.0 / self.first_row.0;
        self.first_row = triple_multiply(self.first_row, factor);

        // add to second row
        self.second_row = add_triples(-self.second_row.0, self.first_row, self.second_row);

        // normalize second row
        let factor = 1.0 / self.second_row.1;
        self.second_row = triple_multiply(self.second_row, factor);

        // add to first row
        self.first_row = add_triples(-self.first_row.1, self.second_row, self.first_row);

        // assert that the solution is valid
        assert!((0.99..=1.01).contains(&self.first_row.0) && (0.99..=1.01).contains(&self.second_row.1));
        (self.first_row.2, self.second_row.2)
    }
}

fn triple_multiply(triple: (f64, f64, f64), factor: f64) -> (f64, f64, f64) {
    (triple.0 * factor, triple.1 * factor, triple.2 * factor)
}

fn add_triples(factor: f64, triple1: (f64, f64, f64), triple2: (f64, f64, f64)) -> (f64, f64, f64) {
    (factor * triple1.0 + triple2.0,
    factor * triple1.1 + triple2.1,
    factor * triple1.2 + triple2.2)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn from_str(s: &str) -> Position {
        let (x, y, z) = parse_three_nums(s);
        Position { x, y, z }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

impl Velocity {
    fn from_str(s: &str) -> Velocity {
        let (x, y, z) = parse_three_nums(s);
        Velocity { x, y, z }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct HailStone {
    pos: Position,
    vel: Velocity,
}

impl HailStone {
    fn from_line(line: &str) -> HailStone {
        let (pos_str, vel_str) = line.split_once('@').unwrap();
        HailStone { pos: Position::from_str(pos_str), vel: Velocity::from_str(vel_str) }
    }
}

fn parse_three_nums(s: &str) -> (i64, i64, i64) {
    let mut splits = s.split(',').map(|t| t.trim());
    let first = splits.next().unwrap().parse::<i64>().unwrap();
    let second = splits.next().unwrap().parse::<i64>().unwrap();
    let third = splits.next().unwrap().parse::<i64>().unwrap();
    (first, second, third)
}

fn to_linear_equation(h1: &HailStone, h2: &HailStone) -> LinearEquation {
    let first_row = (h1.vel.x as f64, -h2.vel.x as f64, (h2.pos.x - h1.pos.x) as f64);
    let second_row = (h1.vel.y as f64, -h2.vel.y as f64, (h2.pos.y - h1.pos.y) as f64);
    LinearEquation { first_row, second_row}
}

fn intersect_in_bounds(h1: &HailStone, h2: &HailStone) -> bool {
    let bounds = if TEST_INPUT {(7.0, 27.0)} else {(200_000_000_000_000_f64, 400_000_000_000_000_f64)};
    let leq = to_linear_equation(h1, h2);
    if !leq.has_unique_solution() {
        false
    } else {
        let (x, y) = leq.solve();
        if x < 0.0 || y < 0.0 {
            return false;
        }
        let x_res = h1.pos.x as f64 + h1.vel.x as f64* x;
        let y_res = h1.pos.y as f64 + h1.vel.y as f64 * x;
        x_res >= bounds.0 && x_res <= bounds.1 && y_res >= bounds.0 && y_res <= bounds.1
    }
}

fn process_input(input: &str) -> InputType {
    input.lines().filter(|line| !line.is_empty()).map(HailStone::from_line).collect()
}

fn part_one(lines: &InputType) -> ResultType {
    let mut sum = 0;
    for i in 0..lines.len() {
        for j in i+1..lines.len() {
            if intersect_in_bounds(&lines[i], &lines[j]) {
                sum += 1;
            }
        }
    }
    sum
}

fn part_two(lines: &InputType2) -> ResultType2 {
    todo!()
}
