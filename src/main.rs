mod robot;
use robot::*;

fn main() {
    let robot1 = Robot::new(5, 5, Direction::West);
    let mut robot2 = Robot::new_default();
    robot2.send_instructions("ARALARALARALARALARALL");
    println!("{}", robot2);
}