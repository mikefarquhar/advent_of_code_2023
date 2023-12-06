struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Vec<Range>>,
}

#[derive(Debug)]
struct Range {
    offset: i64,
    start: i64,
    end: i64,
}

fn parse_input(input: &str) -> Almanac {
    let (seeds_str, maps_str) = input.trim().split_once("\n").unwrap();
    let seeds: Vec<i64> = seeds_str
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let mut maps: Vec<Vec<Range>> = Vec::new();

    for line in maps_str.lines() {
        let line = line.trim();
        if let Some(first_char) = line.chars().nth(0) {
            if first_char.is_alphabetic() {
                maps.push(Vec::new());
            } else if first_char.is_numeric() {
                if let Some(ranges) = maps.last_mut() {
                    let numbers: Vec<&str> = line.split_whitespace().collect();

                    let dst: i64 = numbers[0].parse().unwrap();
                    let src: i64 = numbers[1].parse().unwrap();
                    let len: i64 = numbers[2].parse().unwrap();

                    ranges.push(Range {
                        offset: dst - src,
                        start: src,
                        end: src + len,
                    });
                }
            }
        }
    }

    Almanac { seeds, maps }
}

fn find_nearest_for_planting(almanac: &Almanac) -> i64 {
    almanac
        .seeds
        .iter()
        .map(|&seed| {
            almanac.maps.iter().fold(seed, |acc, cur| {
                let maybe_range = cur.iter().find(|&r| acc > r.start && acc <= r.end);
                if let Some(range) = maybe_range {
                    acc + range.offset
                } else {
                    acc
                }
            })
        })
        .min()
        .unwrap()
}

// This is a travesty of code, but it's almost 2am and I don't care any more.
// Also if there's a way to avoid allocating additional vectors I'm not seeing it.
fn part2(almanac: &Almanac) -> i64 {
    almanac
        .seeds
        .chunks_exact(2)
        .map(|chunk| {
            let mut seeds = vec![Range {
                start: chunk[0],
                end: chunk[0] + chunk[1],
                offset: 0,
            }];

            let mut results = Vec::new();

            for map in &almanac.maps {
                while let Some(seed) = seeds.pop() {
                    let Range {
                        mut start, mut end, ..
                    } = seed;

                    let mut split = false;
                    for range in map {
                        if range.end <= start || range.start >= end {
                            continue;
                        }

                        if start < range.start {
                            seeds.push(Range {
                                start: start,
                                end: range.start,
                                offset: 0,
                            });
                            start = range.start;
                        }

                        if end > range.end {
                            seeds.push(Range {
                                start: range.end,
                                end: end,
                                offset: 0,
                            });
                            end = range.end;
                        }

                        results.push(Range {
                            start: start + range.offset,
                            end: end + range.offset,
                            offset: 0,
                        });

                        split = true;
                        break;
                    }

                    if !split {
                        results.push(seed);
                    }
                }

                (seeds, results) = (results, seeds);
            }

            seeds
        })
        .flatten()
        .map(|seed| seed.start)
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../input.txt");
    let almanac = parse_input(&input);
    let part_1_result = find_nearest_for_planting(&almanac);
    println!("part 1: {}", part_1_result);

    let part_2_result = part2(&almanac);
    println!("part 2: {}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = r"
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4
    ";

    #[test]
    fn part1() {
        let almanac = parse_input(&SAMPLE_INPUT);
        let result = find_nearest_for_planting(&almanac);
        assert_eq!(result, 35)
    }

    #[test]
    fn part2() {
        let almanac = parse_input(&SAMPLE_INPUT);
        let result = super::part2(&almanac);
        assert_eq!(result, 46)
    }
}
