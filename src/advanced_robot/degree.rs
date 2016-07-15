use std::ops::{ Add, Sub };
use std::cmp::Ordering;

pub struct Degree(f64);

impl Degree {
    pub fn new_default() -> Degree {
        Degree(0f64)
    }

    pub fn new(mut x: f64) -> Degree {
        if x > 360f64 { x %= 360f64; }
        if x < 0f64 { x = 360f64 + x % 360f64; }
        Degree(x)
    } 

    pub fn set(&mut self, x: f64) {
        self.0 = x;
    }

    pub fn to_rad(&self) -> f64 {
        use std::f64::consts::PI;
        static PI_180: f64 = PI / 180f64;
        self.0 * PI_180
    }
}

impl From<Degree> for f64 {
    fn from(x: Degree) -> f64 {
        x.0
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
        let mut retval = self.0 + rhs;
        if retval < 0f64 { retval += 360f64; }
        if retval > 360f64 { retval %= 360f64; }
        Degree(retval)
    }
}

impl Add<Degree> for Degree {
    type Output = Degree;

    fn add(self, rhs: Degree) -> Degree {
        let mut retval = self.0 + rhs.0;
        if retval < 0f64 { retval += 360f64; }
        if retval > 360f64 { retval %= 360f64; }
        Degree(retval)
    }
}

impl Sub<f64> for Degree {
    type Output = Degree;

    fn sub(self, rhs: f64) -> Degree {
        let mut retval = self.0 - rhs;
        if retval < 0f64 { retval += 360f64; }
        if retval > 360f64 { retval %= 360f64; }
        Degree(retval)
    }
}

impl Sub<Degree> for Degree {
    type Output = Degree;
    
    fn sub(self, rhs: Degree) -> Degree {
        let mut retval = self.0 - rhs.0;
        if retval < 0f64 { retval += 360f64; }
        if retval > 360f64 { retval %= 360f64; }
        Degree(retval)
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
            false
        }
    }
}

impl PartialOrd<f64> for Degree {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }

        fn lt(&self, other: &f64) -> bool {
        self.0.lt(other)
    }

    fn le(&self, other: &f64) -> bool {
        if self.eq(other) {
            true
        }
        else {
            self.lt(other)
        }
    }
    
    fn gt(&self, other: &f64) -> bool {
        self.0.gt(other)
    }

    fn ge(&self, other: &f64) -> bool {
        if self.0 == *other {
            true
        }
        else {
            self.gt(other)
        }
    }
}

impl PartialOrd<Degree> for Degree {
    fn partial_cmp(&self, other: &Degree) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    fn lt(&self, other: &Degree) -> bool {
        self.0.lt(&other.0)
    }

    fn le(&self, other: &Degree) -> bool {
        if self.eq(other) {
            true
        }
        else {
            self.lt(other)
        }
    }
    
    fn gt(&self, other: &Degree) -> bool {
        self.0.gt(&other.0)
    }

    fn ge(&self, other: &Degree) -> bool {
        if self.0 == other.0 {
            true
        }
        else {
            self.gt(other)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn equality() {
        let (mut x, y) = (Degree(0f64), Degree(0f64));
        assert!(x == y);
        x.set(360f64);
        assert!(x == y);
        assert!(Degree::new(720f64) == Degree::new(360f64));
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
        assert!(Degree::new(20f64) - Degree::new(15f64) == Degree::new(5f64), "1");
        assert!(Degree::new(0f64) - Degree::new(20f64) == Degree::new(340f64), "2");
    }

    #[test]
    #[allow(path_statements, unused_variables)]
    fn clone() {
        let x = Degree::new_default();
        let y = x;
        assert!(x == y);
    }

    #[test]
    fn from() {
        let x = Degree::new(20f64);
        assert!(f64::from(x) == 20f64);
    }

    #[test]
    fn ordinality() {
        let x = Degree::new(20f64);
        assert!(x > Degree::new_default() && x > 0f64);
        assert!(x < Degree::new(30f64) && x < 30f64);
        assert!(!(x < Degree::new(20f64)) && (x <= Degree::new(20f64)) && (x >= Degree::new(20f64)));
        assert!(x.partial_cmp(&20f64) == Option::Some(Ordering::Equal))
    }

    #[test]
    fn to_radian() {
        use std::f64::consts::PI;
        assert!(Degree::new(180f64).to_rad() == PI);
        assert!(Degree::new(90f64).to_rad() == PI / 2f64);
    }
}