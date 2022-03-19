mod data;

extern crate itertools;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn find_unique(digits: &Vec<HashSet<char>>, digit_len: usize) -> &HashSet<char> {
    digits
        .iter()
        .filter(|digit| digit.len() == digit_len)
        .last()
        .unwrap()
}

fn decode_display(digits: &str, reading: &str) -> usize {
    // a..g are segments
    // 1=cf 2=acdeg 3=acdfg 4=bcdf 5=abdfg
    // 6=abdefg 7=acf 8=abcdefg 9=abcdfg 0=abcefg
    let encoded_digits = digits
        .split_ascii_whitespace()
        .map(|digit| digit.chars().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();

    let one = find_unique(&encoded_digits, 2);
    let seven = find_unique(&encoded_digits, 3);
    let four = find_unique(&encoded_digits, 4);

    let a = seven.difference(&one).last().unwrap();
    let cf = one;
    let bd = four.difference(&one).cloned().collect::<HashSet<char>>();

    let six_segment_digits = encoded_digits
        .iter()
        .filter(|digit| digit.len() == 6)
        .collect::<Vec<&HashSet<char>>>();

    let nine = six_segment_digits
        .iter()
        .filter(|digit| digit.contains(a) && digit.is_superset(cf) && digit.is_superset(&bd))
        .last()
        .unwrap();
    let g = nine
        .iter()
        .filter(|s| *s != a && !cf.contains(s) && !bd.contains(s))
        .last()
        .unwrap();
    let zero = six_segment_digits
        .iter()
        .filter(|digit| {
            *digit != nine && digit.contains(a) && digit.contains(g) && digit.is_superset(&cf)
        })
        .last()
        .unwrap();
    let be = zero
        .iter()
        .filter(|s| *s != a && *s != g && !cf.contains(s))
        .cloned()
        .collect::<HashSet<char>>();
    let b = be.intersection(&bd).last().unwrap();
    let d = bd.iter().filter(|s| *s != b).last().unwrap();
    let e = be.iter().filter(|s| *s != b).last().unwrap();

    let six = six_segment_digits
        .iter()
        .filter(|digit| *digit != zero && *digit != nine)
        .last()
        .unwrap();
    let f = six
        .iter()
        .filter(|s| !be.contains(s) && *s != a && *s != d && *s != g)
        .last()
        .unwrap();
    let c = cf.iter().filter(|s| *s != f).last().unwrap();

    let segment_map = HashMap::from([
        ('a', *a),
        ('b', *b),
        ('c', *c),
        ('d', *d),
        ('e', *e),
        ('f', *f),
        ('g', *g),
    ]);

    let digit_map = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .iter()
    .enumerate()
    .map(|(index, digit)| {
        (
            digit
                .chars()
                .map(|s| segment_map[&s])
                .sorted()
                .collect::<String>(),
            index,
        )
    })
    .collect::<HashMap<_, _>>();

    reading
        .split_ascii_whitespace()
        .map(|digit| digit_map[&digit.chars().sorted().collect::<String>()])
        .fold(0usize, |sum, digit| sum * 10 + digit)
}

fn calculate_solution(displays: &str) -> (usize, usize) {
    let easy_digits_count = displays
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
        .count();

    let sum_of_displayed_numbers = displays
        .lines()
        .map(|line| {
            line.split_once(" | ")
                .map(|(digits, reading)| decode_display(digits, reading))
        })
        .flatten()
        .sum();

    (easy_digits_count, sum_of_displayed_numbers)
}

fn main() {
    let (easy_digits_count, sum_of_displayed_numbers) = calculate_solution(data::DISPLAYS);
    println!(
        "Solution {} {}",
        easy_digits_count, sum_of_displayed_numbers
    );
}
