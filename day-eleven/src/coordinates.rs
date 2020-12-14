use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coordinates{
    pub(crate) x: usize,
    pub(crate) y: usize,
}
#[derive(Clone, Copy)]
pub struct Direction {
    x: i32,
    y: i32
}
pub fn Direction(x: i32, y: i32) -> Direction{
    Direction{x, y }
}
impl Add<Direction> for Coordinates{
    type Output = Self;

    fn add(self, rhs: Direction) -> Self {
        Self {
            x: (self.x as i64).checked_add(rhs.x as i64).unwrap_or(self.x as i64) as usize,
            y: (self.y as i64).checked_add(rhs.y as i64).unwrap_or(self.y as i64) as usize
        }
    }
}

impl AddAssign<Direction> for Coordinates{
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self+rhs
    }
}