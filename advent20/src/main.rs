const TEST_INPUT: bool = false;
type InputType<'a> = Vec<(Label, Module)>;
type InputType2<'a> = InputType<'a>;
type ResultType = u32;
type ResultType2 = u64;

type Label = String;

#[derive(Debug, Clone)]
enum Module {
    Broadcaster {
        destinations: Vec<Label>,
    },
    Flipflop {
        on: bool,
        destinations: Vec<Label>,
    },
    Conjunction {
        memory: Vec<(Label, bool)>,
        destinations: Vec<Label>,
    },
}

impl Module {
    fn send_pulse(
        &mut self,
        from: &str,
        high: bool,
        pulse_queue: &mut VecDeque<(Label, Label, bool)>,
        module_label: Label,
    ) {
        match self {
            Broadcaster { destinations } => {
                for dest in destinations {
                    pulse_queue.push_back((module_label.clone(), dest.clone(), high))
                }
            }
            Flipflop { on, destinations } => {
                if !high {
                    if *on {
                        *on = false;
                        for dest in destinations {
                            pulse_queue.push_back((module_label.clone(), dest.clone(), false));
                        }
                    } else {
                        *on = true;
                        for dest in destinations {
                            pulse_queue.push_back((module_label.clone(), dest.clone(), true));
                        }
                    }
                }
            }
            Conjunction {
                memory,
                destinations,
            } => {
                for (label, val) in memory.iter_mut() {
                    if label == from {
                        *val = high;
                        break;
                    }
                }
                if memory.iter().all(|(_, high)| *high) {
                    for dest in destinations {
                        pulse_queue.push_back((module_label.clone(), dest.clone(), false));
                    }
                } else {
                    for dest in destinations {
                        pulse_queue.push_back((module_label.clone(), dest.clone(), true));
                    }
                }
            }
        }
    }
}
use std::collections::{HashMap, VecDeque};

use Module::*;

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
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (module_str, target_str) = line.split_once("->").unwrap();
            match module_str.chars().next().unwrap() {
                '&' => {
                    let label = module_str[1..].trim_end().to_string();
                    let destinations = target_str
                        .split(',')
                        .map(|dest| dest.trim().to_string())
                        .collect::<Vec<_>>();
                    (
                        label,
                        Conjunction {
                            memory: Vec::new(),
                            destinations,
                        },
                    )
                }
                '%' => {
                    let label = module_str[1..].trim_end().to_string();
                    let destinations = target_str
                        .split(',')
                        .map(|dest| dest.trim().to_string())
                        .collect::<Vec<_>>();
                    (
                        label,
                        Flipflop {
                            on: false,
                            destinations,
                        },
                    )
                }
                _ => {
                    let destinations = target_str
                        .split(',')
                        .map(|dest| dest.trim().to_string())
                        .collect::<Vec<_>>();
                    ("broadcaster".to_string(), Broadcaster { destinations })
                }
            }
        })
        .collect::<Vec<_>>()
}

// fn process_input_part_two(input: &str) -> InputType2 {
//     todo!()
// }

fn init_conjunction(modules: &Vec<(Label, Module)>) -> HashMap<Label, Module> {
    let mut module_list: HashMap<Label, Module> = HashMap::new();
    for (label, module) in modules {
        module_list.insert(label.clone(), module.clone());
    }
    for (label, module) in modules {
        match module {
            Conjunction {
                memory: _,
                destinations,
            }
            | Flipflop {
                on: _,
                destinations,
            } => {
                for dest in destinations {
                    if let Some(Conjunction {
                        memory,
                        destinations: _,
                    }) = module_list.get_mut(dest)
                    {
                        memory.push((label.clone(), false));
                    }
                }
            }
            Broadcaster { destinations } => {
                for destination in destinations {
                    if let Some(Conjunction {
                        memory,
                        destinations: _,
                    }) = module_list.get_mut(destination)
                    {
                        memory.push((label.clone(), false));
                    }
                }
            }
        }
    }
    module_list
}

fn part_one(lines: &InputType) -> ResultType {
    let mut module_list = init_conjunction(lines);
    let mut count_low_pulses = 0;
    let mut count_high_pulses = 0;
    for _ in 0..1000 {
        let mut pulse_queue = VecDeque::new();
        pulse_queue.push_back(("".to_string(), "broadcaster".to_string(), false));
        while let Some((from, label_to_send_to, high)) = pulse_queue.pop_front() {
            if high {
                count_high_pulses += 1;
            } else {
                count_low_pulses += 1;
            }
            if let Some(module_to_send_to) = module_list.get_mut(&label_to_send_to) {
                module_to_send_to.send_pulse(&from, high, &mut pulse_queue, label_to_send_to);
            }
        }
    }
    count_high_pulses * count_low_pulses
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut module_list = init_conjunction(lines);
    let mut steps = HashMap::new();
    let mut step_sizes = HashMap::new();
    let mut target_label = "".to_string();
    for (l, module) in &module_list {
        if let Conjunction {
            memory,
            destinations,
        } = module
        {
            if destinations.contains(&"rx".to_string()) {
                target_label = l.clone();
                for (label, _) in memory {
                    steps.insert(label.clone(), 0_u32);
                    step_sizes.insert(label.clone(), vec![]);
                }
            }
        }
    }

    for i in 1..10_000 {
        if i == u32::MAX {
            println!("rip");
        }
        let mut pulse_queue = VecDeque::new();
        pulse_queue.push_back(("".to_string(), "broadcaster".to_string(), false));
        while let Some((from, label_to_send_to, high)) = pulse_queue.pop_front() {
            if let Some(module_to_send_to) = module_list.get_mut(&label_to_send_to) {
                module_to_send_to.send_pulse(&from, high, &mut pulse_queue, label_to_send_to);
            }
            let Conjunction { memory, destinations: _} = module_list.get(&target_label.clone()).unwrap() else {panic!("rip")};
            for (label, high) in memory {
                if *high {
                    let step = i - steps.get(&label.clone()).unwrap();
                    if step != 0 {
                        steps.insert(label.clone().clone(), i).unwrap();
                        if !step_sizes.get(label).unwrap().contains(&step) {
                            step_sizes.get_mut(label).unwrap().push(step);
                        }
                    }
                }
            }
        }
    }
    let mut res = 1;
    for step_size in step_sizes {
        res *= *step_size.1.first().unwrap() as u64; 
    }
    res
}
