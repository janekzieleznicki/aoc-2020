use std::str::FromStr;
use std::sync::atomic::Ordering::AcqRel;
use std::ops::{AddAssign, Mul};
use std::fmt::Pointer;

struct ShipParseError {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    north: i32,
    east: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Velocity {
    north: i32,
    east: i32,
}

impl Velocity {
    fn from_two_points(start: Position, finish: Position) -> Velocity {
        Velocity {
            north: finish.north - start.north,
            east: finish.east - start.east,
        }
    }
}

impl Mul<i32> for Velocity {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            north: self.north * rhs,
            east: self.east * rhs,
        }
    }
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, rhs: Velocity) {
        self.north += rhs.north;
        self.east += rhs.east;
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    Forward { diff: i32 },
    Turn { diff: i32 },
    ByDirection { direction: Direction, diff: i32 },
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.split_at(1).1.parse::<i32>().unwrap();
        match s.split_at(1).0 {
            "N" => Ok(Action::ByDirection { direction: Direction::North {}, diff: val }),
            "S" => Ok(Action::ByDirection { direction: Direction::South {}, diff: val }),
            "E" => Ok(Action::ByDirection { direction: Direction::East {}, diff: val }),
            "W" => Ok(Action::ByDirection { direction: Direction::West {}, diff: val }),
            "L" => Ok(Action::Turn { diff: -val }),
            "R" => Ok(Action::Turn { diff: val }),
            "F" => Ok(Action::Forward { diff: val }),
            _ => Err(())
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ship {
    position: Position,
    directiob: Direction,
    waypoint: Velocity,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            position: Position { north: 0, east: 0 },
            directiob: Direction::East,
            waypoint: Velocity { north: 1, east: 10 },
        }
    }
}

impl Ship {
    pub fn move_part_2(&mut self, inst: Action) {
        match inst {
            Action::Forward { diff: val } => {
                let velocity = self.waypoint * val;
                self.position += velocity;
                // self.waypoint += velocity;
            }
            Action::Turn { diff: val } => {
                self.waypoint = rotate_around_ship(&self.waypoint, val as f64);
                // self.turn(inst);
            }
            Action::ByDirection { direction, diff } => Self::move_self(&mut self.waypoint, direction, diff),
            _ => panic!("Unexpected {:?}", inst)
        }
    }
    pub fn move_part_1(&mut self, inst: Action) {
        match inst {
            Action::Forward { diff: val } => {
                // Self::move_self(&mut self.position, self.directiob, val)
                match self.directiob {
                    Direction::North {} => self.position.north += val,
                    Direction::South {} => self.position.north -= val,
                    Direction::East {} => self.position.east += val,
                    Direction::West {} => self.position.east -= val
                }
            }
            Action::Turn { diff: val } => {
                self.turn(inst);
            }
            Action::ByDirection { direction, diff: val } =>         match direction {
                Direction::North {} => self.position.north += val,
                Direction::South {} => self.position.north -= val,
                Direction::East {} => self.position.east += val,
                Direction::West {} => self.position.east -= val
            },
            _ => panic!("Unexpected {:?}", inst)
        }
    }
    fn move_self(point: &mut Velocity, direction: Direction, val: i32) {
        match direction {
            Direction::North {} => point.north += val,
            Direction::South {} => point.north -= val,
            Direction::East {} => point.east += val,
            Direction::West {} => point.east -= val
        }
    }
    fn turn(&mut self, inst: Action) {
        const direction: [Direction; 4] = [Direction::North {}, Direction::East {}, Direction::South {}, Direction::West {}];
        match inst {
            Action::Turn { diff: 90 | -270 } => {
                match direction.iter().position(|&x| x == self.directiob).unwrap() + 1 {
                    x @ 0..=3 => self.directiob = direction[x],
                    _ => self.directiob = direction[0]
                }
            }
            Action::Turn { diff: -90 | 270 } => {
                self.directiob = direction[direction.iter().position(|&x| x == self.directiob).unwrap().checked_sub(1).unwrap_or(3)]
            }
            Action::Turn { diff: 180 | -180 } => {
                let mut index = direction.iter().position(|&x| x == self.directiob).unwrap();
                index = index.checked_sub(2).unwrap_or(index + 2);
                self.directiob = direction[index];
            }
            _ => panic!("Unexpected {:?}", inst)
        }
    }
}

