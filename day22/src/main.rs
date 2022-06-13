mod cuboid;
mod data;

use cuboid::{Cuboid, Switch};
use regex::RegexBuilder;

fn parse_reboot_steps(input: &str) -> Vec<Cuboid> {
    let regex = RegexBuilder::new(
        "(on|off) x=(-*\\d+)..(-*\\d+),y=(-*\\d+)..(-*\\d+),z=(-*\\d+)..(-*\\d+)",
    )
    .multi_line(true)
    .build()
    .unwrap();
    regex
        .captures_iter(input)
        .map(|capture| {
            let switch = match &capture[1] {
                "on" => Switch::On,
                "off" => Switch::Off,
                unknown => panic!("unknown switch {}", unknown),
            };
            Cuboid {
                x_min: capture[2].parse().unwrap(),
                x_max: capture[3].parse().unwrap(),
                y_min: capture[4].parse().unwrap(),
                y_max: capture[5].parse().unwrap(),
                z_min: capture[6].parse().unwrap(),
                z_max: capture[7].parse().unwrap(),
                switch,
            }
        })
        .collect()
}

fn reboot_reactor(steps: &[cuboid::Cuboid]) -> i64 {
    let mut cubes = Vec::new();
    for step in steps {
        let mut intersections = cubes
            .iter()
            .filter_map(|c| {
                if step.intersects_with(c) {
                    Some(step.intersection(c))
                } else {
                    None
                }
            })
            .collect::<Vec<Cuboid>>();
        cubes.append(&mut intersections);
        if step.switch == Switch::On {
            cubes.push(step.clone());
        }
    }
    cubes.into_iter().fold(0, |acc, c| acc + c.volume())
}

fn calculate_solution(input: &str) -> (i64, i64) {
    let steps = parse_reboot_steps(input);
    debug_assert!(!steps.is_empty());

    let small_region = Cuboid {
        x_min: -50,
        x_max: 50,
        y_min: -50,
        y_max: 50,
        z_min: -50,
        z_max: 50,
        switch: Switch::On,
    };
    let initialized_cubes = reboot_reactor(
        &steps
            .iter()
            .filter(|s| s.intersects_with(&small_region))
            .cloned()
            .collect::<Vec<Cuboid>>(),
    );
    (initialized_cubes, reboot_reactor(&steps))
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::REBOOT_STEPS));
}
