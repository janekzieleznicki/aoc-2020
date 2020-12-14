use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Seat {
    Floor {},
    Empty {},
    Occupied {},
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ParseSeatError;

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor {},
            'L' => Self::Empty {},
            '#' => Self::Occupied {},
            _ => Self::Empty {}
        }
    }
}

impl FromStr for Seat {
    type Err = ParseSeatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<char>() {
            Ok(c) => match c {
                '.' => Ok(Self::Floor {}),
                'L' => Ok(Self::Empty {}),
                '#' => Ok(Self::Occupied {}),
                _ => Err(Self::Err {})
            }
            Err(_) => Err(Self::Err {})
        }
    }
}

pub fn seats_from_line(s: &str) -> Vec<Seat> {
    s.chars().into_iter().map(Seat::from).collect_vec()
}

#[cfg(test)]
mod tests{
    use crate::seat::{Seat, seats_from_line, ParseSeatError};
    use std::str::FromStr;

    #[test]
    fn form_string_test() {
        assert_eq!(Seat::from_str(".").unwrap(), Seat::Floor {});
        assert_eq!(Seat::from_str("L").unwrap(), Seat::Empty {});
        assert_eq!(Seat::from_str("#").unwrap(), Seat::Occupied {});
        assert_eq!(Seat::from_str("1").expect_err(""), ParseSeatError {});

        assert_eq!(seats_from_line("L.LL.LL.LL").len(), 10);
    }
}