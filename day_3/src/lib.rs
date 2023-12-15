use std::{collections::HashMap, usize};

pub fn part_1(input: &str) -> usize {
    let symbol_map = map_symbols(input);
    return input.lines().enumerate().fold(0, |sum, (i, line)| {
        sum + find_nums(line)
            .iter()
            .filter(|num| !num.find_adjacent_symbols(i, &symbol_map).is_empty())
            .map(|part_num| part_num.val)
            .sum::<usize>()
    });
}

pub fn part_2(input: &str) -> usize {
    let symbol_map = map_symbols(input);
    return input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, line)| {
            for num in find_nums(line).iter() {
                let adjacent = num.find_adjacent_symbols(i, &symbol_map);
                for &pos in adjacent.iter() {
                    if symbol_map.get(&pos) == Some(&'*') {
                        map.entry(pos).or_insert(Vec::new()).push(num.val);
                    };
                }
            }
            return map;
        })
        .iter()
        .filter(|(_, nums)| nums.len() == 2)
        .map(|(_, nums)| nums.iter().product::<usize>())
        .sum();
}

struct NumMatch {
    val: usize,
    start: usize,
    end: usize,
}

impl NumMatch {
    fn find_adjacent_symbols(
        &self,
        line: usize,
        symbols: &HashMap<(usize, usize), char>,
    ) -> Vec<(usize, usize)> {
        let mut adjacent = vec![];
        for i in line.saturating_sub(1)..=line + 1 {
            for j in self.start.saturating_sub(1)..=self.end + 1 {
                match symbols.get(&(i, j)) {
                    Some(_) => adjacent.push((i, j)),
                    None => (),
                };
            }
        }
        return adjacent;
    }
}

fn map_symbols(input: &str) -> HashMap<(usize, usize), char> {
    let mut symbol_map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.char_indices() {
            if !char.is_digit(10) && char != '.' {
                symbol_map.insert((i, j), char);
            }
        }
    }
    return symbol_map;
}

fn find_nums(input: &str) -> Vec<NumMatch> {
    let mut res = vec![];
    let mut num = String::new();
    let mut left = 0;

    for (right, char) in input.char_indices() {
        if char.is_digit(10) {
            num.push(char);
        } else {
            if !num.is_empty() {
                res.push(NumMatch {
                    val: num.parse::<usize>().unwrap(),
                    start: left,
                    end: num.len() + left - 1,
                });
                num.clear();
            }
            left = right + 1;
        }
    }

    if !num.is_empty() {
        res.push(NumMatch {
            val: num.parse::<usize>().unwrap(),
            start: left,
            end: num.len() + left - 1,
        });
    }

    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../input_example_1.txt");
        assert_eq!(part_1(input), 4361);
    }

    #[test]
    fn test02() {
        let input = include_str!("../input_example_1.txt");
        assert_eq!(part_2(input), 467835);
    }
}
