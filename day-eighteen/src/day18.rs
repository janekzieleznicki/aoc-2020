use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::expressions::evaluate;

mod expressions;

fn main() {
    let file = File::open("./day-eighteen/expressions.dat");
    // io::BufReader::new(file.unwrap()).lines().filter_map(|res| res.ok()).map(|line| BagRule::from_str(line.as_str()).unwrap()).collect()
    let sum = io::BufReader::new(file.unwrap()).lines().map(|line|
    evaluate(line.unwrap().as_str())).sum::<i64>();

    println!("Sum all: {}",sum);
}
