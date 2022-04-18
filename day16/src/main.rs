mod data;

use std::convert::TryInto;

struct BitReader {
    data: Vec<u8>,
    cursor: usize,
    bit_position: u32,
}

type Number = u64;

impl BitReader {
    fn new(hex_string: &str) -> BitReader {
        BitReader {
            data: (0..hex_string.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex_string[i..i + 2], 16).unwrap())
                .collect(),
            cursor: 0,
            bit_position: 0,
        }
    }

    const fn bit_mask(bits: u32, offset: u32) -> u8 {
        (((1u32 << bits) - 1) << offset) as u8
    }

    fn read_in_current_byte(&mut self, bits: u32) -> Number {
        self.bit_position += bits;
        let offset = u8::BITS - self.bit_position;
        let number = (self.data[self.cursor] & Self::bit_mask(bits, offset)) >> offset;
        if self.bit_position == u8::BITS {
            self.bit_position = 0;
            self.cursor += 1;
        }
        number as Number
    }

    fn read_bits(&mut self, bits: u32) -> Option<Number> {
        debug_assert!(bits > 0 && bits <= Number::BITS);
        if self.cursor == self.data.len() {
            return None;
        }
        let bit_remainder = u8::BITS - self.bit_position;
        if bit_remainder >= bits {
            Some(self.read_in_current_byte(bits))
        } else {
            let mut bits_left = bits;
            let mut number = 0;
            while bits_left > 0 {
                let bit_remainder = u8::BITS - self.bit_position;
                let bits_in_current_byte = std::cmp::min(bits_left, bit_remainder);
                bits_left -= bits_in_current_byte;
                number += self.read_in_current_byte(bits_in_current_byte) << bits_left;
                if self.cursor == self.data.len() {
                    return None;
                }
            }
            Some(number)
        }
    }

    fn bits_left(&self) -> u32 {
        let x = (self.data.len() - self.cursor) as u32 * u8::BITS - self.bit_position;
        debug_assert!(x < 400000);
        x
    }
}

type Version = Number;
type Value = Number;

enum Operation {
    Sum = 0,
    Product = 1,
    Min = 2,
    Max = 3,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

impl TryFrom<Number> for Operation {
    type Error = ();

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        use Operation::*;
        match value {
            x if x == Sum as Number => Ok(Sum),
            x if x == Product as Number => Ok(Product),
            x if x == Min as Number => Ok(Min),
            x if x == Max as Number => Ok(Max),
            x if x == GreaterThan as Number => Ok(GreaterThan),
            x if x == LessThan as Number => Ok(LessThan),
            x if x == EqualTo as Number => Ok(EqualTo),
            _ => Err(()),
        }
    }
}

enum Packet {
    Literal {
        version: Version,
        value: Value,
    },
    Operator {
        version: Version,
        operation: Operation,
        subpackets: Vec<Packet>,
    },
}

const LITERAL: Number = 4;

fn parse_literal(reader: &mut BitReader) -> Option<Number> {
    let mut keep_reading = 1;
    let mut literal = 0;
    while keep_reading == 1 {
        keep_reading = reader.read_bits(1)?;
        let part = reader.read_bits(4)?;
        literal <<= 4;
        literal += part;
    }
    Some(literal)
}

fn parse_expression(reader: &mut BitReader) -> Option<Packet> {
    let version = reader.read_bits(3)?;
    let type_id = reader.read_bits(3)?;
    if type_id == LITERAL {
        let number = parse_literal(reader)?;
        Some(Packet::Literal {
            version,
            value: number,
        })
    } else {
        let mut subpackets = Vec::new();
        let length_type = reader.read_bits(1)?;
        let operation: Operation = type_id.try_into().ok()?;
        if length_type == 0 {
            let mut bits_left = reader.read_bits(15)?;
            while bits_left > 0 {
                let before = reader.bits_left();
                let packet = parse_expression(reader)?;
                subpackets.push(packet);
                let after = reader.bits_left();
                bits_left -= (before - after) as Number;
            }
        } else {
            let count = reader.read_bits(11)?;
            for _ in 0..count {
                let packet = parse_expression(reader)?;
                subpackets.push(packet);
            }
        }
        Some(Packet::Operator {
            version,
            operation,
            subpackets,
        })
    }
}

fn sum_versions(packet: &Packet) -> Number {
    match packet {
        Packet::Literal { version, .. } => *version,
        Packet::Operator {
            version,
            subpackets,
            ..
        } => {
            version
                + subpackets
                    .iter()
                    .map(|p| sum_versions(p))
                    .fold(0, |s, v| s + v)
        }
    }
}

fn evaluate_expression(packet: &Packet) -> Number {
    match packet {
        Packet::Literal { value, .. } => *value,
        Packet::Operator {
            operation,
            subpackets,
            ..
        } => {
            let mut literals = subpackets.iter().map(|p| evaluate_expression(p));
            use Operation::*;
            match *operation {
                Sum => literals.fold(0, |s, v| s + v),
                Product => literals.fold(1, |p, v| p * v),
                Min => literals.min().unwrap(),
                Max => literals.max().unwrap(),
                GreaterThan => (literals.next().unwrap() > literals.next().unwrap()) as Number,
                LessThan => (literals.next().unwrap() < literals.next().unwrap()) as Number,
                EqualTo => (literals.next().unwrap() == literals.next().unwrap()) as Number,
            }
        }
    }
}

fn calculate_solution(expression: &str) -> (Number, Number) {
    let mut reader = BitReader::new(expression);
    let packet = parse_expression(&mut reader).unwrap();
    (sum_versions(&packet), evaluate_expression(&packet))
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::EXPRESSION));
}
