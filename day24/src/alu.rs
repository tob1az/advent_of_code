use itertools::Itertools;
use std::str::FromStr;

pub type Number = i64;

#[derive(Clone, Copy, Debug)]
pub enum Register {
    X = 0,
    Y,
    Z,
    W,
}

#[derive(Clone)]
pub enum Operand {
    Variable(Register),
    Literal(Number),
}

#[derive(Clone)]
pub enum Instruction {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
}

#[derive(Default)]
pub struct Alu {
    registers: [Number; 4],
}

impl Register {
    fn parse(code: &str) -> Self {
        match code {
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            "w" => Register::W,
            bad_register => panic!("Bad register {bad_register}"),
        }
    }
}

impl Operand {
    fn parse(code: &str) -> Self {
        if let Ok(number) = Number::from_str(code) {
            Self::Literal(number)
        } else {
            Self::Variable(Register::parse(code))
        }
    }
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let terminals = line.split(' ').collect::<Vec<&str>>();
        let destination = Register::parse(terminals[1]);
        match terminals[0] {
            "inp" => Instruction::Inp(destination),
            "add" => Instruction::Add(destination, Operand::parse(terminals[2])),
            "mul" => Instruction::Mul(destination, Operand::parse(terminals[2])),
            "div" => Instruction::Div(destination, Operand::parse(terminals[2])),
            "mod" => Instruction::Mod(destination, Operand::parse(terminals[2])),
            "eql" => Instruction::Eql(destination, Operand::parse(terminals[2])),
            _ => panic!("Bad instruction: {line}"),
        }
    }
}

pub type Program = Vec<Instruction>;

pub fn parse_program(program: &str) -> Vec<Instruction> {
    program.lines().map(Instruction::parse).collect()
}

impl Alu {
    pub fn run_program(&mut self, program: &Program, parameter_stack: &mut Vec<Number>) {
        for instruction in program.iter() {
            match instruction {
                Instruction::Inp(register) => {
                    self.set_register(*register, parameter_stack.pop().unwrap());
                    println!("{:?} <== {}", *register, self.get_register(*register));
                }
                Instruction::Add(register, operand) => {
                    self.run_operation(*register, operand, |a, b| {
                        println!("{a} + {b} = {}", a + b);
                        a + b
                    })
                }
                Instruction::Mul(register, operand) => {
                    self.run_operation(*register, operand, |a, b| {
                        println!("{a} * {b} = {}", a * b);
                        a * b
                    })
                }
                Instruction::Div(register, operand) => {
                    self.run_operation(*register, operand, |a, b| {
                        println!("{a} / {b} = {}", a / b);
                        a / b
                    })
                }
                Instruction::Mod(register, operand) => {
                    self.run_operation(*register, operand, |a, b| {
                        println!("{a} % {b} = {}", a % b);
                        a % b
                    })
                }
                Instruction::Eql(register, operand) => {
                    self.run_operation(*register, operand, |a, b| {
                        let r = if a == b { 1 } else { 0 };
                        println!("{a} == {b} = {r}");
                        r
                    })
                }
            }
        }
    }

    fn set_register(&mut self, register: Register, value: Number) {
        self.registers[register as usize] = value;
    }

    pub fn get_register(&self, register: Register) -> Number {
        self.registers[register as usize]
    }

    fn run_operation<F>(&mut self, register: Register, operand: &Operand, operation: F)
    where
        F: Fn(Number, Number) -> Number,
    {
        let result = operation(self.get_register(register), self.operand_value(operand));
        self.set_register(register, result);
    }

    fn operand_value(&self, operand: &Operand) -> Number {
        match operand {
            Operand::Literal(value) => *value,
            Operand::Variable(register) => self.get_register(*register),
        }
    }
}

type Index = usize;
enum MacroOp {
    Push(Index, Number),
    Pop(Index, Number),
}

fn stack_to_number(digits_stack: &[Number]) -> Number {
    let mut number = 0;
    for &digit in digits_stack {
        number *= 10;
        number += digit;
    }
    number
}

const MODEL_NUMBER_LENGTH: usize = 14;
const SUBPROGRAM_LENGTH: usize = 18;

fn deduce_max_valid_number(macro_ops: &[MacroOp]) -> Number {
    let mut digits = vec![0; MODEL_NUMBER_LENGTH];
    let mut stack = Vec::with_capacity(MODEL_NUMBER_LENGTH);
    for op in macro_ops.iter() {
        match *op {
            MacroOp::Push(i, n) => stack.push((i, n)),
            MacroOp::Pop(i, n) => {
                let (prev_i, prev_n) = stack.pop().expect("unexpected stack pop");
                if prev_n + n >= 0 {
                    digits[i] = 9;
                    digits[prev_i] = digits[i] - prev_n - n;
                } else {
                    digits[prev_i] = 9;
                    digits[i] = digits[prev_i] + prev_n + n;
                }
            }
        }
    }
    stack_to_number(&digits)
}

fn deduce_min_valid_number(macro_ops: &[MacroOp]) -> Number {
    let mut digits = vec![0; MODEL_NUMBER_LENGTH];
    let mut stack = Vec::with_capacity(MODEL_NUMBER_LENGTH);
    for op in macro_ops.iter() {
        match *op {
            MacroOp::Push(i, n) => stack.push((i, n)),
            MacroOp::Pop(i, n) => {
                let (prev_i, prev_n) = stack.pop().expect("unexpected stack pop");
                if prev_n + n >= 0 {
                    digits[prev_i] = 1;
                    digits[i] = digits[prev_i] + prev_n + n;
                } else {
                    digits[i] = 1;
                    digits[prev_i] = digits[i] - prev_n - n;
                }
            }
        }
    }
    stack_to_number(&digits)
}

pub fn find_valid_model_number_extrema(monad_program: &[Instruction]) -> (Number, Number) {
    // split into 14 subprograms 18 instructions each
    // find key parameters: kind (push or pop), A, B
    // find max valid digits using a stack
    debug_assert_eq!(monad_program.len(), MODEL_NUMBER_LENGTH * SUBPROGRAM_LENGTH);
    let subprograms = monad_program.chunks(SUBPROGRAM_LENGTH).collect_vec();
    let macro_ops = subprograms
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            let quotient = match s[4] {
                Instruction::Div(Register::Z, Operand::Literal(n)) => n,
                _ => panic!("wrong div z instruction"),
            };
            match quotient {
                1 => match s[15] {
                    Instruction::Add(Register::Y, Operand::Literal(n)) => MacroOp::Push(i, n),
                    _ => panic!("wrong add y instruction"),
                },
                26 => match s[5] {
                    Instruction::Add(Register::X, Operand::Literal(n)) => MacroOp::Pop(i, n),
                    _ => panic!("wrong add x instruction"),
                },
                _ => panic!("unexpected macro op"),
            }
        })
        .collect_vec();

    let max_model_number = deduce_max_valid_number(&macro_ops);
    println!("max model number = {max_model_number}");
    let min_model_number = deduce_min_valid_number(&macro_ops);
    println!("min model number = {min_model_number}");
    (min_model_number, max_model_number)
}
