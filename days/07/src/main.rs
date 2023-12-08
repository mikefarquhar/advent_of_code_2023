struct Hand {
    cards: Vec<char>,
    bid: u32,
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (cards_str, bid_str) = line.split_once(" ").unwrap();
            let cards: Vec<char> = cards_str.chars().collect();
            let bid = bid_str.parse().unwrap();
            Hand { cards, bid }
        })
        .collect()
}

const CARD_VALUES: &'static [u32; 13] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
const CARD_VALUES_JACKS_WILD: &'static [u32; 13] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 10, 11, 12];

fn rank_cards(hand: &Hand, jacks_wild: bool) -> (u32, &Hand) {
    let mut card_values = [0_u32; 5];
    let mut card_counts = [0_u32; 13];
    let mut tie_breaker = 0;

    for (i, card) in hand.cards.iter().enumerate() {
        let value_index = match card {
            'A' => 0xC,
            'K' => 0xB,
            'Q' => 0xA,
            'J' => 0x9,
            'T' => 0x8,
            num => num.to_digit(10).unwrap() as usize - 0x2,
        };

        let value = if jacks_wild {
            CARD_VALUES_JACKS_WILD[value_index]
        } else {
            CARD_VALUES[value_index]
        };

        card_values[i] = value;
        card_counts[value as usize] += 1;
        tie_breaker += value * 16_u32.pow(4 - i as u32);
    }

    let wild_count = if jacks_wild {
        let jacks_count = card_counts[0];
        card_counts[0] = 0;
        jacks_count
    } else {
        0
    };

    card_counts.sort_by(|&a, &b| b.cmp(&a));
    card_counts[0] += wild_count;

    let rank = match &card_counts[0..2] {
        [5, ..] => 0x7,
        [4, ..] => 0x6,
        [3, 2] => 0x5,
        [3, ..] => 0x4,
        [2, 2] => 0x3,
        [2, ..] => 0x2,
        _ => 0x1,
    };

    (rank * 16_u32.pow(5) + tie_breaker, hand)
}

fn calculate_score(ranked_hands: &mut [(u32, &Hand)]) -> u32 {
    ranked_hands.sort_by(|a, b| a.0.cmp(&b.0));

    ranked_hands
        .iter()
        .enumerate()
        .map(|(i, &(_, hand))| hand.bid * (i + 1) as u32)
        .sum()
}

fn part1(hands: &[Hand]) -> u32 {
    let mut ranked_hands: Vec<(u32, &Hand)> =
        hands.iter().map(|hand| rank_cards(hand, false)).collect();

    calculate_score(&mut ranked_hands)
}

fn part2(hands: &[Hand]) -> u32 {
    let mut ranked_hands: Vec<(u32, &Hand)> =
        hands.iter().map(|hand| rank_cards(hand, true)).collect();

    calculate_score(&mut ranked_hands)
}

fn main() {
    let input = include_str!("../input.txt");
    let hands = parse_input(&input);

    let part1_result = part1(&hands);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&hands);
    println!("Part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &'static str = r"
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    ";

    #[test]
    fn it_should_find_the_winnings_when_jacks_are_not_wild() {
        let input = parse_input(&SAMPLE_DATA);
        let result = super::part1(&input);
        assert_eq!(result, 6440);
    }

    #[test]
    fn it_should_find_the_winnings_when_jacks_are_wild() {
        let input = parse_input(&SAMPLE_DATA);
        let result = super::part2(&input);
        assert_eq!(result, 5905);
    }
}
