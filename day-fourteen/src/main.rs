#[macro_use]
extern crate lazy_static;

use crate::docking::{DecoderV1, Decoder, DecoderV2};
use std::fs::File;
use std::io;
use std::io::BufRead;

mod docking;
fn main() {
    {
        let mut interpreter = DecoderV2::default();
        let file = File::open("./day-fourteen/instructions.dat").unwrap();
        io::BufReader::new(file).lines()
            .for_each(|l| interpreter.read(l.unwrap().as_str()));
        println!("Final sum: {}", interpreter.sum_values());
    }
    {
        let mut interpreter = DecoderV2::default();
        let file = File::open("./day-fourteen/harder.dat").unwrap();
        io::BufReader::new(file).lines()
            .for_each(|l| interpreter.read(l.unwrap().as_str()));
        println!("Final sum: {}", interpreter.sum_values());
    }

}
