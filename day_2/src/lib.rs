use std::cmp;

pub fn part_1(input: &str) -> usize {
    return parse_games(input)
        .iter()
        .enumerate()
        .fold(0, |sum, (i, game)| {
            let is_not_valid = game.iter().find(|set| !set.is_valid()).is_some();
            sum + match is_not_valid {
                true => 0,
                false => i + 1,
            }
        });
}

pub fn part_2(input: &str) -> usize {
    return parse_games(input).iter().fold(0, |sum, game| {
        let max_cube = game.iter().fold(CubeSet::new(), |mut max_cube, cube| {
            max_cube.red = cmp::max(cube.red, max_cube.red);
            max_cube.blue = cmp::max(cube.blue, max_cube.blue);
            max_cube.green = cmp::max(cube.green, max_cube.green);
            return max_cube;
        });
        return sum + (max_cube.red * max_cube.blue * max_cube.green);
    });
}

#[derive(Debug)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    fn new() -> Self {
        CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn parse_games(games: &str) -> Vec<Vec<CubeSet>> {
    return games
        .lines()
        .map(|game| {
            let sets = game.split_once(':').unwrap().1;
            return sets
                .split(";")
                .map(|set| {
                    return set.split(',').fold(CubeSet::new(), |mut cube_set, cube| {
                        let mut cube_iter = cube.split_whitespace();
                        let num = cube_iter.next().unwrap().parse::<usize>().unwrap();
                        let color = cube_iter.next().unwrap();

                        match color {
                            "red" => cube_set.red = num,
                            "green" => cube_set.green = num,
                            "blue" => cube_set.blue = num,
                            _ => unreachable!(),
                        };

                        return cube_set;
                    });
                })
                .collect::<Vec<CubeSet>>();
        })
        .collect::<Vec<Vec<CubeSet>>>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../input_example_1.txt");
        assert_eq!(part_1(input), 8);
    }

    #[test]
    fn test02() {
        let input = include_str!("../input_example_1.txt");
        assert_eq!(part_2(input), 2286);
    }
}
