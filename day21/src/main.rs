mod data;
extern crate itertools;

mod dirac_dice;

use dirac_dice::{DeterministicDie, Player, Score};

fn calculate_solution(start_positions: &[usize; 2]) -> (usize, usize) {
    let goal = 1000;
    let mut players = [
        Player::new(start_positions[0], Score(goal)),
        Player::new(start_positions[1], Score(goal)),
    ];
    let mut die = DeterministicDie::with_sides(100);
    let rolls = dirac_dice::play(&mut die, &mut players);
    let (n, loser) = players
        .iter()
        .enumerate()
        .find(|(_, p)| !p.won())
        .unwrap();
    println!(
        "Loser {n} has score {}, number of rolls {}",
        loser.score.0, rolls
    );

    let goal = 21;
    let max_winning_combinations = dirac_dice::count_dirac_dice_wins(
        Player::new(start_positions[0], Score(goal)),
        Player::new(start_positions[1], Score(goal)),
    )
    .into_iter()
    .max()
    .unwrap();

    (loser.score.0 * rolls, max_winning_combinations)
}

fn main() {
    println!(
        "Solution {:?}",
        calculate_solution(&data::PLAYER_STARTING_SPACES)
    );
}
