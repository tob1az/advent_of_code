extern crate rayon;

mod data;

use std::collections::HashMap;
use rayon::prelude::*;

type Polymer = Vec<u8>;
type PolymerView = [u8];
type LookupTable = HashMap<Polymer, Polymer>;

const SPLIT_THRESHOLD: usize = 10;
const STEP_COUNT: usize = 30;
const WINDOW_SIZE: usize = 2;
const HISTOGRAM_SIZE: usize = 256;

struct Histogram {
    frequencies: [usize; HISTOGRAM_SIZE],
}

impl Histogram {

    fn new() -> Histogram {
        Histogram {
            frequencies: [0; HISTOGRAM_SIZE],
        }
    }

    fn update(&mut self, polymer: &PolymerView) {
        for element in polymer.iter() {
            let index = *element as usize;
            self.frequencies[index] += 1;
        }
    }

    fn merge(&mut self, another: Histogram) {
        for i in 0..HISTOGRAM_SIZE {
            self.frequencies[i] += another.frequencies[i];
        }
    }

    fn add(&self, another: Histogram) -> Histogram {
        let mut new_histogram = Self::new();
        for i in 0..HISTOGRAM_SIZE {
            new_histogram.frequencies[i] = self.frequencies[i] + another.frequencies[i];
        }
        new_histogram
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
        let last_window = result.len() - WINDOW_SIZE;
        let update = result.par_windows(WINDOW_SIZE).enumerate().map(|(window_number, window)| {
            let mut new_histogram  = Histogram::new();
            let remaining_steps = step_count - SPLIT_THRESHOLD;
            grow_polymer(
                window,
                lookup_table,
                remaining_steps,
                window_number != last_window,
                &mut new_histogram,
            );
            new_histogram
        }).reduce(Histogram::new, |total, h| total.add(h));
        histogram.merge(update);
    }
}

fn calculate_solution(polymer_template: &str, rules: &[(&str, &str)]) -> usize {
    let lookup_table: LookupTable = rules
        .iter()
        .map(|(k, v)| (k.as_bytes().to_vec(), v.as_bytes().to_vec()))
        .collect();
    let mut histogram = Histogram::new();

    println!("Calculate {} steps", STEP_COUNT);

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
