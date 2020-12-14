use std::fs::File;
use std::path::Path;
use std::io;
use std::io::BufRead;

// use math::round::ceil;
#[derive(Debug)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    pub fn lower(&self) -> Range {
        let new_max = self.max - ((self.max as f64 - self.min as f64) / 2.0f64).ceil() as usize;
        println!("{:?} -lower-> {:?}", self, Range {
            min: self.min,
            max: new_max,
        });
        Range {
            min: self.min,
            max: new_max,
        }
    }
    pub fn upper(&self) -> Range {
        let new_min = self.min + ((self.max as f64 - self.min as f64) / 2.0f64).ceil() as usize;
        println!("{:?} -upper-> {:?}", self, Range {
            min: new_min,
            max: self.max,
        });
        Range {
            min: new_min,
            max: self.max,
        }
    }
}

pub fn row(str: &str) -> usize {
    let mut range = Range { min: 0, max: 127 };
    println!("Finding row for {}", str);
    str.chars().for_each(|c|
        match c {
            'F' => range = range.lower(),
            'B' => range = range.upper(),
            _ => assert!(false)
        }
    );
    range.max
}

pub fn column(str: &str) -> usize {
    let mut range = Range { min: 0, max: 7 };
    println!("Finding column for {}", str);
    str.chars().for_each(|c|
        match c {
            'R' => range = range.upper(),
            'L' => range = range.lower(),
            _ => assert!(false)
        }
    );
    range.max
}

pub fn seat_id(str: &str) -> usize {
    row(&str[0..=6]) * 8 + column(&str[7..=9])
}

pub fn part_one<P>(name: P) where P: AsRef<Path> {
    let file = File::open(name);

    let max = io::BufReader::new(file.unwrap()).lines().map(|line| seat_id(line.unwrap().as_str())).max().unwrap();
    println!("Highest Seat ID: {}", max);
}

pub fn part_two<P>(name: P) where P: AsRef<Path> {
    let file = File::open(name);

    let mut max = io::BufReader::new(file.unwrap()).lines().map(|line| seat_id(line.unwrap().as_str())).collect::<Vec<usize>>();
    max.sort();

    println!("\n\nAll seat numbers {:?} \n",
             max);
    let mut iter = max.into_iter().peekable();
    loop {
        match iter.next() {
            Some(curr) => {
                match iter.peek() {
                    Some(next) => match *next - curr {
                        1 => continue,
                        _ => {
                            println!("Seat number: {}", curr + 1);
                            break;
                        }
                    },
                    None => break
                }
            }
            None => break
        }
    }

    // println!("Highest Seat ID: {}", max);
}

fn main() {
    part_one("./day-five/boarding_passes.dat");
    println!("-------------------------------------");
    part_two("./day-five/boarding_passes.dat");
}

#[cfg(test)]
mod tests {
    use crate::{row, column, seat_id};

    #[test]
    fn row_test() {
        assert_eq!(row("FBFBBFF"), 44);
        assert_eq!(row("BFFFBBF"), 70);
        assert_eq!(row("FFFBBBF"), 14);
        assert_eq!(row("BBFFBBF"), 102);
    }

    #[test]
    fn column_test() {
        assert_eq!(column("RRR"), 7);
        assert_eq!(column("RRR"), 7);
        assert_eq!(column("RLL"), 4);
    }

    #[test]
    fn seat_id_test() {
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}