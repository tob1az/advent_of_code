use itertools::Itertools;

pub trait Die {
    fn roll(&mut self) -> usize;
}

pub struct DeterministicDie {
    value: usize,
    sides: usize,
}

impl DeterministicDie {
    pub fn with_sides(sides: usize) -> Self {
        Self { value: 0, sides }
    }
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> usize {
        self.value += 1;
        if self.value > self.sides {
            self.value -= self.sides;
        }
        self.value
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Score(pub usize);

#[derive(Clone)]
pub struct Player {
    pub space: usize,
    pub score: Score,
    max_score: Score,
}

impl Player {
    pub fn new(space: usize, max_score: Score) -> Self {
        Self {
            space,
            score: Score(0),
            max_score,
        }
    }

    pub fn won(&self) -> bool {
        self.score.0 >= self.max_score.0
    }

    pub fn spin_by(&mut self, spaces: usize) {
        self.space = (self.space + spaces - 1) % 10 + 1;
    }

    pub fn end_move(&mut self) {
        self.score.0 += self.space;
    }
}

type DieRolls = usize;

pub fn play(die: &mut dyn Die, players: &mut [Player; 2]) -> DieRolls {
    let mut rolls = 0;
    loop {
        for player in players.iter_mut() {
            for _ in 0..3 {
                let spaces = die.roll();
                player.spin_by(spaces);
            }
            player.end_move();
            rolls += 3;
            if player.won() {
                return rolls;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Frequency(pub usize);

fn find_roll_frequencies(die_count: usize, sides: usize) -> Vec<(Score, Frequency)> {
    (0..die_count)
        .map(|_| (1..=sides))
        .multi_cartesian_product()
        .map(|d| d.iter().sum())
        .sorted()
        .map(|s| (s, 1))
        .coalesce(|(a, m), (b, n)| {
            if a == b {
                Ok((a, m + 1))
            } else {
                Err(((a, m), (b, n)))
            }
        })
        .map(|(s, f)| (Score(s), Frequency(f)))
        .collect_vec()
}

pub fn count_dirac_dice_wins(player0: Player, player1: Player) -> [usize; 2] {
    let mut player0 = player0;
    let mut player1 = player1;
    let mut player0_wins = 0;
    let mut player1_wins = 0;
    let universes = 1;
    count_dirac_dice_wins_in_one_turn(
        &mut player0,
        &mut player1,
        &mut player0_wins,
        &mut player1_wins,
        universes,
    );

    [player0_wins, player1_wins]
}

fn count_dirac_dice_wins_in_one_turn(
    current_player: &mut Player,
    opponent: &mut Player,
    current_player_wins: &mut usize,
    opponent_wins: &mut usize,
    universes: usize,
) {
    for (Score(roll), Frequency(die_combinations)) in find_roll_frequencies(3, 3).into_iter() {
        let mut player = current_player.clone();
        player.spin_by(roll);
        player.end_move();
        let new_universes = die_combinations * universes;
        if player.won() {
            *current_player_wins += new_universes;
        } else {
            // turn goes to the opponent
            count_dirac_dice_wins_in_one_turn(
                opponent,
                &mut player,
                opponent_wins,
                current_player_wins,
                new_universes,
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_find_roll_frequencies() {
        assert_eq!(
            find_roll_frequencies(3, 3),
            vec![
                (Score(3), Frequency(1)),
                (Score(4), Frequency(3)),
                (Score(5), Frequency(6)),
                (Score(6), Frequency(7)),
                (Score(7), Frequency(6)),
                (Score(8), Frequency(3)),
                (Score(9), Frequency(1)),
            ]
        );
    }

    #[test]
    fn player_is_cloneable() {
        let original = Player {
            space: 1,
            score: Score(2),
            max_score: Score(3),
        };
        let clone = original.clone();
        assert_eq!(original.space, clone.space);
        assert_eq!(original.score, clone.score);
        assert_eq!(original.max_score, clone.max_score);
    }
}
