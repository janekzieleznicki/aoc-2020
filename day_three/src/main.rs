use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
pub struct Map {
    slice: Vec<Vec<char>>
}

pub struct Finished {}

impl Map {
    const THREE: char = '#';
    const EMPTY: char = '.';
    pub fn from_file<P>(name: P) -> Map where P: AsRef<Path> {
        let file = File::open(name);
        let data: Vec<Vec<char>> = io::BufReader::new(file.unwrap()).lines().map(|line|
            line.unwrap().into_bytes().into_iter().map(|b| b as char).collect()
        ).collect();
        data.iter().for_each(|row| assert_eq!(row.len(), data[0].len()));
        println!("Dimensions: X:{} Y:{}", data[0].len(), data.len());
        Map {
            slice: data
        }
    }
    pub fn at(&self, x: usize, y: usize) -> Result<char, Finished> {
        if y >= self.slice.len() {
            Err(Finished {})
        } else if x >= self.slice[0].len() {
            self.at(x - self.slice[0].len(), y)
        } else {
            Ok(self.slice[y][x])
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Velocity {
    x: usize,
    y: usize,
}

pub fn traverse(map: &Map, vel: &Velocity) -> usize {
    let mut tree_count: usize = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;
    loop {
        match map.at(x, y) {
            Ok(Map::EMPTY) => {
                // println!("Empty at {} {} ",x,y);
                x += vel.x;
                y += vel.y;
                continue;
            }
            Ok(Map::THREE) => {
                // println!("Tree at {} {} ",x,y);
                x += vel.x;
                y += vel.y;
                tree_count += 1;
                continue;
            }
            Err(_) => { break; }
            _ => {
                eprintln!("Unexpected char from map: {}", map.at(x, y).unwrap_or_default());
                break;
            }
        }
    }
    println!("With velocity {:?} Trees encountered: {}", vel, tree_count);
    tree_count
}

fn main() {
    let map = Map::from_file("./day_three/map_slice.dat");
    let case = vec![Velocity { x: 1, y: 1 },
                    Velocity { x: 3, y: 1 },
                    Velocity { x: 5, y: 1 },
                    Velocity { x: 7, y: 1 },
                    Velocity { x: 1, y: 2 }, ];

    println!("Total multiply: {}",case.iter().map(|vel|traverse(&map,vel )).product::<usize>());
}
