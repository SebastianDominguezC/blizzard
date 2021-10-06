extern crate math;
use math::matrices::Matrix;

fn main() {
    // Inverse + Identity
    let matrix = Matrix::new(vec![
        vec![4.0, 2.0, 1.0],
        vec![4.0, 1.0, 4.0],
        vec![3.0, 2.0, 3.0],
    ])
    .unwrap();
    let inverse = matrix.inverse();

    let identity = &matrix * &inverse;

    println!("M: {:#?}", matrix.items);
    println!("M-1: {:#?}", inverse.items);
    println!("I: {:#?}", identity.items);

    // Multiplication + Addition
    let m1 = Matrix::new(vec![vec![1.0, 3.0, 5.0], vec![2.0, 4.0, 6.0]]).unwrap();
    let m2 = Matrix::new(vec![
        vec![1.0, 3.0, 5.0],
        vec![2.0, 4.0, 6.0],
        vec![2.0, 4.0, 6.0],
    ])
    .unwrap();

    let m = &m1 + &(&m1 * &m2);
    println!("{:?}", m);
    let m = m - m1;
    println!("{:?}", m);

    // Determinant
    let det = matrix.determinant();
    println!("{}", det);
}
