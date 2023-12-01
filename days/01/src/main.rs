fn load_data() -> Vec<&'static str> {
    let input_str = include_str!("../input.txt");
    input_str.split("\n").collect::<Vec<&str>>()
}

const MATCHERS: [&str; 20] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine", "zero",
];

fn extract(matchers: &[&str], lines: &Vec<&str>) -> i32 {
    let mut total = 0;

    for line in lines {
        let mut first_num_str: Option<&str> = None;
        let mut last_num_str: Option<&str> = None;

        let mut i = 0;
        while i < line.len() {
            for matcher in matchers {
                let j = i + matcher.len();
                if j > line.len() {
                    continue;
                }

                let substr = &line[i..j];
                if substr == *matcher {
                    if first_num_str == None {
                        first_num_str = Some(substr);
                    }

                    last_num_str = Some(substr);
                }
            }
            i += 1;
        }

        match (first_num_str, last_num_str) {
            (Some(first_num), Some(second_num)) => {
                total += str_to_num(first_num) * 10 + str_to_num(second_num);
            }
            _ => (),
        }
    }

    total
}

fn str_to_num(str: &str) -> i32 {
    match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "zero" => 0,
        other => i32::from_str_radix(other, 10).unwrap(),
    }
}

fn main() {
    let input = load_data();
    let part_1_result = extract(&MATCHERS[0..10], &input);
    let part_2_result = extract(&MATCHERS, &input);
    println!("Part 1: {}, Part 2: {}", part_1_result, part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extracts_numeric_symbols() {
        let sample_data: Vec<&str> = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"].into();
        assert_eq!(extract(&MATCHERS, &sample_data), 142);
    }

    #[test]
    fn it_extracts_numeric_words() {
        let sample_data: Vec<&str> = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .into();
        assert_eq!(extract(&MATCHERS, &sample_data), 281);
    }
}
