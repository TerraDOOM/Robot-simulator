pub struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    pub fn new_default() -> Vector {
        Vector { x: 0.0_f64, y: 0.0_f64 }
    }

    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x: x, y: y }
    }

    pub fn k(&self) {
        
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        if (self.x == other.x) && (self.y == other.y) {
            true
        }
        else {
            false
        }
    }
}