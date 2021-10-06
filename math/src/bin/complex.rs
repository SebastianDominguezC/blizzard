extern crate math;
use math::complex::ComplexPoint as Cx;

pub fn form_three(z: Cx) -> Cx {
    z * z
}

fn main() {
    let z = Cx::new(1.0, 1.0);
    z.print();
    println!("\n\n\n");

    println!("---- 3 ----");
    let z = Cx::new(1.0, 1.0);
    let z = form_three(z);
    z.print();
    z.polar().print();
    println!("\n\n\n");

    let z = z + Cx::new(1.0, 3.0);
    let z = z / Cx::new(0.0, 1.0);
    z.print();
    z.polar().print();
}
