const TEST_INPUT: bool = false;
type InputType<'a> = Vec<Vec<char>>;
type InputType2<'a> = InputType<'a>;
type ResultType = u32;
type ResultType2 = ResultType;

struct Box {
    idx: u32,
    lenses: Vec<(String, u8)>,
}

impl Box {
    fn remove_label(&mut self, label: String) {
        let mut remove_idx: i32 = -1;
        for (idx, (l, _)) in self.lenses.iter().enumerate() {
            if l == &label {
                remove_idx = idx as i32;
                break;
            }
        }
        if remove_idx != -1 {
            self.lenses.remove(remove_idx as usize);
        }
    }

    fn add_lens(&mut self, label: String, val: u8) {
        let mut present_idx: i32 = -1;
        for (idx, (l, _)) in self.lenses.iter().enumerate() {
            if l == &label {
                present_idx = idx as i32;
                break;
            }
        }
        if present_idx != -1 {
            self.lenses[present_idx as usize] = (label, val);
        } else {
            self.lenses.push((label, val))
        }
    }

    fn get_value(&self) -> u32 {
        let mut sum = 0;
        for (idx, (_, val)) in self.lenses.iter().enumerate() {
            sum += (idx as u32 + 1) * *val as u32;
        }
        sum * (self.idx + 1)
    }
}

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
        .split(',')
        .map(|part| part.trim().chars().collect())
        .collect()
}

fn get_hash(string: &[char]) -> u32 {
    let mut hash: u32 = 0;
    for c in string {
        hash += *c as u32;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn part_one(lines: &InputType) -> ResultType {
    let mut sum = 0;
    for part in lines {
        sum += get_hash(part);
    }
    sum
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut boxes = Vec::with_capacity(256);
    for i in 0..256 {
        boxes.push(Box {
            idx: i as u32,
            lenses: Vec::new(),
        });
    }
    for part in lines {
        let mut splits = part.split(|&x| x == '=' || x == '-');
        let idx = splits.next().unwrap().len();
        let box_num = get_hash(&part[..idx]) as usize;
        let label: String = part[..idx].to_vec().iter().collect();
        let operation = part[idx];
        match operation {
            '-' => boxes[box_num].remove_label(label),
            '=' => {
                let num = part[idx + 1].to_digit(10).unwrap() as u8;
                boxes[box_num].add_lens(label, num)
            }
            _ => unreachable!(),
        }
    }

    let mut sum = 0;
    for b in boxes {
        sum += b.get_value();
    }
    sum
}
