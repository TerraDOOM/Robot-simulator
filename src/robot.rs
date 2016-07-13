use std::fmt;
use std::io;

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

// Point struct for position
struct Point {
    x: i32,
    y: i32,
}

pub struct Robot {
    position: Point,
    direction: Direction,
}

impl Robot {
    #![allow(dead_code)]
    // Method for creating a default Robot
    pub fn new_default() -> Robot {
        Robot {
            position: Point { x: 0, y: 0 },
            direction: Direction::North,
        }
    }

    // Method for creating a new robot with designated values
    pub fn new(x: i32, y: i32, initial_direction: Direction) -> Robot {
        Robot {
            position: Point { x: x, y: y },
            direction: initial_direction,
        }
    }

    // Get direction
    pub fn direction(&self) -> Direction {
        self.direction
    }

    // Get position
    pub fn position(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        
        self.position.x = x;
        self.position.y = y;
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        };
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        };
    }

    pub fn advance(&mut self) {
        match self.direction {
            Direction::North => self.position.y += 1,
            Direction::South => self.position.y -= 1, 
            Direction::East => self.position.x += 1,
            Direction::West => self.position.x -= 1,
        };
    }

    pub fn send_instructions_err(&mut self, instr: &str) {
        let instr_str = instr.to_string().to_uppercase();
        for i in instr_str.chars() {
            match i {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _ => panic!("Unknown robot instruction: {}", i),
            };
        }
    }

    pub fn send_instructions_log(&mut self, instr: &str) {
        let instr_str = instr.to_string().to_uppercase();
        for i in instr_str.chars() {
            match i {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _ => {
                    use std::io::Write;
                    let err = writeln!(&mut io::stderr(), "Unknown robot instruction: {}", i);
                    err.expect("Failed printing to stderr");
                }
            };
        }
    }

    pub fn send_instructions(&mut self, instr: &str) {
        let instr_str = instr.to_string().to_uppercase();
        for i in instr_str.chars() {
            match i {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _ => {}
            };
        }
    }
}

impl PartialEq for Robot {
    fn eq(&self, other: &Robot) -> bool {
        if (self.position() == other.position()) && (self.direction == other.direction) {
            true
        }
        else {
            false
        }
    }
}

impl fmt::Debug for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Robot::new({}, {}, {:?})", self.position().0, self.position().1, self.direction)
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Robot at ({}, {}) facing {})", self.position().0, self.position().1, self.direction)
    }
}
