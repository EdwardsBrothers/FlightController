#![allow(dead_code)]

use std::{fmt, ops::{Add, Sub}};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Quaternion { s: f64, x: f64, y: f64, z: f64 }


impl Add for Quaternion {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self{s: self.s+other.s, x: self.x+other.x, y: self.y+other.y, z: self.z+other.z}
    }
}


impl Sub for Quaternion {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self{s: self.s-other.s, x: self.x-other.x, y: self.y-other.y, z: self.z-other.z}
    }
}


impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i + {}j + {}k", self.s, self.x, self.y, self.z)
    }
}

impl fmt::LowerExp for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i + {}j + {}k", self.s, self.x, self.y, self.z)
    }
}

impl Quaternion {
    pub fn new(s:f64, x:f64, y:f64, z:f64) -> Self {
        Self {s,x,y,z}
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q.s, 1.0);
        assert_eq!(q.x, 2.0);
        assert_eq!(q.y, 3.0);
        assert_eq!(q.z, 4.0);
    }

    #[test]
    fn test_debug() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let result = format!("{:?}", q);
        let expect = "Quaternion { s: 1.0, x: 2.0, y: 3.0, z: 4.0 }";
        assert_eq!(expect,result);
    }

    #[test]
    fn test_display() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let result = format!("{}", q);
        let expect = "1 + 2i + 3j + 4k";
        assert_eq!(expect,result);
    }

    #[test]
    fn test_display_exp() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let result = format!("{:e}", q);
        let expect = "1 + 2i + 3j + 4k";
        assert_eq!(expect,result);
    }


    #[test]
    fn test_add() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(4.0, 3.0, 2.0, 1.0);
        let q3 = Quaternion::new(5.0, 5.0, 5.0, 5.0);
        assert_eq!(q1+q2, q3);

    }
}
