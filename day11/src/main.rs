#[macro_use(array)]
extern crate ndarray;

#[macro_use(lazy_static)]
extern crate lazy_static;

#[macro_use(iproduct)]
extern crate itertools;

mod data;

fn calculate_solution(octopuses: &ndarray::Array2<u8>) -> (usize, Option<usize>) {
    let mut flash_count = 0;
    let mut first_sync_flash_step: Option<usize> = None;

    let mut energy_levels = octopuses.clone();
    let shape = energy_levels.shape();
    let x_size = shape[0];
    let y_size = shape[1];
    let mut step = 0;
    while first_sync_flash_step.is_none() {
        energy_levels.map_inplace(|level| *level += 1);
        loop {
            let mut new_flashes = 0;
            for (x, y) in iproduct!(0..x_size, 0..y_size) {
                if energy_levels[(x, y)] > 9 {
                    new_flashes += 1;
                    energy_levels[(x, y)] = 0;

                    let start_x = if x > 0 { x - 1 } else { 0 };
                    let start_y = if y > 0 { y - 1 } else { 0 };
                    for neighbor_coord in iproduct!(start_x..=x + 1, start_y..=y + 1) {
                        if let Some(neighbor) = energy_levels.get_mut(neighbor_coord) {
                            if *neighbor != 0 {
                                *neighbor += 1;
                            }
                        }
                    }
                }
            }
            if new_flashes > 0 {
                if step < 100 {
                    flash_count += new_flashes;
                }
            } else {
                if energy_levels.iter().filter(|level| **level > 0).count() == 0 {
                    first_sync_flash_step = Some(step + 1);
                }
                break;
            }
        }
        step += 1;
    }
    (flash_count, first_sync_flash_step)
}

fn main() {
    println!("Solution {:?}", calculate_solution(&data::OCTOPUSES));
}
