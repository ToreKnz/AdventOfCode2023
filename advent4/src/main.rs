const TEST_INPUT: bool = false;
type InputType<'a> = Vec<(Vec<u32>, Vec<u32>)>;
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
    println!("Solution part two: {:?}", res2);
}

fn process_input(input: &str) -> InputType {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (mut winning_cards, mut owned_cards) = line.split_once('|').unwrap();
            winning_cards = winning_cards.split_once(':').unwrap().1;
            winning_cards = winning_cards.trim();
            let winning_cards = winning_cards
                .split(' ')
                .filter_map(|num| num.parse::<u32>().ok())
                .collect::<Vec<_>>();
            owned_cards = owned_cards.trim();
            let owned_cards = owned_cards
                .split(' ')
                .filter_map(|num| num.parse::<u32>().ok())
                .collect::<Vec<_>>();
            (winning_cards, owned_cards)
        })
        .collect::<Vec<_>>()
}

fn part_one(lines: &InputType) -> ResultType {
    let mut sum = 0;
    for (winning_cards, owned_cards) in lines {
        let mut matches = 0;
        for card in owned_cards {
            if winning_cards.contains(card) {
                matches += 1;
            }
        }
        if matches != 0 {
            sum += 2_u32.pow(matches - 1_u32);
        }
    }
    sum as u64
}

fn part_two(lines: &InputType2) -> ResultType2 {
    let mut cards_counted = (std::iter::repeat(1)).zip(lines.iter()).collect::<Vec<_>>();
    for i in 0..cards_counted.len() {
        let (count, (winning_cards, owned_cards)) = cards_counted[i];
        let mut matches = 0;
        for card in owned_cards {
            if winning_cards.contains(card) {
                matches += 1;
            }
        }
        if matches != 0 {
            for j in i + 1..std::cmp::min(cards_counted.len(), i + 1 + matches) {
                cards_counted[j].0 += count;
            }
        }
    }
    cards_counted.iter().map(|(count, (_, _))| *count).sum()
}
