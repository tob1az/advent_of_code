mod data;

use std::collections::HashSet;

type Distance = i32;
type Elevation = i32;
type Point = (Distance, Elevation);
type Velocity = i32;

fn calculate_trajectory(
    start_vx: Velocity,
    start_vy: Velocity,
    max_x: i32,
    min_y: i32,
) -> Vec<Point> {
    let mut x = 0;
    let mut y = 0;
    let mut vx = start_vx;
    let mut vy = start_vy;
    std::iter::from_fn(move || {
        x += vx;
        y += vy;
        if vx > 0 {
            vx -= 1;
        }
        vy -= 1;
        if x <= max_x && y >= min_y {
            Some((x, y))
        } else {
            None
        }
    })
    .collect()
}

fn detect_hit_point(
    trajectory: &[Point],
    from_x: Distance,
    to_x: Distance,
    from_y: Elevation,
    to_y: Elevation,
) -> Option<Point> {
    trajectory
        .iter()
        .cloned()
        .filter(|(x, y)| *x >= from_x && *x <= to_x && *y >= from_y && *y <= to_y)
        .nth(0)
}

fn find_min_acceptable_vx(final_x: Distance) -> i32 {
    let mut x = final_x;
    let mut v = 0;
    while x > 0 {
        v += 1;
        x -= v;
    }
    v
}

fn find_steepest_hit_curve_elevation(
    from_x: Distance,
    to_x: Distance,
    from_y: Elevation,
    to_y: Elevation,
) -> Elevation {
    // hitting at the nearest end of the target area with minimum acceptable horizontal speed
    // gives the steepest projectile curve
    let end_x = from_x;
    let vx = find_min_acceptable_vx(end_x);
    let mut max_elevation = 0;
    // find maximum vertical speed which does not cause the probe pass the target area
    for vy in 1..100 {
        let trajectory = calculate_trajectory(vx, vy, to_x, from_y);
        let zenith = trajectory.iter().max_by_key(|(_, y)| *y).unwrap();
        let hit = detect_hit_point(&trajectory, from_x, to_x, from_y, to_y);
        if !hit.is_none() {
            max_elevation = std::cmp::max(max_elevation, zenith.1);
        }
    }
    max_elevation
}

fn find_matching_start_velocities(to_x: Distance) -> Vec<Velocity> {
    let mut start_vxs = Vec::new();
    for end_vx in 0..to_x {
        let mut x = to_x;
        let mut v = end_vx;
        while x > 0 {
            v += 1;
            x -= v;
        }
        if x == 0 {
            start_vxs.push(v);
        }
    }
    start_vxs
}

fn count_all_possible_hit_velocities(
    from_x: Distance,
    to_x: Distance,
    from_y: Elevation,
    to_y: Elevation,
) -> usize {
    let mut counter = 0;
    let mut velocities = HashSet::new();
    for hit_x in from_x..=to_x {
        velocities.extend(find_matching_start_velocities(hit_x));
    }
    for vx in velocities {
        for vy in from_y..=100 {
            let trajectory = calculate_trajectory(vx, vy, to_x, from_y);
            if !detect_hit_point(&trajectory, from_x, to_x, from_y, to_y).is_none() {
                counter += 1;
            }
        }
    }
    counter
}

fn calculate_solution(
    from_x: Distance,
    to_x: Distance,
    from_y: Elevation,
    to_y: Elevation,
) -> (Elevation, usize) {
    (
        find_steepest_hit_curve_elevation(from_x, to_x, from_y, to_y),
        count_all_possible_hit_velocities(from_x, to_x, from_y, to_y),
    )
}

fn main() {
    println!(
        "Solution {:?}",
        calculate_solution(data::X_FROM, data::X_TO, data::Y_FROM, data::Y_TO)
    );
}
