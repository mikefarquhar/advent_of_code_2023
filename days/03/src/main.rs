use std::collections::HashMap;

struct Number {
    value: u32,
    symbol: (usize, usize, char),
}

#[derive(Hash, PartialEq, Eq)]
struct Pos(usize, usize);

fn parse_input(input: &str) -> Vec<Number> {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut numbers = Vec::new();

    for (y, row) in input.iter().enumerate() {
        let mut checked = 0;

        for (x, &c) in row.iter().enumerate() {
            if c.is_digit(10) && x >= checked {
                let mut w = 1;

                while x + w < row.len() && row[x + w].is_ascii_digit() {
                    w += 1;
                }

                checked = x + w;

                let value: u32 = row[x..x + w].iter().collect::<String>().parse().unwrap();

                let mut symbol: Option<(usize, usize, char)> = None;
                'outer: for i in y.saturating_sub(1)..(y + 2).min(input.len()) {
                    for j in x.saturating_sub(1)..(x + w + 1).min(row.len()) {
                        let c = input[i][j];
                        if !c.is_ascii_digit() && c != '.' {
                            symbol = Some((j, i, c));
                            break 'outer;
                        }
                    }
                }

                if let Some(symbol) = symbol {
                    numbers.push(Number { value, symbol });
                }
            }
        }
    }

    numbers
}

fn find_part_numbers(input: &Vec<Number>) -> u32 {
    input.iter().fold(0, |acc, cur| acc + cur.value)
}

fn find_gear_ratio(input: &Vec<Number>) -> u32 {
    let mut possible_gears = HashMap::new();

    input.iter().for_each(|part| {
        let (x, y, _) = part.symbol;
        possible_gears
            .entry(Pos(x, y))
            .or_insert(Vec::new())
            .push(part.value);
    });

    possible_gears
        .values()
        .filter(|&parts| parts.len() == 2)
        .map(|parts| parts.iter().product::<u32>())
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let input = parse_input(&input);

    let part_1_total = find_part_numbers(&input);
    println!("Part 1: {}", part_1_total);

    let part_2_total = find_gear_ratio(&input);
    println!("Part 2: {}", part_2_total);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    ";

    #[test]
    fn test_part_numbers() {
        let input = parse_input(SAMPLE_INPUT);
        let total = find_part_numbers(&input);
        assert_eq!(total, 4361);
    }

    #[test]
    fn test_gear_ratios() {
        let input = parse_input(SAMPLE_INPUT);
        let total = find_gear_ratio(&input);
        assert_eq!(total, 467835);
    }
}
