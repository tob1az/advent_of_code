
mod data;
mod algo;

extern crate eyre;

use eyre::{Result, WrapErr};

fn calculate_solution() -> Result<usize> {
    Ok(0)
}

fn main() -> Result<()> {
    let solution = calculate_solution().wrap_err("failed to calculate the solution")?;
    println!("Solution {:?}", solution);

    Ok(())
}
