use std::ops::{ Add, Sub };

pub struct Degree(f64);

impl Degree {
    pub fn new_default() -> Degree {
        Degree(0f64)
    }

    pub fn new(x: f64) -> Degree {
        // TODO: FIX THIS
        Degree(if > 360f64 => x % 360f64,
            < 0f64 => 360f64 + x % 360f64,
         })
    } 

    pub fn set(&mut self, x: f64) {
        self.0 = x;
    }
}

impl Into<f64> for Degree {
    fn into(self) -> f64 {
        self.0
    }
}

impl Copy for Degree {}
impl Clone for Degree {
    fn clone(&self) -> Degree {
        Degree::new(self.0)
    }
}

impl Add<f64> for Degree {
    type Output = Degree;

    fn add(self, rhs: f64) -> Degree {
        let mut retval = Degree(self.0 + rhs);
        if retval.0 < 0f64 {
            retval.0 = 360f64 + retval.0;
        }
        if retval.0 > 360f64 {
            retval.0 %= 360f64;
        }
        retval
    }
}

impl Add<Degree> for Degree {
    type Output = Degree;

    fn add(self, rhs: Degree) -> Degree {
        let mut retval = Degree(self.0 + rhs.0);
        if retval.0 < 0f64 {
            retval.0 = 360f64 + retval.0;
        }
        if retval.0 > 360f64 {
            retval.0 %= 360f64;
        }
        retval
    }
}

impl Sub<f64> for Degree {
    type Output = Degree;

    fn sub(self, rhs: f64) -> Degree {
        self + Degree(-(rhs))
    }
}

impl Sub<Degree> for Degree {
    type Output = Degree;
    
    fn sub(self, rhs: Degree) -> Degree {
        self + Degree(-(rhs.0))
    }
}

impl PartialEq<Degree> for Degree {
    fn eq(&self, other: &Degree) -> bool {
        if self.0 == other.0 {
            true
        }
        else {
            match self.0 {
                360f64 | 0f64 => match other.0 {
                    360f64 | 0f64 => true,
                    _ => false, 
                },
                _ => false,
            }
        }
    }
}

impl PartialEq<f64> for Degree {
    fn eq(&self, other: &f64) -> bool {
        if self.0 == *other {
            true
        }
        else {
            match self.0 {
                360f64 | 0f64 => match *other {
                    360f64 | 0f64 => true,
                    _ => false, 
                },
                _ => false,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equality() {
        let (mut x, y) = (Degree(0f64), Degree(0f64));
        assert!(x == y);
        x.set(360f64);
        assert!(x == y);
    }

    #[test]
    fn inequality() {
        let (x, y) = (Degree(1f64), Degree(0f64));
        assert!(x != y);
    }

    #[test]
    fn addition() {
        let (mut x, y) = (Degree(5f64), Degree(5f64));
        assert!(x + y == Degree(10f64));
        x.set(360f64);
        assert!(x + y == 5f64);
    }

    #[test]
    fn subtraction() {
        assert!(Degree(20f64) - Degree(15f64) == Degree(5f64), "1");
        assert!(Degree(0f64) - Degree(20f64) == Degree(340f64), "2");
        assert!(Degree(720f64) == Degree(360f64), "3");
    }
}