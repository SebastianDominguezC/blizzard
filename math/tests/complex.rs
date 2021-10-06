mod tests {
    use math::complex::ComplexPoint as Cx;
    use math::complex::ComplexPolarPoint as Px;
    use std::f64::consts::FRAC_PI_4;

    #[test]
    fn create_point() {
        let z = Cx::new(1.0, 2.0);
        assert_eq!(z.x, 1.0);
        assert_eq!(z.y, 2.0);
    }

    #[test]
    fn create_real() {
        let z = Cx::new_real(1.0);
        assert_eq!(z.x, 1.0);
        assert_eq!(z.y, 0.0);
    }

    #[test]
    fn create_imaginary() {
        let z = Cx::new_imaginary(1.0);
        assert_eq!(z.x, 0.0);
        assert_eq!(z.y, 1.0);
    }

    #[test]
    fn create_conjugate() {
        let z = Cx::new(1.0, 1.0);
        let c = z.conjugate();
        assert_eq!(z.x, c.x);
        assert_eq!(z.y, -c.y);
    }

    #[test]
    fn module() {
        let z = Cx::new(2.0, 2.0);
        let m = (z.x * z.x + z.y * z.y).sqrt();
        assert_eq!(z.module(), m);
    }
    #[test]
    fn modules() {
        let z = Cx::new(2.0, 2.0);
        assert_eq!(z.module(), z.module_conjugate());
    }
    #[test]
    fn z_inverse() {
        let z = Cx::new(3.0, 5.0);
        let i = z.inverse();
        let r = z * i;
        assert_eq!(r.x.round(), 1.0);
        assert_eq!(r.y.round(), 0.0);
    }

    #[test]
    fn convert_polar() {
        let z = Cx::new(1.0, 1.0);
        let p = z.polar();
        let dif = (p.t - FRAC_PI_4).abs();
        assert_eq!(p.r, z.module());
        assert!(dif < 0.0000001);
    }

    #[test]
    fn convert_cartesian() {
        let p = Px::new(2.0_f64.sqrt(), FRAC_PI_4);
        let z = p.cartesian();
        let dif_x = (z.x - 1.0).abs();
        let dif_y = (z.y - 1.0).abs();
        assert_eq!(p.r, z.module());
        assert!(dif_x < 0.0000001);
        assert!(dif_y < 0.0000001);
    }
    #[test]
    fn polar_power() {
        let power = 2.0;
        let z = Px::new(1.0, FRAC_PI_4);
        let powered = z.pow(power);
        assert_eq!(z.r.powf(power), powered.r);
        assert_eq!(z.t * power, powered.t);
    }
}
