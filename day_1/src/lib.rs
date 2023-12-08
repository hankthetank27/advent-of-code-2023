const SPELLED: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_1(input: &str) -> usize {
    return input
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .filter(|char| char.is_digit(10))
                .collect::<Vec<char>>();

            let first = nums.first().unwrap_or(&'0');
            let last = nums.last().unwrap_or(&'0');

            return format!("{first}{last}")
                .parse::<usize>()
                .expect("Should parse as `usize`.");
        })
        .sum();
}

pub fn part_2(input: &str) -> usize {
    return input
        .lines()
        .map(|line| {
            let chars = line.as_bytes();
            let nums = chars
                .iter()
                .enumerate()
                .fold(Vec::new(), |mut acc, (i, char)| {
                    if char.is_ascii_digit() {
                        acc.push((char - b'0') as u32)
                    } else {
                        for (j, word) in SPELLED.iter().enumerate() {
                            if chars[i..].starts_with(word.as_bytes()) {
                                acc.push(j as u32 + 1);
                            }
                        }
                    }
                    return acc;
                });

            let first = nums.first().unwrap_or(&0);
            let last = nums.last().unwrap_or(&0);

            return format!("{first}{last}")
                .parse::<usize>()
                .expect("Should parse as `usize`.");
        })
        .sum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../input_example_1.txt");
        assert_eq!(part_1(input), 142);
    }

    #[test]
    fn test02() {
        let input = include_str!("../input_example_2.txt");
        assert_eq!(part_2(input), 281);
    }
}
