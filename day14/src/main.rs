mod data;

extern crate itertools;

use itertools::Itertools;

use std::collections::HashMap;

fn make_step(polymer: &String, table: &HashMap<&str, &str>) -> String {
    let mut new_polymer = polymer
        .chars()
        .zip(polymer.chars().skip(1))
        .map(|(a, b)| {
            let key = [a, b].into_iter().collect::<String>();
            match table.get(key.as_str()) {
                Some(value) => value.chars().take(2).collect(),
                None => String::from(a),
            }
        })
        .collect::<String>();
    new_polymer.push( polymer
        .chars().last().unwrap());
    new_polymer
}

fn calculate_solution(polymer_template: &str, rules: &[(&str, &str)]) -> usize {
    let lookup_table: HashMap<_, _> = rules.iter().cloned().collect();
    let mut result = polymer_template.to_owned();
    for _ in 0..10 {
        let new_string = make_step(&result, &lookup_table);
        result = new_string;
    }
    println!("{} {}", result, result.len());
    let mut frequencies = result.chars().unique()
        .map(|tc| result.chars().filter(|c| *c == tc).count())
        .collect::<Vec<_>>();
    frequencies.sort();
    println!("{:?}", frequencies);
    frequencies.iter().last().unwrap() - frequencies[0]
}

fn main() {
    println!(
        "Solution {:?}",
        calculate_solution(data::POLYMER_TEMPLATE, &data::RULES)
    );
}
