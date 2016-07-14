use std::fmt;

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Direction::{}", match *self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        })
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        })
    }
}

impl Copy for Direction {}
impl Clone for Direction {
    fn clone(&self) -> Direction {
        *self
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        match *other {
            Direction::North => match *self {
                Direction::North => true,
                _ => false,
            },
            Direction::South => match *self {
                Direction::South => true,
                _ => false,
            },
            Direction::East => match *self {
                Direction::East => true,
                _ => false,
            },
            Direction::West => match *self {
                Direction::West => true,
                _ => false,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equality() {
        let north = Direction::North;
        let south = Direction::South;
        let east = Direction::East;
        let west = Direction::West;
        assert!(north == Direction::North);
        assert!(south == Direction::South);
        assert!(east == Direction::East);
        assert!(west == Direction::West);
    }

    #[test]
    fn copy() {
        #![allow(unused_variables, path_statements)]
        let x = Direction::North;
        let y = x;
        x;
    }
}