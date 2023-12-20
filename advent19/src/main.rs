use std::collections::HashMap;

type Label = String;
type Target = String;

const TEST_INPUT: bool = false;
type InputType<'a> = (HashMap<Label, Workflow>, Vec<Part>);
type InputType2<'a> = InputType<'a>;
type ResultType = u32;
type ResultType2 = u64;

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

struct Workflow {
    conditions: Vec<(Condition, Target)>,
    alt: Target,
}

impl Workflow {
    fn from_str(s: &str) -> (Label, Workflow) {
        let splits = s.split_once('{').unwrap();
        let label = splits.0;
        let conditions = splits.1.split(',');
        let mut condition_vec = Vec::new();
        let mut alt = "".to_string();
        for condition in conditions {
            if condition.contains(':') {
                let cond_split = condition.split_once(':').unwrap();
                let c = Condition::from_str(cond_split.0);
                let target = cond_split.1;
                condition_vec.push((c, target.to_string()));
            } else {
                let target_split = condition.split_once('}').unwrap();
                alt = target_split.0.to_string();
            }
        }

        (
            label.to_string(),
            Workflow {
                conditions: condition_vec,
                alt,
            },
        )
    }

    fn process(&self, part: &Part) -> Label {
        for (condition, target) in &self.conditions {
            if condition.fulfilled_by(part) {
                return target.clone();
            }
        }
        self.alt.clone()
    }

    fn process_range(&self, part_range: &PartRange) -> Vec<(PartRange, Label)> {
        let mut res = Vec::new();
        let mut part_range = part_range.clone();
        for (condition, target) in &self.conditions {
            let (lower, included) = condition.last_lower_included();
            let (lower, higher) = part_range.split_on(condition.c, lower);

            if included {
                if let Some(range) = lower {
                    res.push((range, target.to_string()));
                }
                if let Some(range) = higher {
                    part_range = range;
                } else {
                    return res;
                }
            } else {
                if let Some(range) = higher {
                    res.push((range, target.to_string()));
                }
                if let Some(range) = lower {
                    part_range = range;
                } else {
                    return res;
                }
            }
        }
        res.push((part_range, self.alt.clone()));
        res
    }
}

struct Condition {
    c: char,
    op: Operator,
    val: u32,
}

impl Condition {
    fn from_str(s: &str) -> Condition {
        if s.contains('>') {
            let op = Operator::Greater;
            let splits = s.split_once('>').unwrap();
            let c = splits.0.chars().next().unwrap();
            let val = splits.1.parse::<u32>().unwrap();
            Condition { c, op, val }
        } else {
            let op = Operator::Lower;
            let splits = s.split_once('<').unwrap();
            let c = splits.0.chars().next().unwrap();
            let val = splits.1.parse::<u32>().unwrap();
            Condition { c, op, val }
        }
    }

    fn fulfilled_by(&self, part: &Part) -> bool {
        match self.c {
            'x' => match self.op {
                Operator::Greater => part.x > self.val,
                Operator::Lower => part.x < self.val,
            },
            'm' => match self.op {
                Operator::Greater => part.m > self.val,
                Operator::Lower => part.m < self.val,
            },
            'a' => match self.op {
                Operator::Greater => part.a > self.val,
                Operator::Lower => part.a < self.val,
            },
            's' => match self.op {
                Operator::Greater => part.s > self.val,
                Operator::Lower => part.s < self.val,
            },
            _ => unreachable!(),
        }
    }

    fn last_lower_included(&self) -> (u32, bool) {
        match self.op {
            Operator::Greater => (self.val, false),
            Operator::Lower => {
                assert!(self.val > 0);
                (self.val - 1, true)
            }
        }
    }
}

enum Operator {
    Greater,
    Lower,
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn from_str(s: &str) -> Part {
        let s = &s[3..];
        let splits = s.split_once(",m=").unwrap();
        let x = splits.0.parse::<u32>().unwrap();
        let splits = splits.1.split_once(",a=").unwrap();
        let m = splits.0.parse::<u32>().unwrap();
        let splits = splits.1.split_once(",s=").unwrap();
        let a = splits.0.parse::<u32>().unwrap();
        let s = splits.1.split_once('}').unwrap().0.parse::<u32>().unwrap();
        Part { x, m, a, s }
    }

    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug)]
struct PartRange {
    x_min: u32,
    x_max: u32,
    m_min: u32,
    m_max: u32,
    a_min: u32,
    a_max: u32,
    s_min: u32,
    s_max: u32,
}

