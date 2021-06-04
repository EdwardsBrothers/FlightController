#![allow(dead_code)]

use std::fmt;

#[derive(Debug)]
struct Quaternion {
    r: f64,
    u: f64,
    v: f64,
    w: f64,
}

impl Quaternion {
    pub fn new(r:f64, u:f64, v:f64, w:f64) -> Self {
        Self {r,u,v,w}
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
}
