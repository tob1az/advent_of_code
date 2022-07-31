mod data;

fn calculate_solution(depths: &[u32]) -> (usize, usize) {
    let mut depth_increases = 0;
    let mut previous = depths[0];
    for &d in &depths[1..] {
        if d > previous {
            depth_increases += 1
        }
        previous = d;
    }

    let mut window_depth_increases = 0;
    previous = depths[0..2].iter().sum();
    for d in depths[1..].windows(3).map(|w| w.iter().sum()) {
        if d > previous {
            window_depth_increases += 1
        }
        previous = d;
    }

    (depth_increases, window_depth_increases)
}

fn main() {
    println!("Solution {:?}", calculate_solution(&data::SONAR_DATA));
}
