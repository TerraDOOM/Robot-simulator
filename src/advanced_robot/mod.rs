#![cfg_attr(not(release), allow(dead_code))]

mod vector;
mod degree;

use self::degree::*;
use self::vector::*;

struct AdvancedRobot {
    position: Vector,
    direction: Degree,
}

impl AdvancedRobot {
    pub fn new(x: f64, y: f64, direction: f64) -> AdvancedRobot {
        AdvancedRobot { position: Vector::new(x, y), direction: Degree::new(direction) }
    }

    pub fn new_default() -> AdvancedRobot {
        AdvancedRobot { position: Vector::new_default(), direction: Degree::new_default() }
    }
}
