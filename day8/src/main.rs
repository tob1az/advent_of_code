mod data;

fn calculate_solution(displays: &str) -> usize {
    displays
        .lines()
        .map(|line| {
            line.split_once(" | ")
                .unwrap()
                .1
                .split_ascii_whitespace()
                .filter(|digit| {
                    digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7
                })
        })
        .flatten()
        .count()
}

fn main() {
    println!("Solution {}", calculate_solution(data::DISPLAYS));
}
