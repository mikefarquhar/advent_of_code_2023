type Input = (Vec<f64>, Vec<f64>);

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split_once('\n')
        .map(|(times_str, dists_str)| (parse_line(times_str), parse_line(dists_str)))
        .unwrap()
}

fn parse_line(line: &str) -> Vec<f64> {
    line.split_once(":")
        .map(|(_, nums_str)| {
            nums_str
                .trim()
                .split_whitespace()
                .map(|num_str| num_str.parse::<f64>().unwrap())
        })
        .unwrap()
        .collect()
}

fn concat_nums(nums: &Vec<f64>) -> f64 {
    let concat_str: String = nums.iter().map(|n| n.to_string()).collect();
    concat_str.parse().unwrap()
}

fn calc_num_record_times(t: f64, d: f64) -> f64 {
    let t = -t;
    let m1 = ((2.0 * d) / (-t - (t.powi(2) - 4.0 * d).sqrt())).ceil();
    let m2 = ((2.0 * d) / (-t + (t.powi(2) - 4.0 * d).sqrt())).floor() + 1.0;
    m1 - m2
}

fn find_num_ways_to_win_multiple_races(input: &Input) -> f64 {
    let (times, dists) = input;
    let mut total = 1.0;

    for i in 0..input.0.len() {
        let t = times[i];
        let d = dists[i];
        total *= calc_num_record_times(t, d);
    }

    total
}

fn find_num_ways_to_win_one_big_race(input: &Input) -> f64 {
    let t = concat_nums(&input.0);
    let d = concat_nums(&input.1);
    calc_num_record_times(t, d)
}

fn main() {
    let input = include_str!("../input.txt");
    let input = parse_input(input);

    let part1_result = find_num_ways_to_win_multiple_races(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = find_num_ways_to_win_one_big_race(&input);
    println!("Part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &'static str = r"
    Time:      7  15   30
    Distance:  9  40  200
    ";

    #[test]
    fn it_should_find_the_num_ways_to_win_multiple_races() {
        let input = parse_input(&SAMPLE_DATA);
        let result = find_num_ways_to_win_multiple_races(&input);
        assert_eq!(result, 288.0);
    }

    #[test]
    fn it_should_find_the_num_ways_to_win_one_big_race() {
        let input = parse_input(&SAMPLE_DATA);
        let result = find_num_ways_to_win_one_big_race(&input);
        assert_eq!(result, 71503.0)
    }
}
