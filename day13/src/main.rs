mod data;

extern crate itertools;

use data::Fold;
use itertools::Itertools;

fn fold(direction: &Fold, dots: &[(u32, u32)]) -> Vec<(u32, u32)> {
    match direction {
        Fold::Left(y_fold) => dots
            .iter()
            .cloned()
            .map(|(x, y)| (x, if y > *y_fold { 2 * y_fold - y } else { y }))
            .collect(),
        Fold::Up(x_fold) => dots
            .iter()
            .cloned()
            .map(|(x, y)| (if x > *x_fold { 2 * x_fold - x } else { x }, y))
            .collect(),
    }
}

fn print_dots(dots: &[(u32, u32)]) -> String {
    let width = dots.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let height = dots.iter().map(|(_, y)| *y).max().unwrap() + 1;
    let mut code = vec![false; (width * height) as usize];

    for dot in dots.iter() {
        let index = dot.0 + dot.1 * width;
        code[index as usize] = true;
    }

    code.iter()
        .enumerate()
        .flat_map(|(i, flag)| {
            let c = if *flag { '*' } else { ' ' };
            if i % (width as usize) == 0 {
                [c, '\n']
            } else {
                [c, 'x']
            }
        })
        .filter(|c| *c != 'x' )
        .collect()
}

fn calculate_solution(dots: &[(u32, u32)], instruction: &[Fold]) -> (usize, String) {
    let first_fold_dot_count = fold(instruction.get(0).unwrap(), dots)
        .iter()
        .unique()
        .count();
    let mut code_dots = dots.to_vec();
    for direction in instruction.iter() {
        let new_fold = fold(direction, &code_dots);
        code_dots = new_fold;
    }
    (first_fold_dot_count, print_dots(&code_dots))
}

fn main() {
    let (count, code) = calculate_solution(&data::DOTS, &data::FOLD_INSTRUCTION);
    println!("Solution {}\n{}", count, code);
}
