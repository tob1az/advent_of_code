mod data;
mod amphipod;

fn calculate_solution() -> u32 {
    let mut burrow = amphipod::Burrow::parse(data::BURROW_MAP);
    burrow.organize_occupants().0
}

fn main() {
    println!("Solution: {:?}", calculate_solution());
}
