mod algo;
mod data;

extern crate eyre;
extern crate itertools;

use eyre::{Result, WrapErr};

fn light_pixels_after_enhancing(
    image: algo::Image,
    algorithm: &algo::Algorithm,
    times: usize,
) -> Result<usize> {
    debug_assert!(times > 0);
    let mut enhanced = image;
    for _ in 0..times {
        enhanced = algorithm.enhance_image(&enhanced);
    }
    enhanced.count_light_pixels()
}

fn calculate_solution(image_data: &str, algorithm_data: &str) -> Result<(usize, usize)> {
    let image = algo::Image::parse(image_data)?;
    let algorithm = algo::Algorithm::parse(algorithm_data)?;
    Ok((
        light_pixels_after_enhancing(image.clone(), &algorithm, 2)?,
        light_pixels_after_enhancing(image, &algorithm, 50)?,
    ))
}

fn main() -> Result<()> {
    let solution = calculate_solution(data::INPUT_IMAGE, data::ALGORITHM)
        .wrap_err("failed to calculate the solution")?;
    println!("Solution {:?}", solution);

    Ok(())
}