fn rotate_around_ship(waypoint: &Velocity, angle: f64) -> Velocity {

    let (sin, cos) = angle.to_radians().sin_cos();
    let pos = Velocity {
        north: (cos * (waypoint.north as f64) - sin * (waypoint.east as f64)).round() as i32 ,
        east: (sin * (waypoint.north as f64) + cos * (waypoint.east as f64)).round() as i32 ,
    };
    let manhattan = |w1: &Velocity| -> usize {
        ((w1.north).abs() + (w1.east ).abs()) as usize
    };
    assert_eq!(manhattan(waypoint), manhattan(&pos), "Floating point error for Point:{:?}, With Angle: {}", waypoint, angle);
    pos
}

fn rotate_90(ship: &Position, waypoint: Position) -> Position {
    // Move to 0
    let waypoint_at_0 = Position {
        north: waypoint.north - ship.north,
        east: waypoint.east - ship.east,
    };
    let sin = std::f32::consts::FRAC_PI_2.sin() as i32;
    let cos = std::f32::consts::FRAC_PI_2.cos() as i32;
    Position {
        north: cos * waypoint_at_0.north + sin * waypoint_at_0.east + ship.north,
        east: sin * waypoint_at_0.north + cos * waypoint_at_0.east + ship.east,
    }
}

pub fn manhattan_distance(ship: &Ship) -> usize {
    (ship.position.north.abs() + ship.position.east.abs()) as usize
}

#[cfg(test)]
mod tests {
    use crate::ship::{Action, Direction, Ship, manhattan_distance, Position, rotate_around_ship, Velocity};
    use std::str::FromStr;

    #[test]
    fn from_example() {
        assert_eq!(Action::from_str("F10").unwrap(), Action::Forward { diff: 10 });
        assert_eq!(Action::from_str("N3").unwrap(), Action::ByDirection {
            direction: Direction::North {},
            diff: 3,
        });
        assert_eq!(Action::from_str("F7").unwrap(), Action::Forward { diff: 7 });
        assert_eq!(Action::from_str("R90").unwrap(), Action::Turn { diff: 90 });
        assert_eq!(Action::from_str("L90").unwrap(), Action::Turn { diff: -90 });
    }

    #[test]
    fn turning_test() {
        {
            let mut ship = Ship::default();
            ship.directiob = Direction::South {};
            ship.turn(Action::from_str("L180").unwrap());
            assert_eq!(ship.directiob, Direction::North {});
            ship.turn(Action::from_str("R180").unwrap());
            assert_eq!(ship.directiob, Direction::South {});
        }
        {
            let mut ship = Ship::default();
            ship.directiob = Direction::North {};
            ship.turn(Action::from_str("L180").unwrap());
            assert_eq!(ship.directiob, Direction::South {});
            ship.turn(Action::from_str("R180").unwrap());
            assert_eq!(ship.directiob, Direction::North {});
        }
        {
            let mut ship = Ship::default();
            assert_eq!(ship.directiob, Direction::East {});
            ship.turn(Action::from_str("L180").unwrap());
            assert_eq!(ship.directiob, Direction::West {});
            ship.turn(Action::from_str("R180").unwrap());
            assert_eq!(ship.directiob, Direction::East {});
        }
        {
            let mut ship = Ship::default();
            assert_eq!(ship.directiob, Direction::East {});
            ship.turn(Action::from_str("R90").unwrap());
            assert_eq!(ship.directiob, Direction::South {});
            ship.turn(Action::from_str("L90").unwrap());
            assert_eq!(ship.directiob, Direction::East {});
        }
        {
            let mut ship = Ship::default();
            assert_eq!(ship.directiob, Direction::East {});
            ship.turn(Action::from_str("R270").unwrap());
            assert_eq!(ship.directiob, Direction::North {});
            ship.turn(Action::from_str("R90").unwrap());
            assert_eq!(ship.directiob, Direction::East {});
        }
        {
            let mut ship = Ship::default();
            assert_eq!(ship.directiob, Direction::East {});

            ship.turn(Action::from_str("L270").unwrap());
            assert_eq!(ship.directiob, Direction::South {});
            ship.turn(Action::from_str("L90").unwrap());
            assert_eq!(ship.directiob, Direction::East {});
        }
    }

