mod data;

extern crate itertools;

use itertools::Itertools;
use std::collections::HashSet;

fn generate_polyline(line: &data::Line) -> HashSet<data::Point> {
    // horizontal
    if line.a.x == line.b.x {
        let x = line.a.x;
        let (upper, lower) = if line.a.y > line.b.y {
            (line.a.y, line.b.y)
        } else {
            (line.b.y, line.a.y)
        };
        (lower..=upper)
            .into_iter()
            .map(|y| data::Point { x, y })
            .collect::<HashSet<data::Point>>()
    // vertical
    } else if line.a.y == line.b.y {
        let y = line.a.y;
        let (upper, lower) = if line.a.x > line.b.x {
            (line.a.x, line.b.x)
        } else {
            (line.b.x, line.a.x)
        };
        (lower..=upper)
            .into_iter()
            .map(|x| data::Point { x, y })
            .collect::<HashSet<data::Point>>()
    } else {
        unimplemented!()
    }
}

fn calculate_solution(lines: &[data::Line]) -> u32 {
    // find all points of each orthogonal line
    // find all intersections
    let orthogonal_lines: Vec<HashSet<data::Point>> = lines
        .iter()
        .filter(|l| l.a.x == l.b.x || l.a.y == l.b.y)
        .map(|l| generate_polyline(&l))
        .collect();
    let mut overlapping_points: HashSet<&data::Point> = HashSet::new();
    for (x, y) in orthogonal_lines.iter().tuple_combinations() {
        if x.intersection(&y).count() > 0 {
            overlapping_points = overlapping_points
                .drain()
                .chain(x.intersection(&y))
                .collect();
        }
    }
    overlapping_points.len() as u32
}

fn main() {
    println!("Solution {}", calculate_solution(&data::LINES));
}
