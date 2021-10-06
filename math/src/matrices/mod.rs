use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct Matrix<T: Add + Sub + Copy> {
    pub items: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T: Add + Sub + Mul + Div + Copy> Matrix<T> {
    pub fn new(vecs: Vec<Vec<T>>) -> Option<Matrix<T>> {
        let mut same_cols = true;
        let first_col_len = vecs[0].len();
        for i in 0..vecs.len() {
            same_cols = same_cols && vecs[i].len() == first_col_len;
            if !same_cols {
                return None;
            }
        }
        Some(Matrix {
            rows: vecs.len(),
            cols: first_col_len,
            items: vecs,
        })
    }
    pub fn new_from(x: T, m: i32, n: i32) -> Matrix<T> {
        let mut matrix = vec![];
        for _ in 0..m {
            let mut r = vec![];
            for _ in 0..n {
                r.push(x);
            }
            matrix.push(r);
        }
        Matrix::new(matrix).unwrap()
    }

    pub fn compare_dimensions(&self, other: &Matrix<T>) -> bool {
        let same_rows = self.rows == other.rows;
        let same_cols = self.cols == other.cols;
        same_cols && same_rows
    }

    pub fn transpose(&self) -> Matrix<T> {
        let mut t = vec![];
        // n == t rows
        for n in 0..self.cols {
            t.push(vec![]);

            // m == t cols
            for m in 0..self.rows {
                t[n].push(self.items[m][n]);
            }
        }
        Matrix::new(t).expect("Cannot transpose matrix")
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }
    fn panic_if_not_square(&self) {
        if !self.is_square() {
            panic!("Matrix must be square!");
        }
    }
}

impl Matrix<f64> {
    pub fn determinant_2(&self) -> f64 {
        let minors = self.minors();
        if minors.rows == 2 && minors.cols == 2 {
            return minors.items[0][0] * minors.items[1][1]
                - minors.items[0][1] * minors.items[1][0];
        } else {
            return minors.determinant_2();
        }
    }
    pub fn minor(&self) -> f64 {
        if self.rows == 1 && self.cols == 1 {
            return self.items[0][0];
        } else {
            return self.determinant_2();
        }
    }
    pub fn minors(&self) -> Matrix<f64> {
        let mut minors: Vec<Vec<f64>> = vec![];
        for a in 0..self.rows {
            let mut minor_row = vec![];
            // remove row
            let m: Vec<&Vec<f64>> = self
                .items
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != a)
                .map(|(_, row)| row)
                .collect();

            for b in 0..self.cols {
                // remove col
                let m: Matrix<f64> = Matrix::new(
                    m.iter()
                        .map(|row| {
                            row.iter()
                                .enumerate()
                                .filter(|(i, _)| *i != b)
                                .map(|(_, col)| *col)
                                .collect()
                        })
                        .collect(),
                )
                .expect("Cannot turn into matrix");
                minor_row.push(m.minor());
            }
            minors.push(minor_row);
        }
        Matrix::new(minors).unwrap()
    }

    pub fn cofactor(&self) -> Matrix<f64> {
        let mut m = vec![];
        for i in 0..self.rows {
            m.push(vec![]);
            for j in 0..self.cols {
                let item = self.items[i][j];
                let p = (i + 1) as i32 + (j + 1) as i32;
                let v = item * (-1.0_f64).powi(p);
                m[i].push(v);
            }
        }
        Matrix::new(m).unwrap()
    }

    pub fn determinant(&self) -> f64 {
        self.panic_if_not_square();
        let cofactor = self.minors().cofactor();
        let mut sum = 0.0;
        for j in 0..cofactor.cols {
            sum += self.items[0][j] * cofactor.items[0][j];
        }
        sum
    }

    pub fn adjoint(&self) -> Matrix<f64> {
        self.panic_if_not_square();
        self.minors().cofactor().transpose()
    }

    pub fn inverse(&self) -> Matrix<f64> {
        self.panic_if_not_square();
        let determinant = self.determinant();
        let adjoint = self.adjoint();
        adjoint / determinant
    }
}

// Matrix + Matrix
impl<T: Add<T, Output = T> + Sub + Mul + Div + Copy> Add<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, other: &Matrix<T>) -> Matrix<T> {
        let same_d = self.compare_dimensions(other);
        if !same_d {
            panic!("Can not add matrices with different dimensions");
        }
        let mut mat = vec![];
        for i in 0..self.rows {
            let mut r = vec![];
            for j in 0..self.items[i].len() {
                r.push(self.items[i][j] + other.items[i][j]);
            }
            mat.push(r);
        }
        Matrix::new(mat).unwrap()
    }
}

// Matrix - Matrix
impl<T: Sub<T, Output = T> + Add + Mul + Div + Copy> Sub<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, other: &Matrix<T>) -> Matrix<T> {
        let same_d = self.compare_dimensions(other);
        if !same_d {
            panic!("Can not add matrices with different dimensions");
        }
        let mut mat = vec![];
        for i in 0..self.rows {
            let mut r = vec![];
            for j in 0..self.items[i].len() {
                r.push(self.items[i][j] - other.items[i][j]);
            }
            mat.push(r);
        }
        Matrix::new(mat).unwrap()
    }
}

