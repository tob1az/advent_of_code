mod algo;
mod data;

extern crate eyre;
extern crate itertools;

use eyre::{Result, WrapErr};

fn calculate_solution(image_data: &str, algorithm_data: &str) -> Result<usize> {
    let image = algo::Image::parse(image_data)?;
    println!("original\n{}", image);
    let algorithm = algo::Algorithm::parse(algorithm_data)?;
    let enhanced = algorithm.enhance_image(&image);
    println!("enhanced\n{}", enhanced);
    algorithm.enhance_image(&enhanced).count_light_pixels()
}

fn main() -> Result<()> {
    let solution = calculate_solution(data::INPUT_IMAGE, data::ALGORITHM)
        .wrap_err("failed to calculate the solution")?;
    println!("Solution {:?}", solution);

    Ok(())
}
