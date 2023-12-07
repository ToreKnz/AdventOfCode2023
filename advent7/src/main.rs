use std::collections::HashMap;

const TEST_INPUT: bool = false;
type InputType<'a> = Vec<(Hand,u32)>;
type InputType2<'a> = Vec<(Hand2,u32)>;
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

    let input2 = process_input_part_two(input_str.clone());
    let res2 = part_two(&input2);
    println!("Solution part two: {:?}",res2);
}

#[derive(Eq, PartialEq, Ord, Clone, Copy, Hash, Debug)]
struct Card {
    val: char,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let vals = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'].to_vec();
        let self_idx = vals.iter().position(|&x| x == self.val).unwrap();
        let other_idx = vals.iter().position(|&x| x == other.val).unwrap();
        return other_idx.partial_cmp(&self_idx);
    }
}

impl Card {
    fn new(c: char) -> Self {
        Card { val: c }
    }
}

#[derive(Eq, PartialEq, Ord, Clone, Hash, Debug)]
struct Hand {
    cards: Vec<Card>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_val = self.type_value();
        let other_val = other.type_value();
        if self_val != other_val {
            return self_val.partial_cmp(&other_val);
        }

        self.compare_cards(&other)
    }
}

impl Hand {
    fn type_value(&self) -> u32 {
        if self.five_of_a_kind() {
            7
        }
        else if self.four_of_a_kind() {
            6
        }
        else if self.full_house() {
            5
        }
        else if self.three_of_a_kind() {
            4
        }
        else if self.two_pair() {
            3
        }
        else if self.one_pair() {
            2
        }
        else if self.high_card() {
            1
        }
        else {
            0
        }
    }

    fn compare_cards(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            let cmp = self_card.partial_cmp(&other_card);
            if cmp != Some(std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return Some(std::cmp::Ordering::Equal);
    }
    
    fn new(cards: &str) -> Self {
        let mut card_vec = vec![];
        for card in cards.chars() {
            card_vec.push(Card::new(card));
        }
        Hand { cards: card_vec }
    }

    fn five_of_a_kind(&self) -> bool {
        let mut copy = self.cards.clone();
        copy.dedup();
        if copy.len() == 1 {
            true
        }
        else {
            false
        }
    }

    fn four_of_a_kind(&self) -> bool {
        let mut counts: HashMap<Card,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        if counts.values().collect::<Vec<_>>().contains(&&4) {
            return true;
        }
        else {
            return false;
        }
    }

    fn  three_of_a_kind(&self) -> bool {
        let mut counts: HashMap<Card,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        if counts.values().collect::<Vec<_>>().contains(&&3) {
            return true;
        }
        else {
            return false;
        }
    }

    fn full_house(&self) -> bool {
        let mut counts: HashMap<Card,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        let counts = counts.values().collect::<Vec<_>>();
        if counts.contains(&&3) && counts.contains(&&2) {
            return true;
        }
        else {
            return false;
        }
    }

    fn two_pair(&self) -> bool {
        let mut counts: HashMap<Card,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        counts.values().filter(|value| **value == 2).count() == 2
    }

    fn one_pair(&self) -> bool {
        let mut counts: HashMap<Card,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        counts.values().filter(|value| **value == 2).count() == 1
    }

    fn high_card(&self) -> bool {
        let mut counts: HashMap<Card,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        counts.values().filter(|value| **value == 1).count() == 5
    }
}

fn process_input(input: &str) -> InputType {
    input.lines().filter(|line| !line.is_empty()).map(|line| { 
        let splits = line.split(' ').collect::<Vec<_>>();
        (Hand::new(splits[0]), splits[1].parse::<u32>().unwrap())
    }).collect::<Vec<_>>()
}

fn process_input_part_two(input: &str) -> InputType2 {
    input.lines().filter(|line| !line.is_empty()).map(|line| { 
        let splits = line.split(' ').collect::<Vec<_>>();
        (Hand2::new(splits[0]), splits[1].parse::<u32>().unwrap())
    }).collect::<Vec<_>>()
}

#[derive(Eq, PartialEq, Ord, Clone, Copy, Hash, Debug)]
struct Card2 {
    val: char,
}

impl PartialOrd for Card2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let vals = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'].to_vec();
        let self_idx = vals.iter().position(|&x| x == self.val).unwrap();
        let other_idx = vals.iter().position(|&x| x == other.val).unwrap();
        return other_idx.partial_cmp(&self_idx);
    }
}