// Matrix * Scalar
impl<T: Mul<T, Output = T> + Add + Sub + Div + Copy> Mul<T> for &Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, n: T) -> Matrix<T> {
        let mut mat = vec![];
        for i in 0..self.rows {
            let mut r = vec![];
            for j in 0..self.items[i].len() {
                r.push(self.items[i][j] * n);
            }
            mat.push(r);
        }
        Matrix::new(mat).unwrap()
    }
}

// Matrix / Scalar
impl<T: Div<T, Output = T> + Add + Sub + Mul + Copy> Div<T> for &Matrix<T> {
    type Output = Matrix<T>;
    fn div(self, n: T) -> Matrix<T> {
        let mut mat = vec![];
        for i in 0..self.rows {
            let mut r = vec![];
            for j in 0..self.items[i].len() {
                r.push(self.items[i][j] / n);
            }
            mat.push(r);
        }
        Matrix::new(mat).unwrap()
    }
}

// Matrix * Matrix
impl<T: Mul<T, Output = T> + Add<T, Output = T> + Sub + Div + Copy> Mul<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, other: &Matrix<T>) -> Matrix<T> {
        if self.cols != other.rows {
            panic!("A's columns are not the same as B's rows")
        }

        let mut mat = vec![];

        for a in 0..self.rows {
            let mut r = vec![];

            for c in 0..other.cols {
                let mut dot: Option<T> = None;

                for b in 0..self.cols {
                    let a1 = self.items[a][b];
                    let b1 = other.items[b][c];
                    let mul = a1 * b1;

                    // MODIFY DOT
                    match dot {
                        Some(v) => dot = Some(v + mul),
                        None => dot = Some(mul),
                    }
                }
                r.push(dot.expect("There is no inner product"));
            }
            mat.push(r);
        }
        Matrix::new(mat).expect("Matrix dimensions are not correct")
    }
}

// f64 bindings
impl Add<Matrix<f64>> for Matrix<f64> {
    type Output = Matrix<f64>;
    fn add(self, other: Matrix<f64>) -> Matrix<f64> {
        let same_d = self.compare_dimensions(&other);
        if !same_d {
            panic!("Can not add matrices with different dimensions");
        }
        let mut mat = vec![];
        for i in 0..self.rows {
            let mut r = vec![];
            for j in 0..self.items[i].len() {
                r.push(self.items[i][j] + other.items[i][j]);
            }
            mat.push(r);
        }
        Matrix::new(mat).unwrap()
    }
}

// Matrix - Matrix
impl Sub<Matrix<f64>> for Matrix<f64> {
    type Output = Matrix<f64>;
    fn sub(self, other: Matrix<f64>) -> Matrix<f64> {
        let same_d = self.compare_dimensions(&other);
        if !same_d {
            panic!("Can not add matrices with different dimensions");
        }
        let mut mat = vec![];
        for i in 0..self.rows {
            let mut r = vec![];
            for j in 0..self.items[i].len() {
                r.push(self.items[i][j] - other.items[i][j]);
            }
            mat.push(r);
        }
        Matrix::new(mat).unwrap()
    }
}

// Matrix * Scalar
impl Mul<f64> for Matrix<f64> {
    type Output = Matrix<f64>;
    fn mul(self, n: f64) -> Matrix<f64> {
        let mut mat = vec![];
        for i in 0..self.rows {
            let mut r = vec![];
            for j in 0..self.items[i].len() {
                r.push(self.items[i][j] * n);
            }
            mat.push(r);
        }
        Matrix::new(mat).unwrap()
    }
}

// Matrix / Scalar
impl Div<f64> for Matrix<f64> {
    type Output = Matrix<f64>;
    fn div(self, n: f64) -> Matrix<f64> {
        let mut mat = vec![];
        for i in 0..self.rows {
            let mut r = vec![];
            for j in 0..self.items[i].len() {
                r.push(self.items[i][j] / n);
            }
            mat.push(r);
        }
        Matrix::new(mat).unwrap()
    }
}

// Matrix * Matrix
impl Mul for Matrix<f64> {
    type Output = Matrix<f64>;
    fn mul(self, other: Matrix<f64>) -> Matrix<f64> {
        if self.cols != other.rows {
            panic!("A's columns are not the same as B's rows")
        }

        let mut mat = vec![];

        for a in 0..self.rows {
            let mut r = vec![];

            for c in 0..other.cols {
                let mut dot: Option<f64> = None;

                for b in 0..self.cols {
                    let a1 = self.items[a][b];
                    let b1 = other.items[b][c];
                    let mul = a1 * b1;

                    // MODIFY DOT
                    match dot {
                        Some(v) => dot = Some(v + mul),
                        None => dot = Some(mul),
                    }
                }
                r.push(dot.expect("There is no inner product"));
            }
            mat.push(r);
        }
        Matrix::new(mat).expect("Matrix dimensions are not correct")
    }
}
