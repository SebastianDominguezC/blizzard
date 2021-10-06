use crate::complex::cartesian::ComplexPoint;
use std::ops::{Div, Mul};

#[derive(Clone, Copy)]
pub struct ComplexPolarPoint {
    pub r: f64,
    pub t: f64,
}

impl ComplexPolarPoint {
    pub fn new(r: f64, t: f64) -> Self {
        Self { r, t }
    }
    pub fn print(&self) {
        println!("{}e ^ i{}", self.r, self.t);
    }
    pub fn inverse(&self) -> Self {
        Self {
            r: self.r,
            t: -self.t,
        }
    }
    pub fn pow(&self, n: f64) -> Self {
        let r = self.r.powf(n);
        let t = self.t * n;
        Self { r, t }
    }
    pub fn cartesian(&self) -> ComplexPoint {
        let x = self.r * self.t.cos();
        let y = self.r * self.t.sin();
        ComplexPoint::new(x, y)
    }
}

impl Mul for ComplexPolarPoint {
    type Output = ComplexPolarPoint;
    fn mul(self, other: ComplexPolarPoint) -> ComplexPolarPoint {
        let r = self.r * other.r;
        let t = self.t + other.t;
        ComplexPolarPoint::new(r, t)
    }
}
impl Div for ComplexPolarPoint {
    type Output = ComplexPolarPoint;
    fn div(self, other: ComplexPolarPoint) -> ComplexPolarPoint {
        self * other.inverse()
    }
}
