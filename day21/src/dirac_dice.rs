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

pub struct Player {
    pub space: usize,
    pub score: usize,
    max_score: usize,
}

impl Player {
    pub fn new(space: usize, max_score: usize) -> Self {
        Self {
            space,
            score: 0,
            max_score,
        }
    }

    pub fn won(&self) -> bool {
        self.score >= self.max_score
    }

    pub fn spin_by(&mut self, spaces: usize) {
        self.space = (self.space + spaces - 1) % 10 + 1;
    }

    pub fn end_move(&mut self) {
        self.score += self.space;
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
