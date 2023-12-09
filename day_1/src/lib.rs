use std::{str::Chars, u32};

const SPELLED: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

trait ProcessLine {
    fn filter_nums(self) -> usize;
    fn convert_words(self, line: &str) -> String;
}

impl ProcessLine for Chars<'_> {
    fn filter_nums(self) -> usize {
        let nums = self.filter(|char| char.is_digit(10)).collect::<Vec<char>>();

        let first = nums.first().unwrap_or(&'0');
        let last = nums.last().unwrap_or(&'0');

        return format!("{first}{last}")
            .parse::<usize>()
            .expect("Should parse as `usize`.");
    }

    fn convert_words(self, line: &str) -> String {
        return self
            .enumerate()
            .fold(String::new(), |mut digits, (i, char)| {
                let substring = line.chars().skip(i).collect::<String>();
                let digit_word = SPELLED
                    .iter()
                    .enumerate()
                    .find(|(_, &w)| substring.starts_with(w));

                match digit_word {
                    Some((j, _)) => {
                        digits.push(char::from_digit((j + 1) as u32, 10).unwrap_or('0'));
                    }
                    None => digits.push(char),
                }
                return digits;
            });
    }
}

pub fn part_1(input: &str) -> usize {
    return input.lines().map(|line| line.chars().filter_nums()).sum();
}

pub fn part_2(input: &str) -> usize {
    return input
        .lines()
        .map(|line| line.chars().convert_words(&line).chars().filter_nums())
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