impl PartRange {
    fn split_on(&self, c: char, val: u32) -> (Option<PartRange>, Option<PartRange>) {
        let lower_split = match c {
            'x' => {
                if val < self.x_min {
                    None
                } else if val > self.x_max {
                    Some(self.clone())
                } else {
                    Some(PartRange {
                        x_max: val,
                        ..self.clone()
                    })
                }
            }
            'm' => {
                if val < self.m_min {
                    None
                } else if val > self.m_max {
                    Some(self.clone())
                } else {
                    Some(PartRange {
                        m_max: val,
                        ..self.clone()
                    })
                }
            }
            'a' => {
                if val < self.a_min {
                    None
                } else if val > self.a_max {
                    Some(self.clone())
                } else {
                    Some(PartRange {
                        a_max: val,
                        ..self.clone()
                    })
                }
            }
            's' => {
                if val < self.s_min {
                    None
                } else if val > self.s_max {
                    Some(self.clone())
                } else {
                    Some(PartRange {
                        s_max: val,
                        ..self.clone()
                    })
                }
            }
            _ => unreachable!(),
        };
        let higher_split = match c {
            'x' => {
                if val + 1 > self.x_max {
                    None
                } else if val + 1 < self.x_min {
                    Some(self.clone())
                } else {
                    Some(PartRange {
                        x_min: val + 1,
                        ..self.clone()
                    })
                }
            }
            'm' => {
                if val + 1 > self.m_max {
                    None
                } else if val + 1 < self.m_min {
                    Some(self.clone())
                } else {
                    Some(PartRange {
                        m_min: val + 1,
                        ..self.clone()
                    })
                }
            }
            'a' => {
                if val + 1 > self.a_max {
                    None
                } else if val + 1 < self.a_min {
                    Some(self.clone())
                } else {
                    Some(PartRange {
                        a_min: val + 1,
                        ..self.clone()
                    })
                }
            }
            's' => {
                if val + 1 > self.s_max {
                    None
                } else if val + 1 < self.s_min {
                    Some(self.clone())
                } else {
                    Some(PartRange {
                        s_min: val + 1,
                        ..self.clone()
                    })
                }
            }
            _ => unreachable!(),
        };
        (lower_split, higher_split)
    }

    fn combinations(&self) -> u64 {
        (self.x_max - self.x_min + 1) as u64
            * (self.a_max - self.a_min + 1) as u64
            * (self.m_max - self.m_min + 1) as u64
            * (self.s_max - self.s_min + 1) as u64
    }
}

fn process_input(input: &str) -> InputType {
    let lines = input.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();
    for line in lines.0.lines().filter(|line| !line.is_empty()) {
        let (label, workflow) = Workflow::from_str(line.trim_end());
        workflows.insert(label, workflow);
    }

    let mut parts = Vec::new();
    for line in lines.1.lines().filter(|line| !line.is_empty()) {
        parts.push(Part::from_str(line.trim_end()));
    }
    (workflows, parts)
}

fn part_one(lines: &InputType) -> ResultType {
    let mut sum = 0;
    let workflows = &lines.0;
    let parts = &lines.1;
    for part in parts {
        let mut workflow_label = "in".to_string();
        while workflow_label != "A" && workflow_label != "R" {
            let workflow = workflows.get(&workflow_label).unwrap();
            workflow_label = workflow.process(part);
        }
        if workflow_label == 'A'.to_string() {
            sum += part.sum();
        }
    }

    sum
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut sum: u64 = 0;
    let workflows = &lines.0;
    let part_range = PartRange {
        x_min: 1,
        x_max: 4000,
        a_min: 1,
        a_max: 4000,
        m_min: 1,
        m_max: 4000,
        s_min: 1,
        s_max: 4000,
    };
    let mut part_ranges = vec![(part_range, "in".to_string())];
    while !part_ranges.is_empty() {
        let (range, label) = part_ranges.first().unwrap();
        if label == "R" {
            part_ranges.remove(0);
        } else if label == "A" {
            sum += range.combinations();
            part_ranges.remove(0);
        } else {
            let new_parts = workflows.get(label).unwrap().process_range(range);
            part_ranges.remove(0);
            part_ranges.extend(new_parts);
        }
        if part_ranges.len() > 1000 {
            println!("{:?}", part_ranges);
        }
    }
    sum
}
