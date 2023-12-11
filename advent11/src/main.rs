const TEST_INPUT: bool = false;
type InputType<'a> = Vec<Vec<char>>;
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
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn expand_universe(universe: &mut Vec<Vec<char>>) {
    let mut rows_to_expand = vec![];
    let mut columns_to_expand = vec![];
    for i in 0..universe.len() {
        if row_is_empty(universe, i) {
            rows_to_expand.push(i);
        }
    }
    for i in 0..universe[0].len() {
        if column_is_empty(universe, i) {
            columns_to_expand.push(i);
        }
    }
    for &row_idx in rows_to_expand.iter().rev() {
        let row_copy = universe[row_idx].clone();
        universe.insert(row_idx, row_copy);
    }
    for &col_idx in columns_to_expand.iter().rev() {
        for line in &mut *universe {
            let col_el =line[col_idx];
            line.insert(col_idx, col_el);
        }
    }
}

fn column_is_empty(universe: &[Vec<char>], idx: usize) -> bool {
    universe.iter().all(|line| line[idx] == '.')
}

fn row_is_empty(universe: &[Vec<char>], idx: usize) -> bool {
    universe[idx].iter().all(|character| *character == '.')
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

fn dist(pos1: Position, pos2: Position) -> u64 {
    (pos2.y - pos1.y).unsigned_abs() as u64 + (pos2.x - pos1.x).unsigned_abs() as u64
}

fn part_one(lines: &InputType) -> ResultType {
    let mut universe = lines.clone();
    expand_universe(&mut universe);
    let mut galaxies = vec![];
    for x in 0..universe.len() {
        for y in 0..universe[0].len() {
            if universe[x][y] == '#' {
                galaxies.push(Position {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    let mut sum_distances = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum_distances += dist(galaxies[i], galaxies[j]);
        }
    }
    sum_distances
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut galaxies = vec![];
    for x in 0..lines.len() {
        for y in 0..lines[0].len() {
            if lines[x][y] == '#' {
                galaxies.push(Position {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    let mut rows_to_expand = vec![];
    let mut columns_to_expand = vec![];
    for i in 0..lines.len() {
        if row_is_empty(lines, i) {
            rows_to_expand.push(i);
        }
    }
    for i in 0..lines[0].len() {
        if column_is_empty(lines, i) {
            columns_to_expand.push(i);
        }
    }

    let mut sum_distances = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum_distances += dist(galaxies[i], galaxies[j]);
            let rows_traversed = std::cmp::min(galaxies[i].x, galaxies[j].x)
                ..std::cmp::max(galaxies[i].x, galaxies[j].x);
            let cols_traversed = std::cmp::min(galaxies[i].y, galaxies[j].y)
                ..std::cmp::max(galaxies[i].y, galaxies[j].y);
            let duplicated_row_count = rows_to_expand
                .iter()
                .filter(|&&row_idx| rows_traversed.contains(&(row_idx as i32)))
                .count();
            let duplicated_col_count = columns_to_expand
                .iter()
                .filter(|&&col_idx| cols_traversed.contains(&(col_idx as i32)))
                .count();
            sum_distances += (duplicated_col_count as u64 + duplicated_row_count as u64) * (1_000_000 - 1);
        }
    }
    sum_distances
}
