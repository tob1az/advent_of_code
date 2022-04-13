extern crate itertools;

mod data;

use itertools::Itertools;
use std::collections::HashMap;

type Pair = (u8, u8);
type LookupTable = HashMap<Pair, (Pair, Pair)>;
type PairFrequencies = HashMap<Pair, usize>;

#[derive(Clone)]
struct Polymer {
    frequencies: PairFrequencies,
}

impl Polymer {
    fn from(polymer_template: &str) -> Self {
        let mut frequencies: PairFrequencies = HashMap::new();

        for w in polymer_template.as_bytes().windows(2) {
            let pair = w.iter().cloned().next_tuple().unwrap();
            *frequencies.entry(pair).or_insert(0) += 1;
        }

        Self { frequencies }
    }

    fn grow_into(&self, rules: &LookupTable) -> Self {
        let mut frequencies: PairFrequencies = HashMap::new();

        self.frequencies.iter().for_each(|(k, v)| {
            let new_pairs = rules.get(k).unwrap();
            *frequencies.entry(new_pairs.0).or_insert(0) += v;
            *frequencies.entry(new_pairs.1).or_insert(0) += v;
        });

        Self { frequencies }
    }
}

fn grow_polymer(rules: &LookupTable, base: &Polymer, step_count: usize) -> Polymer {
    let mut polymer = base.clone();
    for _i in 0..step_count {
        let new_polymer = polymer.grow_into(rules);
        polymer = new_polymer;
    }
    polymer
}

// Almost each element is duplicated in 2 pairs that wrap it around, except for elements at both ends.
// To compensate missing neighbor pairs, we increase frequency. It will not affect other elements
// because of integer division
fn element_frequency(pair_element_frequency: usize) -> usize {
    (pair_element_frequency + 1) / 2
}

fn compute_element_frequencies_span(histogram: Polymer) -> usize {
    let mut pair_element_frequencies: HashMap<u8, usize> = HashMap::new();

    histogram.frequencies.into_iter().for_each(|(k, v)| {
        *pair_element_frequencies.entry(k.0).or_insert(0) += v;
        *pair_element_frequencies.entry(k.1).or_insert(0) += v;
    });

    let element_frequencies: Vec<_> = pair_element_frequencies
        .values()
        .map(|v| element_frequency(*v))
        .sorted()
        .collect();

    let least_common = element_frequencies.iter().next().unwrap();
    let most_common = element_frequencies.iter().last().unwrap();
    println!("min {} max {}", least_common, most_common);

    most_common - least_common
}

fn create_pair(value: &str) -> Pair {
    value.as_bytes().iter().cloned().next_tuple().unwrap()
}

fn calculate_solution(polymer_template: &str, rules: &[(&str, (&str, &str))]) -> (usize, usize) {
    let lookup_table: LookupTable = rules
        .iter()
        .map(|(k, v)| (create_pair(*k), (create_pair(v.0), create_pair(v.1))))
        .collect();
    let base = Polymer::from(polymer_template);

    (
        compute_element_frequencies_span(grow_polymer(&lookup_table, &base, 10)),
        compute_element_frequencies_span(grow_polymer(&lookup_table, &base, 40)),
    )
}

fn main() {
    println!(
        "Solution {:?}",
        calculate_solution(data::POLYMER_TEMPLATE, &data::RULES)
    );
}
