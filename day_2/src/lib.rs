pub fn part_1(input: &str) -> usize {
    return input
        .lines()
        .map(|game| {
            let (id_str, reveals) = game.split_once(':').unwrap_or(("", ""));
            let id = get_num(id_str);

            let is_not_valid = reveals.split(";").find(|reveal| {
                return reveal
                    .split(',')
                    .find(|cube| {
                        let num = get_num(cube);
                        return match cube.split_whitespace().nth(1).unwrap_or("invalid") {
                            "red" => num > 12,
                            "green" => num > 13,
                            "blue" => num > 14,
                            _ => true,
                        };
                    })
                    .is_some();
            });

            return match is_not_valid {
                Some(_) => 0,
                None => id,
            };
        })
        .sum();
}

fn get_num(word: &str) -> usize {
    return word
        .chars()
        .filter(|char| char.is_digit(10))
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../input_example_1.txt");
        assert_eq!(part_1(input), 8);
    }
}