impl Card2 {
    fn new(c: char) -> Self {
        Card2 { val: c }
    }
}

#[derive(Eq, PartialEq, Ord, Clone, Hash, Debug)]
struct Hand2 {
    cards: Vec<Card2>,
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_val = self.type_value();
        let other_val = other.type_value();
        if self_val != other_val {
            return self_val.partial_cmp(&other_val);
        }

        self.compare_cards(&other)
    }
}

impl Hand2 {
    fn type_value(&self) -> u32 {
        if self.five_of_a_kind() {
            7
        }
        else if self.four_of_a_kind() {
            6
        }
        else if self.full_house() {
            5
        }
        else if self.three_of_a_kind() {
            4
        }
        else if self.two_pair() {
            3
        }
        else if self.one_pair() {
            2
        }
        else if self.high_card() {
            1
        }
        else {
            0
        }
    }

    fn compare_cards(&self, other: &Hand2) -> Option<std::cmp::Ordering> {
        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            let cmp = self_card.partial_cmp(&other_card);
            if cmp != Some(std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return Some(std::cmp::Ordering::Equal);
    }
    
    fn new(cards: &str) -> Self {
        let mut card_vec = vec![];
        for card in cards.chars() {
            card_vec.push(Card2::new(card));
        }
        Hand2 { cards: card_vec }
    }

    fn five_of_a_kind(&self) -> bool {
        let mut counts: HashMap<Card2,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        for key in counts.keys() {
            if counts.get(key).unwrap() + counts.get(&Card2::new('J')).unwrap_or_else(|| &0) == 5 {
                return true;
            }
        }
        if *counts.get(&Card2::new('J')).unwrap_or_else(|| &0) == 5 {
            return true;
        }
        false
    }

    fn four_of_a_kind(&self) -> bool {
        let mut counts: HashMap<Card2,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        for key in counts.keys() {
            if key != &Card2::new('J') && counts.get(key).unwrap() + counts.get(&Card2::new('J')).unwrap_or_else(|| &0) == 4 {
                return true;
            }
        }
        false
    }

    fn  three_of_a_kind(&self) -> bool {
        let mut counts: HashMap<Card2,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        for key in counts.keys() {
            if counts.get(key).unwrap() + counts.get(&Card2::new('J')).unwrap_or_else(|| &0) == 3 {
                return true;
            }
        }
        false
    }

    fn full_house(&self) -> bool {
        let mut counts: HashMap<Card2,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        let card_count = counts.values().filter(|val| **val != 0).count();
        if card_count == 2 || (card_count == 3 && counts.get(&Card2::new('J')).is_some()) {
            true
        } else {
            false
        }
    }

    fn two_pair(&self) -> bool {
        let mut counts: HashMap<Card2,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        let pair_count = counts.values().filter(|val| **val == 2).count();
        if pair_count == 2{
            return true;
        }
        false
    }

    fn one_pair(&self) -> bool {
        let mut counts: HashMap<Card2,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        let pair_count = counts.values().filter(|val| **val == 2).count();
        if pair_count == 1 || *counts.get(&Card2::new('J')).unwrap_or_else(|| &0) > 0 {
            true
        }
        else {
            false
        }
    }

    fn high_card(&self) -> bool {
        let mut counts: HashMap<Card2,u32> = HashMap::new();
        for card in &self.cards {
            let count = counts.get(&card).unwrap_or_else(|| &0);
            counts.insert(*card, count + 1);
        }
        for (x, y) in counts {
            if x != Card2::new('J') && y > 1 {
                return false;
            }
        }
        true
    }
}

fn part_one(lines: &InputType) -> ResultType {
    let mut total = 0;
    let mut lines = lines.clone();
    lines.sort_by_key(|line| line.0.clone());
    for i in 0..lines.len() {
        total += (i as u32+1) * lines[i].1;
    }
    total
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut total = 0;
    let mut lines = lines.clone();
    lines.sort_by_key(|line| line.0.clone());
    for i in 0..lines.len() {
        total += (i as u32+1) * lines[i].1;
    }
    total
}
