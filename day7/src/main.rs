mod data;

fn progression(n: i32) -> i32 {
    n * (2 + n - 1) / 2
}

fn calculate_solution(positions: &[i32]) -> (i32, i32) {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (
        (min..=max)
            .map(|center| positions.iter().map(|p| (p - center).abs()).sum::<i32>())
            .min()
            .unwrap(),
        (min..=max)
            .map(|center| {
                positions
                    .iter()
                    .map(|p| progression((p - center).abs()))
                    .sum::<i32>()
            })
            .min()
            .unwrap(),
    )
}

fn main() {
    let (cost1, cost2) = calculate_solution(&data::CRAB_POSITIONS);
    println!("Solution {} {}", cost1, cost2);
}
