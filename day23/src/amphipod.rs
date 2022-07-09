use itertools::Itertools;
use regex::RegexBuilder;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Species {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position(u32, u32);

#[derive(Clone, Copy, Default, Debug)]
pub struct Energy(pub u32);

#[derive(Debug)]
struct Room {
    species: Species,
    space: Vec<Position>,
}

enum State<'a> {
    Settled,
    Entering(&'a Room),
    Leaving(&'a Room),
}

struct Hallway {
    space: Vec<Position>,
}

type Arrangement = HashMap<Position, Species>;

pub struct Burrow {
    rooms: [Room; 4],
    hallway: Hallway,
    arrangement: Arrangement,
}

type SerializedArrangement = String;

#[derive(Clone)]
struct OrganizingResult {
    cost: Energy,
    arrangement: Arrangement,
    previous_achievements: HashMap<SerializedArrangement, Energy>,
}

type Move = (Position, Energy);

impl Species {
    fn move_cost(&self) -> Energy {
        match self {
            Species::Amber => Energy(1),
            Species::Bronze => Energy(10),
            Species::Copper => Energy(100),
            Species::Desert => Energy(1000),
        }
    }

    fn parse(symbol: &str) -> Self {
        match symbol {
            "A" => Species::Amber,
            "B" => Species::Bronze,
            "C" => Species::Copper,
            "D" => Species::Desert,
            _ => panic!("Wrong symbol {symbol}"),
        }
    }

    fn serialize(&self) -> &str {
        match self {
            Species::Amber => "A",
            Species::Bronze => "B",
            Species::Copper => "C",
            Species::Desert => "D",
        }
    }
}

impl Room {
    fn contains(&self, position: &Position) -> bool {
        self.space.iter().find(|s| **s == *position).is_some()
    }

    fn x(&self) -> u32 {
        self.space[0].0
    }
}

impl Burrow {
    pub fn organize(&mut self) -> Energy {
        let mut best_result = OrganizingResult::new();

        self.try_all_moves(&mut best_result, self.arrangement.clone(), Energy(0), 0);

        if best_result.cost.0 != u32::MAX {
            self.arrangement = best_result.arrangement;
        }

        best_result.cost
    }

    fn try_all_moves(
        &self,
        best_result: &mut OrganizingResult,
        arrangement: Arrangement,
        total_cost: Energy,
        moves: u32,
    ) {
        println!("try: total cost {}, moves {}", total_cost.0, moves);
        if self.are_amphipods_organized(&arrangement) {
            return;
        }
        if total_cost.0 > best_result.cost.0 {
            return;
        }
        let achievement = self.serialize_achievement(&arrangement);
        println!("achievement: {achievement} cost {}", total_cost.0);
        if let Some(cost) = best_result.previous_achievements.get_mut(&achievement) {
            if total_cost.0 >= cost.0 {
                println!("too costly option");
                return;
            }
            *cost = total_cost.clone();
        } else {
            best_result
                .previous_achievements
                .insert(achievement, total_cost);
        }
        for (old_position, species) in arrangement.iter().sorted_by_key(|(_, s)| s.move_cost().0) {
            for (position, move_cost) in self
                .evaluate_moves_from(&old_position, &arrangement)
                .iter()
                .sorted_by_key(|(_, e)| e.0)
            {
                let mut modified = arrangement.clone();
                println!(
                    "move {}, {} => {}, {}",
                    old_position.0, old_position.1, position.0, position.1
                );
                modified.remove(&old_position);
                modified.insert(position.clone(), *species);

                let new_cost = Energy(total_cost.0 + move_cost.0);

                if self.are_amphipods_organized(&modified) {
                    if new_cost.0 < best_result.cost.0 {
                        println!("organized with cost {}", new_cost.0);
                        best_result.cost = new_cost;
                        best_result.arrangement = modified;
                    }
                    break;
                }
                // recurse
                self.try_all_moves(best_result, modified, new_cost, moves + 1);
            }
        }
    }

    fn are_amphipods_organized(&self, arrangement: &Arrangement) -> bool {
        for room in self.rooms.iter() {
            for position in room.space.iter() {
                if let Some(species) = arrangement.get(&position) {
                    if *species != room.species {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn evaluate_moves_from(&self, position: &Position, arrangement: &Arrangement) -> Vec<Move> {
        if let Some(amphipod) = arrangement.get(position) {
            let moves = match self.classify(position, *amphipod, &arrangement) {
                State::Settled => Vec::new(),
                State::Entering(room) => self
                    .go_to_destination_from(position, room, *amphipod, &arrangement)
                    .iter()
                    .cloned()
                    .collect_vec(),
                State::Leaving(room) => {
                    self.go_to_hallway_from(position, room, *amphipod, arrangement)
                }
            };
            if moves.len() > 0 {
                println!(
                    "found {} moves from {}, {}",
                    moves.len(),
                    position.0,
                    position.1
                );
            }
            moves
        } else {
            Vec::new()
        }
    }

    fn go_to_destination_from(
        &self,
        position: &Position,
        destination: &Room,
        amphipod: Species,
        arrangement: &Arrangement,
    ) -> Option<Move> {
        let destination_x = destination.x();
        let exit_blocked = if destination_x > position.0 {
            self.hallway
                .space
                .iter()
                .filter(|s| {
                    s.0 > position.0 && s.0 < destination_x && arrangement.get(&s).is_some()
                })
                .count()
                > 0
        } else {
            self.hallway
                .space
                .iter()
                .filter(|s| {
                    s.0 < position.0 && s.0 > destination_x && arrangement.get(&s).is_some()
                })
                .count()
                > 0
        };
        if exit_blocked || arrangement.get(&destination.space[0]).is_some() {
            None
        } else {
            let foreigner = destination.space.iter().find(|p| {
                if let Some(a) = arrangement.get(p) {
                    if *a != destination.species {
                        return true;
                    }
                };
                return false;
            });
            if foreigner.is_some() {
                return None;
            }
            let moves_to_enter_room = destination_x.abs_diff(position.0);
            let deepest = destination
                .space
                .iter()
                .take_while(|p| arrangement.get(p).is_none())
                .last();
            if let Some(new_position) = deepest {
                Some((
                    new_position.clone(),
                    Energy((moves_to_enter_room + new_position.1) * amphipod.move_cost().0),
                ))
            } else {
                None
            }
        }
    }

    fn go_to_hallway_from(
        &self,
        position: &Position,
        room: &Room,
        amphipod: Species,
        arrangement: &Arrangement,
    ) -> Vec<Move> {
        if position.1 != 1 {
            if room
                .space
                .iter()
                .find(|p| p.1 < position.1 && arrangement.get(&p).is_some())
                .is_some()
            {
                return Vec::new();
            }
        }
        self.hallway
            .space
            .iter()
            .rev()
            .filter(|s| s.0 < position.0)
            .take_while(|s| arrangement.get(&s).is_none())
            .interleave(
                self.hallway
                    .space
                    .iter()
                    .filter(|s| s.0 > position.0)
                    .take_while(|s| arrangement.get(&s).is_none()),
            )
            .cloned()
            .map(|p| {
                let cost = Energy((p.0.abs_diff(position.0) + position.1) * amphipod.move_cost().0);
                (p, cost)
            })
            .collect()
    }

    fn classify(&self, position: &Position, amphipod: Species, arrangement: &Arrangement) -> State {
        if let Some(room) = self.rooms.iter().filter(|r| r.contains(position)).next() {
            if room.species != amphipod {
                println!("Leave {}, {}", position.0, position.1);
                return State::Leaving(room);
            }
            if room
                .space
                .iter()
                .find(|p| p.1 > position.1 && *arrangement.get(&p).unwrap_or(&amphipod) != amphipod)
                .is_none()
            {
                println!("{}, {} is settled", position.0, position.1);
                return State::Settled;
            }
            println!("Leave (2) {}, {}", position.0, position.1);
            return State::Leaving(room);
        }
        let destination = self.rooms.iter().find(|r| r.species == amphipod).unwrap();
        println!("Roaming from {}, {}", position.0, position.1);
        State::Entering(destination)
    }

    fn serialize_achievement(&self, achievement: &Arrangement) -> SerializedArrangement {
        self.hallway
            .space
            .iter()
            .cloned()
            .chain(self.rooms.iter().flat_map(|r| r.space.clone()))
            .flat_map(|p| {
                if let Some(a) = achievement.get(&p) {
                    a.serialize().chars()
                } else {
                    "-".chars()
                }
            })
            .collect()
    }

    pub fn parse(map: &str) -> Self {
        let room_xs = [3, 5, 7, 9];
        let room_coords = (1..)
            .cartesian_product(room_xs.iter().cloned())
            .map(|(y, x)| Position(x, y));
        let amphipods = RegexBuilder::new("[A-D]")
            .multi_line(true)
            .build()
            .unwrap()
            .captures_iter(map)
            .zip(room_coords)
            .map(|(c, p)| (p.clone(), Species::parse(&c[0])))
            .collect::<HashMap<_, _>>();

        let hallway = Hallway {
            space: (1..12)
                .filter_map(|x| {
                    if room_xs.contains(&x) {
                        None
                    } else {
                        Some(Position(x, 0))
                    }
                })
                .collect::<Vec<_>>(),
        };

        let room_depth = amphipods.len() / room_xs.len();
        let rooms = room_xs
            .into_iter()
            .zip(
                [
                    Species::Amber,
                    Species::Bronze,
                    Species::Copper,
                    Species::Desert,
                ]
                .repeat(room_depth),
            )
            .map(|(x, s)| Room {
                species: s,
                space: (1..=room_depth).map(|y| Position(x, y as u32)).collect(),
            })
            .collect_vec()
            .try_into()
            .unwrap();

        Burrow {
            rooms,
            arrangement: amphipods,
            hallway,
        }
    }
}

impl OrganizingResult {
    fn new() -> Self {
        OrganizingResult {
            cost: Energy(u32::MAX),
            arrangement: Arrangement::new(),
            previous_achievements: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_tell_if_amphipods_are_organized() {
        let burrow = Burrow::parse(
            r#"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #########"#,
        );

        assert_eq!(burrow.are_amphipods_organized(&burrow.arrangement), true);
    }
}
