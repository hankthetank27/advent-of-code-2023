use day_1::{part_1, part_2};

fn main() {
    let input = include_str!("../input.txt");
    let answer_one = part_1(input);
    let answer_two = part_2(input);
    println!("answer part 1: {answer_one}");
    println!("answer part 2: {answer_two}");
}
