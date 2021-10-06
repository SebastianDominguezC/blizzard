mod tests {
    use math::matrices::Matrix;

    #[test]
    fn create_matrix() {
        let r1 = vec![1.0, 2.0];
        let r2 = vec![1.0, 2.0];
        let m = vec![r1, r2];
        let _ = Matrix::new(m).unwrap();
    }
    #[test]
    #[should_panic]
    fn fail_create_matrix() {
        let r1 = vec![1.0, 2.0];
        let r2 = vec![1.0, 2.0, 3.0];
        let m = vec![r1, r2];
        let _ = Matrix::new(m).unwrap();
    }

    #[test]
    fn check_same_dimensions() {
        let r1 = vec![1.0, 2.0];
        let r2 = vec![1.0, 2.0];
        let m = vec![r1, r2];
        let m1 = Matrix::new(m).unwrap();

        let r1 = vec![1.0, 2.0];
        let r2 = vec![1.0, 2.0];
        let m = vec![r1, r2];
        let m2 = Matrix::new(m).unwrap();

        assert!(m1.compare_dimensions(&m2));
    }

    #[test]
    #[should_panic]
    fn fail_check_same_dimensions() {
        let r1 = vec![1.0, 2.0];
        let r2 = vec![1.0, 2.0];
        let m = vec![r1, r2];
        let m1 = Matrix::new(m).unwrap();

        let r1 = vec![1.0, 2.0, 3.0];
        let r2 = vec![1.0, 2.0, 3.0];
        let m = vec![r1, r2];
        let m2 = Matrix::new(m).unwrap();

        assert!(m1.compare_dimensions(&m2));
    }

    #[test]
    fn create_zero() {
        let m = Matrix::new_from(0.0, 2, 2);
        let mut zeros = true;
        for i in 0..m.rows {
            for j in 0..m.cols {
                zeros = zeros && m.items[i][j] == 0.0;
            }
        }
        assert!(zeros);
    }

    #[test]
    fn add() {
        let m1 = Matrix::new_from(2.0, 2, 3);
        let m2 = Matrix::new_from(1.0, 2, 3);
        let m3 = &m1 + &m2;

        let mut same = true;
        for i in 0..m1.rows {
            for j in 0..m1.cols {
                same = same && m3.items[i][j] == 3.0;
            }
        }
        assert!(same);
    }

    #[test]
    #[should_panic]
    fn fail_add() {
        let m1 = Matrix::new_from(2.0, 3, 3);
        let m2 = Matrix::new_from(1.0, 2, 2);
        let _ = &m1 + &m2;
    }

    #[test]
    fn sub() {
        let m1 = Matrix::new_from(2.0, 4, 2);
        let m2 = Matrix::new_from(1.0, 4, 2);
        let m3 = &m1 - &m2;

        let mut same = true;
        for i in 0..m1.rows {
            for j in 0..m1.cols {
                same = same && m3.items[i][j] == 1.0;
            }
        }
        assert!(same);
    }

    #[test]
    #[should_panic]
    fn fail_sub() {
        let m1 = Matrix::new_from(2.0, 3, 3);
        let m2 = Matrix::new_from(1.0, 2, 2);
        let _ = &m1 - &m2;
    }

    #[test]
    fn multiply_scalar() {
        let m1 = Matrix::new_from(2.0, 4, 2);
        let m3 = &m1 * -2.0;

        let mut same = true;
        for i in 0..m1.rows {
            for j in 0..m1.cols {
                same = same && m3.items[i][j] == -4.0;
            }
        }
        assert!(same);
    }

    #[test]
    fn divide_scalar() {
        let m1 = Matrix::new_from(2.0, 4, 2);
        let m3 = &m1 / -2.0;

        let mut same = true;
        for i in 0..m1.rows {
            for j in 0..m1.cols {
                same = same && m3.items[i][j] == -1.0;
            }
        }
        assert!(same);
    }

    #[test]
    fn multiply_square_matrix() {
        let m1 = Matrix::new_from(2.0, 2, 2);
        let m2 = Matrix::new_from(2.0, 2, 2);
        let m3 = &m1 * &m2;
        let mut same = true;
        for i in 0..m3.rows {
            for j in 0..m3.cols {
                same = same && m3.items[i][j] == 8.0;
            }
        }
        assert!(same);
    }
    #[test]
    fn multiply_matrix() {
        let m1 = Matrix::new_from(3.0, 2, 3);
        let m2 = Matrix::new_from(2.0, 3, 2);
        let m3 = &m1 * &m2;
        let mut same = true;
        for i in 0..m3.rows {
            for j in 0..m3.cols {
                same = same && m3.items[i][j] == 18.0;
            }
        }
        assert!(same);
    }

    // Only for f64
    #[test]
    fn minors() {
        let r1 = vec![1.0, -2.0, 1.0];
        let r2 = vec![6.0, 2.0, 3.0];
        let r3 = vec![2.0, 1.0, 4.0];
        let m: Matrix<f64> = Matrix::new(vec![r1, r2, r3]).unwrap();
        let minors = m.minors();
        println!("{:?}", minors);
        assert_eq!(5.0, minors.items[0][0]);
    }

    #[test]
    fn determinant() {
        let r1 = vec![1.0, -2.0, 1.0];
        let r2 = vec![6.0, 2.0, 3.0];
        let r3 = vec![2.0, 1.0, 4.0];
        let m: Matrix<f64> = Matrix::new(vec![r1, r2, r3]).unwrap();
        let d = m.determinant();
        assert_eq!(43.0, d);
    }

    #[test]
    fn determinant2() {
        let r1 = vec![2.0, 1.0];
        let r2 = vec![3.0, 7.0];
        let m: Matrix<f64> = Matrix::new(vec![r1, r2]).unwrap();
        let d = m.determinant();
        assert_eq!(11.0, d);
    }

    #[test]
    #[should_panic]
    fn fail_determinant() {
        let r1 = vec![2.0, 1.0, 3.0];
        let r2 = vec![3.0, 7.0, 3.0];
        let m: Matrix<f64> = Matrix::new(vec![r1, r2]).unwrap();
        let d = m.determinant();
        assert_eq!(11.0, d);
    }

    #[test]
    fn transpose() {
        let r1 = vec![1.0, 1.0, 1.0];
        let r2 = vec![2.0, 2.0, 2.0];
        let m: Matrix<f64> = Matrix::new(vec![r1, r2]).unwrap();
        let t = m.transpose();
        for i in 0..t.rows {
            assert_eq!(t.items[i], vec![1.0, 2.0]);
        }
    }

    #[test]
    fn identity() {
        let r1 = vec![1.0, 2.0];
        let r2 = vec![3.0, 4.0];
        let m: Matrix<f64> = Matrix::new(vec![r1, r2]).unwrap();
        let inv = m.inverse();
        let identity = &inv * &m;
        for i in 0..identity.rows {
            assert_eq!(identity.items[i][i], 1.0);
        }
    }
}
