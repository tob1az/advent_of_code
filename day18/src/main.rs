mod data;
mod snailfish;

fn calculate_solution(numbers: &str) -> usize {
    let sum = numbers
        .lines()
        .map(|line| snailfish::Number::parse(line).unwrap())
        .reduce(|a, b| snailfish::add(a, b))
        .unwrap();
    sum.magnitude()
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}
