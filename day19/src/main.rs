mod data;
mod scanner;

extern crate itertools;
extern crate once_cell;
extern crate petgraph;
extern crate rayon;

use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::{Graph, Undirected};
use rayon::prelude::*;

use std::collections::{HashMap, HashSet};

fn find_scanner_mappings(reports: &[scanner::Report]) -> Vec<scanner::Mapping> {
    reports
        .iter()
        .enumerate()
        .combinations(2)
        .collect_vec()
        .par_iter()
        .filter_map(|report_pair| {
            println!(
                "find intersection between {} and {}",
                report_pair[0].0, report_pair[1].0
            );
            let origin_pairs = report_pair[0].1.beacon_pairs();
            let pairs = report_pair[1].1.beacon_pairs();
            for transformation in origin_pairs
                .into_iter()
                .cartesian_product(pairs)
                .flat_map(|(origin, pair)| pair.find_possible_transformations(&origin))
            {
                let transformations = vec![transformation];
                let transformed_beacons = report_pair[1]
                    .1
                    .transformed(&transformations)
                    .beacons
                    .into_iter()
                    .collect::<HashSet<_>>();
                let intersection_count = report_pair[0]
                    .1
                    .beacons
                    .iter()
                    .filter(|b| transformed_beacons.contains(b))
                    .count();
                if intersection_count >= 12 {
                    println!(
                        "intersection_count = {:?} for scanners ({}, {})",
                        intersection_count, report_pair[0].0, report_pair[1].0,
                    );
                    return Some(scanner::Mapping {
                        from_index: report_pair[1].0,
                        to_index: report_pair[0].0,
                        transformations: transformations,
                    });
                }
            }
            None
        })
        .collect::<Vec<_>>()
}

fn deduce_transformations_to_origin(
    reports: &[scanner::Report],
    mappings: Vec<scanner::Mapping>,
    transformation_to_origin: &HashMap<usize, Vec<scanner::Transformation>>,
) -> HashMap<usize, Vec<scanner::Transformation>> {
    let mut deduces_transformations = HashMap::new();
    let unmapped_indices = (1..reports.len())
        .filter(|i| !transformation_to_origin.contains_key(i))
        .collect_vec();
    println!("unmapped: {:?}", unmapped_indices);
    let mut graph: Graph<(), usize, Undirected, usize> = Graph::default();
    let nodes = (0..reports.len()).map(|_| graph.add_node(())).collect_vec();
    let mut transformation_map = HashMap::new();
    for m in mappings.iter() {
        graph.add_edge(nodes[m.from_index], nodes[m.to_index], 1);
        transformation_map.insert((m.from_index, m.to_index), m.clone());
        transformation_map.insert((m.to_index, m.from_index), m.inverted());
    }
    for (source_index, transformations) in transformation_to_origin.iter() {
        graph.add_edge(nodes[*source_index], nodes[0], 1);
        let mapping = scanner::Mapping {
            from_index: *source_index,
            to_index: 0,
            transformations: transformations.clone(),
        };
        transformation_map.insert((0, *source_index), mapping.inverted());
        transformation_map.insert((*source_index, 0), mapping);
    }

    for report_index in unmapped_indices.into_iter() {
        if let Some((_, path)) = astar(
            &graph,
            nodes[report_index],
            |finish| finish == nodes[0],
            |_| 1,
            |_| 0,
        ) {
            println!("transformation from {} to 0: {:?}", report_index, path);
            let transformations = path
                .into_iter()
                .tuple_windows()
                .flat_map(|(a, b)| {
                    transformation_map[&(a.index(), b.index())]
                        .transformations
                        .clone()
                })
                .collect_vec();
            deduces_transformations.insert(report_index, transformations);
        } else {
            println!("failed to find transformation from {} to 0", report_index);
        }
    }
    deduces_transformations
}

fn calculate_solution<T>(coordinates: &[T]) -> (usize, i32)
where
    T: AsRef<[i32]>,
{
    let reports = coordinates
        .iter()
        .map(|c| scanner::Report::new(c.as_ref()))
        .collect_vec();
    println!("report count {}", reports.len());
    debug_assert!(reports.len() > 1);
    let mut mappings = find_scanner_mappings(&reports);

    let transformation_to_origin = mappings
        .iter()
        .filter_map(|t| {
            if t.from_index == 0 {
                debug_assert!(!t.transformations.is_empty());
                Some((t.to_index, vec![t.transformations[0].inverted()]))
            } else if t.to_index == 0 {
                debug_assert!(!t.transformations.is_empty());
                Some((t.from_index, vec![t.transformations[0].clone()]))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    mappings.retain(|t| t.from_index != 0 && t.to_index != 0);
    let deduced_transformations =
        deduce_transformations_to_origin(&reports, mappings, &transformation_to_origin);

    let mut unique_beacons = reports[0]
        .beacons
        .iter()
        .cloned()
        .collect::<HashSet<scanner::Object>>();

    let mut scanners = Vec::new();
    for (report_index, transformations) in transformation_to_origin
        .into_iter()
        .chain(deduced_transformations.into_iter())
        .sorted_by_key(|(_, t)| t.len())
    {
        scanners.push(
            scanner::Object {
                coordinates: [0, 0, 0],
            }
            .transformed(&transformations),
        );
        let report = reports[report_index].transformed(&transformations);
        let beacons = report.beacons.into_iter().collect();
        unique_beacons = unique_beacons.union(&beacons).cloned().collect();
    }
    let max_manhattan = scanners
        .iter()
        .combinations(2)
        .map(|pair| scanner::manhattan(pair[0], pair[1]))
        .max()
        .unwrap();
    (unique_beacons.len(), max_manhattan)
}

fn main() {
    println!(
        "Solution {:?}",
        calculate_solution(&data::SCANNER_COORDINATES)
    );
}
