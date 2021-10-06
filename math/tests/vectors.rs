mod tests {
    use math::vectors::Vector;

    // Properties
    #[test]
    fn zero() {
        let vec = Vector::zero(5);
        assert_eq!(5, vec.dimension);
    }

    #[test]
    fn dot_product() {
        let vec1 = Vector::new(vec![1.0, 1.0, 1.0]);
        let vec2 = Vector::new(vec![2.0, 2.0, 2.0]);
        let dot_product = Vector::dot_product(&vec1, &vec2).unwrap();
        assert_eq!(6.0, dot_product);
    }

    #[test]
    #[should_panic]
    fn fail_dot_product() {
        let vec1 = Vector::new(vec![1.0, 1.0, 1.0]);
        let vec2 = Vector::new(vec![2.0, 2.0, 2.0, 2.0]);
        let dot_product = Vector::dot_product(&vec1, &vec2).unwrap();
        assert_eq!(6.0, dot_product);
    }

    #[test]
    fn cross_prodct() {
        let vec1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let vec2 = Vector::new(vec![3.0, 2.0, 1.0]);

        if let Some(crossed_vec) = Vector::cross_product(&vec1, &vec2) {
            println!("{:?}", crossed_vec);
            assert_eq!(-4.0, crossed_vec.components[0]);
        }
    }

    #[test]
    #[should_panic]
    fn fail_cross_product() {
        let vec1 = Vector::new(vec![1.0, 2.0, 3.0, 2.0]);
        let vec2 = Vector::new(vec![3.0, 2.0, 1.0, 2.0]);
        Vector::cross_product(&vec1, &vec2).unwrap();
    }

    #[test]
    fn equal_dimensions() {
        let vec1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let vec2 = Vector::new(vec![3.0, 2.0, 1.0]);
        assert!(Vector::equal_dimensions(&vec1, &vec2));
    }

    #[test]
    fn not_equal_dimensions() {
        let vec1 = Vector::new(vec![1.0, 2.0]);
        let vec2 = Vector::new(vec![3.0, 2.0, 1.0]);
        assert!(!Vector::equal_dimensions(&vec1, &vec2));
    }

    #[test]
    fn magnitude() {
        let vec1 = Vector::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(vec1.get_magnitude(), (14.0_f64).sqrt());
    }

    #[test]
    fn normalized() {
        let vec1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let vec1 = vec1.normalize();
        assert_eq!(vec1.get_magnitude(), 1.0);
    }

    // Operator overrides
    #[test]
    fn add() {
        let vec1 = Vector::new(vec![1.0, 2.0]);
        let vec2 = Vector::new(vec![3.0, 2.0]);
        let vec3 = &vec1 + &vec2;
        assert!(vec3.components[0] == 4.0 && vec3.components[1] == 4.0);
    }
    #[test]
    fn add_assgin() {
        let mut vec1 = Vector::new(vec![1.0, 2.0]);
        vec1 += &Vector::new(vec![3.0, 2.0]);
        assert!(vec1.components[0] == 4.0 && vec1.components[1] == 4.0);
    }
    #[test]
    fn sub() {
        let vec1 = Vector::new(vec![1.0, 2.0]);
        let vec2 = Vector::new(vec![3.0, 2.0]);
        let vec3 = &vec1 - &vec2;
        assert!(vec3.components[0] == -2.0 && vec3.components[1] == 0.0);
    }
    #[test]
    fn sub_assgin() {
        let mut vec1 = Vector::new(vec![1.0, 2.0]);
        vec1 -= &Vector::new(vec![3.0, 2.0]);
        assert!(vec1.components[0] == -2.0 && vec1.components[1] == 0.0);
    }
    #[test]
    fn mul() {
        let vec1 = Vector::new(vec![1.0, 2.0]);
        let vec2 = &vec1 * 2.0;
        assert!(vec2.components[0] == 2.0 && vec2.components[1] == 4.0);
    }
    #[test]
    fn mul_assgin() {
        let mut vec1 = Vector::new(vec![1.0, 2.0]);
        vec1 *= 2.0;
        assert!(vec1.components[0] == 2.0 && vec1.components[1] == 4.0);
    }
    #[test]
    fn div() {
        let vec1 = Vector::new(vec![1.0, 2.0]);
        let vec2 = &vec1 / -2.0;
        assert!(vec2.components[0] == -0.5 && vec2.components[1] == -1.0);
    }
    #[test]
    fn div_assgin() {
        let mut vec1 = Vector::new(vec![1.0, 2.0]);
        vec1 /= -2.0;
        assert!(vec1.components[0] == -0.5 && vec1.components[1] == -1.0);
    }
}
