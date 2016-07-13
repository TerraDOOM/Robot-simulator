#![allow(unused_variables)]
#![allow(path_statements)]

mod direction {
    use super::super::*;

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
        let x = Direction::North;
        let y = x;
        x;
    }
}

mod robot {
    use super::super::*;

    // Test if the default constructor works
    #[test]
    fn default_values() {
        let robot = Robot::new_default();
        assert_eq!((0, 0), robot.position());
        assert_eq!(Direction::North, robot.direction());
    }

    // Test if non-default constructor works
    #[test]
    fn non_default_values() {
        let robot = Robot::new(1, 1, Direction::West);
        assert_eq!((1, 1), robot.position());
        assert_eq!(Direction::West, robot.direction());
    }

    // Test if Robots can have negative position
    #[test]
    fn negative_position() {
        let robot = Robot::new(-1, -1, Direction::North);
        assert_eq!(robot.position(), (-1, -1));
    }
    
    #[test]
    fn equality() {
        let robot1 = Robot::new_default();
        let robot2 = Robot::new_default();
        assert!(robot1 == robot2);
    }

    #[test]
    fn instructions() {
        let robot1 = Robot::new(5, 5, Direction::West);
        let mut robot2 = Robot::new_default();
        robot2.send_instructions("ARALARALARALARALARALL");
        assert!(robot1 == robot2);
    }

    #[test]
    fn multiple_instructions() {
        let robot1 = Robot::new(5, 5, Direction::West);
        let mut robot2 = Robot::new_default();
        robot2.send_instructions("ARALARALAR");
        robot2.send_instructions("ALARALARALL");
        assert!(robot1 == robot2);
    }

    #[test]
    fn wrong_instructions_noerr() {
        let mut robot = Robot::new_default();
        robot.send_instructions("x");
    }

    #[test]
    #[should_panic]
    fn wrong_instructions_err() {
        let mut robot = Robot::new_default();
        robot.send_instructions_err("x");
    }

    #[test]
    fn instructions_case_insensitive() {
        // same as instructions() but with mixed case
        let robot1 = Robot::new(5, 5, Direction::West);
        let mut robot2 = Robot::new_default();
        robot2.send_instructions("araLARAlaRAlaralarALL");
        assert!(robot1 == robot2);
    }
}