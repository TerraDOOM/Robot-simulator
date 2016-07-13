use std::string;
use std::io;

mod robot_simulator {
    pub enum Direction {
        North,
        South,
        East,
        West,
    }

    pub struct Robot {
        position: (i32, i32),
        direction: Direction,
    }

    impl Robot {
        fn new(x: i32, y: i32, initialDirection: Direction) -> Robot {
            Robot { position: (x, y), direction: initialDirection }
        }

        fn direction(&self) -> &Direction {
            &self.direction
        }

        fn position(&self) -> (i32, i32) {
            self.position
        }

        fn turn_right(&mut self) -> () {
            self.direction = match self.direction {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East  => Direction::South,
                Direction::West  => Direction::North
            };
        }

    fn turn_left(&mut self) -> () {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East  => Direction::North,
            Direction::West  => Direction::South
        };
    }

    fn advance(&mut self) -> () {
        self.position = match self.direction {
            Direction::North => (self.position.0, self.position.1 + 1),
            Direction::South => (self.position.0, self.position.1 - 1),
            Direction::East  => (self.position.0 + 1, self.position.1),
            Direction::West  => (self.position.0 - 1, self.position.1),
        };
    }

    fn get_instructions_err(&self, instr: &str) -> () {
        let instr_str = instr.to_string().to_uppercase();
        for i in &instr_str.chars() {
            match i {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _   => panic!("Unknown robot instruction: {}", i),
            };
        }
    }

    fn get_instructions_log(&self, instr: &str) -> () {
        let instr_str = instr.to_string().to_uppercase();
        for i in &instr_str.chars() {
            match i {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _   => {
                         let err = io::Write(&mut io::stderr(), "Unknown robot instruction: {}", i);
                         err.expect("Failed printing to stderr");
                       },
            };
        }
    }
    
    fn get_instructions(&self, instr: &str) -> () {
        let instr_str = instr.to_string().to_uppercase();
        for i in &instr_str.chars() {
            match i {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _   => {},
            };
        }
    }
}