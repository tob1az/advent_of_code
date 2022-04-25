mod data;

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

fn estimate_start_vx(final_x: Distance) -> i32 {
    let mut x = final_x;
    let mut v = 0;
    while x > 0 {
        v += 1;
        x -= v;
    }
    v
}

fn calculate_solution(
    from_x: Distance,
    to_x: Distance,
    from_y: Elevation,
    to_y: Elevation,
) -> Elevation {
    let end_x = from_x;
    let vx = estimate_start_vx(end_x);
    println!("end_x={} start_vx={}", end_x, vx);

    let mut max_elevation = 0;
    for vy in 1..100 {
        let trajectory = calculate_trajectory(vx, vy, to_x, from_y);
        let zenith = trajectory.iter().max_by_key(|(_, y)| *y).unwrap();
        let hit = trajectory
            .iter()
            .filter(|(x, y)| *x >= from_x && *x <= to_x && *y >= from_y && *y <= to_y)
            .nth(0);
        println!("zenith {:?}, hit {:?}", zenith, hit);
        if !hit.is_none() {
            max_elevation = std::cmp::max(max_elevation, zenith.1);
        }
    }
    max_elevation
}

fn main() {
    println!(
        "Solution {:?}",
        calculate_solution(data::X_FROM, data::X_TO, data::Y_FROM, data::Y_TO)
    );
}
