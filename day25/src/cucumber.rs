pub type SeaBottom = Vec<Vec<Option<Cucumber>>>;

pub enum Cucumber {
    EastBound,
    SouthBound,
    GoEast,
    GoSouth,
}

pub fn parse_sea_bottom(input: &str) -> SeaBottom {
    debug_assert!(input.lines().count() > 0);
    debug_assert!(!input.lines().next().unwrap().is_empty());
    debug_assert!(input
        .lines()
        .all(|l| l.len() == input.lines().next().unwrap().len()));
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '>' => Some(Cucumber::EastBound),
                    'v' => Some(Cucumber::SouthBound),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn make_step(sea_bottom: &mut SeaBottom) -> bool {
    let width = sea_bottom[0].len();
    let height = sea_bottom.len();

    let mut success = false;
    // move eastbound sea cucumbers
    for row in sea_bottom.iter_mut() {
        for x in 0..width {
            if let Some(Cucumber::EastBound) = row[x] {
                let next_x = (x + 1) % width;
                if row[next_x].is_none() {
                    row[x] = Some(Cucumber::GoEast);
                    success = true;
                }
            }
        }

        for x in 0..width {
            if let Some(Cucumber::GoEast) = row[x] {
                let next_x = (x + 1) % width;
                row[next_x] = Some(Cucumber::EastBound);
                row[x] = None;
            }
        }
    }
    // move southbound sea cucumbers
    for x in 0..width {
        for y in 0..height {
            if let Some(Cucumber::SouthBound) = sea_bottom[y][x] {
                let next_y = (y + 1) % height;
                if sea_bottom[next_y][x].is_none() {
                    sea_bottom[y][x] = Some(Cucumber::GoSouth);
                    success = true;
                }
            }
        }
        for y in 0..height {
            if let Some(Cucumber::GoSouth) = sea_bottom[y][x] {
                let next_y = (y + 1) % height;
                sea_bottom[next_y][x] = Some(Cucumber::SouthBound);
                sea_bottom[y][x] = None;
            }
        }
    }
    success
}

pub fn print_sea_bottom(sea_bottom: &SeaBottom) {
    for row in sea_bottom.iter() {
        for cucumber in row.iter() {
            let c = match cucumber {
                Some(Cucumber::EastBound) => ">",
                Some(Cucumber::SouthBound) => "v",
                _ => ".",
            };
            print!("{}", c)
        }
        println!();
    }
    println!();
}
