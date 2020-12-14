use debug_print::{debug_print};

use std::collections::VecDeque;
use itertools::Itertools;
use num::{Num,  Integer};
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, SubAssign};
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;
use itertools::__std_iter::Sum;
use prefix_sum::summable::Summable;
use itertools::MinMaxResult::MinMax;
fn main() {
    let (mut decypher, data) = from_file::<u128, &str>("./day-nine/code.dat");
    // println!("Preamble {:?}\nData {:?}",decypher.received, data);
    let first_failed = data.iter().skip_while(|&num| decypher.push(*num)).next().unwrap();
    println!("Received buffer: {:?}", decypher.received);
    println!("First failed: {}", first_failed);
    let found_set = decypher.contigous_set(*first_failed);
    println!("Found set {:?}", found_set);
    println!("{:#?}", found_set.iter().minmax());
    let found_set = decypher.contigous(&data_from_file::<u128, &str>("./day-nine/code.dat"),*first_failed);
    println!("From all set {:?}", found_set);
    match found_set.iter().minmax() {
        MinMax(min,max) => {
            println!("Min {} Max {} Sum {}",min,max,min+max)
        },
        _ => {}
    }
}
const PREAMBLE_SIZE: usize = 45;

pub fn from_file<I, P>(path: P) -> (Decypher<I>, Vec<I>)
    where P: AsRef<Path>,
          I: Copy + Display + Debug + Integer + FromStr + Summable + Sum + AddAssign + SubAssign,
          <I as std::str::FromStr>::Err: std::fmt::Debug, {
    let file = File::open(path);
    let iter = io::BufReader::new(file.unwrap()).lines().into_iter().map(|str| str.unwrap()).collect_vec();
    (Decypher::from_preamble(iter.iter()
        .take(PREAMBLE_SIZE)
        .map(|line| line.parse::<I>().unwrap())
        .collect_vec().as_slice()),
     iter.iter()
         .skip(PREAMBLE_SIZE)
         .map(|line| line.parse::<I>().unwrap())
         .collect())
}


pub fn data_from_file<I, P>(path: P) -> Vec<I>
    where P: AsRef<Path>,
          I: Copy + Display + Debug + Integer + FromStr + Summable + Sum + AddAssign + SubAssign,
          <I as std::str::FromStr>::Err: std::fmt::Debug, {
    let file = File::open(path);
    io::BufReader::new(file.unwrap()).lines().into_iter().map(|str| str.unwrap())
        .map(|line| line.parse::<I>().unwrap())
        .collect_vec()
}

pub struct Decypher<I> where I: Num {
    received: VecDeque<I>
}

impl<I> Decypher<I> where I: Copy + Display + Debug + Integer + Eq + Summable + Sum + AddAssign + SubAssign {
    pub fn from_preamble(preamble: &[I]) -> Decypher<I> {
        Decypher {
            received: preamble.iter().copied().collect()
        }
    }
    pub fn push(&mut self, item: I) -> bool {
        debug_print!("Matching {} against {:?} | ", item, &self.received);
        let test = self.received.iter().copied()
            .collect::<Vec<I>>()
            .into_iter()
            .permutations(2)
            .map(|vec| (vec[0], vec[1]))
            .filter(|(left, right)| left != right)
            .filter(|(left, right)| *left + *right == item)
            .map(|tup| {
                debug_print!("{:?} => {}\n", tup, item);
                tup
            })
            .any(|(left, right)| left + right == item);
        if test {
            self.received.pop_front();
            self.received.push_back(item);
        }
        test
    }
    pub fn contigous_set(&mut self, item: I) -> Vec<I> {
        let data = self.received.iter().copied().collect_vec();
        debug_print!("Data vec {:?}\t Item: {}\n",data,item);
        for start in 0..self.received.len() {
            for end in start + 2..=self.received.len() {
                match check(&data[start..end], item) {
                    Ok(res) => return res,
                    Err(_) => continue
                }
            }
        }
        Vec::new()
    }
    pub fn contigous(self, data: &[I], item: I) -> Vec<I> {
        debug_print!("Data vec {:?}\t Item: {}\n",data,item);
        for start in 0..data.len() {
            for end in start + 2..=data.len() {
                match check(&data[start..end], item) {
                    Ok(res) => return res,
                    Err(_) => continue
                }
            }
        }
        Vec::new()
    }
}

pub fn check<Int>(slice: &[Int], item: Int) -> Result<Vec<Int>, bool>
    where Int: Summable + Copy + Sum + Display + Debug + std::cmp::PartialEq
{
    match slice.iter().copied().sum::<Int>() {
        x if x == item => {
            let vec = slice.iter().copied().collect_vec();
            debug_print!("Found match Sum({:?}) == {}\n",vec,item);
            Ok(vec)
        }
        x => {
            debug_print!("Sum{:?}: {} != {}\n",slice,x,item);
            Err(false)
        }
    }
}

#[cfg(test)]
mod xmas_code_tests {
    use crate::Decypher;

    #[test]
    fn from_example_5() {
        let preamble = vec![35, 20, 15, 25, 47, 40];
        let numbers = vec![62, 55, 65, 95, 102, 117, 150, 182, 127, 219];
        let mut decypher = Decypher::from_preamble(&preamble);
        numbers.into_iter().for_each(|num|
            match num {
                127 => assert!(!decypher.push(num), "{} accepted, but it shouldn't", num),
                _ => assert!(decypher.push(num), "{} not accepted, but it should", num)
            });
        assert_ne!(decypher.contigous_set(127), preamble);
        let mut decypher = Decypher::from_preamble(&preamble);
        assert_eq!(decypher.contigous_set(127), vec![15, 25, 47, 40]);
    }

    #[test]
    fn from_example_25() {
        let preamble: Vec<i32> = (0..=25).into_iter().collect();
        let numbers = vec![100, 50, 26, 49]; //order is important
        let mut decypher = Decypher::from_preamble(&preamble);
        numbers.into_iter().for_each(|num|
            match num {
                0..=49 => assert!(decypher.push(num), "{} not accepted, but it should", num),
                _ => assert!(!decypher.push(num), "{} accepted, but it shouldn't", num)
            });
    }
}