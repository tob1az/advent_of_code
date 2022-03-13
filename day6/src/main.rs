mod data;

// count breeds of 1 fish for each day
fn count_breeds(days: usize) -> Vec<usize> {
    let mut respawn_timers = Vec::from([1]);
    // reserve memory for an approximate number of timers
    respawn_timers.reserve(2usize.pow(days as u32/8));

    let mut counts: Vec<usize> = Vec::with_capacity(days);
    for d in 0..days {
        let mut new_fish_count: usize = 0;
        for timer in respawn_timers.iter_mut() {
            if *timer == 0 {
                *timer = data::RESPAWN_PERIOD;
                new_fish_count += 1;
            } else {
                *timer -= 1;
            }
        }
        if new_fish_count > 0 {
            respawn_timers.resize(
                respawn_timers.len() + new_fish_count,
                data::FIRST_RESPAWN_PERIOD,
            );
        }
        println!("Day {} -> {}", d, respawn_timers.len());
        counts.push(respawn_timers.len());
    }

    counts
}

fn calculate_solution(fish: &[u8], days: usize) -> usize {
    let breed_counts = count_breeds(days);
    fish.iter().map(|f| breed_counts[days - *f as usize]).sum()
}

fn main() {
    println!(
        "Solution {} {}",
        calculate_solution(&data::LANTERNFISH, 80),
        calculate_solution(&data::LANTERNFISH, 256)
    );
}
