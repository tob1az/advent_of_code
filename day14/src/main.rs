mod data;

extern crate itertools;

use itertools::Itertools;

use std::collections::HashMap;

type LookupTable<'a> = HashMap<&'a str, &'a str>;
type Histogram = HashMap<char, usize>;

fn make_step(polymer: &String, table: &LookupTable) -> String {
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
    new_polymer.push(polymer.chars().last().unwrap());
    new_polymer
}

fn grow_polymer(polymer: String, lookup_table: &LookupTable) -> String {
    let mut result = polymer;
    for i in 0..10 {
        let new_string = make_step(&result, lookup_table);
        result = new_string;
        println!("{} => {}", i, result.len());
    }
    result
}

fn make_histogram(polymer: String) -> Histogram {
    polymer
        .chars()
        .unique()
        .map(|tc| (tc, polymer.chars().filter(|c| *c == tc).count()))
        .collect::<Histogram>()
}

fn calculate_solution(polymer_template: &str, rules: &[(&str, &str)]) -> usize {
    let lookup_table: LookupTable = rules.iter().cloned().collect();
    let result = grow_polymer(polymer_template.to_string(), &lookup_table);
    //println!("{} {}", result, result.len());
    let mut frequencies = make_histogram(result).values().cloned().collect::<Vec<_>>();
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
