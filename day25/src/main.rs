mod cucumber;
mod data;

fn calculate_solution(sea_bottom: &str) -> usize {
    let mut sea_bottom = cucumber::parse_sea_bottom(sea_bottom);
    let mut steps_till_stop = 0;

    println!("Initial state:");
    cucumber::print_sea_bottom(&sea_bottom);
    while cucumber::make_step(&mut sea_bottom) {
        steps_till_stop += 1;
        //println!("After step {steps_till_stop}");
        //cucumber::print_sea_bottom(&sea_bottom);
    }
    println!("Final state:");
    cucumber::print_sea_bottom(&sea_bottom);

    steps_till_stop + 1
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::SEA_BOTTOM));
}