    #[test]
    fn move_by_string_input() {
        {
            let INPUT = r#"F10
N3
F7
R90
F11"#;
            let mut ship = Ship::default();
            INPUT.lines().for_each(|line|
                ship.move_part_1(Action::from_str(line).unwrap())
            );
            assert_eq!(ship.position.east, 17);
            assert_eq!(ship.position.north, -8);
            assert_eq!(manhattan_distance(&ship), 25)
        }
        {
            let check_if_pos = |ship: &Ship, north, east| {
                assert_eq!(ship.position.east, east);
                assert_eq!(ship.position.north, north);
            };
            let mut ship = Ship::default();
            ship.move_part_2(Action::from_str("F10").unwrap());
            check_if_pos(&ship,10,100);

            ship.move_part_2(Action::from_str("N3").unwrap());
            check_if_pos(&ship,10,100);


            assert_eq!(ship.waypoint.east, 10);
            assert_eq!(ship.waypoint.north, 4);

            ship.move_part_2(Action::from_str("F7").unwrap());
            check_if_pos(&ship,38,170);

            ship.move_part_2(Action::from_str("R90").unwrap());
            check_if_pos(&ship,38,170);

            ship.move_part_2(Action::from_str("F11").unwrap());
            check_if_pos(&ship,-72,214);

            assert_eq!(manhattan_distance(&ship), 286)
        }
        {
            let INPUT = r#"F10
N3
F7
R90
F11"#;
            let mut ship = Ship::default();
            INPUT.lines().for_each(|line|
                ship.move_part_2(Action::from_str(line).unwrap())
            );
            assert_eq!(ship.position.east, 214);
            assert_eq!(ship.position.north, -72);
            assert_eq!(manhattan_distance(&ship), 286)
        }
    }

    #[test]
    fn waypoint_rotation() {
        {
            let waypoint = Velocity { north: 4, east: 10 };
            assert_eq!(rotate_around_ship( &waypoint, 90f64), Velocity { north: -10, east: 4 });
        }
        {
            let waypoint = Velocity { north: 3, east: 5 };
            let mut rotated_waypoint = rotate_around_ship(&waypoint, 90f64);
            rotated_waypoint = rotate_around_ship(&rotated_waypoint, 90f64);
            rotated_waypoint = rotate_around_ship(&rotated_waypoint, 90f64);
            rotated_waypoint = rotate_around_ship(&rotated_waypoint, 90f64);
            rotated_waypoint = rotate_around_ship(&rotated_waypoint, 90f64);
            assert_eq!(rotate_around_ship(&waypoint, 90f64), rotated_waypoint);
        }
        {
            let waypoint = Velocity { north: 3, east: 5 };
            assert_eq!(rotate_around_ship(&waypoint, 90f64), Velocity { north: -5, east: 3 });
            assert_eq!(rotate_around_ship( &waypoint, 180f64), Velocity { north: -3, east: -5 });
            assert_eq!(rotate_around_ship( &waypoint, 270f64), Velocity { north: 5, east: -3 });
            assert_eq!(rotate_around_ship( &waypoint, 360f64), Velocity { north: 3, east: 5 });
        }
    }
}