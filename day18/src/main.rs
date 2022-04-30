extern crate itertools;
mod data;
mod snailfish;

use itertools::Itertools;

fn calculate_solution(number_string: &str) -> (usize, usize) {
    let numbers = number_string
        .lines()
        .map(|line| snailfish::Number::parse(line).unwrap())
        .collect::<Vec<_>>();

    let sum = numbers
        .iter()
        .cloned()
        .reduce(|a, b| snailfish::add(a, b))
        .unwrap();

    let max_sum_magnitude = numbers
        .into_iter()
        .permutations(2)
        .map(|ab| {
            ab.into_iter()
                .reduce(|a, b| snailfish::add(a, b))
                .unwrap()
                .magnitude()
        })
        .max()
        .unwrap();
    (sum.magnitude(), max_sum_magnitude)
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}
