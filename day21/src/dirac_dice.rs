pub trait Die {
    fn roll(&mut self) -> usize;
}

#[derive(Default)]
pub struct DeterministicDie {
    value: usize,
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> usize {
        self.value += 1;
        if self.value > 100 {
            self.value -= 100;
        }
        self.value
    }
}

pub struct Player {
    pub space: usize,
    pub score: usize,
}

impl Player {
    pub fn with_position(space: usize) -> Self {
        Self { space, score: 0 }
    }

    pub fn won(&self) -> bool {
        self.score >= 1000
    }

    pub fn roll_die(&mut self, die: &mut dyn Die, times: usize) {
        print!("rolls");
        for _ in 0..times {
            let spaces = die.roll();
            print!(" {spaces}");
            self.space = (self.space + spaces - 1) % 10 + 1;
        }
        self.score += self.space;
        println!(
            " and moves to space {} for a total score of {}.",
            self.space, self.score
        );
    }
}

type DieRolls = usize;

pub fn play(die: &mut dyn Die, players: &mut [Player; 2]) -> DieRolls {
    let mut rolls = 0;
    loop {
        for (i, player) in players.iter_mut().enumerate() {
            print!("Player {i} ");
            player.roll_die(die, 3);
            rolls += 3;
            if player.won() {
                return rolls;
            }
        }
    }
}
