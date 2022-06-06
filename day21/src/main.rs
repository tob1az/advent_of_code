mod data;
mod dirac_dice;

use dirac_dice::{DeterministicDie, Player};

fn calculate_solution(start_positions: &[usize; 2]) -> usize {
    let goal = 1000;
    let mut players = [
        Player::new(start_positions[0], goal),
        Player::new(start_positions[1], goal),
    ];
    let mut die = DeterministicDie::with_sides(100);
    let rolls = dirac_dice::play(&mut die, &mut players);
    let (n, loser) = players
        .iter()
        .enumerate()
        .filter(|(_, p)| !p.won())
        .next()
        .unwrap();
    println!(
        "Loser {n} has score {}, number of rolls {}",
        loser.score, rolls
    );
    loser.score * rolls
}

fn main() {
    println!(
        "Solution {:?}",
        calculate_solution(&data::PLAYER_STARTING_SPACES)
    );
}
