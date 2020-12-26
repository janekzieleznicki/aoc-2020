#![feature(generators, generator_trait, str_split_once)]


mod tickets;

use crate::tickets::*;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("./day-sixteen/tickets.dat").unwrap();

    println!("ticket scanning error rate: {}",TicketsData::from_str(input.as_str()).unwrap().scanning_error_rate());
    println!("Departure multiple: {}",TicketsData::from_str(input.as_str()).unwrap().sort_rules().get_departure_multiple());
}
