#![allow(dead_code)]

type Scalar = f64;

use std::{fmt, ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign}};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Quaternion { s: Scalar, x: Scalar, y: Scalar, z: Scalar }

impl Quaternion {
    pub fn new(s:Scalar, x:Scalar, y:Scalar, z:Scalar) -> Self {
        Self {s,x,y,z}
    }

    #[inline]
    fn conj(self) -> Self {
        Self { s: self.s, x: -self.x, y: -self.y, z: -self.z }
    } 

    #[inline]
    fn normsq(self) -> Scalar {
        self.s*self.s + self.x*self.x + self.y*self.y + self.z*self.z
    }

    #[inline]
    fn norm(self) -> Scalar {
        self.normsq().sqrt()
    }

    #[inline]
    fn dot(self, rhs: Quaternion) -> Scalar {
        self.s*rhs.s + self.x*rhs.x + self.y*rhs.y + self.z*rhs.z 
    }
    
    #[inline]
    fn inv(self) -> Self {
        let n = self.normsq();
        Self { s: self.s/n, x: -self.x/n, y: -self.y/n, z: -self.z/n }
    }

    fn exp(self) -> Self {
        let ph = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
        let q0 = (self.s).exp();
        let qs = q0*(ph+Scalar::EPSILON).sin()/(ph+Scalar::EPSILON);
        Self {s: q0*ph.cos(), x: qs*self.x, y: qs*self.y,  z: qs*self.z}
    }

    fn ln(self) -> Self {
        let phsq = self.x*self.x + self.y*self.y + self.z*self.z;
        let q0 = (self.s*self.s + phsq).sqrt();
        if phsq <= 0.0 {
            Self {s: q0.ln(), x: 0.0, y: 0.0,  z: 0.0}
        } else {
            let qs = (self.s/q0).acos()/phsq.sqrt();
            Self {s: q0.ln(), x: qs*self.x, y: qs*self.y,  z: qs*self.z}    
        }
    }

    fn pow(self, x: Scalar) -> Self {
        let phsq = self.x*self.x + self.y*self.y + self.z*self.z;
        let ph = phsq.sqrt();
        let qx = (self.s*self.s + phsq).sqrt().powf(x);
        if phsq <= 0.0 {
            Self {s: qx*(x*ph).cos(), x: 0.0, y: 0.0,  z: 0.0}
        } else {
            let qs = qx*(x*ph).sin()/ph;
            Self {s: qx*(x*ph).cos(), x: qs*self.x, y: qs*self.y,  z: qs*self.z}
        }
    }
}


// Addition

