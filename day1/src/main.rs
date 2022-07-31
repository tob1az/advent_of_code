mod data;

fn calculate_solution(depths: &[u32]) -> usize {
    let mut depth_increases = 0;
    let mut previous = depths[0];
    for &d in &depths[1..] {
        if d > previous {
            depth_increases += 1
        }
        previous = d;
    }
    depth_increases
}

fn main() {
    println!("Solution {:?}", calculate_solution(&data::SONAR_DATA));
}
