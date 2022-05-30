mod data;
mod dirac_dice;

use dirac_dice::{DeterministicDie, Player};

fn calculate_solution(start_positions: &[usize; 2]) -> usize {
    let mut players = [
        Player::with_position(start_positions[0]),
        Player::with_position(start_positions[1]),
    ];
    let mut die = DeterministicDie::default();
    let rolls = dirac_dice::play(&mut die, &mut players);
    let (n, loser) = players
        .iter()
        .enumerate()
        .filter(|(_, p)| !p.won())
        .nth(0)
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
