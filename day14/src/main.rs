mod data;

use std::collections::HashMap;

type Polymer = Vec<u8>;
type PolymerView = [u8];
type LookupTable = HashMap<Polymer, Polymer>;

const SPLIT_THRESHOLD: usize = 10;
const STEP_COUNT: usize = 30;

struct Histogram {
    frequencies: [usize; 256],
}

impl Histogram {
    fn new() -> Histogram {
        Histogram {
            frequencies: [0; 256],
        }
    }

    fn update(&mut self, polymer: &PolymerView) {
        for element in polymer.iter() {
            let index = *element as usize;
            self.frequencies[index] += 1;
        }
    }
}

fn make_step(polymer: &PolymerView, table: &LookupTable) -> Polymer {
    let mut new_polymer = Polymer::with_capacity(polymer.len() * 2);
    for key in polymer.windows(2) {
        match table.get(key) {
            Some(value) => {
                new_polymer.push(value[0]);
                new_polymer.push(value[1])
            }
            None => new_polymer.push(key[0]),
        }
    }
    new_polymer.push(*polymer.last().unwrap());
    new_polymer
}

fn to_string(polymer: &PolymerView) -> String {
    std::str::from_utf8(polymer).unwrap().to_string()
}

fn grow_polymer(
    polymer: &PolymerView,
    lookup_table: &LookupTable,
    step_count: usize,
    cut_last_element: bool,
    histogram: &mut Histogram,
) {
    let mut result = polymer;
    let steps_left = std::cmp::min(step_count, SPLIT_THRESHOLD);
    let mut new_polymer;
    for _i in 0..steps_left {
        new_polymer = make_step(&result, lookup_table);
        /*println!(
            "{} - {} => {}, {}",
            _i,
            to_string(&result),
            new_polymer.len(),
            to_string(&new_polymer),
        );*/
        result = new_polymer.as_slice();
    }

    if step_count <= SPLIT_THRESHOLD {
        if cut_last_element {
            result = &result[0..result.len() - 1];
        }
        histogram.update(&result);
    } else {
        let mut window_number = 0;
        let last_window = result.len() - 2;
        for window in result.windows(2) {
            let remaining_steps = step_count - SPLIT_THRESHOLD;
            grow_polymer(
                window,
                lookup_table,
                remaining_steps,
                window_number != last_window,
                histogram,
            );
            window_number += 1;
        }
    }
}

fn calculate_solution(polymer_template: &str, rules: &[(&str, &str)]) -> usize {
    let lookup_table: LookupTable = rules
        .iter()
        .map(|(k, v)| (k.as_bytes().to_vec(), v.as_bytes().to_vec()))
        .collect();
    let mut histogram = Histogram::new();

    grow_polymer(
        polymer_template.as_bytes(),
        &lookup_table,
        STEP_COUNT,
        false,
        &mut histogram,
    );
    histogram.frequencies.iter().max().unwrap()
        - histogram
            .frequencies
            .iter()
            .filter(|f| **f != 0)
            .min()
            .unwrap()
}

fn main() {
    println!(
        "Solution {:?}",
        calculate_solution(data::POLYMER_TEMPLATE, &data::RULES)
    );
}
