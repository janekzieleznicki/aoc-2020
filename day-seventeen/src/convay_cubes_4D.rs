use std::str::FromStr;
use itertools::Itertools;
use itertools::MinMaxResult::{OneElement, MinMax, NoElements};
use std::fmt::{Display, Formatter};
use std::ops::{RangeInclusive, Sub};
use std::fmt;
use num::traits::AsPrimitive;
use debug_print::{debug_print};
use num::Signed;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Position4D {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

#[derive(Debug, Clone)]
pub(crate) struct PocketDimension4D {
    active_cubes: Vec<Position4D>
}

impl PocketDimension4D {
    pub fn active_cubes(&self) -> usize {
        self.active_cubes.len()
    }
    fn active_neighbours(&self, pos: &Position4D) -> usize {
        self.active_cubes.iter()
            .filter(|&active| active != pos)
            .filter(|&active| active.x.sub(&pos.x).abs() <= 1)
            .filter(|&active| active.y.sub(&pos.y).abs() <= 1)
            .filter(|&active| active.z.sub(&pos.z).abs() <= 1)
            .filter(|&active| active.w.sub(&pos.w).abs() <= 1)
            .count()
    }
    pub fn update(&mut self) -> &mut Self {
        let ranges_to_check = self.get_active_ranges().expand_by_one();
        let mut new_active = Vec::new();
        ranges_to_check.x.clone().for_each(|x| {
            ranges_to_check.y.clone().for_each(|y| {
                ranges_to_check.z.clone().for_each(|z| {
                    ranges_to_check.w.clone().for_each(|w| {
                        let pos = Position4D { x, y, z, w };
                        // let active_neighbours = self.active_neighbours(&pos);
                        match (self.active_cubes.iter().find(|&&ac| ac == pos), self.active_neighbours(&pos)) {
                            (None, 3) => { new_active.push(pos) }
                            (Some(_), 2 | 3) => { new_active.push(pos) }
                            _ => {}
                        }
                    })
                })
            })
        });
        if self.active_cubes == new_active {
            panic!("PocketDimension in stable state {}",self)
        }
        self.active_cubes = new_active;
        self
    }
    fn get_active_ranges(&self) -> PositionRange4D {
        let x_range = match self.active_cubes.iter().map(|cube| cube.x).minmax() {
            OneElement(x) => RangeInclusive::new(x, x),
            MinMax(min, max) => RangeInclusive::new(min, max),
            NoElements => panic!()
        };
        let y_range = match self.active_cubes.iter().map(|cube| cube.y).minmax() {
            OneElement(y) => RangeInclusive::new(y, y),
            MinMax(min, max) => RangeInclusive::new(min, max),
            NoElements => panic!()
        };
        let z_range = match self.active_cubes.iter().map(|cube| cube.z).minmax() {
            OneElement(z) => RangeInclusive::new(z, z),
            MinMax(min, max) => RangeInclusive::new(min, max),
            NoElements => panic!()
        };
        let w_range = match self.active_cubes.iter().map(|cube| cube.w).minmax() {
            OneElement(w) => RangeInclusive::new(w, w),
            MinMax(min, max) => RangeInclusive::new(min, max),
            NoElements => panic!()
        };
        PositionRange4D {
            x: x_range,
            y: y_range,
            z: z_range,
            w: w_range,
        }
    }
}

impl FromStr for PocketDimension4D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let active = s.split("\n").enumerate().map(|(y, line_str)|
            line_str.chars().enumerate().filter_map(|(x, char)|
                match char {
                    '#' => Some(Position4D { x: x as i32, y: y as i32, z: 0, w:0}),
                    _ => None
                })
                .collect_vec()
        ).flatten().collect_vec();
        Ok(PocketDimension4D { active_cubes: active })
    }
}

#[derive(Debug)]
struct PositionRange4D {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
    w: RangeInclusive<i32>,
}

impl PositionRange4D {
    pub fn expand_by_one(&self) -> PositionRange4D {
        self.expand_by(1)
    }
    pub fn expand_by(&self, i: i32) -> PositionRange4D {
        PositionRange4D {
            x: RangeInclusive::new(self.x.start() - 1, self.x.end() + 1),
            y: RangeInclusive::new(self.y.start() - 1, self.y.end() + 1),
            z: RangeInclusive::new(self.z.start() - 1, self.z.end() + 1),
            w: RangeInclusive::new(self.w.start() - 1, self.w.end() + 1),
        }
    }
}

impl Display for PocketDimension4D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let range = self.get_active_ranges();
        let str = range.z.clone().map(|z| {
            [format!("z={}", z),
                range.y.clone().map(|y| {
                    let mut line = ".".repeat(range.x.clone().count()).into_bytes();
                    self.active_cubes.iter().filter(|pos| pos.z == z && pos.y == y)
                        .for_each(|pos| line[(pos.x - range.x.start()) as usize] = '#'.as_());
                    String::from_utf8(line).unwrap()
                })
                    .join("\n")].join("\n")
        }).join("\n\n");
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use crate::convay_cubes_4D::PocketDimension4D;
    use std::str::FromStr;
    use debug_print::{debug_print};
    use crate::convay_cubes_4D::Position4D;

    #[test]
    pub fn example() {
        let input = r#".#.
..#
###"#;
        let pocket = PocketDimension4D::from_str(input);
        assert!(pocket.is_ok());
        let pocket = pocket.unwrap();
        assert_eq!(pocket.active_cubes.len(), 5);
        // debug_print!("MAP: {:?}",pocket);
        debug_print!("{}",pocket);
        debug_print!("{:?}\n",pocket);
        assert_eq!(pocket.active_neighbours(&Position4D { x: 1, y: 1, z: 0, w:0 }), 5);
        assert_eq!(pocket.active_neighbours(&Position4D { x: 0, y: 0, z: 0, w:0 }), 1);
        assert_eq!(pocket.active_neighbours(&Position4D { x: 2, y: 0, z: 0, w:0 }), 2);
        assert_eq!(pocket.active_neighbours(&Position4D { x: 2, y: 2, z: 0, w:0 }), 2);


        let mut pocket = pocket.clone();
        debug_print!("After 1 cycle:\n\n{}\n",pocket.update());
        debug_print!("After 2 cycles:\n\n{}\n",pocket.update());
        debug_print!("After 3 cycles:\n\n{}\n",pocket.update());
        debug_print!("After 4 cycles:\n\n{}\n",pocket.update());
        debug_print!("After 5 cycles:\n\n{}\n",pocket.update());
        debug_print!("After 6 cycles:\n\n{}\n",pocket.update());
        assert_eq!(pocket.active_cubes(), 848)
    }
}