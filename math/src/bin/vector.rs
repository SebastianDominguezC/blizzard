extern crate math;
use math::vectors::Vector;

fn main() {
    let v1 = Vector::new(vec![1.0, 1.0]);
    let v2 = Vector::new(vec![1.0, 1.0]);
    let v3 = &v1 + &v2;
    println!("{:?}", v3);

    let dot = Vector::dot_product(&v1, &v3).unwrap();
    println!("{:?}", dot);

    let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
    let cross = Vector::cross_product(&v1, &v2).unwrap();
    println!("{:?}", cross);

    println!("{:?}", cross.normalize());
}
