mod data;

extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;

type Polymer = Vec<u8>;
type LookupTable = HashMap<Polymer, Polymer>;
type Histogram = HashMap<u8, usize>;

fn make_step(polymer: &Polymer, table: &LookupTable) -> Polymer {
    let mut new_polymer = polymer
        .windows(2)
        .flat_map(|key| match table.get(key) {
            Some(value) => value.clone(),
            None => vec![key[0]],
        })
        .collect::<Polymer>();
    new_polymer.push(*polymer.last().unwrap());
    new_polymer
}

fn grow_polymer(polymer: Polymer, lookup_table: &LookupTable, step_count: usize) -> Polymer {
    let mut result = polymer;
    for i in 0..step_count {
        let new_string = make_step(&result, lookup_table);
        result = new_string;
        println!("{} => {}", i, result.len());
    }
    result
}

fn make_histogram(polymer: Polymer) -> Histogram {
    polymer
        .iter()
        .unique()
        .map(|tc| (*tc, polymer.iter().filter(|c| *c == tc).count()))
        .collect::<Histogram>()
}

fn calculate_solution(polymer_template: &str, rules: &[(&str, &str)]) -> usize {
    let lookup_table: LookupTable = rules
        .iter()
        .cloned()
        .map(|(k, v)| (k.as_bytes().to_vec(), v.as_bytes().to_vec()))
        .collect();
    let result = grow_polymer(polymer_template.as_bytes().to_vec(), &lookup_table, 10);
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