impl Add for Quaternion {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self { s: self.s + rhs.s, x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Add<Scalar> for Quaternion {
    type Output = Self;
    #[inline]
    fn add(self, rhs:Scalar) -> Self {
        Self { s: self.s + rhs, x: self.x, y: self.y, z: self.z }        
    }
}

impl Add<Quaternion> for Scalar {
    type Output = Quaternion;
    #[inline]
    fn add(self, rhs:Quaternion) -> Quaternion {
        Quaternion { s: self + rhs.s, x: rhs.x, y: rhs.y, z: rhs.z }        
    }
}

impl AddAssign for Quaternion {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<Scalar> for Quaternion {
    fn add_assign(&mut self, rhs: Scalar) {
        *self = *self + rhs;
    }
}


// Subtraction

impl Neg for Quaternion {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self { s: -self.s, x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Sub for Quaternion {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self { s: self.s - rhs.s, x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Sub<Scalar> for Quaternion {
    type Output = Self;
    #[inline]
    fn sub(self, rhs:Scalar) -> Self {
        Self { s: self.s - rhs, x: self.x, y: self.y, z: self.z }        
    }
}

impl Sub<Quaternion> for Scalar {
    type Output = Quaternion;
    #[inline]
    fn sub(self, rhs:Quaternion) -> Quaternion {
        Quaternion { s: self - rhs.s, x: -rhs.x, y: -rhs.y, z: -rhs.z }        
    }
}

impl SubAssign for Quaternion {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<Scalar> for Quaternion {
    fn sub_assign(&mut self, rhs: Scalar) {
        *self = *self - rhs;
    }
}


// Multiplication

impl Mul for Quaternion {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            s: self.s*rhs.s - self.x*rhs.x - self.y*rhs.y - self.z*rhs.z, 
            x: self.s*rhs.x + self.x*rhs.s + self.y*rhs.z - self.z*rhs.y, 
            y: self.s*rhs.y + self.y*rhs.s + self.z*rhs.x - self.x*rhs.z, 
            z: self.s*rhs.z + self.z*rhs.s + self.x*rhs.y - self.y*rhs.x, 
        }
    }
}

impl Mul<Scalar> for Quaternion {
    type Output = Self;
    #[inline]
    fn mul(self, rhs:Scalar) -> Self {
        Self { s: self.s * rhs, x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }        
    }
}

impl Mul<Quaternion> for Scalar {
    type Output = Quaternion;
    #[inline]
    fn mul(self, rhs:Quaternion) -> Quaternion {
        Quaternion { s: self * rhs.s, x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }        
    }
}

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<Scalar> for Quaternion {
    fn mul_assign(&mut self, rhs: Scalar) {
        *self = *self * rhs;
    }
}


// Division

impl Div<Scalar> for Quaternion {
    type Output = Self;
    #[inline]
    fn div(self, rhs:Scalar) -> Self {
        Self { s: self.s / rhs, x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }        
    }
}

impl DivAssign<Scalar> for Quaternion {
    fn div_assign(&mut self, rhs: Scalar) {
        *self = *self / rhs;
    }
}


// Display 
impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i + {}j + {}k", self.s, self.x, self.y, self.z)
    }
}

// impl fmt::LowerExp for Quaternion {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
//         let q = *self/self.norm();
//         write!(f, "|{}|, phi: {}, n: {}i {}j {}k", self.norm(), )
//     }
// }


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
        // assert_eq!(((q*p).norm()-q.norm()*p.norm()),0.0); // (3.3)
        let result = ((q*p).norm()-q.norm()*p.norm()).abs();
        let tol = 1e-14;
        assert!(result<tol, "{:e}<{:e}",result,tol);
    }

    #[test]
    fn test_dot() {
        let p = Quaternion::new(1.0, -2.0, -3.0, 1.0);
        let q = Quaternion::new(2.0, -2.0, 1.0, -3.0);
        assert_eq!(p.dot(q), 0.0);
    }

