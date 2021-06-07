#![allow(dead_code)]

use std::{fmt, ops::{Neg, Add, Sub, Mul, AddAssign, SubAssign, MulAssign}};


#[derive(Debug, Copy, Clone, PartialEq)]
struct Quaternion { s: f64, x: f64, y: f64, z: f64 }

impl Quaternion {
    pub fn new(s:f64, x:f64, y:f64, z:f64) -> Self {
        Self {s,x,y,z}
    }

    fn conj(self) -> Self {
        Self { s: self.s, x: -self.x, y: -self.y, z: -self.z }
    } 

    fn normsq(self) -> f64 {
        self.s*self.s + self.x*self.x + self.y*self.y + self.z*self.z
    }

    fn norm(self) -> f64 {
        self.normsq().sqrt()
    }
    
    fn inv(self) -> Self {
        let n = self.normsq();
        Self { s: self.s/n, x: -self.x/n, y: -self.y/n, z: -self.z/n }
    }
}


// Operators

impl Neg for Quaternion {
    type Output = Self;
    fn neg(self) -> Self {
        Self { s: -self.s, x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add for Quaternion {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { s: self.s + rhs.s, x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for Quaternion {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {s: self.s - rhs.s, x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl Mul for Quaternion {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            s: self.s*rhs.s - self.x*rhs.x - self.y*rhs.y - self.z*rhs.z, 
            x: self.s*rhs.x + self.x*rhs.s + self.y*rhs.z - self.z*rhs.y, 
            y: self.s*rhs.y + self.y*rhs.s + self.z*rhs.x - self.x*rhs.z, 
            z: self.s*rhs.z + self.z*rhs.s + self.x*rhs.y - self.y*rhs.x, 
        }
    }
}


// Assignment operators
impl AddAssign for Quaternion {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Quaternion {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}


// Display 
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
    fn test_neg() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(-1.0, -2.0, -3.0, -4.0);
        assert_eq!(-q1, q2);
    }

    #[test]
    fn test_conj() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let p = Quaternion::new(-1.3, -2.1, 8.0, -4.0);
        assert_eq!(q.conj(), Quaternion::new(1.0, -2.0, -3.0, -4.0));
        assert_eq!((q.conj()).conj(), q);
        assert_eq!((p*q).conj(), q.conj()*p.conj());
        assert_eq!((p+q).conj(), p.conj()+q.conj());
        assert_eq!(q*q.conj(), q.conj()*q);
    }

    #[test]
    fn test_norm() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let p = Quaternion::new(-2.0, -2.0, 7.0, -4.0);
        assert_eq!(q.norm(), (q.s*q.s + q.x*q.x + q.y*q.y + q.z*q.z).sqrt()); // (3.1)
        assert_eq!((q.conj()).norm(), q.norm()); // (3.2)
        assert!(((q*p).norm()-q.norm()*p.norm()).abs()<1e-14); // (3.3)
    }

    #[test]
    fn test_add() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(4.0, 3.0, 2.0, 1.0);
        let q3 = Quaternion::new(5.0, 5.0, 5.0, 5.0);
        assert_eq!(q1+q2, q3);
        assert_eq!(q1+q2, q2+q1);
        assert_eq!(q1+(q2+q3), (q1+q2)+q3);
    }

    #[test]
    fn test_sub() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(4.0, 3.0, 2.0, 1.0);
        let q3 = Quaternion::new(-3.0, -1.0, 1.0, 3.0);
        assert_eq!(q1-q2, q3);
    }

    #[test]
    fn test_mul() {
        let i = Quaternion::new(0.0, 1.0, 0.0, 0.0);
        let j = Quaternion::new(0.0, 0.0, 1.0, 0.0);
        let k = Quaternion::new(0.0, 0.0, 0.0, 1.0);
        let e = Quaternion::new(-1.0,0.0,0.0,0.0);
        assert_eq!(i*i, e);
        assert_eq!(j*j, e);
        assert_eq!(k*k, e);
        assert_eq!(i*j*k, e);
        assert_eq!(i*j, k);
        assert_eq!(j*i, -k);
    }

    #[test]
    fn test_inv() {
        let p = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(p*p.inv(), Quaternion::new(1.0, 0.0, 0.0, 0.0));
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

}
