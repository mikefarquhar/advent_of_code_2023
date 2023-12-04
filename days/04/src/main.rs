use std::collections::{HashMap, VecDeque};

struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    revealed_numbers: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .map(|line| {
            let (title_str, numbers_str) = line.split_once(":").unwrap();

            let id_str = title_str.split_whitespace().nth(1).unwrap();
            let id: usize = id_str.parse().unwrap();

            let (winning_num_str, revealed_num_str) = numbers_str.split_once("|").unwrap();
            let winning_numbers = parse_numbers(winning_num_str);
            let revealed_numbers = parse_numbers(revealed_num_str);

            Card {
                id,
                winning_numbers,
                revealed_numbers,
            }
        })
        .collect()
}

fn parse_numbers(numbers_str: &str) -> Vec<usize> {
    numbers_str
        .trim()
        .split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn find_winning_numbers_score(cards: &[Card]) -> usize {
    cards
        .iter()
        .map(|card| {
            let count = card
                .revealed_numbers
                .iter()
                .filter(|&num| card.winning_numbers.contains(num))
                .count() as u32;

            if count > 0 {
                2_usize.pow(count - 1)
            } else {
                0
            }
        })
        .sum()
}

fn count_cards_scratched(cards: &[Card]) -> u32 {
    let mut queue = VecDeque::with_capacity(cards.len());
    queue.extend(cards.iter());

    let mut cached_results = HashMap::new();
    let mut visited = cards.len() as u32;

    while let Some(card) = queue.pop_back() {
        let count = *cached_results.entry(card.id).or_insert_with(|| {
            card.revealed_numbers
                .iter()
                .filter(|&num| card.winning_numbers.contains(num))
                .count()
        });

        visited += count as u32;

        for i in 0..count {
            let next_card = &cards[card.id + i];
            queue.push_back(next_card);
        }
    }

    visited
}

fn main() {
    let input = include_str!("../input.txt");
    let cards = parse_input(input);
    println!("part 1: {}", find_winning_numbers_score(&cards));
    println!("part 2: {}", count_cards_scratched(&cards));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = r"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    #[test]
    fn it_should_parse_input() {
        let cards = parse_input(&SAMPLE_INPUT);

        assert_eq!(cards[0].winning_numbers[3], 86);
        assert_eq!(cards[0].revealed_numbers[4], 17);

        assert_eq!(cards[5].winning_numbers[4], 72);
        assert_eq!(cards[5].revealed_numbers[7], 11);
    }

    #[test]
    fn it_should_find_the_total_of_the_winning_numbers() {
        let cards = parse_input(&SAMPLE_INPUT);
        let total = find_winning_numbers_score(&cards);
        assert_eq!(total, 13);
    }

    #[test]
    fn it_should_count_the_number_of_scratch_cards() {
        let cards = parse_input(&SAMPLE_INPUT);
        let total = count_cards_scratched(&cards);
        assert_eq!(total, 30);
    }
}
