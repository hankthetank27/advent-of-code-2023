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
        let max_cubes = game
            .iter()
            .fold(CubeSet::new(), |acc_cubes, set| acc_cubes.to_maxes(&set));
        return sum + (max_cubes.red * max_cubes.blue * max_cubes.green);
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

    fn to_maxes(mut self, other: &CubeSet) -> Self {
        self.red = cmp::max(other.red, self.red);
        self.blue = cmp::max(other.blue, self.blue);
        self.green = cmp::max(other.green, self.green);
        self
    }
}

fn parse_games(games: &str) -> Vec<Vec<CubeSet>> {
    return games
        .lines()
        .map(|game| {
            let sets = game.split_once(':').unwrap().1;
            return sets.split(";").map(|set| parse_cube_set(set)).collect();
        })
        .collect();
}

fn parse_cube_set(set: &str) -> CubeSet {
    return set.split(',').fold(CubeSet::new(), |mut cube_set, cube| {
        let mut iter_cube = cube.split_whitespace();
        let num = iter_cube.next().unwrap().parse::<usize>().unwrap();
        let color = iter_cube.next().unwrap();

        match color {
            "red" => cube_set.red = num,
            "green" => cube_set.green = num,
            "blue" => cube_set.blue = num,
            _ => unreachable!(),
        };

        return cube_set;
    });
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
