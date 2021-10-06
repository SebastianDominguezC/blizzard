use crate::complex::polar::ComplexPolarPoint;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct ComplexPoint {
    pub x: f64,
    pub y: f64,
}

impl ComplexPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn new_real(x: f64) -> Self {
        Self { x, y: 0.0 }
    }
    pub fn new_imaginary(y: f64) -> Self {
        Self { x: 0.0, y }
    }
    pub fn print(&self) {
        println!("{} + i {}", self.x, self.y);
    }
    pub fn conjugate(&self) -> Self {
        Self::new(self.x, -self.y)
    }
    pub fn module(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn module_conjugate(&self) -> f64 {
        let c = self.conjugate();
        (*self * c).x.sqrt()
    }
    pub fn inverse(&self) -> Self {
        self.conjugate() / (self.module() * self.module())
    }
    pub fn polar(&self) -> ComplexPolarPoint {
        let r = self.module();
        let t = (self.x / r).acos();
        ComplexPolarPoint::new(r, t)
    }
}

impl Add for ComplexPoint {
    type Output = ComplexPoint;
    fn add(self, other: ComplexPoint) -> ComplexPoint {
        let x = self.x + other.x;
        let y = self.y + other.y;
        ComplexPoint::new(x, y)
    }
}
impl Add<f64> for ComplexPoint {
    type Output = ComplexPoint;
    fn add(self, other: f64) -> ComplexPoint {
        let x = self.x + other;
        ComplexPoint::new(x, self.y)
    }
}

impl Sub for ComplexPoint {
    type Output = ComplexPoint;
    fn sub(self, other: ComplexPoint) -> ComplexPoint {
        let x = self.x - other.x;
        let y = self.y - other.y;
        ComplexPoint::new(x, y)
    }
}
impl Sub<f64> for ComplexPoint {
    type Output = ComplexPoint;
    fn sub(self, other: f64) -> ComplexPoint {
        let x = self.x - other;
        ComplexPoint::new(x, self.y)
    }
}

impl Mul for ComplexPoint {
    type Output = ComplexPoint;
    fn mul(self, other: ComplexPoint) -> ComplexPoint {
        let real_1 = self.x * other.x;
        let real_2 = -1.0 * self.y * other.y;
        let im_1 = self.x * other.y;
        let im_2 = other.x * self.y;
        let x = real_1 + real_2;
        let y = im_1 + im_2;
        ComplexPoint::new(x, y)
    }
}
impl Mul<f64> for ComplexPoint {
    type Output = ComplexPoint;
    fn mul(self, n: f64) -> ComplexPoint {
        let x = self.x * n;
        let y = self.y * n;
        ComplexPoint::new(x, y)
    }
}

impl Div for ComplexPoint {
    type Output = ComplexPoint;
    fn div(self, other: ComplexPoint) -> ComplexPoint {
        let denom = other.module() * other.module();
        let numer = self * other.conjugate();
        let x = numer.x / denom;
        let y = numer.y / denom;
        ComplexPoint::new(x, y)
    }
}
impl Div<f64> for ComplexPoint {
    type Output = ComplexPoint;
    fn div(self, n: f64) -> ComplexPoint {
        let x = self.x / n;
        let y = self.y / n;
        ComplexPoint::new(x, y)
    }
}
