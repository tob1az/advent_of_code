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
        let x_range = if line.b.x > line.a.x {
            (line.a.x..=line.b.x).into_iter().collect::<Vec<u32>>()
        } else {
            (line.b.x..=line.a.x)
                .into_iter()
                .rev()
                .collect::<Vec<u32>>()
        };

        let y_range = if line.b.y > line.a.y {
            (line.a.y..=line.b.y).into_iter().collect::<Vec<u32>>()
        } else {
            (line.b.y..=line.a.y)
                .into_iter()
                .rev()
                .collect::<Vec<u32>>()
        };
        x_range
            .into_iter()
            .zip(y_range.into_iter())
            .map(|(x, y)| data::Point { x, y })
            .collect::<HashSet<data::Point>>()
    }
}

fn count_overlapping_points(polylines: &Vec<HashSet<data::Point>>) -> u32 {
    let mut overlapping_points: HashSet<&data::Point> = HashSet::new();
    for (x, y) in polylines.iter().tuple_combinations() {
        for point in x.intersection(&y) {
            overlapping_points.insert(point);
        }
    }
    overlapping_points.len() as u32
}

fn calculate_solution(lines: &[data::Line]) -> (u32, u32) {
    let orthogonal_lines: Vec<HashSet<data::Point>> = lines
        .iter()
        .filter(|l| l.a.x == l.b.x || l.a.y == l.b.y)
        .map(|l| generate_polyline(&l))
        .collect();
    let all_lines: Vec<HashSet<data::Point>> =
        lines.iter().map(|l| generate_polyline(&l)).collect();

    let cross_points = count_overlapping_points(&orthogonal_lines);
    let all_intersection_points = count_overlapping_points(&all_lines);
    (cross_points, all_intersection_points)
}

fn main() {
    let (cross_count, intersection_count) = calculate_solution(&data::LINES);
    println!("Solution {} {}", cross_count, intersection_count);
}
