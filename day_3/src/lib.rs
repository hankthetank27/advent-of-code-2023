use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let mut symbol_map = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.char_indices() {
            if !char.is_ascii_digit() && char != '.' {
                symbol_map.insert((i as isize, j as isize));
            }
        }
    }

    return input.lines().enumerate().fold(0, |mut sum, (i, line)| {
        let mut curr_num = String::new();
        let mut curr_idx = -1;

        for (j, char) in line.char_indices() {
            if char.is_ascii_digit() {
                curr_num.push(char);
            } else {
                if !curr_num.is_empty() {
                    let curr = (curr_idx, curr_num.len() as isize + curr_idx);
                    if check_ajacent(i, curr, &symbol_map) {
                        sum += curr_num.parse::<usize>().unwrap();
                    }
                }
                curr_num.clear();
                curr_idx = j as isize;
            }
        }

        let curr = (curr_idx, curr_num.len() as isize + curr_idx);
        if !curr_num.is_empty() && check_ajacent(i, curr, &symbol_map) {
            sum += curr_num.parse::<usize>().unwrap();
        }

        sum
    });
}

fn check_ajacent(line_idx: usize, num_idx: (isize, isize), map: &HashSet<(isize, isize)>) -> bool {
    for i in (line_idx as isize - 1)..=(line_idx as isize + 1) {
        for j in num_idx.0..=num_idx.1 + 1 {
            if map.contains(&(i as isize, j as isize)) {
                return true;
            };
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../input_example_1.txt");
        assert_eq!(part_1(input), 4361);
    }
}
