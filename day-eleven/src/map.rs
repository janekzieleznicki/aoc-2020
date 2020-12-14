use crate::seat::*;
use crate::coordinates::*;
use itertools::Itertools;
use core::ops;
use std::ops::DerefMut;
use std::fmt::{Formatter, Debug};
use std::fmt;

pub trait TaskLogic {
    fn should_stop(iter: usize) -> bool;
    fn new_occupied(occupied_neighbors: usize) -> Seat;
}

pub struct PartOneLogic {}

impl TaskLogic for PartOneLogic {
    fn should_stop(iter: usize) -> bool {
        iter < 2
    }

    fn new_occupied(occupied_neighbors: usize) -> Seat {
        match occupied_neighbors {
            0..4 => Seat::Occupied {},
            _ => Seat::Empty {}
        }
    }
}

pub struct partTwoLogic {}

impl TaskLogic for partTwoLogic {
    fn should_stop(_: usize) -> bool {
        false
    }

    fn new_occupied(occupied_neighbors: usize) -> Seat {
        match occupied_neighbors {
            0..5 => Seat::Occupied {},
            _ => Seat::Empty {}
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Map(pub Vec<Vec<Seat>>);

impl Map where {
    pub fn from_lines<'a, LineIterator>(iter: LineIterator) -> Map where LineIterator: IntoIterator<Item=&'a str> {
        Map(iter.into_iter().map(|line| seats_from_line(line)).collect_vec())
    }
    pub fn at(&self, coord: &Coordinates) -> Option<Seat> {
        let x_range = 0..self.len();
        let y_range = 0..self.first().unwrap().len();
        if x_range.contains(&coord.x) && y_range.contains(&coord.y) {
            Some(self[coord.x][coord.y])
        } else {
            None
        }
    }
}

impl ops::Deref for Map {
    type Target = Vec<Vec<Seat>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn update<I: TaskLogic>(map: &Map, x: usize, y: usize) -> Option<Seat> {
    let occupied_neighbours = occupied_neighbours_part_2::<I>(map, x, y);
    match map[x][y] {
        Seat::Empty {} => {
            if occupied_neighbours == 0 {
                Some(Seat::Occupied {})
            } else {
                None
            }
        }
        Seat::Occupied {} => {
            match I::new_occupied(occupied_neighbours)
            {
                Seat::Empty {} => Some(Seat::Empty {}),
                _ => None
            }
        }
        _ => None
    }
}

pub fn update_map<I: TaskLogic>(map: &Map) -> Map {
    let mut updated = map.clone();
    for x in 0..map.len() {
        for y in 0..map.first().unwrap().len() {
            match map[x][y] {
                Seat::Floor {} => {}
                Seat::Empty {} | Seat::Occupied {} => {
                    match update::<I>(&map, x, y) {
                        Some(seat) => updated[x][y] = seat,
                        None => {}
                    }
                }
            }
        }
    }
    updated
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Map : \n{}", self.iter().map(|row|
            row.iter().map(|seat|
                match seat {
                    Seat::Floor {} => ".",
                    Seat::Empty {} => "L",
                    Seat::Occupied {} => "#"
                }
            ).join("")
        ).join("\n"))
    }
}

pub(crate) fn occupied_seats(map: &Map) -> usize {
    map.iter()
        .map(|row| {
            row.iter()
                .filter_map(|seat| match seat {
                    Seat::Occupied {} => Some(seat),
                    _ => None
                }).count()
        })
        .sum()
}

fn check_direction<I: TaskLogic>(map: &Map, start: Coordinates, dir: Direction) -> usize {
    let mut coord = start;
    let mut iter = 0;
    loop {
        iter += 1;
        coord += dir;
        match map.at(&coord) {
            Some(seat) => match seat {
                Seat::Occupied {} => return 1,
                Seat::Empty {} => return 0,
                _ => {
                    if I::should_stop(iter) { return 0; } else { continue; }
                }
            },
            None => return 0
        };
    }
}

fn occupied_neighbours_part_2<I: TaskLogic>(map: &Map, x: usize, y: usize) -> usize {
    let directions = vec![
        Direction(-1, -1), Direction(-1, 0), Direction(-1, 1),
        Direction(-0, -1), Direction(0, 1),
        Direction(1, -1), Direction(1, 0), Direction(1, 1),
    ];
    let start = Coordinates { x, y };
    directions.iter().map(|&dir| check_direction::<I>(map, start, dir)).sum()
}

#[cfg(test)]
mod test {
    use crate::map::{update_map, occupied_neighbours_part_2, Map, PartOneLogic, partTwoLogic, occupied_seats};
    use crate::coordinates::{Coordinates, Direction};
    use crate::seat::Seat;

    #[test]
    fn read_map() {
        let input = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        // println!("{:?}",seats_from_line_iter(input.lines()));
        let map = Map::from_lines(input.lines());
        assert_eq!(occupied_neighbours_part_2::<PartOneLogic>(&map, 0, 0), 0);
        assert_eq!(map.first().unwrap().len(), 10);
        assert_eq!(map.len(), 10);
        assert_eq!(Map::from_lines(input.lines()), Map::from_lines(input.lines()));
        let first_iter = r#"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"#;
        let updated_map = update_map::<PartOneLogic>(&Map::from_lines(input.lines()));
        assert_eq!(occupied_neighbours_part_2::<PartOneLogic>(&updated_map, 0, 0), 2);
        assert_eq!(updated_map, Map::from_lines(first_iter.lines()));

        let second_iter = r#"#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##"#;
        assert_eq!(update_map::<PartOneLogic>(&Map::from_lines(first_iter.lines())),
                   Map::from_lines(second_iter.lines()));
        let mut live_map = updated_map;
        loop {
            let new_state = update_map::<PartOneLogic>(&live_map);
            if new_state == live_map {
                break;
            } else {
                live_map = new_state
            }
        }
        assert_eq!(occupied_seats(&live_map), 37);
    }

    #[test]
    fn example_1() {
        let empty_input = r#".............
.L.L.#.#.#.#.
............."#;
        let map = Map::from_lines(empty_input.lines());
        assert_eq!(map.at(&Coordinates { x: 1, y: 1 }), Some(Seat::Empty {}));
        assert_eq!(map.at(&(Coordinates { x: 1, y: 1 } + Direction(1, 0))), Some(Seat::Floor {}));
        assert_eq!(occupied_neighbours_part_2::<partTwoLogic>(&map, 1, 1), 0);
        // assert_eq!(occupied_neighbours_part_2(&map, 1, 3), 1);
    }

    #[test]
    fn example_2() {
        let empty_input = r#".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##."#;
        let map = Map::from_lines(empty_input.lines());
        assert_eq!(occupied_neighbours_part_2::<partTwoLogic>(&map, 3, 3), 0);
        // assert_eq!(occupied_neighbours_part_2(&map, 2, 4), 1);
    }

    #[test]
    fn part_2_test() {
        let first_iter = r#"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"#;
        let mut live_map = Map::from_lines(first_iter.lines());
        loop {
            let new_state = update_map::<partTwoLogic>(&live_map);
            if new_state == live_map {
                break;
            } else {
                println!("{:?}", new_state);
                live_map = new_state
            }
        }
        assert_eq!(occupied_seats(&live_map), 26);
    }
}