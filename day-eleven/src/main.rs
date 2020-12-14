#![feature(exclusive_range_pattern)]
pub mod seat;
pub mod map;
pub mod coordinates;
use std::{io};
use debug_print::{debug_print};
use std::fs::File;
use std::io::{Read};
use crate::map::{Map, update_map, PartOneLogic, occupied_seats, partTwoLogic};


fn main() {
    let file = File::open("./day-eleven/map.dat").unwrap();
    let mut buffer = String::new();
    io::BufReader::new(file).read_to_string(&mut buffer);
    debug_print!("{:?}",buffer);
    let mut live_map = Map::from_lines(buffer.lines());
    debug_print!("{:?}",live_map);
    loop {
        let new_state = update_map::<partTwoLogic>(&live_map);
        if new_state == live_map {
            break;
        } else {
            debug_print!("{:?}",new_state);
            live_map = new_state
        }
    }
    println!("In stable state we have {} occupied seats", occupied_seats(&live_map));
}

#[cfg(test)]
mod convay_game_of_seats_tests {

}