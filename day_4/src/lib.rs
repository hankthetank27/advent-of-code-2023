use std::collections::{HashMap, HashSet};

pub fn part_1(input: &str) -> usize {
    return parse_cards(input)
        .iter()
        .map(|card| {
            card.winning_nums
                .iter()
                .filter(|num| card.my_nums_map.get(num).is_some())
                .fold(0, |res, _| if res == 0 { 1 } else { res * 2 })
        })
        .sum();
}

pub fn part_2(input: &str) -> usize {
    let mut memo = HashMap::new();
    let cards = parse_cards(input);
    return cards
        .iter()
        .map(|card| 1 + card.find_next(&cards, &mut memo))
        .sum();
}

#[derive(Debug)]
struct Card {
    card_num: usize,
    winning_nums: Vec<usize>,
    my_nums_map: HashSet<usize>,
}

impl Card {
    fn find_next(&self, rest: &Vec<Card>, memo: &mut HashMap<usize, usize>) -> usize {
        let mut matches = self
            .winning_nums
            .iter()
            .filter(|num| self.my_nums_map.get(num).is_some())
            .collect::<Vec<&usize>>()
            .len();

        for card_num in self.card_num + 1..=matches + self.card_num {
            if memo.contains_key(&card_num) {
                matches += memo.get(&card_num).unwrap();
            } else {
                let sum = rest[card_num].find_next(&rest, memo);
                matches += *memo.entry(card_num).or_insert(sum);
            }
        }

        return matches;
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    return input
        .lines()
        .enumerate()
        .map(|(card_num, line)| {
            let line = line.split_once(":").unwrap().1;
            let (winning_nums, my_nums) = line.split_once("|").unwrap();

            let my_nums_map = my_nums
                .split_whitespace()
                .fold(HashSet::new(), |mut map, num| {
                    map.insert(num.parse::<usize>().unwrap());
                    return map;
                });

            let winning_nums = winning_nums
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            return Card {
                card_num,
                winning_nums,
                my_nums_map,
            };
        })
        .collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../input_example_1.txt");
        assert_eq!(part_1(input), 13);
    }

    #[test]
    fn test02() {
        let input = include_str!("../input_example_1.txt");
        assert_eq!(part_2(input), 30);
    }
}
