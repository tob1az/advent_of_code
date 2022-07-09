mod data;
mod amphipod;

fn calculate_solution() -> (u32, u32) {
    let mut small_burrow = amphipod::Burrow::parse(data::SMALL_BURROW_MAP);
    let mut big_burrow = amphipod::Burrow::parse(data::BIG_BURROW_MAP);
    (small_burrow.organize().0, big_burrow.organize().0)
}

fn main() {
    println!("Solution: {:?}", calculate_solution());
}
