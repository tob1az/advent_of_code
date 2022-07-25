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

fn calculate_solution(code: &str) -> Number {
    let program = alu::parse_program(code);
    let model_number = alu::find_largest_valid_model_number(&program);

    let mut computer = alu::Alu::default();
    let mut input = number_to_stack(model_number);
    println!("input={:?}", input);

    computer.run_program(&program, &mut input);
    let crc = computer.get_register(alu::Register::Z);
    println!("CRC({model_number})={crc}");
    if crc == 0 {
        return model_number;
    }
    panic!("could not find a valid number");
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::MONAD));
}
