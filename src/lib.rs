use std::string;

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
        let pos = &mut self.position;
        *pos = match self.direction {
            Direction::North => (pos.0, pos.1 + 1),
            Direction::South => (pos.0, pos.1 - 1),
            Direction::East  => (pos.0 + 1, pos.1),
            Direction::West  => (pos.0 - 1, pos.1),
        };
    }
}