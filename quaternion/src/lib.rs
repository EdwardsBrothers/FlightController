#![allow(dead_code)]

use std::{fmt, ops::{Add, Sub}};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Quaternion { r: f64, u: f64, v: f64, w: f64 }


impl Add for Quaternion {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self{r: self.r+other.r, u: self.u+other.u, v: self.v+other.v, w: self.w+other.w}
    }
}


impl Sub for Quaternion {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self{r: self.r-other.r, u: self.u-other.u, v: self.v-other.v, w: self.w-other.w}
    }
}


impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i + {}j + {}k", self.r, self.u, self.v, self.w)
    }
}

impl fmt::LowerExp for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i + {}j + {}k", self.r, self.u, self.v, self.w)
    }
}

impl Quaternion {
    pub fn new(r:f64, u:f64, v:f64, w:f64) -> Self {
        Self {r,u,v,w}
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q.r, 1.0);
        assert_eq!(q.u, 2.0);
        assert_eq!(q.v, 3.0);
        assert_eq!(q.w, 4.0);
    }

    #[test]
    fn test_debug() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let result = format!("{:?}", q);
        let expect = "Quaternion { r: 1.0, u: 2.0, v: 3.0, w: 4.0 }";
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
