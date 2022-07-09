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

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord)]
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

struct BurrowOccupant {
    species: Species,
    position: Position,
}

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
        self.space.iter().any(|s| *s == *position)
    }

    fn x(&self) -> u32 {
        self.space[0].0
    }
}

impl Burrow {
    pub fn organize(&mut self) -> Energy {
        let mut best_result = OrganizingResult::new();

        let moves_made = 0;
        self.try_all_moves(
            &mut best_result,
            self.arrangement.clone(),
            Energy(0),
            moves_made,
        );

        if best_result.successful() {
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
        // stop if already organized
        if self.are_amphipods_organized(&arrangement) {
            return;
        }
        // can it improve the result?
        if total_cost > best_result.cost {
            return;
        }
        // pruning
        let key = self.serialize_arrangement(&arrangement);
        if let Some(cost) = best_result.previous_achievements.get_mut(&key) {
            // too costly
            if total_cost >= *cost {
                return;
            }
            *cost = total_cost;
        } else {
            // remember the achievement to prune more expensive results
            best_result.previous_achievements.insert(key, total_cost);
        }
        for (old_position, species) in arrangement.iter().sorted_by_key(|(_, s)| s.move_cost()) {
            for (position, move_cost) in self
                .evaluate_moves_from(&old_position, &arrangement)
                .iter()
                .sorted_by_key(|(_, e)| e)
            {
                // make move
                let mut modified = arrangement.clone();
                modified.remove(old_position);
                modified.insert(position.clone(), *species);

                let new_cost = Energy(total_cost.0 + move_cost.0);

                if self.are_amphipods_organized(&modified) {
                    if new_cost < best_result.cost {
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
            let occupant = BurrowOccupant {
                species: *amphipod,
                position: position.clone(),
            };
            match self.classify(&occupant, arrangement) {
                State::Settled => Vec::new(),
                State::Entering(room) => self
                    .go_to_destination(&occupant, room, arrangement)
                    .iter()
                    .cloned()
                    .collect_vec(),
                State::Leaving(room) => self.go_to_hallway(&occupant, room, arrangement),
            }
        } else {
            Vec::new()
        }
    }

    fn go_to_destination(
        &self,
        occupant: &BurrowOccupant,
        destination: &Room,
        arrangement: &Arrangement,
    ) -> Option<Move> {
        let destination_x = destination.x();
        let destination_entry_blocked = if destination_x > occupant.position.0 {
            self.hallway
                .space
                .iter()
                .filter(|s| {
                    s.0 > occupant.position.0 && s.0 < destination_x && arrangement.get(s).is_some()
                })
                .count()
                > 0
        } else {
            self.hallway
                .space
                .iter()
                .filter(|s| {
                    s.0 < occupant.position.0 && s.0 > destination_x && arrangement.get(s).is_some()
                })
                .count()
                > 0
        };
        // don't move if destination is blocked or full
        if destination_entry_blocked || arrangement.get(&destination.space[0]).is_some() {
            None
        } else {
            // find foreign species in the destination room
            let foreigner = destination.space.iter().find(|p| {
                if let Some(a) = arrangement.get(p) {
                    if *a != destination.species {
                        return true;
                    }
                };
                false
            });
            // don't move to the destination room is it's occupied by foreigners
            if foreigner.is_some() {
                return None;
            }
            // go to the deepest position in the room
            let moves_to_enter_room = destination_x.abs_diff(occupant.position.0);
            let deepest = destination
                .space
                .iter()
                .take_while(|p| arrangement.get(p).is_none())
                .last();
            deepest.map(|new_position| {
                (
                    new_position.clone(),
                    Energy((moves_to_enter_room + new_position.1) * occupant.species.move_cost().0),
                )
            })
        }
    }

    fn go_to_hallway(
        &self,
        occupant: &BurrowOccupant,
        room: &Room,
        arrangement: &Arrangement,
    ) -> Vec<Move> {
        // if not at the exit
        if occupant.position.1 != 1
            && room
                .space
                .iter()
                .any(|p| p.1 < occupant.position.1 && arrangement.get(p).is_some())
        {
            return Vec::new();
        }

        // find all possible positions in the hallway not blocked by other amphipods
        self.hallway
            .space
            .iter()
            .rev()
            .filter(|s| s.0 < occupant.position.0)
            .take_while(|s| arrangement.get(s).is_none())
            .interleave(
                self.hallway
                    .space
                    .iter()
                    .filter(|s| s.0 > occupant.position.0)
                    .take_while(|s| arrangement.get(s).is_none()),
            )
            .cloned()
            .map(|p| {
                let cost = Energy(
                    (p.0.abs_diff(occupant.position.0) + occupant.position.1)
                        * occupant.species.move_cost().0,
                );
                (p, cost)
            })
            .collect()
    }

    fn classify(&self, occupant: &BurrowOccupant, arrangement: &Arrangement) -> State {
        if let Some(room) = self.rooms.iter().find(|r| r.contains(&occupant.position)) {
            if room.species != occupant.species {
                return State::Leaving(room);
            }
            let no_foreigners_in_rear = !room.space.iter().any(|p| {
                p.1 > occupant.position.1
                    && *arrangement.get(p).unwrap_or(&occupant.species) != occupant.species
            });
            return if no_foreigners_in_rear {
                State::Settled
            } else {
                State::Leaving(room)
            };
        }
        let destination = self
            .rooms
            .iter()
            .find(|r| r.species == occupant.species)
            .unwrap();
        State::Entering(destination)
    }

    fn serialize_arrangement(&self, achievement: &Arrangement) -> SerializedArrangement {
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
            .map(|(c, p)| (p, Species::parse(&c[0])))
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

    fn successful(&self) -> bool {
        self.cost.0 != u32::MAX
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
