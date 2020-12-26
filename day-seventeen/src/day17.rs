#![feature(or_patterns)]
mod convay_cubes_3D;
mod convay_cubes_4D;
use crate::convay_cubes_3D::PocketDimension;
use std::str::FromStr;
use debug_print::{debug_print};
use crate::convay_cubes_4D::PocketDimension4D;
use std::time::Instant;


fn main() {
    let input = r#"..#....#
##.#..##
.###....
#....#.#
#.######
##.#....
#.......
.#......"#;
    let mut pocket = PocketDimension4D::from_str(input).unwrap();
    (0..6).for_each(|i|{
        pocket.update();
        debug_print!("After {} cycle:\n\n{}\n",i,pocket);
    });
    println!("active state after the sixth cycle: {}",pocket.active_cubes());
        (6..).for_each(|i|{
            let start = Instant::now();
            pocket.update();
            println!("Iteration {} took {:?}",i,start.elapsed().as_micros());
            debug_print!("After {} cycle:\n\n{}\n",i,pocket);
        });
}
