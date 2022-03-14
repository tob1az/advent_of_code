mod data;

fn calculate_solution(positions: &[i32]) -> i32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (min..=max)
        .map(|center| positions.iter().map(|p| (p - center).abs()).sum::<i32>())
        .min()
        .unwrap()
}

fn main() {
    println!("Solution {}", calculate_solution(&data::CRAB_POSITIONS));
}
