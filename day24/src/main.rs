mod alu;
mod data;

use alu::Number;

fn number_to_stack(n: Number) -> Vec<Number> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.push(n);
    digits
}

fn verify_model_number(number: Number, program: &alu::Program) -> bool {
    let mut computer = alu::Alu::default();
    let mut input = number_to_stack(number);
 
    computer.run_program(program, &mut input);
    let crc = computer.get_register(alu::Register::Z);
    println!("CRC({number})={crc}");
    crc == 0
}

fn calculate_solution(code: &str) -> (Number, Number) {
    let program = alu::parse_program(code);
    let (min, max) = alu::find_valid_model_number_extrema(&program);
    if !verify_model_number(min, &program) {
        panic!("invalid number {min}");
    }
    if !verify_model_number(max, &program) {
        panic!("invalid number {max}");
    }
    (min, max)
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::MONAD));
}
