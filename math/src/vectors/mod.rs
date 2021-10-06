use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// 3 dimensional vector implementation
/// # New vector
#[derive(Debug, Clone)]
pub struct Vector {
    pub components: Vec<f64>,
    pub dimension: usize,
}

impl Vector {
    /// Creates a new Vector from a Vec
    pub fn new(components: Vec<f64>) -> Vector {
        Vector {
            dimension: components.len() as usize,
            components,
        }
    }

    /// Creates a zero vector in a specific dimension
    pub fn zero(dimension: u32) -> Vector {
        let mut components = vec![];
        for _ in 0..dimension {
            components.push(0.0);
        }
        Vector::new(components)
    }

    /// Calculates dot product between two vectors
    pub fn dot_product(vec1: &Vector, vec2: &Vector) -> Option<f64> {
        if Vector::equal_dimensions(&vec1, &vec2) {
            let mut sum: f64 = 0.0;
            for i in 0..vec1.dimension {
                sum += vec1.components[i] * vec2.components[i];
            }
            return Some(sum);
        }
        None
    }

    /// Calculates cross product ONLY for 3 dimensional vectors
    /// If the vectors are not same dimension or if the dimensions are not == 3, None is returned
    pub fn cross_product(vec1: &Vector, vec2: &Vector) -> Option<Vector> {
        if Vector::equal_dimensions(&vec1, &vec2) && vec1.dimension == 3 {
            let mut vector = Vector::zero(vec1.dimension as u32);
            vector.components[0] =
                vec1.components[1] * vec2.components[2] - vec1.components[2] * vec2.components[1];
            vector.components[1] =
                vec1.components[2] * vec2.components[0] - vec1.components[0] * vec2.components[2];
            vector.components[2] =
                vec1.components[0] * vec2.components[1] - vec1.components[1] * vec2.components[0];
            return Some(vector);
        }
        return None;
    }

    /// Checks if two vectors are in the same dimension
    pub fn equal_dimensions(vec1: &Vector, vec2: &Vector) -> bool {
        vec1.dimension == vec2.dimension
    }

    /// Get's orthogonal proyection
    pub fn orthogonal_proyection(vec1: &Vector, vec2: &Vector) -> Vector {
        let numerator =
            Vector::dot_product(vec1, vec2).expect("Vectors don't have equal dimensions");
        let denominator =
            Vector::dot_product(vec2, vec2).expect("Vectors don't have equal dimensions");
        let proyection = vec2 * (numerator / denominator);
        proyection
    }

    /// Get's orthogonal component, uses orthogonal proyection
    pub fn orthogonal_component(vec1: &Vector, vec2: &Vector) -> Vector {
        let component = vec1 - &Vector::orthogonal_proyection(vec1, vec2);
        component
    }

    /// Copy a vector
    pub fn copy(&self) -> Vector {
        let mut vec = Vector::zero(self.dimension as u32);
        for i in 0..self.dimension {
            vec.components[i] = self.components[i]
        }
        vec
    }

    /// Gets the magnitude of the vector and returns it
    pub fn get_magnitude(&self) -> f64 {
        let mut addition = 0.0;
        for c in self.components.iter() {
            addition += c * c;
        }
        addition.sqrt()
    }

    /// Returns the normalized vector
    pub fn normalize(&self) -> Vector {
        let mut v = Vector::zero(self.dimension as u32);
        let magnitude = self.get_magnitude();
        for i in 0..self.dimension {
            v.components[i] = self.components[i] / magnitude;
        }
        v
    }
}

// Operator overrides
impl Add<&Vector> for &Vector {
    type Output = Vector;
    fn add(self, other: &Vector) -> Vector {
        let mut v = Vector::zero(self.dimension as u32);
        for i in 0..self.dimension {
            v.components[i] = self.components[i] + other.components[i];
        }
        v
    }
}
impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, other: &Vector) {
        for i in 0..self.dimension {
            self.components[i] += other.components[i]
        }
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;
    fn sub(self, other: &Vector) -> Vector {
        let mut v = Vector::zero(self.dimension as u32);
        for i in 0..self.dimension {
            v.components[i] = self.components[i] - other.components[i];
        }
        v
    }
}
impl SubAssign<&Vector> for Vector {
    fn sub_assign(&mut self, other: &Vector) {
        for i in 0..self.dimension {
            self.components[i] -= other.components[i]
        }
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;
    fn mul(self, n: f64) -> Vector {
        let mut v = Vector::zero(self.dimension as u32);
        for i in 0..self.dimension {
            v.components[i] = self.components[i] * n;
        }
        v
    }
}
impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, n: f64) {
        for i in 0..self.dimension {
            self.components[i] *= n;
        }
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;
    fn div(self, n: f64) -> Vector {
        let mut v = Vector::zero(self.dimension as u32);
        for i in 0..self.dimension {
            v.components[i] = self.components[i] / n;
        }
        v
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, n: f64) {
        for i in 0..self.dimension {
            self.components[i] /= n;
        }
    }
}
