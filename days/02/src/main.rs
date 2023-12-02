#[derive(Debug, Default, PartialEq)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    pulls: Vec<Cubes>,
}

fn parse_input(input_str: &str) -> Vec<Game> {
    input_str
        .lines()
        .filter(|&line| !line.trim().is_empty())
        .map(|line| {
            let (game_str, pulls_str) = line.split_once(":").unwrap();

            let game_id_str = game_str.trim().split_once(" ").unwrap().1;
            let game_id = u32::from_str_radix(game_id_str, 10).unwrap();

            let pulls: Vec<Cubes> = pulls_str
                .split(";")
                .map(|pull| {
                    pull.split(",").fold(Cubes::default(), |acc, cubes| {
                        let (count, color) = cubes.trim().split_once(" ").unwrap();
                        let count = u32::from_str_radix(count, 10).unwrap();
                        match color {
                            "red" => Cubes { red: count, ..acc },
                            "green" => Cubes {
                                green: count,
                                ..acc
                            },
                            "blue" => Cubes { blue: count, ..acc },
                            _ => panic!("Unexpected color: {}", color),
                        }
                    })
                })
                .collect();

            Game { id: game_id, pulls }
        })
        .collect()
}

fn possible_games(max_cubes: &Cubes, games: &[Game]) -> u32 {
    games
        .iter()
        .filter(|&game| {
            game.pulls.iter().all(|pull| {
                pull.red <= max_cubes.red
                    && pull.green <= max_cubes.green
                    && pull.blue <= max_cubes.blue
            })
        })
        .fold(0, |acc, cur| acc + cur.id)
}

fn minimum_cubes(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            // let mut min_cubes =
            let min_cubes = game.pulls.iter().fold(Cubes::default(), |acc, pull| Cubes {
                red: u32::max(acc.red, pull.red),
                green: u32::max(acc.green, pull.green),
                blue: u32::max(acc.blue, pull.blue),
            });
            min_cubes.red * min_cubes.green * min_cubes.blue
        })
        .fold(0, |acc, power| acc + power)
}

fn main() {
    let input_str = include_str!("../input.txt");
    let games = parse_input(input_str);
    let max_cubes = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    let part_1_total = possible_games(&max_cubes, &games);
    println!("part 1: {}", part_1_total);

    let part_2_total = minimum_cubes(&games);
    println!("part 2: {}", part_2_total);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &'static str = r"
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ";

    #[test]
    fn input_parsing() {
        let games = parse_input(SAMPLE_DATA);
        assert_eq!(games[0].id, 1);
        assert_eq!(games[0].pulls[0].blue, 3);

        assert_eq!(games[4].id, 5);
        assert_eq!(games[4].pulls[1].green, 2);
    }

    #[test]
    fn it_should_find_the_possible_games() {
        let max_cubes = Cubes {
            red: 12,
            green: 13,
            blue: 14,
        };

        let input = parse_input(SAMPLE_DATA);
        let total = possible_games(&max_cubes, &input);
        assert_eq!(total, 8);
    }

    #[test]
    fn it_should_find_the_minimum_number_of_cubes_for_each_game() {
        let input = parse_input(SAMPLE_DATA);
        let total = minimum_cubes(&input);
        assert_eq!(total, 2286);
    }
}
