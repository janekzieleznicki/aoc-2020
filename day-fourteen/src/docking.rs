use std::str::FromStr;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{BitOrAssign, BitAndAssign, BitAnd, BitOr};
use std::collections::HashMap;
use either::*;
use debug_print::{debug_print};

extern crate regex;

use regex::Regex;
use itertools::{Either, Itertools};

#[derive(Clone)]
pub struct Mask {
    ones: u64,
    zeroes: u64,
    floating: Vec<u16>,
}

impl Mask {
    pub fn apply(&self, mut val: u64) -> u64 {
        val.bitor_assign(self.ones);
        val.bitand_assign(!self.zeroes);
        val
    }
    fn all_addresses(&self, inst: &WriteInstruction) -> Vec<u64> {
        let masked = inst.addr.bitand(self.zeroes);
        let masked = masked.bitor(self.ones);
        let mut addresses = vec![masked];
        debug_print!("{} => \n{:0>35b}\n", inst.addr, masked);
        self.floating.iter()
            .for_each(
                |index| {
                    addresses.extend(addresses.clone().iter().map(|addr| addr ^ (1u64 << index)));
                }
            );
        addresses.iter().for_each(|addr|
            { debug_print!("{:0>35b}\n", addr); }
        );
        addresses
    }
}

impl FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = Mask {
            ones: 0,
            zeroes: 0,
            floating: Vec::new(),
        };
        s.chars().into_iter().rev().enumerate().for_each(|(iter, c)|
            match c {
                '1' => mask.ones += 1u64 << iter,
                '0' => mask.zeroes += 1u64 << iter,
                'X' => mask.floating.push(iter as u16),
                _ => panic!("Unexpected char: {} at [{}] of {}", c, iter, s)
            }
        );
        Ok(mask)
    }
}

impl Display for Mask {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Mask: \n[Ones:  {:b}\nZeroes: {:b}]", self.ones, self.zeroes)
    }
}

pub struct Interpreter {
    current_mask: Mask,
    memory: HashMap<u64, u64>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {
            current_mask: Mask { ones: 0, zeroes: 0, floating: Vec::new() },
            memory: Default::default(),
        }
    }
}

struct WriteInstruction {
    addr: u64,
    val: u64,
}

impl Interpreter {
    pub fn read(&mut self, s: &str) -> Either<Mask, WriteInstruction> {
        lazy_static! {
        //TODO consider MatchSet
        static ref MASK_RE: Regex = Regex::new(r#"mask = (?P<mask>[01X]+)"#).unwrap();
        static ref MEM_RE: Regex  = Regex::new(r#"mem\[(?P<address>\d+)\]\s=\s(?P<value>\w+)$"#).unwrap();
        }
        match MASK_RE.captures(s) {
            Some(matched) => {
                let mask_str = matched.name("mask").unwrap().as_str();
                debug_print!("{}\n", mask_str);
                return Left(Mask::from_str(mask_str).unwrap());
            }
            None => {}
        };
        match MEM_RE.captures(s) {
            Some(matched) => return Right(self.decode(matched)),
            None => {}
        };
        Right(WriteInstruction { addr: 0, val: 0 })
    }

    fn decode(&mut self, matched: regex::Captures) -> WriteInstruction {
        let addr = matched.name("address").unwrap().as_str().parse::<usize>().unwrap();
        let val = matched.name("value").unwrap().as_str().parse::<u64>().unwrap();
        debug_print!("{} => {}\n", addr, val);
        WriteInstruction { addr: addr as u64, val }
    }

    pub fn sum_values(&self) -> u64 {
        self.memory.iter().map(|(key, val)| val).sum()
    }
}

pub trait Decoder {
    fn read(&mut self, s: &str);
    fn sum_values(&self) -> u64;
}

pub struct DecoderV1 {
    interpreter: Interpreter
}

impl Decoder for DecoderV1 {
    fn read(&mut self, s: &str) {
        let either = self.interpreter.read(s);
        either.as_ref().left().map(|mask| self.interpreter.current_mask = mask.clone());
        either.as_ref().right().map(|inst| { self.interpreter.memory.insert(inst.addr, self.interpreter.current_mask.apply(inst.val)); });
    }

    fn sum_values(&self) -> u64 {
        self.interpreter.sum_values()
    }
}

impl Default for DecoderV1 {
    fn default() -> Self {
        DecoderV1 {
            interpreter: Default::default()
        }
    }
}

pub struct DecoderV2 {
    interpreter: Interpreter
}

impl Default for DecoderV2 {
    fn default() -> Self {
        DecoderV2 {
            interpreter: Default::default()
        }
    }
}

impl Decoder for DecoderV2 {
    fn read(&mut self, s: &str) {
        let either = self.interpreter.read(s);
        either.as_ref().left().map(|mask| self.interpreter.current_mask = mask.clone());
        either.as_ref().right()
            .map(|inst| {
                self.interpreter.current_mask.all_addresses(&inst).iter()
                    .for_each(|&addr|
                        { self.interpreter.memory.insert(addr, inst.val); }
                    );
            });
    }
    fn sum_values(&self) -> u64 {
        self.interpreter.sum_values()
    }
}

#[cfg(test)]
mod tests {
    use crate::docking::{Mask, Interpreter, DecoderV1, Decoder, WriteInstruction, DecoderV2};
    use std::str::FromStr;

    #[test]
    fn mask_from_str() {
        let mask = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        println!("{:}", mask);
        assert_eq!(mask.apply(11), 73);
        assert_eq!(mask.apply(101), 101);
        assert_eq!(mask.apply(0), 64);
    }

    #[test]
    fn one_segment() {
        let input = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;
        let mut interpreter = DecoderV1::default();
        input.lines().for_each(|line| interpreter.read(line));
        assert_eq!(interpreter.sum_values(), 165)
    }

    #[test]
    fn two_segments() {
        let input = r#"mask = 1100X10X01001X111001X00010X00100X011
mem[24821] = 349
mem[34917] = 13006
mem[53529] = 733
mem[50289] = 245744
mem[23082] = 6267
mask = 011X1X00X100100XXXX11100X0000100X010
mem[21316] = 14188354
mem[53283] = 7137
mem[57344] = 62358
mem[63867] = 9443"#;
        let mut interpreter = DecoderV1::default();
        input.lines().for_each(|line| interpreter.read(line))
    }

    #[test]
    fn all_addresses_test() {
        let mask = Mask::from_str("000000000000000000000000000000X1001X").unwrap();
        println!("{:?}", mask.floating);
        let inst = WriteInstruction { addr: 42, val: 100 };
        assert_eq!(mask.all_addresses(&inst), vec![26, 27, 58, 59]);
    }

    #[test]
    fn one_segment_v2() {
        let input = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;
        let mut interpreter = DecoderV2::default();
        input.lines().for_each(|line| interpreter.read(line));
        assert_eq!(interpreter.sum_values(), 208)
    }
}