    #[test]
    fn test_inv() {
        let p = Quaternion::new(1.0, -2.0, 3.0, 4.0);
        assert_eq!(p*p.inv(), Quaternion::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(p.inv()*p, Quaternion::new(1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_exp() {
        let one: Scalar = 1.0;
        assert_eq!(Quaternion::new(0.0, 0.0, 0.0, 0.0).exp(), Quaternion{s: 1.0, x: 0.0, y: 0.0, z: 0.0});
        assert_eq!(Quaternion::new(1.0, 0.0, 0.0, 0.0).exp(), Quaternion{s: one.exp(), x: 0.0, y: 0.0, z: 0.0});
    }

    #[test]
    fn test_ln() {
        let one: Scalar = 1.0;
        let pi_2 = 2.0*one.atan();
        assert_eq!(Quaternion::new(0.0, 0.0, 0.0, 0.0).ln(), Quaternion{s: Scalar::NEG_INFINITY, x: 0.0, y: 0.0, z: 0.0});
        assert_eq!(Quaternion::new(1.0, 0.0, 0.0, 0.0).ln(), Quaternion{s: 0.0, x: 0.0, y: 0.0, z: 0.0});
        assert_eq!(Quaternion::new(0.0, 1.0, 0.0, 0.0).ln(), Quaternion{s: 0.0, x: pi_2, y: 0.0, z: 0.0});
        assert_eq!(Quaternion::new(0.0, 0.0, 1.0, 0.0).ln(), Quaternion{s: 0.0, x: 0.0, y: pi_2, z: 0.0});
        assert_eq!(Quaternion::new(0.0, 0.0, 0.0, 1.0).ln(), Quaternion{s: 0.0, x: 0.0, y: 0.0, z: pi_2});
    }

    #[test]
    fn test_pow() {
        assert_eq!(Quaternion::new(1.0, 0.0, 0.0, 0.0).pow(0.0), Quaternion::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(Quaternion::new(1.0, 0.0, 0.0, 0.0).pow(1.0), Quaternion::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(Quaternion::new(2.0, 0.0, 0.0, 0.0).pow(1.0), Quaternion::new(2.0, 0.0, 0.0, 0.0));
        assert_eq!(Quaternion::new(2.0, 0.0, 0.0, 0.0).pow(2.0), Quaternion::new(4.0, 0.0, 0.0, 0.0));

        assert_eq!(Quaternion::new(1.0, 2.0, 3.0, 4.0).pow(0.0), Quaternion::new(1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_add() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(4.0, 3.0, 2.0, 1.0);
        let q3 = Quaternion::new(5.0, 5.0, 5.0, 5.0);
        let mut q0 = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        q0 += q1;
        q0 += 3.0;
        assert_eq!(q0, Quaternion::new(4.0, 2.0, 3.0, 4.0));
        assert_eq!(q1+3.0, Quaternion::new(4.0, 2.0, 3.0, 4.0));  // scalar addition right
        assert_eq!(3.0+q1, Quaternion::new(4.0, 2.0, 3.0, 4.0));  // scalar addition left
        assert_eq!(q1+q2, q3);
        assert_eq!(q1+q2, q2+q1);
        assert_eq!(q1+(q2+q3), (q1+q2)+q3);
    }

    
    #[test]
    fn test_neg() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(-1.0, -2.0, -3.0, -4.0);
        assert_eq!(-q1, q2);
    }

    #[test]
    fn test_sub() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(4.0, 3.0, 2.0, 1.0);
        let q3 = Quaternion::new(-3.0, -1.0, 1.0, 3.0);
        let mut q0 = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        q0 -= q1;
        q0 -= 3.0;
        assert_eq!(q0, Quaternion::new(-4.0, -2.0, -3.0, -4.0));
        assert_eq!(q1-3.0, Quaternion::new(-2.0, 2.0, 3.0, 4.0));  // scalar addition right
        assert_eq!(3.0-q1, Quaternion::new(2.0, -2.0, -3.0, -4.0));  // scalar addition left
        assert_eq!(q1-q2, q3);
        assert_eq!(q1-q2, -(q2-q1));
        assert_eq!(q1-(q2-q3), (q1-q2)+q3);
    }


    #[test]
    fn test_mul() {
        let i = Quaternion::new(0.0, 1.0, 0.0, 0.0);
        let j = Quaternion::new(0.0, 0.0, 1.0, 0.0);
        let k = Quaternion::new(0.0, 0.0, 0.0, 1.0);
        let e = Quaternion::new(-1.0,0.0,0.0,0.0);
        assert_eq!(i*j*k, e);
        assert_eq!(i*i, e);
        assert_eq!(j*j, e);
        assert_eq!(k*k, e);
        assert_eq!(i*j, k);
        assert_eq!(j*i,-k);
        assert_eq!(j*k, i);
        assert_eq!(k*j,-i);
        assert_eq!(k*i, j);
        assert_eq!(i*k,-j);
    }

    #[test]
    fn test_mulscalar() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let s = -1.4;
        assert_eq!(q1*s, s*q1);
    }

    #[test]
    fn test_divscalar() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let s = q1.norm();
        assert!(((q1/s).norm()-1.0).abs() < 1e-14);
    }


    #[test]
    fn test_debug() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let result = format!("{:?}", q);
        let expect = "Quaternion { s: 1.0, x: 2.0, y: 3.0, z: 4.0 }";
        assert_eq!(expect,result);
    }

    // #[test]
    // fn test_display() {
    //     let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    //     let result = format!("{}", q);
    //     let expect = "1 + 2i + 3j + 4k";
    //     assert_eq!(expect,result);
    // }

    // #[test]
    // fn test_display_exp() {
    //     let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    //     let result = format!("{:e}", q);
    //     let expect = "1 + 2i + 3j + 4k";
    //     assert_eq!(expect,result);
    // }

}
