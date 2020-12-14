#![feature(or_patterns)]
mod ship;

use crate::ship::*;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let mut ship = Ship::default();
    let file = File::open("./day-twelve/instructions.dat").unwrap();
    io::BufReader::new(file).lines()
        .for_each(|l| ship.move_part_2(l.unwrap().parse::<Action>().unwrap()));
    println!("{:?}\n Manhattan distance: {}", ship, manhattan_distance(&ship));
}
